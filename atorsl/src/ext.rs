pub mod object {
    use crate::data::Address;
    use object::{Object, ObjectSegment};

    pub trait File {
        fn is_dwarf(&self) -> bool;

        fn runtime_endian(&self) -> gimli::RunTimeEndian;

        fn vmaddr(&self) -> Result<Address, crate::Error>;
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

        fn vmaddr(&self) -> Result<Address, crate::Error> {
            self.segments()
                .find_map(|seg| match seg.name().ok().flatten() {
                    Some(name) if name == "__TEXT" => Some(seg.address()),
                    _ => None,
                })
                .ok_or(crate::Error::TextSegmentNotFound)
                .map(Address::new)
        }
    }
}

pub mod gimli {
    use crate::data::Address;

    pub trait ArangeEntry {
        fn contains(&self, addr: Address) -> Result<bool, gimli::Error>;
    }

    impl ArangeEntry for gimli::ArangeEntry {
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
}
