use crate::data::{self, endian::Endian};
use anyhow::Result;
use fallible_iterator::{convert, FallibleIterator};
//use gimli::{DW_TAG_inlined_subroutine, DW_TAG_subprogram, UnitOffset};
use gimli::{Attribute, DebuggingInformationEntry, Dwarf, EndianSlice, RunTimeEndian, UnitHeader};
use memmap2::Mmap;
use object::{Object, ObjectSection};
use std::{borrow::Cow, fs};

type Header<'a> = UnitHeader<EndianSlice<'a, RunTimeEndian>, usize>;
type Entry<'a> = DebuggingInformationEntry<'a, 'a, EndianSlice<'a, RunTimeEndian>, usize>;
type Attr<'a> = Attribute<EndianSlice<'a, RunTimeEndian>>;
type Unit<'a> = gimli::Unit<EndianSlice<'a, RunTimeEndian>, usize>;

pub fn lookup(options: data::Options) -> Result<Vec<String>> {
    let mmap = unsafe { Mmap::map(&fs::File::open(options.object)?) }?;
    dump_file(&object::File::parse(&*mmap)?)
}

fn format_header(header: &Header) -> String {
    format!(
        "Unit at <.debug_info+{:#018x}>",
        header.offset().as_debug_info_offset().unwrap().0
    )
}

fn format_entry(depth: isize, entry: &Entry) -> String {
    format!("<{}><{:#018x}> {}", depth, entry.offset().0, entry.tag())
}

fn format_attr(attr: &Attr) -> String {
    format!("\t{}: {:?}", attr.name(), attr.value())
}

#[allow(dead_code)]
fn format_unit(_: &Unit) -> String {
    String::default()
}

fn dump_file(object: &object::File) -> Result<Vec<String>> {
    let dwarf = Dwarf::load(|section_id| -> Result<Cow<[u8]>> {
        Ok(object
            .section_by_name(section_id.name())
            .and_then(|section| section.uncompressed_data().ok())
            .unwrap_or(Cow::Borrowed(&[][..])))
    })?;

    let dwarf = dwarf.borrow(|section| EndianSlice::new(&*section, object.runtime_endian()));

    Ok(dwarf
        .units()
        .map(|header| Ok((header, dwarf.unit(header)?)))
        .flat_map(|(header, unit)| {
            //dwarf.unit(header)?.entries_tree(Some(UnitOffset(0)))
            let mut lines = vec![format_header(&header)];
            let mut depth = 0;
            let mut entries = unit.entries();
            while let Some((delta_depth, entry)) = entries.next_dfs()? {
                depth += delta_depth;
                lines.push(format_entry(depth, entry));
                lines.append(
                    &mut entry
                        .attrs()
                        .map(|attr| Ok(format_attr(&attr)))
                        .collect()?,
                );
            }

            Ok(convert(lines.into_iter().map(Ok)))
        })
        .collect()?)
}
