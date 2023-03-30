use super::{Attr, Entry, Header, Unit};

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