#![allow(unstable_name_collisions)]

mod cli;
mod context;

use atorsl::{ext::object::File, *};
use context::{Context, Loc};
use itertools::Itertools;

fn format(symbol: Symbol) -> String {
    format!(
        "{} (in {}) ({}{}{})",
        symbol.linkage,
        symbol.module,
        symbol.file.unwrap_or_default().display(),
        symbol
            .line
            .map(|l| format!(":{}", l))
            .unwrap_or_default(),
        symbol
            .col
            .map(|l| format!(":{}", l))
            .unwrap_or_default(),
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
            symbolicator
                .atos(*addr, &base_addr, ctx.include_inlined)
                .map(|symbols| symbols.into_iter().map(format).join("\n"))
                .unwrap_or(addr.to_string())
        })
        .intersperse(ctx.delimiter.to_string())
        .collect()
}

fn main() -> anyhow::Result<()> {
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
