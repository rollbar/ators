mod cli;
mod opt;

use atorsl::{ext::object::File, *};
use opt::FromArgs;

fn main() -> anyhow::Result<()> {
    let (mmap, cow);

    let context = Context::from_args(cli::build().get_matches())
        .expect("Couldn't build Context from arguments");
    let object = load_object!(context.path, mmap)?;

    load_dwarf!(&object, cow)
        .symbolicate(object.vmaddr()?, &context)?
        .iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
