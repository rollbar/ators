use gimli::{EndianSlice, RunTimeEndian};
use object::ObjectSection;

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

pub fn entry<'a>(
    depth: isize,
    entry: &gimli::DebuggingInformationEntry<'a, 'a, EndianSlice<'_, RunTimeEndian>, usize>,
) -> String {
    format!("<{}><{:#018x}> {}", depth, entry.offset().0, entry.tag())
}

pub fn attr(attr: &gimli::Attribute<EndianSlice<'_, RunTimeEndian>>) -> String {
    format!("\t{}: {:?}", attr.name(), attr.value())
}
