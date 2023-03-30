use super::{Attr, Entry, Header, Unit};
use anyhow::Result;
use object::{ObjectSection, Section};

pub fn section(section: &Section) -> Result<String> {
    Ok(format!(
        "{:#018x}:  {:#8}  {:#12}  {:#18}  ({:?})",
        section.address(),
        section.size(),
        section.segment_name()?.unwrap_or("None"),
        section.name()?,
        section.kind(),
    ))
}

pub fn header(header: &Header) -> String {
    format!(
        "Unit at <.debug_info+{:#018x}>",
        header.offset().as_debug_info_offset().unwrap().0
    )
}

pub fn entry(depth: isize, entry: &Entry) -> String {
    format!("<{}><{:#018x}> {}", depth, entry.offset().0, entry.tag())
}

pub fn attr(attr: &Attr) -> String {
    format!("\t{}: {:?}", attr.name(), attr.value())
}

#[allow(dead_code)]
pub fn unit(_: &Unit) -> String {
    String::default()
}
