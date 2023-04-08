#![allow(unstable_name_collisions)]

mod cli;
mod context;

use anyhow::{Context as _, Result};
use atorsl::{data::*, ext::object::File, *};
use context::{Context, Loc};
use itertools::{Either, Itertools};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<()> {
    let (mmap, cow);

    let args = cli::build().get_matches();
    let ctx = Context::from_args(&args)?;

    let obj = load_object!(ctx.obj_path, mmap)?;
    let dwarf = load_dwarf!(&obj, cow);

    symbolicate(&dwarf, &obj, &obj.vmaddr()?, &ctx)?
        .iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}

fn symbolicate(
    dwarf: &Dwarf,
    object: &object::File,
    vmaddr: &Addr,
    ctx: &Context,
) -> Result<Vec<String>> {
    let base_addr = match ctx.loc {
        Loc::Load(load_addr) => load_addr - vmaddr,
        Loc::Slide(slide_addr) => *slide_addr,
    };

    let addrs = if let Some(file) = ctx.input_addr_file {
        addrs_from_file(file)?
    } else {
        ctx.addrs.clone().context("No input address")?
    };

    Ok(addrs
        .iter()
        .map(
            |addr| match atos_dwarf(dwarf, addr, &base_addr, ctx.include_inlined) {
                Err(Error::AddrNotFound(addr)) => Ok(atos_obj(object, &addr, &base_addr)?),
                result @ _ => result,
            },
        )
        .map(|result_of_symbol_with_inlines| {
            Ok(result_of_symbol_with_inlines?
                .iter()
                .map(|symbol| format(symbol, ctx))
                .join("\n"))
        })
        .map(|result_of_symbol| match result_of_symbol {
            Ok(symbol) => symbol,
            Err(Error::AddrNotFound(addr)) => addr.to_string(),
            Err(err) => err.to_string(),
        })
        .intersperse(ctx.delimiter.to_string())
        .collect())
}

fn format(symbol: &Symbol, ctx: &Context) -> String {
    match symbol.loc.as_ref() {
        Either::Left(source_loc) => {
            format!(
                "{} (in {}) ({}:{})",
                symbol.linkage,
                symbol.module,
                if ctx.show_full_path {
                    source_loc.file.display().to_string()
                } else {
                    source_loc
                        .file
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                },
                source_loc.line,
            )
        }
        Either::Right(offset) => {
            format!(
                "{} (in {}) + {}",
                symbol.linkage,
                ctx.obj_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                **offset
            )
        }
    }
}

fn addrs_from_file(file: &Path) -> Result<Vec<Addr>> {
    Ok(fs::File::open(file)
        .map(BufReader::new)?
        .split(b' ')
        .flat_map(|buf| Result::<Addr>::Ok(buf?.try_into()?))
        .collect())
}
