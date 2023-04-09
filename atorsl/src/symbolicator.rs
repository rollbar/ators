use crate::{
    data::*,
    ext::gimli::{ArangeEntry, DebuggingInformationEntry},
    *,
};
use fallible_iterator::FallibleIterator;
use gimli::{
    DW_AT_abstract_origin, DW_AT_artificial, DW_AT_call_column, DW_AT_call_file, DW_AT_call_line,
    DW_AT_comp_dir, DW_AT_decl_column, DW_AT_decl_file, DW_AT_decl_line, DW_AT_language,
    DW_AT_linkage_name, DW_AT_name, DebugInfoOffset, DwAt, DwLang,
};
use itertools::Either;
use object::Object;
use std::path::{Path, PathBuf};

pub fn atos_dwarf(dwarf: &Dwarf, addr: Addr, include_inlined: bool) -> Result<Vec<Symbol>, Error> {
    let mut module = String::default();
    let mut comp_dir = PathBuf::default();
    let mut lang = DwLang(0);

    let unit = dwarf.unit_from_addr(&addr)?;
    let mut entries = unit.entries();
    let mut symbols = Vec::default();

    loop {
        let (_, entry) = entries.next_dfs()?.ok_or(Error::AddrNotFound(addr))?;

        // guarantee: depth order compile_unit > module > subprogram > inlined_subroutine
        match entry.tag() {
            gimli::DW_TAG_compile_unit => {
                lang = dwarf.entry_lang(entry).unwrap_or(DwLang(0));
                comp_dir = dwarf
                    .entry_string(DW_AT_comp_dir, entry, &unit)
                    .map(PathBuf::from)
                    .unwrap_or_default();
            }
            gimli::DW_TAG_module => {
                module = dwarf
                    .entry_string(DW_AT_name, entry, &unit)
                    .unwrap_or_default();
            }
            gimli::DW_TAG_subprogram => {
                if !entry.pc().is_some_and(|pc| pc.contains(&addr)) {
                    continue;
                }

                symbols.push(dwarf.symbol(entry, None, &unit, &module, &comp_dir, &lang)?);

                if include_inlined && entry.has_children() {
                    let mut parent_entry = None;
                    let mut depth = 0;

                    let last_child = loop {
                        let Some((step, child_entry)) = entries.next_dfs()? else {
                            break parent_entry;
                        };

                        depth += step;

                        if depth == 0 {
                            break parent_entry;
                        }

                        if child_entry.tag() == gimli::DW_TAG_inlined_subroutine
                            && child_entry.pc().is_some_and(|pc| pc.contains(&addr))
                        {
                            if let Some(ref parent_entry) = parent_entry {
                                symbols.insert(
                                    0,
                                    dwarf.symbol(
                                        parent_entry,
                                        Some(child_entry),
                                        &unit,
                                        &module,
                                        &comp_dir,
                                        &lang,
                                    )?,
                                )
                            }

                            parent_entry = Some(child_entry.clone());
                        }
                    };

                    if let Some(last_child) = last_child {
                        symbols.insert(
                            0,
                            dwarf.symbol(&last_child, None, &unit, &module, &comp_dir, &lang)?,
                        )
                    }
                }

                break;
            }
            _ => continue,
        }
    }

    Ok(symbols)
}

pub fn atos_obj(obj: &object::File, addr: Addr) -> Result<Vec<Symbol>, Error> {
    let map = obj.symbol_map();
    let Some(symbol) = map.get(*addr) else {
        return Err(Error::AddrNotFound(addr))
    };

    Ok(vec![SymbolBuilder::default()
        .module(String::default())
        .linkage(demangler::demangle(symbol.name()))
        .lang(DwLang(0))
        .loc(Either::Right(addr - symbol.address()))
        .build()?])
}

trait DwarfExt {
    fn entry_linkage(&self, entry: &Entry, unit: &Unit) -> Result<String, Error>;
    fn entry_string(&self, name: DwAt, entry: &Entry, unit: &Unit) -> Option<String>;
    fn entry_file(&self, entry: &Entry, unit: &Unit) -> Option<PathBuf>;
    fn entry_line(&self, entry: &Entry) -> Option<u16>;
    fn entry_col(&self, entry: &Entry) -> Option<u16>;
    fn entry_lang(&self, entry: &Entry) -> Option<DwLang>;
    fn entry_is_artificial(&self, entry: &Entry) -> Option<bool>;

    fn symbol(
        &self,
        entry: &Entry,
        child: Option<&Entry>,
        unit: &Unit,
        module: &str,
        comp_dir: &Path,
        lang: &DwLang,
    ) -> Result<Symbol, Error>;

    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error>;
    fn debug_info_offset(&self, addr: &Addr) -> Result<DebugInfoOffset, Error>;
}

impl DwarfExt for Dwarf<'_> {
    fn symbol(
        &self,
        entry: &Entry,
        child: Option<&Entry>,
        unit: &Unit,
        module: &str,
        comp_dir: &Path,
        lang: &DwLang,
    ) -> Result<Symbol, Error> {
        let linkage = self.entry_linkage(entry, unit)?;
        let mut symbol = SymbolBuilder::default();
        symbol
            .linkage(demangler::demangle(&linkage))
            .module(module.to_string())
            .lang(*lang);

        let artificial = self.entry_is_artificial(entry);
        let entry_with_call = child.unwrap_or(entry);
        let file = self.entry_file(entry_with_call, unit);
        match (file, artificial) {
            (None, _) | (_, Some(true)) => {
                symbol.loc(Either::Left(SourceLoc {
                    file: comp_dir.join("<compile-generated>"),
                    line: u16::default(),
                    col: None,
                }));
            }
            (Some(file), _) => {
                symbol.loc(Either::Left(SourceLoc {
                    file,
                    line: self.entry_line(entry_with_call).unwrap_or_default(),
                    col: self.entry_col(entry_with_call),
                }));
            }
        }

        Ok(symbol.build()?)
    }

    fn entry_string(&self, name: DwAt, entry: &Entry, unit: &Unit) -> Option<String> {
        entry.attr_value(name).ok()?.and_then(|attr| {
            Some(
                self.attr_string(unit, attr)
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
            .ok_or(Error::EntryInAddrNotSymbol)
            .and_then(|attr| match attr {
                AttrValue::UnitRef(offset) => self.entry_linkage(&unit.entry(offset)?, unit),
                attr => Ok(self
                    .attr_string(unit, attr)?
                    .to_string_lossy()
                    .to_string()),
            })
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

        self.attr_string(unit, file.path_name())
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
