#![allow(unstable_name_collisions)]

mod cli;
mod context;

use anyhow::{Context as _, Result};
use atorsl::{
    data::{Addr, Symbol},
    ext::object::File as _,
    *,
};
use context::{Context, Loc};
use itertools::{Either, Itertools};
use std::{fs, io, io::BufRead, path::Path};

fn main() -> Result<()> {
    let (mmap, cow);

    let args = cli::build().get_matches();
    let ctx = Context::from_args(&args)?;

    let obj = load_object!(ctx.obj_path, ctx.arch, mmap)?;
    let dwarf = load_dwarf!(&obj, cow);

    let addrs = compute_addrs(&obj, &ctx)?;

    symbolicate(&dwarf, &obj, &addrs, &ctx)
        .iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}

fn symbolicate(dwarf: &Dwarf, obj: &object::File, addrs: &[Addr], ctx: &Context) -> Vec<String> {
    let iter_symbols = addrs
        .iter()
        .map(|addr| {
            let symbols = match atos_dwarf(dwarf, addr, ctx.include_inlined) {
                Err(Error::AddrNotFound(addr)) | Err(Error::AddrNoDebugOffset(addr)) => {
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

    match ctx.delimiter {
        Some(delimiter) if ctx.include_inlined => iter_symbols
            .intersperse(delimiter.to_string())
            .chain([delimiter.to_string()])
            .collect(),

        _ => iter_symbols.collect(),
    }
}

fn format(symbol: &Symbol, ctx: &Context) -> String {
    match symbol.loc.as_ref() {
        Either::Left(source_loc) => {
            format!(
                "{} (in {}) ({}:{})",
                symbol.name,
                ctx.obj_path.lossy_file_name(),
                if ctx.show_full_path {
                    source_loc.file.to_string_lossy().to_string()
                } else {
                    source_loc.file.lossy_file_name()
                },
                source_loc.line,
            )
        }
        Either::Right(offset) => {
            format!(
                "{} (in {}) + {}",
                symbol.name,
                ctx.obj_path.lossy_file_name(),
                **offset
            )
        }
    }
}

fn compute_addrs(obj: &object::File, ctx: &Context) -> Result<Vec<Addr>> {
    let addrs = if let Some(file) = ctx.input_addr_file {
        fs::File::open(file)
            .map(io::BufReader::new)?
            .split(b' ')
            .flat_map(|buf| Result::<Addr>::Ok(buf?.try_into()?))
            .collect()
    } else {
        ctx.addrs.clone().context("No input address")?
    };

    let offset_addr = match ctx.base_addr {
        Loc::Load(load_addr) => {
            -(load_addr
                .checked_sub(*obj.vmaddr()?)
                .context(format!("Invalid load address: {}", load_addr))? as i64)
        }
        Loc::Slide(slide) => -(**slide as i64),
        Loc::Offset => *obj.vmaddr()? as i64,
    };

    addrs
        .iter()
        .map(|addr| {
            Ok(addr
                .checked_add_signed(offset_addr)
                .context(format!("Invalid address: {}", addr))?
                .into())
        })
        .collect()
}

trait LossyFileName {
    fn lossy_file_name(&self) -> String;
}

impl LossyFileName for Path {
    fn lossy_file_name(&self) -> String {
        self.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
}
