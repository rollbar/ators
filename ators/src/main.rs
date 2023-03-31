mod cli;

use anyhow::Result;
use atorsl::{data::Context, load_object, read::Dump};
use cli::FromArgs;

fn main() -> Result<()> {
    let binding;
    let context = Context::from_args(cli::build().get_matches());
    load_object!(context.object_path, binding)?
        .dump()?
        .into_iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
