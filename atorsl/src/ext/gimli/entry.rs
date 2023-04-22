use crate::{data::*, *};
use gimli::{EndianSlice, RunTimeEndian};
use std::{borrow::Cow, ops};

pub(crate) trait DebuggingInformationEntry {
    fn name<'a>(&'a self, dwarf: &'a Dwarf, unit: &'a Unit) -> Result<Cow<'a, str>, Error>;
    fn symbol_name<'a>(&'a self, dwarf: &'a Dwarf, unit: &'a Unit) -> Result<Cow<'a, str>, Error>;

    fn pc(&self) -> Option<ops::Range<Addr>>;
}

impl DebuggingInformationEntry
    for gimli::DebuggingInformationEntry<'_, '_, EndianSlice<'_, RunTimeEndian>, usize>
{
    fn name<'a>(&'a self, dwarf: &'a Dwarf, unit: &'a Unit) -> Result<Cow<'a, str>, Error> {
        match self.attr_value(gimli::DW_AT_name)? {
            Some(AttrValue::UnitRef(offset)) => Ok(Cow::from(
                unit.entry(offset)?.name(dwarf, unit)?.into_owned(),
            )),
            Some(attr) => Ok(dwarf.attr_string(unit, attr)?.to_string_lossy()),
            None => Err(Error::AddrNotNamed),
        }
    }

    fn symbol_name<'a>(&'a self, dwarf: &'a Dwarf, unit: &'a Unit) -> Result<Cow<'a, str>, Error> {
        [
            gimli::DW_AT_linkage_name,
            gimli::DW_AT_abstract_origin,
            gimli::DW_AT_name,
        ]
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

    fn pc(&self) -> Option<ops::Range<Addr>> {
        let low: Addr = match self.attr_value(gimli::DW_AT_low_pc).ok()? {
            Some(AttrValue::Addr(addr)) => Some(addr.into()),
            _ => None,
        }?;

        let high = match self.attr_value(gimli::DW_AT_high_pc).ok()? {
            Some(AttrValue::Addr(addr)) => Some(addr.into()),
            Some(AttrValue::Udata(len)) => Some(low + len),
            _ => None,
        }?;

        Some(low..high)
    }
}
