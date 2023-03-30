use crate::control::format;
use crate::data::Endian;
use anyhow::Result;
use fallible_iterator::{convert, FallibleIterator};
use gimli::{Dwarf, EndianSlice};
use object::{Object, ObjectSection};
use std::borrow::Cow;

pub trait Dump {
    fn dump(&self) -> Result<Vec<String>>;
    fn dump_sections(&self) -> Result<Vec<String>>;
}

impl<'data> Dump for object::File<'data> {
    fn dump(&self) -> Result<Vec<String>> {
        let dwarf = Dwarf::load(|section_id| -> Result<Cow<[u8]>> {
            Ok(self
                .section_by_name(section_id.name())
                .and_then(|section| section.uncompressed_data().ok())
                .unwrap_or(Cow::Borrowed(&[][..])))
        })?;

        let dwarf = dwarf.borrow(|section| EndianSlice::new(&*section, self.runtime_endian()));

        let lines = dwarf
            .units()
            .map(|header| Ok((header, dwarf.unit(header)?)))
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

    fn dump_sections(&self) -> Result<Vec<String>> {
        Ok(convert(self.sections().map(|s| format::section(&s))).collect()?)
    }
}
