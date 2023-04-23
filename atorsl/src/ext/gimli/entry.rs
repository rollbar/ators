use crate::{data::*, *};
use gimli::{DW_AT_artificial, DW_AT_high_pc, DW_AT_low_pc, EndianSlice, RunTimeEndian};
use std::ops::Range;

pub(crate) trait DebuggingInformationEntry {
    fn pc(&self) -> Option<Range<Addr>>;

    fn is_artificial(&self) -> Option<bool>;
}

impl DebuggingInformationEntry
    for gimli::DebuggingInformationEntry<'_, '_, EndianSlice<'_, RunTimeEndian>, usize>
{
    fn pc(&self) -> Option<Range<Addr>> {
        let low = match self.attr_value(DW_AT_low_pc).ok()?? {
            AttrValue::Addr(addr) => addr.into(),
            _ => None?,
        };

        let high = match self.attr_value(DW_AT_high_pc).ok()?? {
            AttrValue::Addr(addr) => addr.into(),
            AttrValue::Udata(len) => low + len,
            _ => None?,
        };

        Some(low..high)
    }

    fn is_artificial(&self) -> Option<bool> {
        let AttrValue::Flag(is_artificial) = self.attr_value(DW_AT_artificial).ok()?? else {
            None?
        };

        Some(is_artificial)
    }
}
