use crate::{
    data::{Address, Context},
    ext::gimli::{ArangeEntry, DebuggingInformationEntry},
    format, Error,
};
use fallible_iterator::FallibleIterator;
use gimli::{AttributeValue, DW_TAG_subprogram};
use gimli::{DebugInfoOffset, Dwarf, EndianSlice, RunTimeEndian};

type Unit<'input> = gimli::Unit<EndianSlice<'input, RunTimeEndian>, usize>;
type UnitHeader<'input> = gimli::UnitHeader<EndianSlice<'input, RunTimeEndian>, usize>;
type Entry<'abbrev, 'unit, 'input> =
    gimli::DebuggingInformationEntry<'abbrev, 'unit, EndianSlice<'input, RunTimeEndian>, usize>;
type AttrValue<'input> = AttributeValue<EndianSlice<'input, RunTimeEndian>>;

pub trait Lookup {
    fn lookup(&self, vmaddr: Address, context: &Context) -> Result<Vec<String>, Error>;
    fn lookup_addr(&self, address: Address, context: &Context) -> Result<String, Error>;

    fn symbol(&self, value: AttrValue, unit: &Unit) -> Result<String, Error>;
    fn unit_from_addr(&self, addr: Address) -> Result<(UnitHeader, Unit), Error>;
    fn debug_info_offset_from_addr(&self, addr: Address) -> Result<DebugInfoOffset, Error>;
}

impl<'data> Lookup for Dwarf<EndianSlice<'_, RunTimeEndian>> {
    fn lookup(&self, vmaddr: Address, context: &Context) -> Result<Vec<String>, Error> {
        fallible_iterator::convert(
            context
                .addrs
                .to_owned()
                .into_iter()
                .map(|addr| self.lookup_addr(addr - context.loadaddr + vmaddr, &context)),
        )
        .collect()
    }

    fn lookup_addr(&self, addr: Address, context: &Context) -> Result<String, Error> {
        let (header, unit) = self.unit_from_addr(addr)?;
        let mut entries = unit.entries();

        loop {
            let Some((_, entry)) = entries.next_dfs()? else {
                break Err(Error::AddressNotFound(addr))
            };

            if context.verbose {
                println!("{}", format::entry(entry, self, &header, &unit));
            }

            match entry.pc() {
                Some(pc) if entry.tag() == DW_TAG_subprogram && pc.contains(&addr) => {
                    break entry
                        .linkage_name()
                        .ok_or(Error::EntryHasNoLinkageName)
                        .and_then(|val| self.symbol(val, &unit))
                }
                _ => continue,
            }
        }
    }

    fn symbol(&self, value: AttrValue, unit: &Unit) -> Result<String, Error> {
        Ok(self
            .attr_string(&unit, value)?
            .to_string_lossy()
            .to_string())
    }

    fn unit_from_addr(&self, addr: Address) -> Result<(UnitHeader, Unit), Error> {
        let offset = self.debug_info_offset_from_addr(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        Ok((header, self.unit(header)?))
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
            .ok_or(Error::NoDebugOffsetInAddress(addr))
    }
}
