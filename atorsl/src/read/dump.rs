use crate::read::format;
use anyhow::Result;
use fallible_iterator::{convert, FallibleIterator};
use object::Object;

use super::Dwarf;

pub trait Dump {
    fn dump(&self) -> Result<Vec<String>>;
}

impl Dump for Dwarf<'_> {
    fn dump(&self) -> Result<Vec<String>> {
        // let cow;
        // let dwarf = load_dwarf!(self, cow);
        let lines = self
            .units()
            .map(|header| Ok((header, self.unit(header)?)))
            .flat_map(|(header, unit)| {
                //dwarf.unit(header)?.entries_tree(Some(UnitOffset(0)))
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
    fn dump(&self) -> Result<Vec<String>> {
        Ok(convert(self.sections().map(|s| format::section(&s))).collect()?)
    }
}
