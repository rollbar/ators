use crate::{
    data::*,
    ext::gimli::{ArangeEntry, DebuggingInformationEntry, Range},
    *,
};
use fallible_iterator::FallibleIterator;
use gimli::{
    DW_AT_abstract_origin, DW_AT_artificial, DW_AT_call_column, DW_AT_call_file, DW_AT_call_line,
    DW_AT_comp_dir, DW_AT_decl_column, DW_AT_decl_file, DW_AT_decl_line, DW_AT_language,
    DW_AT_linkage_name, DW_AT_name, DW_AT_ranges, DebugInfoOffset, DwLang,
};
use itertools::Either;
use object::Object;
use std::path::{Path, PathBuf};

pub fn atos_dwarf(
    dwarf: &Dwarf,
    addr: &Addr,
    include_inlined: bool,
) -> Result<Vec<Symbol>, Error> {
    let mut path = PathBuf::default();
    let mut lang = DwLang(0);

    let unit = dwarf.unit_from_addr(addr)?;
    let mut entries = unit.entries();
    let mut symbols = Vec::default();

    loop {
        let entry = entries
            .next_dfs()?
            .ok_or_else(|| Error::AddrNotFound(*addr))?
            .1;

        // guarantee: depth order compile_unit > module > subprogram > inlined_subroutine
        match entry.tag() {
            gimli::DW_TAG_compile_unit => {
                lang = dwarf.entry_lang(entry).unwrap_or(DwLang(0));
                path = entry
                    .attr_value(DW_AT_comp_dir)
                    .ok()
                    .and_then(|attr| Some(dwarf.attr_string(&unit, attr?).ok()?.to_string_lossy()))
                    .map(|comp_dir| PathBuf::from(&*comp_dir))
                    .unwrap_or_default();
            }

            gimli::DW_TAG_subprogram if dwarf.entry_contains(addr, entry, &unit) => {
                symbols.push(Symbol {
                    name: dwarf.entry_demangled_name(entry, &lang, &unit)?,
                    loc: Either::Left(dwarf.entry_loc(entry, &path, &unit)),
                });

                if include_inlined && entry.has_children() {
                    let mut parent = None;
                    let mut depth = 0;

                    let last_child = loop {
                        let Some((step, child)) = entries.next_dfs()? else {
                            break parent
                        };

                        depth += step;

                        if depth <= 0 {
                            break parent;
                        }

                        if child.tag() == gimli::DW_TAG_inlined_subroutine
                            && dwarf.entry_contains(addr, child, &unit)
                        {
                            if let Some(ref parent) = parent {
                                symbols.insert(
                                    0,
                                    Symbol {
                                        name: dwarf.entry_demangled_name(parent, &lang, &unit)?,
                                        loc: Either::Left(dwarf.entry_loc(child, &path, &unit)),
                                    },
                                );
                            }

                            parent = Some(child.clone());
                        }
                    };

                    if let Some(last_child) = last_child {
                        symbols.insert(
                            0,
                            Symbol {
                                name: dwarf.entry_demangled_name(&last_child, &lang, &unit)?,
                                loc: Either::Left(dwarf.entry_loc(&last_child, &path, &unit)),
                            },
                        );
                    }
                }

                break Ok(symbols);
            }

            _ => continue,
        }
    }
}

pub fn atos_obj(obj: &object::File, addr: Addr) -> Result<Vec<Symbol>, Error> {
    let map = obj.symbol_map();
    let Some(symbol) = map.get(*addr) else {
        return Err(Error::AddrNotFound(addr))
    };

    Ok(vec![Symbol {
        name: demangler::demangle(symbol.name(), &demangler::language_of(symbol.name())),
        loc: Either::Right(addr - symbol.address()),
    }])
}

trait DwarfExt {
    fn entry_name(&self, entry: &Entry, unit: &Unit) -> Result<String, Error>;
    fn entry_demangled_name(
        &self,
        entry: &Entry,
        lang: &DwLang,
        unit: &Unit,
    ) -> Result<String, Error>;

    //fn entry_string(&self, name: DwAt, entry: &Entry, unit: &Unit) -> Option<String>;
    fn entry_file(&self, entry: &Entry, unit: &Unit) -> Option<PathBuf>;
    fn entry_line(&self, entry: &Entry) -> Option<u16>;
    fn entry_col(&self, entry: &Entry) -> Option<u16>;
    fn entry_lang(&self, entry: &Entry) -> Option<DwLang>;
    fn entry_is_artificial(&self, entry: &Entry) -> Option<bool>;
    fn entry_loc(&self, entry: &Entry, path: &Path, unit: &Unit) -> SourceLoc;

    fn entry_contains(&self, addr: &Addr, entry: &Entry, unit: &Unit) -> bool;
    fn entry_ranges_contain(&self, addr: &Addr, entry: &Entry, unit: &Unit) -> Option<bool>;

    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error>;
    fn debug_info_offset(&self, addr: &Addr) -> Result<DebugInfoOffset, Error>;
}

impl DwarfExt for Dwarf<'_> {
    fn entry_loc(&self, entry: &Entry, path: &Path, unit: &Unit) -> SourceLoc {
        let artificial = self.entry_is_artificial(entry);
        let file = self.entry_file(entry, unit);
        match (file, artificial) {
            (None, _) | (_, Some(true)) => SourceLoc {
                file: path.join("<compile-generated>"),
                line: u16::default(),
                col: None,
            },
            (Some(file), _) => SourceLoc {
                file,
                line: self.entry_line(entry).unwrap_or_default(),
                col: self.entry_col(entry),
            },
        }
    }

    fn entry_demangled_name(
        &self,
        entry: &Entry,
        lang: &DwLang,
        unit: &Unit,
    ) -> Result<String, Error> {
        Ok(demangler::demangle(&self.entry_name(entry, unit)?, lang))
    }

    fn entry_name(&self, entry: &Entry, unit: &Unit) -> Result<String, Error> {
        [DW_AT_linkage_name, DW_AT_abstract_origin, DW_AT_name]
            .into_iter()
            .find_map(|dw_at| entry.attr_value(dw_at).ok().flatten())
            .ok_or(Error::EntryInAddrNotSymbol)
            .and_then(|attr| match attr {
                AttrValue::UnitRef(offset) => self.entry_name(&unit.entry(offset)?, unit),
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

    fn entry_contains(&self, addr: &Addr, entry: &Entry, unit: &Unit) -> bool {
        entry.pc().is_some_and(|pc| pc.contains(addr))
            || self.entry_ranges_contain(addr, entry, unit) == Some(true)
    }

    fn entry_ranges_contain(&self, addr: &Addr, entry: &Entry, unit: &Unit) -> Option<bool> {
        match entry.attr_value(DW_AT_ranges).ok()?? {
            AttrValue::RangeListsRef(offset) => self
                .ranges(unit, self.ranges_offset_from_raw(unit, offset))
                .and_then(|mut ranges| ranges.any(|range| Ok(range.contains(addr))))
                .ok(),
            _ => None,
        }
    }

    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error> {
        let offset = self.debug_info_offset(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        Ok(self.unit(header)?)
    }

    fn debug_info_offset(&self, addr: &Addr) -> Result<gimli::DebugInfoOffset, Error> {
        self.debug_aranges
            .headers()
            .find_map(|header| {
                Ok(if header.entries().any(|arange| arange.contains(addr))? {
                    Some(header.debug_info_offset())
                } else {
                    None
                })
            })?
            .ok_or(Error::AddrNoDebugOffset(*addr))
    }
}
