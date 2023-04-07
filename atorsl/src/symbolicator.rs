#![allow(unstable_name_collisions)]

use crate::{
    ext::gimli::{ArangeEntry, DebuggingInformationEntry},
    *,
};
use derive_builder::Builder;
use fallible_iterator::FallibleIterator;
use gimli::{
    DW_AT_abstract_origin, DW_AT_artificial, DW_AT_call_column, DW_AT_call_file, DW_AT_call_line,
    DW_AT_comp_dir, DW_AT_decl_column, DW_AT_decl_file, DW_AT_decl_line, DW_AT_language,
    DW_AT_linkage_name, DW_AT_name, DebugInfoOffset, DwAt, DwLang,
};
use std::path::{Path, PathBuf};

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
        let mut comp_dir = PathBuf::default();
        let mut lang = DwLang(0);

        let unit = self.unit_from_addr(&addr)?;
        let mut entries = unit.entries();
        let mut symbols = Vec::<Symbol>::default();

        loop {
            let (_, entry) = entries.next_dfs()?.ok_or(Error::AddrNotFound(addr))?;

            // guarantee: depth order compile_unit > module > subprogram > inlined_subroutine
            match entry.tag() {
                gimli::DW_TAG_compile_unit => {
                    lang = self.entry_lang(entry).unwrap_or(DwLang(0));
                    comp_dir = self
                        .entry_string(DW_AT_comp_dir, entry, &unit)
                        .map(PathBuf::from)
                        .unwrap_or_default();
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

                    symbols.push(self.symbol_from_entry(entry, &unit, &module, &comp_dir, &lang)?);

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

                            if entry.tag() == gimli::DW_TAG_inlined_subroutine
                                && entry.pc().is_some_and(|pc| pc.contains(&addr))
                            {
                                symbols.insert(
                                    0,
                                    self.symbol_from_entry(
                                        entry, &unit, &module, &comp_dir, &lang,
                                    )?,
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
    fn entry_file(&self, entry: &Entry, unit: &Unit) -> Option<PathBuf>;
    fn entry_line(&self, entry: &Entry) -> Option<u16>;
    fn entry_col(&self, entry: &Entry) -> Option<u16>;
    fn entry_lang(&self, entry: &Entry) -> Option<DwLang>;
    fn entry_is_artificial(&self, entry: &Entry) -> Option<bool>;

    fn symbol_from_entry(
        &self,
        entry: &Entry,
        unit: &Unit,
        module: &String,
        comp_dir: &Path,
        lang: &DwLang,
    ) -> Result<Symbol, Error>;

    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error>;
    fn debug_info_offset(&self, addr: &Addr) -> Result<DebugInfoOffset, Error>;
}

impl DwarfExt for Dwarf<'_> {
    fn symbol_from_entry(
        &self,
        entry: &Entry,
        unit: &Unit,
        module: &String,
        comp_dir: &Path,
        lang: &DwLang,
    ) -> Result<Symbol, Error> {
        let mut symbol = SymbolBuilder::default();
        let linkage = self.entry_linkage(entry, &unit)?;
        symbol.linkage(demangler::demangle(&linkage).to_owned());
        symbol.module(module.clone());
        symbol.lang(*lang);
        if let Some(true) = self.entry_is_artificial(entry) {
            symbol.file(Some(comp_dir.join("<compile-generated>")));
            symbol.line(Some(0));
            symbol.col(None);
        } else {
            symbol.file(self.entry_file(entry, &unit));
            symbol.line(self.entry_line(entry));
            symbol.col(self.entry_col(entry));
        }
        Ok(symbol.build()?)
    }

    fn entry_string(&self, name: DwAt, entry: &Entry, unit: &Unit) -> Option<String> {
        entry.attr_value(name).ok()?.and_then(|attr| {
            Some(
                self.attr_string(&unit, attr)
                    .ok()?
                    .to_string_lossy()
                    .to_string(),
            )
        })
    }

    fn entry_linkage(&self, entry: &Entry, unit: &Unit) -> Result<String, Error> {
        [DW_AT_linkage_name, DW_AT_abstract_origin, DW_AT_name]
            .into_iter()
            .find_map(|dw_at| entry.attr_value(dw_at).ok().flatten())
            .ok_or(Error::EntryHasNoSymbol)
            .and_then(|attr| match attr {
                AttrValue::UnitRef(offset) => self.entry_linkage(&unit.entry(offset)?, &unit),
                attr @ _ => Ok(self
                    .attr_string(&unit, attr)?
                    .to_string_lossy()
                    .to_string()),
            })
            .map(|value| demangler::demangle(&value).to_owned())
    }

    fn entry_file(&self, entry: &Entry, unit: &Unit) -> Option<PathBuf> {
        let Some(AttrValue::FileIndex(offset)) = [DW_AT_decl_file, DW_AT_call_file]
            .into_iter()
            .find_map(|name| entry.attr_value(name).ok()?)
        else {
            return None
        };

        let header = unit.line_program.as_ref()?.header();
        let file = header.file(offset)?;
        let dir = match file.directory(header) {
            Some(attr) => {
                self.attr_string(unit, attr)
                    .ok()?
                    .to_string_lossy()
                    .to_string()
                    + "/"
            }
            _ => String::default(),
        };

        self.attr_string(&unit, file.path_name())
            .map(|file| dir + &file.to_string_lossy())
            .map(PathBuf::from)
            .ok()
    }

    fn entry_lang(&self, entry: &Entry) -> Option<DwLang> {
        match entry.attr_value(DW_AT_language).ok()?? {
            AttrValue::Language(dw_lang) => Some(dw_lang),
            _ => None,
        }
    }

    fn entry_line(&self, entry: &Entry) -> Option<u16> {
        [DW_AT_decl_line, DW_AT_call_line]
            .into_iter()
            .find_map(|name| entry.attr_value(name).ok()??.u16_value())
    }

    fn entry_col(&self, entry: &Entry) -> Option<u16> {
        [DW_AT_decl_column, DW_AT_call_column]
            .into_iter()
            .find_map(|name| entry.attr_value(name).ok()??.u16_value())
    }

    /// Whether the entry is compiler generated
    fn entry_is_artificial(&self, entry: &Entry) -> Option<bool> {
        match entry.attr_value(DW_AT_artificial).ok()?? {
            AttrValue::Flag(is_artificial) => Some(is_artificial),
            _ => None,
        }
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
