use crate::{
    data::*,
    ext::gimli::{ArangeEntry, DebuggingInformationEntry, Range},
    *,
};
use fallible_iterator::FallibleIterator;
use gimli::{
    ColumnType, DW_AT_call_column, DW_AT_call_file, DW_AT_call_line, DW_AT_decl_column,
    DW_AT_decl_file, DW_AT_decl_line, DW_AT_ranges, DebugInfoOffset,
};
use itertools::Either;
use object::Object;
use std::{borrow::Cow, path::PathBuf};

pub fn atos_dwarf(
    dwarf: &Dwarf,
    addr: &Addr,
    include_inlined: bool,
) -> Result<Vec<Symbol>, Error> {
    let unit = dwarf.unit_from_addr(addr)?;
    let mut entries = unit.entries();
    let mut symbols = Vec::default();

    let (_, comp_unit) = entries
        .next_dfs()?
        .ok_or_else(|| Error::AddrNotFound(*addr))?;

    let comp_unit = comp_unit
        .attrs()
        .fold(
            &mut CompilationUnitBuilder::default(),
            |builder, attr| match attr.name() {
                gimli::DW_AT_name => {
                    let value = attr.value();
                    Ok(builder.name((value, dwarf.attr_string(&unit, value)?.to_string_lossy())))
                }
                gimli::DW_AT_comp_dir => {
                    let value = attr.value();
                    Ok(builder.dir((
                        value,
                        PathBuf::from(&*dwarf.attr_string(&unit, value)?.to_string_lossy()),
                    )))
                }
                gimli::DW_AT_language => match attr.value() {
                    AttrValue::Language(dw_lang) => Ok(builder.lang(dw_lang)),
                    _ => Ok(builder),
                },
                _ => Ok(builder),
            },
        )?
        .build()?;

    let subprogram = loop {
        let (_, entry) = entries
            .next_dfs()?
            .ok_or_else(|| Error::AddrNotFound(*addr))?;

        match entry.tag() {
            gimli::DW_TAG_subprogram if entry.pc().is_some_and(|pc| pc.contains(addr)) => {
                symbols.push(Symbol {
                    name: demangler::demangle(&entry.symbol_name(dwarf, &unit)?, comp_unit.lang),
                    loc: Either::Left(dwarf.entry_source_loc(entry, &unit)),
                });

                break entry;
            }
            _ => continue,
        }
    };

    if include_inlined && subprogram.has_children() {
        let mut parent = Option::<Entry>::None;
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
                            name: demangler::demangle(
                                &parent.symbol_name(dwarf, &unit)?,
                                comp_unit.lang,
                            ),
                            loc: Either::Left(dwarf.entry_source_loc(child, &unit)),
                        },
                    );
                }

                parent = Some(child.clone());
            }
        };

        if let Some(last_child) = last_child {
            let mut rows = unit
                .line_program
                .clone()
                .ok_or_else(|| Error::CompUnitLineProgramMissing(*addr))?
                .rows();

            let source_loc = loop {
                let (header, row) = rows
                    .next_row()?
                    .ok_or_else(|| Error::AddrLineInfoMissing(*addr))?;

                if row.address() == addr {
                    let path = row
                        .file(header)
                        .ok_or_else(|| Error::AddrFileInfoMissing(*addr))
                        .and_then(|file| {
                            let mut path = match file.directory(header) {
                                Some(dir) if file.directory_index() != 0 => PathBuf::from(
                                    &*dwarf.attr_string(&unit, dir)?.to_string_lossy(),
                                ),
                                _ => comp_unit.dir.1.clone(),
                            };

                            path.push(
                                &*dwarf
                                    .attr_string(&unit, file.path_name())?
                                    .to_string_lossy(),
                            );

                            Ok(path)
                        })?;

                    break SourceLoc {
                        file: path,
                        line: row.line().map(|l| l.get()).unwrap_or_default() as u16,
                        col: match row.column() {
                            ColumnType::LeftEdge => Some(0),
                            ColumnType::Column(c) => Some(c.get() as u16),
                        },
                    };
                }
            };

            symbols.insert(
                0,
                Symbol {
                    name: demangler::demangle(
                        &last_child.symbol_name(dwarf, &unit)?,
                        comp_unit.lang,
                    ),
                    loc: Either::Left(Some(source_loc)),
                },
            );
        }
    }

    Ok(symbols)
}

pub fn atos_obj(obj: &object::File, addr: Addr) -> Result<Vec<Symbol>, Error> {
    let map = obj.symbol_map();
    let Some(symbol) = map.get(*addr) else {
        Err(Error::AddrNotFound(addr))?
    };

    Ok(vec![Symbol {
        name: demangler::demangle(symbol.name(), None),
        loc: Either::Right(addr - symbol.address()),
    }])
}

trait DwarfExt {
    fn entry_source_loc(&self, entry: &Entry, unit: &Unit) -> Option<SourceLoc>;

    fn entry_contains(&self, addr: &Addr, entry: &Entry, unit: &Unit) -> bool;
    fn entry_ranges_contain(&self, addr: &Addr, entry: &Entry, unit: &Unit) -> Option<bool>;

    fn attr_lossy_string<'a>(&'a self, unit: &Unit<'a>, attr: AttrValue<'a>) -> Option<Cow<str>>;
    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error>;
    fn debug_info_offset(&self, addr: &Addr) -> Result<DebugInfoOffset, Error>;
}

impl DwarfExt for Dwarf<'_> {
    fn entry_source_loc(&self, entry: &Entry, unit: &Unit) -> Option<SourceLoc> {
        let AttrValue::FileIndex(offset) = [DW_AT_decl_file, DW_AT_call_file]
            .into_iter()
            .find_map(|name| entry.attr_value(name).ok()?)?
        else {
            None?
        };

        let header = unit.line_program.as_ref()?.header();
        let file = header.file(offset)?;

        Some(SourceLoc {
            file: PathBuf::from(&*self.attr_lossy_string(unit, file.directory(header)?)?)
                .join(&*self.attr_lossy_string(unit, file.path_name())?),

            line: [DW_AT_decl_line, DW_AT_call_line]
                .into_iter()
                .find_map(|name| entry.attr_value(name).ok()??.u16_value())?,

            col: [DW_AT_decl_column, DW_AT_call_column]
                .into_iter()
                .find_map(|name| entry.attr_value(name).ok()??.u16_value()),
        })
    }

    fn entry_contains(&self, addr: &Addr, entry: &Entry, unit: &Unit) -> bool {
        entry.pc().is_some_and(|pc| pc.contains(addr))
            || self.entry_ranges_contain(addr, entry, unit) == Some(true)
    }

    fn entry_ranges_contain(&self, addr: &Addr, entry: &Entry, unit: &Unit) -> Option<bool> {
        let AttrValue::RangeListsRef(offset) = entry.attr_value(DW_AT_ranges).ok()?? else {
            None?
        };

        self.ranges(unit, self.ranges_offset_from_raw(unit, offset))
            .and_then(|mut ranges| ranges.any(|range| Ok(range.contains(addr))))
            .ok()
    }

    fn attr_lossy_string<'a>(&'a self, unit: &Unit<'a>, attr: AttrValue<'a>) -> Option<Cow<str>> {
        Some(self.attr_string(unit, attr).ok()?.to_string_lossy())
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
