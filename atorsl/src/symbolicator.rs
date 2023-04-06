#![allow(unstable_name_collisions)]

use crate::{
    demangle::swift,
    ext::gimli::{ArangeEntry, DebuggingInformationEntry},
    *,
};
use derive_builder::Builder;
use fallible_iterator::FallibleIterator;
use gimli::{
    DW_AT_abstract_origin, DW_AT_call_column, DW_AT_call_file, DW_AT_call_line, DW_AT_language,
    DW_AT_linkage_name, DW_AT_name, DwAt, DwLang,
};
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Builder)]
pub struct Symbol {
    pub module: String,
    pub linkage: String,

    pub lang: gimli::DwLang,

    pub file: Option<PathBuf>,
    pub line: Option<u16>,
    pub col: Option<u16>,
}

pub trait Symbolicator {
    fn atos<'a>(
        &self,
        addr: &Addr,
        base: &Addr,
        include_inlined: bool,
    ) -> Result<Vec<Symbol>, Error>;
}

impl Symbolicator for Dwarf<'_> {
    fn atos(&self, addr: &Addr, base: &Addr, include_inlined: bool) -> Result<Vec<Symbol>, Error> {
        let addr = addr
            .checked_sub(**base)
            .map(Addr::from)
            .ok_or(Error::AddrOffsetOverflow(*addr, *base))?;

        let mut module = String::default();
        let mut lang = DwLang(0);

        let unit = self.unit_from_addr(&addr)?;
        let mut entries = unit.entries();
        let mut symbols = Vec::<Symbol>::default();

        loop {
            let (_, entry) = entries.next_dfs()?.ok_or(Error::AddrNotFound(addr))?;

            // guarantee: depth order compile_unit > module > subprogram > inlined_subroutine
            match entry.tag() {
                gimli::DW_TAG_compile_unit => {
                    lang = DwLang(
                        self.entry_u16(DW_AT_language, entry)
                            .unwrap_or_default(),
                    );
                }
                gimli::DW_TAG_module => {
                    module = self
                        .entry_string(DW_AT_name, entry, &unit)
                        .unwrap_or_default();
                }
                gimli::DW_TAG_subprogram => {
                    if !entry.pc().is_some_and(|pc| pc.contains(&addr)) {
                        continue;
                    }

                    symbols.push(self.symbol_from_entry(entry, &unit, &module, &lang)?);

                    if include_inlined && entry.has_children() {
                        let mut depth = 0;
                        loop {
                            let Some((step, entry)) = entries.next_dfs()? else {
                                break
                            };

                            depth += step;

                            if depth.signum() < 1 {
                                break;
                            }

                            if entry.tag() == gimli::DW_TAG_inlined_subroutine {
                                symbols.insert(
                                    0,
                                    self.symbol_from_entry(entry, &unit, &module, &lang)?,
                                );
                            }
                        }
                    }

                    break;
                }
                _ => continue,
            }
        }

        Ok(symbols)
    }
}

trait DwarfExt {
    fn entry_linkage(&self, entry: &Entry, unit: &Unit) -> Result<String, Error>;
    fn entry_string(&self, name: DwAt, entry: &Entry, unit: &Unit) -> Option<String>;
    fn entry_path(&self, name: DwAt, entry: &Entry, unit: &Unit) -> Option<PathBuf>;
    fn entry_u16(&self, name: DwAt, entry: &Entry) -> Option<u16>;

    fn symbol_from_entry(
        &self,
        entry: &Entry,
        unit: &Unit,
        module: &String,
        lang: &DwLang,
    ) -> Result<Symbol, Error>;

    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error>;
    fn debug_info_offset(&self, addr: &Addr) -> Result<gimli::DebugInfoOffset, Error>;
}

impl DwarfExt for Dwarf<'_> {
    fn symbol_from_entry(
        &self,
        entry: &Entry,
        unit: &Unit,
        module: &String,
        lang: &DwLang,
    ) -> Result<Symbol, Error> {
        let mut symbol = SymbolBuilder::default();
        let linkage = self.entry_linkage(entry, &unit)?;
        symbol.linkage(swift::demangle(&linkage).unwrap_or(linkage));
        symbol.module(module.clone());
        symbol.lang(*lang);
        symbol.file(self.entry_path(DW_AT_call_file, entry, &unit));
        symbol.line(self.entry_u16(DW_AT_call_line, entry));
        symbol.col(self.entry_u16(DW_AT_call_column, entry));
        Ok(symbol.build()?)
    }

    fn entry_linkage(&self, entry: &Entry, unit: &Unit) -> Result<String, Error> {
        [DW_AT_linkage_name, DW_AT_abstract_origin, DW_AT_name]
            .into_iter()
            .find_map(|dw_at| entry.attr_value(dw_at).ok().flatten())
            .ok_or(Error::EntryHasNoSymbol)
            .and_then(|attr| match attr {
                AttrValue::UnitRef(offset) => self.entry_linkage(&unit.entry(offset)?, &unit),
                attr @ _ => self
                    .attr_string(&unit, attr)
                    .map_err(Error::Gimli)
                    .map(|value| value.to_string_lossy().to_string())
                    .map(|value| swift::demangle(&value).unwrap_or(value)),
            })
    }

    fn entry_path(&self, name: DwAt, entry: &Entry, unit: &Unit) -> Option<PathBuf> {
        let Some(ref program) = unit.line_program else {
            return None
        };

        let Some(AttrValue::FileIndex(offset)) = entry.attr_value(name).ok()? else {
            return None
        };

        let header = program.header();
        let file = header.file(offset)?;
        let dir = match file.directory(header) {
            Some(attr) => self
                .attr_string(unit, attr)
                .ok()
                .map(|dir| "/".to_string() + &dir.to_string_lossy())
                .unwrap_or_default(),
            _ => String::default(),
        };

        self.attr_string(&unit, file.path_name())
            .map(|file| dir + &file.to_string_lossy())
            .map(PathBuf::from)
            .ok()
    }

    fn entry_string(&self, name: DwAt, entry: &Entry, unit: &Unit) -> Option<String> {
        entry.attr_value(name).ok().flatten().and_then(|attr| {
            self.attr_string(&unit, attr)
                .ok()
                .map(|slice| slice.to_string_lossy().to_string())
        })
    }

    fn entry_u16(&self, name: DwAt, entry: &Entry) -> Option<u16> {
        entry
            .attr_value(name)
            .ok()
            .flatten()
            .and_then(|attr| attr.u16_value())
    }

    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error> {
        let offset = self.debug_info_offset(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        self.unit(header).map_err(Error::Gimli)
    }

    fn debug_info_offset(&self, addr: &Addr) -> Result<gimli::DebugInfoOffset, Error> {
        self.debug_aranges
            .headers()
            .find_map(|header| {
                Ok(if header.entries().any(|entry| entry.contains(addr))? {
                    Some(header.debug_info_offset())
                } else {
                    None
                })
            })?
            .ok_or(Error::AddrNoDebugOffset(*addr))
    }
}
