mod cli;

use atorsl::{ext::object::File, *};
use cli::FromArgs;

fn main() -> anyhow::Result<()> {
    let (mmap, cow);

    let context = Context::from_args(cli::build().get_matches());
    let object = load_object!(context.objpath, mmap)?;

    load_dwarf!(&object, cow)
        .symbolicate(object.vmaddr()?, &context)?
        .into_iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
