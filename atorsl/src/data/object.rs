use object::Object;

pub trait ObjectExt {
    fn is_dwarf(&self) -> bool;

    fn runtime_endian(&self) -> gimli::RunTimeEndian;
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
}
