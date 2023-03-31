use crate::{
    data::{Address, Context},
    ext::gimli::{ArangeEntry, DebuggingInformationEntry},
    Error,
};
use fallible_iterator::FallibleIterator;
use gimli::{DW_TAG_inlined_subroutine, DW_TAG_subprogram};
use gimli::{DebugInfoOffset, Dwarf, EndianSlice, RunTimeEndian};

pub trait Lookup {
    fn lookup(&self, vmaddr: Address, context: Context) -> Result<Vec<String>, Error>;
    fn lookup_address(&self, address: Address, inline: bool) -> Result<String, Error>;

    fn debug_info_offset_from_addr(&self, addr: Address) -> Result<DebugInfoOffset, Error>;
}

impl<'data> Lookup for Dwarf<EndianSlice<'_, RunTimeEndian>> {
    fn lookup(&self, vmaddr: Address, context: Context) -> Result<Vec<String>, Error> {
        fallible_iterator::convert(context.addresses.into_iter().map(|addr| {
            self.lookup_address(addr - context.load_address + vmaddr, context.expand_inline)
        }))
        .collect()
    }

    fn lookup_address(&self, addr: Address, inline: bool) -> Result<String, Error> {
        let offset = self.debug_info_offset_from_addr(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        let unit = self.unit(header)?;
        let mut entries = unit.entries();

        loop {
            let Some(entry) = entries.next_entry()?.and_then(|_| entries.current()) else {
                break Err(Error::AddressNotFound)
            };

            match entry.pc() {
                Some(pc) if entry.tag() != DW_TAG_subprogram && pc.contains(&addr) => {
                    break entry
                        .name()
                        .ok_or(Error::AddressNotSymbol)
                        .and_then(|name| self.attr_string(&unit, name).map_err(Error::Gimli))
                        .map(|symbol| symbol.to_string_lossy().to_string());
                }
                _ => continue,
            }
        }
    }

    fn debug_info_offset_from_addr(&self, addr: Address) -> Result<DebugInfoOffset, Error> {
        self.debug_aranges
            .headers()
            .find_map(|header| {
                Ok(if header.entries().any(|entry| entry.contains(addr))? {
                    Some(header.debug_info_offset())
                } else {
                    None
                })
            })?
            .ok_or(Error::AddressNotFound)
    }
}
