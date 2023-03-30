use object::Object;

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
