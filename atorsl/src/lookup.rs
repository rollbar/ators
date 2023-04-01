use crate::{
    ext::gimli::{ArangeEntry, DebuggingInformationEntry},
    Addr, Context, Error,
};
use fallible_iterator::FallibleIterator;
use gimli::{AttributeValue, DebugInfoOffset, EndianSlice, RunTimeEndian};
use gimli::{DW_TAG_inlined_subroutine, DW_TAG_subprogram};

type Dwarf<'input> = gimli::Dwarf<EndianSlice<'input, RunTimeEndian>>;
type Unit<'input> = gimli::Unit<EndianSlice<'input, RunTimeEndian>, usize>;
type UnitHeader<'input> = gimli::UnitHeader<EndianSlice<'input, RunTimeEndian>, usize>;
type Entry<'abbrev, 'unit, 'input> =
    gimli::DebuggingInformationEntry<'abbrev, 'unit, EndianSlice<'input, RunTimeEndian>, usize>;

pub trait Lookup {
    fn lookup(&self, vmaddr: Addr, context: &Context) -> Result<Vec<String>, Error>;
    fn lookup_addr(&self, address: Addr, context: &Context) -> Result<String, Error>;

    fn symbolicate(&self, entry: &Entry, unit: &Unit) -> Result<String, Error>;
    fn unit_from_addr(&self, addr: Addr) -> Result<(UnitHeader, Unit), Error>;
    fn debug_info_offset_from_addr(&self, addr: Addr) -> Result<DebugInfoOffset, Error>;
}

impl Lookup for Dwarf<'_> {
    fn lookup(&self, vmaddr: Addr, context: &Context) -> Result<Vec<String>, Error> {
        fallible_iterator::convert(
            context
                .addrs
                .to_owned()
                .into_iter()
                .map(|addr| self.lookup_addr(addr - context.loadaddr + vmaddr, &context)),
        )
        .collect()
    }

    fn lookup_addr(&self, addr: Addr, context: &Context) -> Result<String, Error> {
        let (_, unit) = self.unit_from_addr(addr)?;
        let mut entries = unit.entries();

        let (entry, result) = loop {
            let Some((_, entry)) = entries.next_dfs()? else {
                break (None, Err(Error::AddrNotFound(addr)))
            };

            match entry.pc() {
                Some(pc) if entry.tag() == DW_TAG_subprogram && pc.contains(&addr) => {
                    break (Some(entry), self.symbolicate(entry, &unit))
                }
                _ => continue,
            }
        };

        match entry {
            Some(entry) if context.inline && entry.has_children() => {
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

                    if entry.tag() == DW_TAG_inlined_subroutine {
                        symbol.insert(0, '\n');
                        symbol.insert_str(0, self.symbolicate(entry, &unit)?.as_str());
                    }
                }

                Ok(symbol)
            }
            _ => result,
        }
    }

    fn symbolicate(&self, entry: &Entry, unit: &Unit) -> Result<String, Error> {
        entry
            .linkage_name()
            .or_else(|| entry.abstract_origin())
            .or_else(|| entry.name())
            .ok_or(Error::AddrHasNoSymbol)
            .and_then(|value| match value {
                AttributeValue::UnitRef(offset) => self.symbolicate(&unit.entry(offset)?, &unit),
                _ => Ok(self
                    .attr_string(&unit, value)
                    .map_err(Error::Gimli)?
                    .to_string_lossy()
                    .to_string()),
            })
    }

    fn unit_from_addr(&self, addr: Addr) -> Result<(UnitHeader, Unit), Error> {
        let offset = self.debug_info_offset_from_addr(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        Ok((header, self.unit(header)?))
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
