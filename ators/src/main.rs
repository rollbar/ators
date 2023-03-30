use anyhow::Result;
use control::Dump;
use data::Context;
use memmap2::Mmap;
use std::fs;

mod cli;
mod control;
mod data;

fn main() -> Result<()> {
    let context = Context::from(cli::build().get_matches());
    let file = fs::File::open(&context.object_path)?;
    let mmap = unsafe { Mmap::map(&file) }?;

    object::File::parse(&*mmap)?
        .dump_sections()?
        .into_iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
