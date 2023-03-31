mod cli;
mod control;
mod data;

use anyhow::Result;
use control::Dump;
use std::fs;

fn main() -> Result<()> {
    let context = data::Context::from(cli::build().get_matches());

    let file = fs::File::open(&context.object_path)?;
    let mmap = unsafe { memmap2::Mmap::map(&file) }?;

    object::File::parse(&*mmap)?
        .dump_sections()?
        .into_iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
