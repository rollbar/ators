use crate::{
    data::{Address, Context},
    Error,
};
use fallible_iterator::{convert, FallibleIterator};
use gimli::{DW_TAG_subprogram, DebugInfoOffset, Dwarf, EndianSlice, RunTimeEndian};
use object::{Object, ObjectSegment};

pub trait Lookup {
    fn lookup(&self, object: object::File, context: Context) -> Result<Vec<String>, Error>;

    fn lookup_address(&self, address: Address) -> Result<String, Error>;

    fn debug_info_offset_from_addr(&self, addr: Address) -> Result<DebugInfoOffset, Error>;
}

impl<'data> Lookup for Dwarf<EndianSlice<'_, RunTimeEndian>> {
    fn lookup(&self, object: object::File, context: Context) -> Result<Vec<String>, Error> {
        let vmaddr = object.vmaddr()?;

        fallible_iterator::convert(
            context
                .addresses
                .into_iter()
                .map(|addr| self.lookup_address(addr - context.load_address + vmaddr)),
        )
        .collect()
    }

    fn lookup_address(&self, addr: Address) -> Result<String, Error> {
        let offset = self.debug_info_offset_from_addr(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        let unit = self.unit(header)?;
        let mut entries = unit.entries();

        while entries.next_entry()?.is_some() {
            let entry = entries.current().unwrap();
            if entry.tag() != DW_TAG_subprogram {}
        }

        unimplemented!()
    }

    fn debug_info_offset_from_addr(&self, addr: Address) -> Result<DebugInfoOffset, Error> {
        self.debug_aranges
            .headers()
            .find_map(|header| {
                if header.entries().any(|entry| entry.contains(addr))? {
                    Ok(Some(header.debug_info_offset()))
                } else {
                    Ok(None)
                }
            })?
            .ok_or(Error::AddressNotFound)
    }
}

pub trait ContainsAddress {
    fn contains(&self, addr: Address) -> Result<bool, gimli::Error>;
}

impl ContainsAddress for gimli::ArangeEntry {
    fn contains(&self, addr: Address) -> Result<bool, gimli::Error> {
        let range = (
            self.address(),
            self.address()
                .checked_add(self.length())
                .ok_or(gimli::Error::InvalidAddressRange)?,
        );

        Ok(addr >= range.0 && addr < range.1)
    }
}

trait VmAddr {
    fn vmaddr(&self) -> Result<Address, Error>;
}

impl VmAddr for object::File<'_> {
    fn vmaddr(&self) -> Result<Address, Error> {
        self.segments()
            .find_map(|seg| match seg.name().ok().flatten() {
                Some(name) if name == "__TEXT" => Some(seg.address()),
                _ => None,
            })
            .ok_or(Error::TextSegmentNotFound)
            .map(Address::new)
    }
}
