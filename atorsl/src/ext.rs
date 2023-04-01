pub mod object {
    use crate::{Addr, Error};
    use object::{Object, ObjectSegment};

    pub trait File {
        fn is_dwarf(&self) -> bool;
        fn runtime_endian(&self) -> gimli::RunTimeEndian;
        fn vmaddr(&self) -> Result<Addr, Error>;
    }

    impl File for object::File<'_> {
        fn is_dwarf(&self) -> bool {
            self.section_by_name("__debug_line").is_some()
        }

        fn runtime_endian(&self) -> gimli::RunTimeEndian {
            if self.is_little_endian() {
                gimli::RunTimeEndian::Little
            } else {
                gimli::RunTimeEndian::Big
            }
        }

        fn vmaddr(&self) -> Result<Addr, Error> {
            self.segments()
                .find_map(|seg| match seg.name().ok().flatten() {
                    Some(name) if name == "__TEXT" => Some(seg.address()),
                    _ => None,
                })
                .ok_or(Error::VmAddrTextSegmentNotFound)
                .map(Addr::from)
        }
    }
}

pub mod gimli {
    use std::ops::Range;

    use crate::Addr;
    use gimli::{AttributeValue, EndianSlice, RunTimeEndian};

    pub trait Dwarf {
        fn try_attr_string(
            &self,
            unit: &gimli::Unit<EndianSlice<RunTimeEndian>, usize>,
            value: AttributeValue<EndianSlice<RunTimeEndian>>,
        ) -> Option<String>;
    }

    impl Dwarf for gimli::Dwarf<EndianSlice<'_, RunTimeEndian>> {
        fn try_attr_string(
            &self,
            unit: &gimli::Unit<EndianSlice<RunTimeEndian>, usize>,
            value: AttributeValue<EndianSlice<RunTimeEndian>>,
        ) -> Option<String> {
            self.attr_string(&unit, value)
                .ok()
                .map(|slice| slice.to_string_lossy().to_string())
        }
    }

    pub trait DebuggingInformationEntry {
        fn name(&self) -> Option<AttributeValue<EndianSlice<RunTimeEndian>>>;
        fn linkage_name(&self) -> Option<AttributeValue<EndianSlice<RunTimeEndian>>>;
        fn pc(&self) -> Option<Range<Addr>>;
    }

    impl DebuggingInformationEntry
        for gimli::DebuggingInformationEntry<'_, '_, EndianSlice<'_, RunTimeEndian>, usize>
    {
        fn name(&self) -> Option<AttributeValue<EndianSlice<RunTimeEndian>>> {
            self.attr_value(gimli::DW_AT_name).ok().flatten()
        }

        fn linkage_name(&self) -> Option<AttributeValue<EndianSlice<RunTimeEndian>>> {
            self.attr_value(gimli::DW_AT_linkage_name)
                .ok()
                .flatten()
        }

        fn pc(&self) -> Option<Range<Addr>> {
            let low = match self.attr_value(gimli::DW_AT_low_pc).ok().flatten() {
                Some(AttributeValue::Addr(addr)) => Some(addr.into()),
                _ => None,
            };

            let high = match self.attr_value(gimli::DW_AT_high_pc).ok().flatten() {
                Some(AttributeValue::Addr(addr)) => Some(addr.into()),
                Some(AttributeValue::Udata(len)) if low.is_some() => Some(low.unwrap() + len),
                _ => None,
            };

            low.zip(high).map(|pc| pc.0..pc.1)
        }
    }

    pub trait ArangeEntry {
        fn contains(&self, addr: Addr) -> Result<bool, gimli::Error>;
    }

    impl ArangeEntry for gimli::ArangeEntry {
        fn contains(&self, addr: Addr) -> Result<bool, gimli::Error> {
            let range = (
                self.address(),
                self.address()
                    .checked_add(self.length())
                    .ok_or(gimli::Error::InvalidAddressRange)?,
            );

            Ok(addr >= range.0 && addr < range.1)
        }
    }
}
