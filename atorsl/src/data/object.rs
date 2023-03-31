use super::Address;
use object::{Object, ObjectSegment};

pub trait ObjectExt {
    fn is_dwarf(&self) -> bool;

    fn runtime_endian(&self) -> gimli::RunTimeEndian;

    fn vmaddr(&self) -> Result<Address, crate::Error>;
}

impl ObjectExt for object::File<'_> {
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
