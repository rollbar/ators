use crate::{data::*, *};
use fallible_iterator::FallibleIterator;
use gimli::{
    ColumnType, DW_AT_abstract_origin, DW_AT_artificial, DW_AT_call_column, DW_AT_call_file,
    DW_AT_call_line, DW_AT_high_pc, DW_AT_linkage_name, DW_AT_low_pc, DW_AT_name, DW_AT_ranges,
    DW_AT_specification, DebugInfoOffset, LineRow, UnitSectionOffset,
};
use itertools::Either;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

pub fn atos_dwarf(dwarf: &Dwarf, addr: Addr, include_inlined: bool) -> Result<Vec<Symbol>, Error> {
    let unit = dwarf.unit_from_addr(addr)?;
    let mut entries = unit.entries();

    let comp_dir = PathBuf::from(
        &*unit
            .comp_dir
            .ok_or_else(|| Error::CompUnitDirMissing(addr))?
            .to_string_lossy(),
    );

    let mut line_rows = unit
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
            gimli::DW_TAG_subprogram if dwarf.entry_contains(entry, addr, &unit)
        ) {
            break entry;
        }
    };

    if include_inlined && subprogram.has_children() {
        let mut parent = subprogram.clone();
        let mut depth = 0;

        let leaf = loop {
            let Some((step, child)) = entries.next_dfs()? else {
                break parent;
            };

            depth += step;

            if depth <= 0 {
                break parent;
            }

            if matches!(
                child.tag(),
                gimli::DW_TAG_inlined_subroutine if dwarf.entry_contains(child, addr, &unit)
            ) {
                symbols.insert(
                    0,
                    Symbol {
                        addr,
                        name: dwarf.entry_symbol(addr, &parent, &unit)?,
                        loc: Either::Left(dwarf.entry_call_loc(
                            child,
                            &mut line_rows,
                            &comp_dir,
                            &unit,
                        )?),
                    },
                );

                parent = child.clone();
            }
        };

        symbols.insert(
            0,
            Symbol {
                addr,
                name: dwarf.entry_symbol(addr, &leaf, &unit)?,
                loc: Either::Left(dwarf.entry_debug_line(addr, &mut line_rows, &unit)?),
            },
        );
    } else {
        symbols.push(Symbol {
            addr,
            name: dwarf.entry_symbol(addr, subprogram, &unit)?,
            loc: Either::Left(dwarf.entry_debug_line(addr, &mut line_rows, &unit)?),
        });
    }

    Ok(symbols)
}

pub fn atos_map(
    symbol_map: &object::SymbolMap<object::SymbolMapName>,
    addr: Addr,
) -> Result<Vec<Symbol>, Error> {
    let mut symbols = Vec::default();
    let mut symbol_map_iter = symbol_map.symbols().iter().peekable();

    while let (Some(symbol), next_symbol) = (symbol_map_iter.next(), symbol_map_iter.peek()) {
        if addr == symbol.address()
            || (addr > symbol.address()
                && (next_symbol.is_none()
                    || next_symbol.is_some_and(|next| addr < next.address())))
        {
            let demangled_symbol_name = demangler::demangle(
                symbol
                    .name()
                    .strip_prefix('_')
                    .unwrap_or(symbol.name()),
            );

            symbols.push(Symbol {
                addr,
                name: demangled_symbol_name.to_string(),
                loc: Either::Right(Offset::from(*addr - symbol.address())),
            });
        }
    }

    Ok(symbols)
}

trait DwarfExt {
    fn entry_symbol<'a>(
        &'a self,
        addr: Addr,
        entry: &'a Entry,
        unit: &'a Unit,
    ) -> Result<String, Error>;

    fn entry_call_loc(
        &self,
        entry: &Entry,
        line_rows: &mut IncompleteLineProgramRows,
        path: &Path,
        unit: &Unit,
    ) -> Result<SourceLoc, Error>;

    fn entry_debug_line(
        &self,
        addr: Addr,
        line_rows: &mut IncompleteLineProgramRows,
        unit: &Unit,
    ) -> Result<SourceLoc, Error>;

    fn entry_contains(&self, entry: &Entry, addr: Addr, unit: &Unit) -> bool;
    fn entry_pc_contains(&self, entry: &Entry, addr: Addr) -> Option<bool>;
    fn entry_ranges_contain(&self, entry: &Entry, addr: Addr, unit: &Unit) -> Option<bool>;

    fn line_row_file(
        &self,
        row: &LineRow,
        header: &LineProgramHeader,
        unit: &Unit,
    ) -> Result<PathBuf, Error>;

    fn attr_lossy_string<'a>(
        &'a self,
        unit: &Unit<'a>,
        attr: AttrValue<'a>,
    ) -> Result<Cow<str>, gimli::Error>;

    fn unit_from_offset(&self, addr: Addr, offset: DebugInfoOffset) -> Result<Unit, Error>;
    fn unit_from_addr(&self, addr: Addr) -> Result<Unit, Error>;

    fn debug_info_offset(&self, addr: Addr) -> Result<DebugInfoOffset, Error>;
}

impl DwarfExt for Dwarf<'_> {
    fn entry_symbol<'a>(
        &'a self,
        addr: Addr,
        entry: &'a Entry,
        unit: &'a Unit,
    ) -> Result<String, Error> {
        let attr_value = [
            DW_AT_linkage_name,
            DW_AT_abstract_origin,
            DW_AT_specification,
            DW_AT_name,
        ]
        .into_iter()
        .find_map(|dw_at| entry.attr_value(dw_at).ok()?)
        .ok_or(Error::AddrSymbolMissing(addr))?;

        let symbol = match attr_value {
            AttrValue::UnitRef(offset) => self.entry_symbol(addr, &unit.entry(offset)?, unit)?,
            AttrValue::DebugInfoRef(offset) => {
                let new_unit = self.unit_from_offset(addr, offset)?;
                let new_entry = new_unit.entry(
                    UnitSectionOffset::from(offset)
                        .to_unit_offset(&new_unit)
                        .ok_or(Error::AddrDebugInfoRefOffsetOutOfBounds(addr))?,
                )?;

                self.entry_symbol(addr, &new_entry, &new_unit)?
            }
            attr => demangler::demangle(&self.attr_lossy_string(unit, attr)?).into_owned(),
        };

        Ok(symbol)
    }

    fn entry_debug_line(
        &self,
        addr: Addr,
        line_rows: &mut IncompleteLineProgramRows,
        unit: &Unit,
    ) -> Result<SourceLoc, Error> {
        let mut source_locs = Vec::default();

        let mut prev_row: Option<LineRow> = None;
        while let Some((header, line_row)) = line_rows.next_row()? {
            if let Some(prev_row_value) = prev_row {
                let previous_match = prev_row_value.address() <= addr && addr < line_row.address();

                if previous_match {
                    source_locs.push(SourceLoc {
                        file: self.line_row_file(&prev_row_value, header, unit)?,
                        line: prev_row_value
                            .line()
                            .map(|line| line.get())
                            .unwrap_or_default(),
                        col: match prev_row_value.column() {
                            ColumnType::LeftEdge => 0,
                            ColumnType::Column(col) => col.get(),
                        },
                    });
                }
            }

            prev_row = Some(line_row.to_owned());

            if line_row.end_sequence() {
                prev_row = None;
            }
        }

        source_locs
            .pop()
            .ok_or(Error::AddrLineInfoMissing(addr))
    }

    fn entry_call_loc(
        &self,
        entry: &Entry,
        line_rows: &mut IncompleteLineProgramRows,
        path: &Path,
        unit: &Unit,
    ) -> Result<SourceLoc, Error> {
        let Some(file) = (match entry.attr_value(DW_AT_call_file)? {
            Some(AttrValue::FileIndex(offset)) => line_rows.header().file(offset),
            _ => None,
        }) else {
            return Ok(SourceLoc {
                file: path.join("<compiler-generated>"),
                line: 0,
                col: 0,
            });
        };

        let is_artificial = entry.attr_value(DW_AT_artificial) == Ok(Some(AttrValue::Flag(true)));

        Ok(SourceLoc {
            file: file
                .directory(line_rows.header())
                .and_then(|dir| Some(PathBuf::from(&*self.attr_lossy_string(unit, dir).ok()?)))
                .unwrap_or(path.to_path_buf())
                .join(&*self.attr_lossy_string(unit, file.path_name())?),

            line: if !is_artificial {
                entry
                    .attr_value(DW_AT_call_line)?
                    .and_then(|line| line.udata_value())
                    .unwrap_or(0)
            } else {
                0
            },

            col: if !is_artificial {
                entry
                    .attr_value(DW_AT_call_column)?
                    .and_then(|col| col.udata_value())
                    .unwrap_or(0)
            } else {
                0
            },
        })
    }

    fn entry_contains(&self, entry: &Entry, addr: Addr, unit: &Unit) -> bool {
        self.entry_pc_contains(entry, addr)
            .or_else(|| self.entry_ranges_contain(entry, addr, unit))
            .unwrap_or(false)
    }

    fn entry_pc_contains(&self, entry: &Entry, addr: Addr) -> Option<bool> {
        let low = match entry.attr_value(DW_AT_low_pc).ok()?? {
            AttrValue::Addr(addr) => addr,
            _ => None?,
        };

        let high = match entry.attr_value(DW_AT_high_pc).ok()?? {
            AttrValue::Addr(addr) => addr,
            AttrValue::Udata(len) => low + len,
            _ => None?,
        };

        Some((low..high).contains(&addr))
    }

    fn entry_ranges_contain(&self, entry: &Entry, addr: Addr, unit: &Unit) -> Option<bool> {
        let AttrValue::RangeListsRef(offset) = entry.attr_value(DW_AT_ranges).ok()?? else {
            None?
        };

        self.ranges(unit, self.ranges_offset_from_raw(unit, offset))
            .and_then(|mut rs| rs.any(|r| Ok((r.begin..r.end).contains(&addr))))
            .ok()
    }

    fn line_row_file(
        &self,
        row: &LineRow,
        header: &LineProgramHeader,
        unit: &Unit,
    ) -> Result<PathBuf, Error> {
        row.file(header)
            .ok_or_else(|| Error::AddrFileInfoMissing(Addr::from(row.address())))
            .and_then(|file| {
                Ok(match file.directory(header) {
                    Some(dir) if file.directory_index() != 0 => {
                        PathBuf::from(&*self.attr_lossy_string(unit, dir)?)
                    }
                    _ => PathBuf::default(),
                }
                .join(&*self.attr_lossy_string(unit, file.path_name())?))
            })
    }

    fn attr_lossy_string<'input>(
        &'input self,
        unit: &Unit<'input>,
        attr: AttrValue<'input>,
    ) -> Result<Cow<'_, str>, gimli::Error> {
        Ok(self.attr_string(unit, attr)?.to_string_lossy())
    }

    fn unit_from_offset(&self, addr: Addr, offset: DebugInfoOffset) -> Result<Unit, Error> {
        let unit_offset = UnitSectionOffset::from(offset);
        let mut headers = self.units().peekable();
        let header = loop {
            match (headers.next()?, headers.peek()?) {
                (Some(header), Some(next_header))
                    if (header.offset()..next_header.offset()).contains(&unit_offset) =>
                {
                    break header
                }
                (Some(header), None) if unit_offset > header.offset() => break header,
                (None, _) => Err(Error::AddrDebugInfoRefOffsetNofFound(addr))?,
                (_, _) => continue,
            };
        };
        Ok(self.unit(header)?)
    }

    fn unit_from_addr(&self, addr: Addr) -> Result<Unit, Error> {
        let offset = self.debug_info_offset(addr)?;
        let header = self.debug_info.header_from_offset(offset)?;
        Ok(self.unit(header)?)
    }

    fn debug_info_offset(&self, addr: Addr) -> Result<DebugInfoOffset, Error> {
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
                    .any(contains(&addr))?
                    .then(|| header.debug_info_offset()))
            })?
            .ok_or(Error::AddrDebugInfoOffsetMissing(addr))
    }
}
