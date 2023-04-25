#![allow(unstable_name_collisions)]

mod cli;
mod context;

use anyhow::{Context as _, Result};
use atorsl::{
    data::{Addr, Symbol},
    ext::object::{Architecture as _, File as _},
    *,
};
use context::{Context, Loc, Mode};
use fallible_iterator::FallibleIterator;
use itertools::{Either, Itertools};
use memmap2::Mmap;
use object::Object;
use std::{
    borrow::Cow,
    fs,
    io::{self, BufRead},
    path::Path,
    str,
};
use uuid::Uuid;

fn main() -> Result<()> {
    let args = cli::build().get_matches();
    let ctx = Context::from_args(&args)?;

    let mmap = unsafe { Mmap::map(&fs::File::open(&ctx.obj_path)?) }?;
    let obj = object::File::parse_data(&mmap, ctx.arch)?;

    match ctx.mode {
        Mode::Symbolicate => {
            let cow;
            let dwarf = load_dwarf!(&obj, cow);
            let addrs = compute_addrs(&obj, &ctx)?;

            symbolicate(&dwarf, &obj, &addrs, &ctx)
                .iter()
                .for_each(|symbol| println!("{symbol}"));
        }

        Mode::PrintUuid => {
            println!(
                "    {:X} {:#8} {}",
                Uuid::from_bytes(obj.mach_uuid()?.ok_or(Error::ObjectHasNoUuid)?).hyphenated(),
                obj.architecture().name(),
                ctx.obj_path.to_string_lossy(),
            );
        }
    }

    Ok(())
}

fn symbolicate(dwarf: &Dwarf, obj: &object::File, addrs: &[Addr], ctx: &Context) -> Vec<String> {
    let iter_symbols = addrs
        .iter()
        .map(|addr| {
            let symbols = match atos_dwarf(dwarf, addr, ctx.include_inlined) {
                Err(Error::AddrNotFound(addr)) | Err(Error::AddrDebugInfoOffsetMissing(addr)) => {
                    atos_obj(obj, addr)?
                }
                symbols => symbols?,
            };

            let symbol = symbols
                .iter()
                .map(|symbol| format(symbol, ctx))
                .join("\n");

            Ok(symbol)
        })
        .map(|symbol| match symbol {
            Ok(symbol) => symbol,
            Err(Error::AddrNotFound(addr)) => addr.to_string(),
            Err(err) => err.to_string(),
        });

    if ctx.include_inlined {
        iter_symbols
            .intersperse(ctx.delimiter.to_string())
            .chain([ctx.delimiter.to_string()])
            .collect()
    } else {
        iter_symbols.collect()
    }
}

fn format(symbol: &Symbol, ctx: &Context) -> String {
    let symbol_addr_fmt = if ctx.show_addrs {
        format!("{}: ", symbol.addr)
    } else {
        String::default()
    };

    match symbol.loc.as_ref() {
        Either::Left(Some(source_loc)) => {
            format!(
                "{}{} (in {}) ({}:{})",
                symbol_addr_fmt,
                symbol.name,
                ctx.obj_path.lossy_file_name(),
                if ctx.show_full_path {
                    source_loc.file.to_string_lossy()
                } else {
                    source_loc.file.lossy_file_name()
                },
                source_loc.line,
            )
        }
        Either::Left(None) => {
            format!(
                "{}{} (in {}) (?)",
                symbol_addr_fmt,
                symbol.name,
                ctx.obj_path.lossy_file_name(),
            )
        }
        Either::Right(offset) => {
            format!(
                "{}{} (in {}) + {}",
                symbol_addr_fmt,
                symbol.name,
                ctx.obj_path.lossy_file_name(),
                **offset
            )
        }
    }
}

fn compute_addrs(obj: &object::File, ctx: &Context) -> Result<Vec<Addr>> {
    let addrs = match &ctx.addrs {
        Either::Left(addrs) => addrs.clone(),
        Either::Right(file) => io::BufReader::new(fs::File::open(file)?)
            .split(b' ')
            .flat_map(|buf| Addr::try_from(&*buf?))
            .collect(),
    };

    let offset_addr = match ctx.base_addr {
        Loc::Offset => *obj.vmaddr()? as i64,
        Loc::Slide(slide) => -(**slide as i64),
        Loc::Load(load_addr) => {
            -(load_addr
                .checked_sub(*obj.vmaddr()?)
                .context(format!("Invalid load address: {}", load_addr))? as i64)
        }
    };

    fallible_iterator::convert(addrs.iter().map(|addr| {
        addr.checked_add_signed(offset_addr)
            .map(Addr::from)
            .context(format!("Invalid address: {}", addr))
    }))
    .collect()
}

trait LossyFileName {
    fn lossy_file_name(&self) -> Cow<'_, str>;
}

impl LossyFileName for Path {
    fn lossy_file_name(&self) -> Cow<'_, str> {
        self.file_name().unwrap_or_default().to_string_lossy()
    }
}
