#![allow(unstable_name_collisions)]

mod cli;
mod context;

use anyhow::Result;
use atorsl::{ext::object::File, *};
use context::{Context, Loc};
use itertools::Itertools;

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

fn symbolicate<S: Symbolicator>(symbolicator: &S, vmaddr: &Addr, ctx: &Context) -> Vec<String> {
    let base_addr = match ctx.loc {
        Loc::Load(addr) => addr - vmaddr,
        Loc::Slide(addr) => *addr,
        Loc::Offset => Addr::nil(),
    };

    ctx.addrs
        .iter()
        .map(|addr| {
            Ok(symbolicator
                .atos(*addr, &base_addr, ctx.include_inlined)?
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
        .collect()
}

fn main() -> Result<()> {
    let (mmap, cow);

    let args = cli::build().get_matches();
    let ctx = Context::from_args(&args)?;

    let obj = load_object!(ctx.path, mmap)?;
    let dwarf = load_dwarf!(&obj, cow);

    symbolicate(&dwarf, &obj.vmaddr()?, &ctx)
        .iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
