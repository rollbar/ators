use crate::{
    demangle::swift,
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
        fallible_iterator::convert(context.addrs.clone().iter().map(|addr| {
            self.atos(addr - context.loadaddr + vmaddr, context.inline)
                .and_then(demangle)
                .map(|symbols| symbols.join("\n"))
        }))
        .collect()
    }
}

trait Lookup {
    fn atos(&self, address: Addr, include_inlined: bool) -> Result<Vec<String>, Error>;
    fn symbol(&self, entry: &Entry, unit: &Unit) -> Result<String, Error>;
    fn try_attr_string(&self, unit: &Unit, value: AttrValue) -> Option<String>;
    fn unit_from_addr(&self, addr: Addr) -> Result<Unit, Error>;
    fn debug_info_offset(&self, addr: Addr) -> Result<DebugInfoOffset, Error>;
}

impl Lookup for Dwarf<'_> {
    fn atos(&self, addr: Addr, include_inlined: bool) -> Result<Vec<String>, Error> {
        let unit = self.unit_from_addr(addr)?;
        let mut entries = unit.entries();

        let (entry, symbol) = loop {
            let Some((_, entry)) = entries.next_dfs()? else {
                break (None, Err(Error::AddrNotFound(addr)))
            };

            match entry.pc() {
                Some(pc) if entry.tag() == gimli::DW_TAG_subprogram && pc.contains(&addr) => {
                    break (Some(entry), self.symbol(entry, &unit))
                }
                _ => continue,
            }
        };

        let mut symbols = vec![symbol];

        if include_inlined && entry.map(Entry::has_children).unwrap_or(false) {
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
                    symbols.push(self.symbol(entry, &unit));
                }
            }
        }

        fallible_iterator::convert(symbols.into_iter().rev()).collect()
    }

    fn symbol(&self, entry: &Entry, unit: &Unit) -> Result<String, Error> {
        entry
            .symbol()
            .ok_or(Error::AddrHasNoSymbol)
            .and_then(|value| match value {
                AttrValue::UnitRef(offset) => self.symbol(&unit.entry(offset)?, &unit),
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
        let offset = self.debug_info_offset(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        Ok(self.unit(header)?)
    }

    fn debug_info_offset(&self, addr: Addr) -> Result<DebugInfoOffset, Error> {
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

fn demangle(symbols: Vec<String>) -> Result<Vec<String>, Error> {
    fallible_iterator::convert(symbols.into_iter().map(|symbol| {
        if swift::is_mangled(&symbol) {
            swift::demangle(&symbol).ok_or_else(|| Error::CannotDemangleSymbol(symbol))
        } else {
            Ok(symbol)
        }
    }))
    .collect()
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
