use crate::{
    ext::gimli::{ArangeEntry, DebuggingInformationEntry},
    *,
};
use fallible_iterator::FallibleIterator;
use gimli::DebugInfoOffset;

pub trait Symbolicate {
    fn symbolicate(&self, vmaddr: Addr, context: &Context) -> Result<Vec<String>, Error>;
}

impl Symbolicate for Dwarf<'_> {
    fn symbolicate(&self, vmaddr: Addr, context: &Context) -> Result<Vec<String>, Error> {
        fallible_iterator::convert(
            context
                .addrs
                .to_owned()
                .into_iter()
                .map(|addr| self.lookup(addr - context.loadaddr + vmaddr, context.inline)),
        )
        .collect()
    }
}

trait Lookup {
    fn lookup(&self, address: Addr, expand_inlined: bool) -> Result<String, Error>;
    fn translate(&self, entry: &Entry, unit: &Unit) -> Result<String, Error>;
    fn try_attr_string(&self, unit: &Unit, value: AttrValue) -> Option<String>;
    fn unit_from_addr(&self, addr: Addr) -> Result<Unit, Error>;
    fn debug_info_offset_from_addr(&self, addr: Addr) -> Result<DebugInfoOffset, Error>;
}

impl Lookup for Dwarf<'_> {
    fn lookup(&self, addr: Addr, expand_inlined: bool) -> Result<String, Error> {
        let unit = self.unit_from_addr(addr)?;
        let mut entries = unit.entries();

        let (entry, result) = loop {
            let Some((_, entry)) = entries.next_dfs()? else {
                break (None, Err(Error::AddrNotFound(addr)))
            };

            match entry.pc() {
                Some(pc) if entry.tag() == gimli::DW_TAG_subprogram && pc.contains(&addr) => {
                    break (Some(entry), self.translate(entry, &unit))
                }
                _ => continue,
            }
        };

        match entry {
            Some(entry) if expand_inlined && entry.has_children() => {
                let mut symbol = result?;
                let mut depth = 0;
                loop {
                    let Some((step, entry)) = entries.next_dfs()? else {
                        break;
                    };

                    depth += step;

                    if depth.signum() < 1 {
                        break;
                    }

                    if entry.tag() == gimli::DW_TAG_inlined_subroutine {
                        symbol.insert(0, '\n');
                        symbol.insert_str(0, self.translate(entry, &unit)?.as_str());
                    }
                }

                Ok(symbol)
            }
            _ => result,
        }
    }

    fn translate(&self, entry: &Entry, unit: &Unit) -> Result<String, Error> {
        entry
            .symbol()
            .ok_or(Error::AddrHasNoSymbol)
            .and_then(|value| match value {
                AttrValue::UnitRef(offset) => self.translate(&unit.entry(offset)?, &unit),
                _ => Ok(self
                    .attr_string(&unit, value)
                    .map_err(Error::Gimli)?
                    .to_string_lossy()
                    .to_string()),
            })
    }

    fn try_attr_string(&self, unit: &Unit, value: AttrValue) -> Option<String> {
        self.attr_string(&unit, value)
            .ok()
            .map(|slice| slice.to_string_lossy().to_string())
    }

    fn unit_from_addr(&self, addr: Addr) -> Result<Unit, Error> {
        let offset = self.debug_info_offset_from_addr(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        Ok(self.unit(header)?)
    }

    fn debug_info_offset_from_addr(&self, addr: Addr) -> Result<DebugInfoOffset, Error> {
        self.debug_aranges
            .headers()
            .find_map(|header| {
                Ok(if header.entries().any(|entry| entry.contains(addr))? {
                    Some(header.debug_info_offset())
                } else {
                    None
                })
            })?
            .ok_or(Error::AddrNoDebugOffset(addr))
    }
}

#[allow(dead_code)]
fn fmt(entry: &Entry, dwarf: &Dwarf, header: &UnitHeader, unit: &Unit) -> String {
    format!(
        "│ {:#010x} │ {:^#39.39} │ {:#25} │ {:#80.80} │",
        entry.offset().to_debug_info_offset(&header).unwrap().0,
        format!("{:?}", entry.pc().unwrap_or(Addr::nil()..Addr::nil())),
        entry.tag(),
        entry
            .symbol()
            .and_then(|v| dwarf.try_attr_string(&unit, v))
            .unwrap_or_else(String::default),
    )
}
