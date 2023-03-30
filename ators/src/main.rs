use anyhow::Result;
use control::{dump_object, dump_sections};
use memmap2::Mmap;
use std::fs;

mod cli;
mod control;
mod data;

fn lookup(context: data::Context) -> Result<Vec<String>> {
    let file = fs::File::open(context.object_path)?;
    let mmap = unsafe { Mmap::map(&file) }?;
    let object = object::File::parse(&*mmap)?;
    dump_sections(&object)
}

fn main() -> Result<()> {
    lookup(
        cli::build()
            .try_get_matches()
            .map(data::Context::from)?,
    )?
    .into_iter()
    .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
