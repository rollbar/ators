pub mod object {
    use crate::{Addr, Error};
    use object::{Object, ObjectSegment};

    pub trait File {
        fn vmaddr(&self) -> Result<Addr, Error>;
    }

    impl File for object::File<'_> {
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

    use crate::{Addr, AttrValue, Unit};
    use gimli::{AttributeValue, EndianSlice, RunTimeEndian};

    pub(crate) trait Dwarf {
        fn try_attr_string(&self, unit: &Unit, value: AttrValue) -> Option<String>;
    }

    impl Dwarf for gimli::Dwarf<EndianSlice<'_, RunTimeEndian>> {
        fn try_attr_string(&self, unit: &Unit, value: AttrValue) -> Option<String> {
            self.attr_string(&unit, value)
                .ok()
                .map(|slice| slice.to_string_lossy().to_string())
        }
    }

    pub(crate) trait DebuggingInformationEntry {
        fn symbol(&self) -> Option<AttrValue>;
        fn pc(&self) -> Option<Range<Addr>>;
    }

    impl DebuggingInformationEntry
        for gimli::DebuggingInformationEntry<'_, '_, EndianSlice<'_, RunTimeEndian>, usize>
    {
        fn symbol(&self) -> Option<AttrValue> {
            [
                gimli::DW_AT_linkage_name,
                gimli::DW_AT_abstract_origin,
                gimli::DW_AT_name,
            ]
            .into_iter()
            .find_map(|dw_at| self.attr_value(dw_at).ok().flatten())
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

    pub(crate) trait ArangeEntry {
        fn contains(&self, addr: Addr) -> Result<bool, gimli::Error>;
    }

    impl ArangeEntry for gimli::ArangeEntry {
        fn contains(&self, addr: Addr) -> Result<bool, gimli::Error> {
            self.address()
                .checked_add(self.length())
                .map(|address_end| (self.address()..address_end).contains(&addr))
                .ok_or(gimli::Error::InvalidAddressRange)
        }
    }
}
