mod cli;

use anyhow::Result;
use atorsl::{data::Context, load_dwarf, load_object, read::Dump};
use cli::FromArgs;

fn main() -> Result<()> {
    let mmap;
    let cow;
    let context = Context::from_args(cli::build().get_matches());
    let object = load_object!(context.object_path, mmap)?;
    let dwarf = load_dwarf!(object, cow);

    object
        .dump()?
        .into_iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
