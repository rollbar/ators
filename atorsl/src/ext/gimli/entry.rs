use crate::{data::*, *};
use gimli::{
    DW_AT_abstract_origin, DW_AT_artificial, DW_AT_high_pc, DW_AT_linkage_name, DW_AT_low_pc,
    DW_AT_name, EndianSlice, RunTimeEndian,
};
use std::{borrow::Cow, ops::Range};

pub(crate) trait DebuggingInformationEntry {
    fn name<'a>(&'a self, dwarf: &'a Dwarf, unit: &'a Unit) -> Result<Cow<'a, str>, Error>;
    fn symbol_name<'a>(&'a self, dwarf: &'a Dwarf, unit: &'a Unit) -> Result<Cow<'a, str>, Error>;

    fn pc(&self) -> Option<Range<Addr>>;

    fn is_artificial(&self) -> Option<bool>;
}

impl DebuggingInformationEntry
    for gimli::DebuggingInformationEntry<'_, '_, EndianSlice<'_, RunTimeEndian>, usize>
{
    fn name<'a>(&'a self, dwarf: &'a Dwarf, unit: &'a Unit) -> Result<Cow<'a, str>, Error> {
        match self
            .attr_value(DW_AT_name)?
            .ok_or(Error::AddrNotNamed)?
        {
            AttrValue::UnitRef(offset) => Ok(Cow::from(
                unit.entry(offset)?.name(dwarf, unit)?.into_owned(),
            )),
            attr => Ok(dwarf.attr_string(unit, attr)?.to_string_lossy()),
        }
    }

    fn symbol_name<'a>(&'a self, dwarf: &'a Dwarf, unit: &'a Unit) -> Result<Cow<'a, str>, Error> {
        [DW_AT_linkage_name, DW_AT_abstract_origin, DW_AT_name]
            .into_iter()
            .find_map(|dw_at| self.attr_value(dw_at).ok()?)
            .ok_or(Error::AddrNotSymbol)
            .and_then(|attr| match attr {
                AttrValue::UnitRef(offset) => Ok(Cow::from(
                    unit.entry(offset)?
                        .symbol_name(dwarf, unit)?
                        .into_owned(),
                )),
                attr => Ok(dwarf.attr_string(unit, attr)?.to_string_lossy()),
            })
    }

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
