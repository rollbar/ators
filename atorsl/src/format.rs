#![allow(dead_code)]

use gimli::{EndianSlice, RunTimeEndian};
use object::ObjectSection;

use crate::{
    ext::{self, gimli::DebuggingInformationEntry},
    Addr, Dwarf, Entry, Unit, UnitHeader,
};

pub fn section(section: &object::Section) -> object::read::Result<String> {
    Ok(format!(
        "{:#018x}:  {:#8}  {:#12}  {:#18}  ({:?})",
        section.address(),
        section.size(),
        section.segment_name()?.unwrap_or("None"),
        section.name()?,
        section.kind(),
    ))
}

pub fn header(header: &UnitHeader) -> String {
    format!(
        "Unit at <.debug_info+{:#018x}>",
        header.offset().as_debug_info_offset().unwrap().0
    )
}

pub fn entry(entry: &Entry, dwarf: &Dwarf, header: &UnitHeader, unit: &Unit) -> String {
    format!(
        "│ {:#010x} │ {:^#39.39} │ {:#25} │ {:#80.80} │",
        entry.offset().to_debug_info_offset(&header).unwrap().0,
        format!("{:?}", entry.pc().unwrap_or(Addr::nil()..Addr::nil())),
        entry.tag(),
        entry
            .symbol()
            .and_then(|v| <Dwarf as ext::gimli::Dwarf>::try_attr_string(dwarf, &unit, v))
            .unwrap_or_else(String::default),
    )
}

pub fn attr(attr: &gimli::Attribute<EndianSlice<'_, RunTimeEndian>>) -> String {
    format!("\t{}: {:?}", attr.name(), attr.value())
}
