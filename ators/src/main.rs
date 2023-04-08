#![allow(unstable_name_collisions)]

mod cli;
mod context;

use anyhow::{Context as _, Result};
use atorsl::{ext::object::File, *};
use context::{Context, Loc};
use itertools::Itertools;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

fn format(symbol: &Symbol, show_full_path: bool) -> String {
    format!(
        "{} (in {}) ({}:{})",
        symbol.linkage,
        symbol.module,
        if show_full_path {
            symbol.file.display().to_string()
        } else {
            symbol
                .file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        },
        symbol.line,
    )
}

fn addrs_from_file(file: &Path) -> Result<Vec<Addr>> {
    Ok(fs::File::open(file)
        .map(BufReader::new)?
        .split(b' ')
        .flat_map(|buf| -> Result<Addr> { Ok(Addr::from_str(&String::from_utf8(buf?)?)?) })
        .collect())
}

fn symbolicate<S: Symbolicator>(
    symbolicator: &S,
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
        .map(|addr| {
            Ok(symbolicator
                .atos(addr, &base_addr, ctx.include_inlined)?
                .iter()
                .map(|symbol| format(symbol, ctx.show_full_path))
                .join("\n"))
        })
        .map(|symbol| match symbol {
            Ok(symbol) => symbol.to_owned(),
            Err(Error::AddrNotFound(addr)) => addr.to_string(),
            Err(err) => err.to_string(),
        })
        .intersperse(ctx.delimiter.to_string())
        .collect())
}

fn main() -> Result<()> {
    let (mmap, cow);

    let args = cli::build().get_matches();
    let ctx = Context::from_args(&args)?;

    let obj = load_object!(ctx.obj_path, mmap)?;
    let dwarf = load_dwarf!(&obj, cow);

    symbolicate(&dwarf, &obj.vmaddr()?, &ctx)?
        .iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
