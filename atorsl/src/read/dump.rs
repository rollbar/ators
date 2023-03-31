use crate::format;
use fallible_iterator::{convert, FallibleIterator};
use gimli::{Dwarf, EndianSlice, RunTimeEndian};
use object::Object;

pub trait Dump {
    fn dump(&self) -> Result<Vec<String>, crate::Error>;
}

impl Dump for Dwarf<EndianSlice<'_, RunTimeEndian>> {
    fn dump(&self) -> Result<Vec<String>, crate::Error> {
        let lines = self
            .units()
            .map(|header| Ok((header, self.unit(header)?)))
            .flat_map(|(header, unit)| {
                let mut lines = vec![format::header(&header)];
                let mut depth = 0;
                let mut entries = unit.entries();
                while let Some((delta_depth, entry)) = entries.next_dfs()? {
                    depth += delta_depth;
                    lines.push(format::entry(depth, entry));
                    lines.append(
                        &mut entry
                            .attrs()
                            .map(|attr| Ok(format::attr(&attr)))
                            .collect()?,
                    );
                }

                Ok(convert(lines.into_iter().map(Ok)))
            })
            .collect()?;

        Ok(lines)
    }
}

impl Dump for object::File<'_> {
    fn dump(&self) -> Result<Vec<String>, crate::Error> {
        Ok(convert(self.sections().map(|s| format::section(&s))).collect()?)
    }
}
