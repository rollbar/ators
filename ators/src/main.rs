mod cli;

use anyhow::Result;
use atorsl::{data, read::Dump};
use cli::FromArgs;
use std::fs;

fn main() -> Result<()> {
    let context = data::Context::from_args(cli::build().get_matches());

    let file = fs::File::open(&context.object_path)?;
    let mmap = unsafe { memmap2::Mmap::map(&file) }?;

    object::File::parse(&*mmap)?
        .dump()?
        .into_iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
