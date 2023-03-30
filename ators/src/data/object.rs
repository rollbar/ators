use object::Object;

pub trait Dwarf {
    fn is_dwarf(&self) -> bool;
}

impl Dwarf for object::File<'_> {
    fn is_dwarf(&self) -> bool {
        self.section_by_name("__debug_line").is_some()
    }
}

pub trait Endian {
    fn runtime_endian(&self) -> gimli::RunTimeEndian;
}

impl Endian for object::File<'_> {
    fn runtime_endian(&self) -> gimli::RunTimeEndian {
        if self.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        }
    }
}
