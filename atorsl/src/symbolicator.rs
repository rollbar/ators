use crate::{data::*, *};
use fallible_iterator::FallibleIterator;
use gimli::{
    ColumnType, DW_AT_abstract_origin, DW_AT_artificial, DW_AT_call_column, DW_AT_call_file,
    DW_AT_call_line, DW_AT_decl_column, DW_AT_decl_file, DW_AT_decl_line, DW_AT_high_pc,
    DW_AT_linkage_name, DW_AT_low_pc, DW_AT_name, DW_AT_ranges, DebugInfoOffset,
};
use itertools::Either;
use object::Object;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

pub fn atos_dwarf(dwarf: &Dwarf, addr: Addr, include_inlined: bool) -> Result<Vec<Symbol>, Error> {
    let unit = dwarf.unit_from_addr(&addr)?;
    let mut entries = unit.entries();

    let builder = &mut CompilationUnitBuilder::default();

    let (_, comp_unit) = entries
        .next_dfs()?
        .ok_or_else(|| Error::AddrNotFound(addr))?;

    let comp_unit = comp_unit
        .attrs()
        .fold(builder, |builder, attr| {
            Ok(match attr.name() {
                gimli::DW_AT_name => builder.name(
                    dwarf
                        .attr_lossy_string(&unit, attr.value())?
                        .into_owned(),
                ),
                gimli::DW_AT_comp_dir => builder.dir(PathBuf::from(
                    &*dwarf.attr_lossy_string(&unit, attr.value())?,
                )),
                gimli::DW_AT_language => match attr.value() {
                    AttrValue::Language(dw_lang) => builder.lang(dw_lang),
                    _ => builder,
                },
                _ => builder,
            })
        })?
        .build()?;

    let mut debug_line_rows = unit
        .line_program
        .clone()
        .ok_or_else(|| Error::CompUnitLineProgramMissing(addr))?
        .rows();

    let mut symbols = Vec::default();

    let subprogram = loop {
        let (_, entry) = entries
            .next_dfs()?
            .ok_or_else(|| Error::AddrNotFound(addr))?;

        if matches!(
            entry.tag(),
            gimli::DW_TAG_subprogram if dwarf.entry_contains(entry, &addr, &unit)
        ) {
            break entry;
        }
    };

    if !subprogram.has_children() {
        symbols.push(Symbol {
            addr,
            name: demangler::demangle(&dwarf.entry_symbol(subprogram, &unit)?, comp_unit.lang),
            loc: Either::Left(Some(dwarf.entry_debug_line(
                &addr,
                &comp_unit.dir,
                &mut debug_line_rows,
                &unit,
            )?)),
        });
    } else if include_inlined && subprogram.has_children() {
        let mut parent = subprogram.clone();
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
                && dwarf.entry_contains(child, &addr, &unit)
            {
                symbols.insert(
                    0,
                    Symbol {
                        addr,
                        name: demangler::demangle(
                            &dwarf.entry_symbol(&parent, &unit)?,
                            comp_unit.lang,
                        ),
                        loc: Either::Left(dwarf.entry_source_loc(child, &comp_unit.dir, &unit)),
                    },
                );

                parent = child.clone();
            }
        };

        symbols.insert(
            0,
            Symbol {
                addr,
                name: demangler::demangle(
                    &dwarf.entry_symbol(&last_child, &unit)?,
                    comp_unit.lang,
                ),
                loc: Either::Left(Some(dwarf.entry_debug_line(
                    &addr,
                    &comp_unit.dir,
                    &mut debug_line_rows,
                    &unit,
                )?)),
            },
        );
    }

    Ok(symbols)
}

pub fn atos_obj(obj: &object::File, addr: Addr) -> Result<Vec<Symbol>, Error> {
    let map = obj.symbol_map();
    let Some(symbol) = map.get(*addr) else {
        Err(Error::AddrNotFound(addr))?
    };

    Ok(vec![Symbol {
        addr: Addr::from(symbol.address()),
        name: demangler::demangle(symbol.name(), None),
        loc: Either::Right(addr - symbol.address()),
    }])
}

trait DwarfExt {
    fn entry_name<'a>(&'a self, entry: &'a Entry, unit: &'a Unit) -> Result<Cow<str>, Error>;
    fn entry_symbol<'a>(&'a self, entry: &'a Entry, unit: &'a Unit) -> Result<Cow<str>, Error>;

    fn entry_source_loc(&self, entry: &Entry, path: &Path, unit: &Unit) -> Option<SourceLoc>;
    fn entry_debug_line(
        &self,
        addr: &Addr,
        comp_dir: &Path,
        line_rows: &mut IncompleteLineProgramRows,
        unit: &Unit,
    ) -> Result<SourceLoc, Error>;

    fn entry_contains(&self, entry: &Entry, addr: &Addr, unit: &Unit) -> bool;
    fn entry_pc_contains(&self, entry: &Entry, addr: &Addr) -> Option<bool>;
    fn entry_ranges_contain(&self, entry: &Entry, addr: &Addr, unit: &Unit) -> Option<bool>;

    fn attr_lossy_string<'a>(
        &'a self,
        unit: &Unit<'a>,
        attr: AttrValue<'a>,
    ) -> Result<Cow<str>, gimli::Error>;
    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error>;
    fn debug_info_offset(&self, addr: &Addr) -> Result<DebugInfoOffset, Error>;
}

impl DwarfExt for Dwarf<'_> {
    fn entry_name<'a>(&'a self, entry: &'a Entry, unit: &'a Unit) -> Result<Cow<str>, Error> {
        Ok(match entry.attr_value(DW_AT_name)? {
            Some(AttrValue::UnitRef(offset)) => Cow::Owned(
                self.entry_name(&unit.entry(offset)?, unit)?
                    .into_owned(),
            ),
            Some(attr) => self.attr_lossy_string(unit, attr)?,
            None => Err(Error::AddrNameMissing)?,
        })
    }

    fn entry_symbol<'a>(&'a self, entry: &'a Entry, unit: &'a Unit) -> Result<Cow<str>, Error> {
        [DW_AT_linkage_name, DW_AT_abstract_origin, DW_AT_name]
            .into_iter()
            .find_map(|dw_at| entry.attr_value(dw_at).ok()?)
            .ok_or(Error::AddrSymbolMissing)
            .and_then(|attr| match attr {
                AttrValue::UnitRef(offset) => Ok(Cow::Owned(
                    self.entry_symbol(&unit.entry(offset)?, unit)?
                        .into_owned(),
                )),
                attr => Ok(self.attr_lossy_string(unit, attr)?),
            })
    }

    fn entry_debug_line(
        &self,
        addr: &Addr,
        comp_dir: &Path,
        line_rows: &mut IncompleteLineProgramRows,
        unit: &Unit,
    ) -> Result<SourceLoc, Error> {
        Ok(loop {
            let (header, row) = line_rows
                .next_row()?
                .ok_or_else(|| Error::AddrLineInfoMissing(*addr))?;

            if row.address() == addr {
                let path = row
                    .file(header)
                    .ok_or_else(|| Error::AddrFileInfoMissing(*addr))
                    .and_then(|file| {
                        let mut path = match file.directory(header) {
                            Some(dir) if file.directory_index() != 0 => {
                                PathBuf::from(&*self.attr_lossy_string(unit, dir)?)
                            }
                            _ => comp_dir.to_path_buf(),
                        };

                        path.push(&*self.attr_lossy_string(unit, file.path_name())?);

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
        })
    }

    fn entry_source_loc(&self, entry: &Entry, path: &Path, unit: &Unit) -> Option<SourceLoc> {
        let Some(AttrValue::FileIndex(offset)) = [DW_AT_decl_file, DW_AT_call_file]
            .into_iter()
            .find_map(|name| entry.attr_value(name).ok()?)
        else {
            return Some(SourceLoc {
                file: path.join("<compiler-generated>"),
                line: 0,
                col: None,
            })
        };

        let header = unit.line_program.as_ref()?.header();
        let file = header.file(offset)?;
        let is_artificial = entry.attr_value(DW_AT_artificial) == Ok(Some(AttrValue::Flag(true)));

        Some(SourceLoc {
            file: PathBuf::from(
                &*self
                    .attr_lossy_string(unit, file.directory(header)?)
                    .ok()?,
            )
            .join(&*self.attr_lossy_string(unit, file.path_name()).ok()?),

            line: if is_artificial {
                0
            } else {
                [DW_AT_decl_line, DW_AT_call_line]
                    .into_iter()
                    .find_map(|name| entry.attr_value(name).ok()??.u16_value())?
            },

            col: if is_artificial {
                Some(0)
            } else {
                [DW_AT_decl_column, DW_AT_call_column]
                    .into_iter()
                    .find_map(|name| entry.attr_value(name).ok()??.u16_value())
            },
        })
    }

    fn entry_contains(&self, entry: &Entry, addr: &Addr, unit: &Unit) -> bool {
        self.entry_pc_contains(entry, addr)
            .or_else(|| self.entry_ranges_contain(entry, addr, unit))
            .unwrap_or(false)
    }

    fn entry_pc_contains(&self, entry: &Entry, addr: &Addr) -> Option<bool> {
        let low = match entry.attr_value(DW_AT_low_pc).ok()?? {
            AttrValue::Addr(addr) => addr,
            _ => None?,
        };

        let high = match entry.attr_value(DW_AT_high_pc).ok()?? {
            AttrValue::Addr(addr) => addr,
            AttrValue::Udata(len) => low + len,
            _ => None?,
        };

        Some((low..high).contains(addr))
    }

    fn entry_ranges_contain(&self, entry: &Entry, addr: &Addr, unit: &Unit) -> Option<bool> {
        let AttrValue::RangeListsRef(offset) = entry.attr_value(DW_AT_ranges).ok()?? else {
            None?
        };

        self.ranges(unit, self.ranges_offset_from_raw(unit, offset))
            .and_then(|mut rs| rs.any(|r| Ok((r.begin..r.end).contains(addr))))
            .ok()
    }

    fn attr_lossy_string<'input>(
        &'input self,
        unit: &Unit<'input>,
        attr: AttrValue<'input>,
    ) -> Result<Cow<'_, str>, gimli::Error> {
        Ok(self.attr_string(unit, attr)?.to_string_lossy())
    }

    fn unit_from_addr(&self, addr: &Addr) -> Result<Unit, Error> {
        let offset = self.debug_info_offset(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        Ok(self.unit(header)?)
    }

    fn debug_info_offset(&self, addr: &Addr) -> Result<DebugInfoOffset, Error> {
        let contains = |addr| {
            move |arange: gimli::ArangeEntry| {
                arange
                    .address()
                    .checked_add(arange.length())
                    .map(|address_end| (arange.address()..address_end).contains(addr))
                    .ok_or(gimli::Error::InvalidAddressRange)
            }
        };

        self.debug_aranges
            .headers()
            .find_map(|header| {
                Ok(header
                    .entries()
                    .any(contains(addr))?
                    .then(|| header.debug_info_offset()))
            })?
            .ok_or(Error::AddrDebugInfoOffsetMissing(*addr))
    }
}
