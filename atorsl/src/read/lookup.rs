use crate::data::Context;
use crate::data::ObjectExt;
use crate::load_dwarf;
use anyhow::Result;
use fallible_iterator::FallibleIterator;
use object::{Object, ObjectSection};

pub trait Lookup {
    fn lookup(&self, context: Context) -> Result<Vec<String>>;
}

impl<'data> Lookup for object::File<'data> {
    fn lookup<'a>(&'a self, _: Context) -> Result<Vec<String>> {
        let cow;
        let dwarf = load_dwarf!(self, cow);

        let _ = dwarf
            .units()
            .map(|header| Ok((header, dwarf.unit(header)?)));

        unimplemented!()
    }
}
