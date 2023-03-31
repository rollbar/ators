mod cli;

use anyhow::Result;
use atorsl::{
    data::{Context, ObjectExt},
    load_dwarf, load_object,
    read::Lookup,
};
use cli::FromArgs;

fn main() -> Result<()> {
    let context = Context::from_args(cli::build().get_matches());

    let (mmap, cow);
    let object = load_object!(context.object_path, mmap)?;

    load_dwarf!(object, cow)
        .lookup(object.vmaddr()?, context)?
        .into_iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
