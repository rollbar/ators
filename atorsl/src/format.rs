use gimli::{EndianSlice, RunTimeEndian};
use object::ObjectSection;

use crate::{
    data::Address,
    ext::gimli::{DebuggingInformationEntry, Dwarf},
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

pub fn header(header: &gimli::UnitHeader<EndianSlice<RunTimeEndian>, usize>) -> String {
    format!(
        "Unit at <.debug_info+{:#018x}>",
        header.offset().as_debug_info_offset().unwrap().0
    )
}

pub fn entry(
    entry: &gimli::DebuggingInformationEntry<'_, '_, EndianSlice<'_, RunTimeEndian>, usize>,
    dwarf: &gimli::Dwarf<EndianSlice<'_, RunTimeEndian>>,
    header: &gimli::UnitHeader<EndianSlice<RunTimeEndian>, usize>,
    unit: &gimli::Unit<EndianSlice<RunTimeEndian>, usize>,
) -> String {
    format!(
        "{:#010x}:  {:#45?}  {:#26}: {:?} : {:?}",
        entry.offset().to_debug_info_offset(&header).unwrap().0,
        entry.pc().unwrap_or(Address::nil()..Address::nil()),
        entry.tag(),
        entry
            .name()
            .and_then(|value| dwarf.try_attr_string(&unit, value)),
        entry
            .linkage_name()
            .and_then(|value| dwarf.try_attr_string(&unit, value))
    )
}

pub fn attr(attr: &gimli::Attribute<EndianSlice<'_, RunTimeEndian>>) -> String {
    format!("\t{}: {:?}", attr.name(), attr.value())
}
