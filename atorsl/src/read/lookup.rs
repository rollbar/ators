use crate::{data::Context, Error};
use gimli::{Dwarf, EndianSlice, RunTimeEndian};
use object::{Object, ObjectSegment};

pub trait Lookup {
    fn lookup(&self, object: object::File, context: Context) -> Result<Vec<String>, Error>;
}

impl<'data> Lookup for Dwarf<EndianSlice<'_, RunTimeEndian>> {
    fn lookup<'a>(&'a self, object: object::File, _: Context) -> Result<Vec<String>, Error> {
        let vmaddr = object
            .segments()
            .find_map(|seg| match seg.name().ok().flatten() {
                Some(name) if name == "__TEXT" => Some(seg.address()),
                _ => None,
            })
            .ok_or(Error::TextSegmentNotFound)?;

        unimplemented!()
    }
}
