mod cli;
mod opt;

use atorsl::{ext::object::File, *};
use opt::FromArgs;

fn main() -> anyhow::Result<()> {
    let (mmap, cow);

    let args = cli::build().get_matches();
    let ctx = Context::from_args(&args).expect("Cannot build Context from arguments");
    let obj = load_object!(ctx.path, mmap)?;

    load_dwarf!(&obj, cow)
        .symbolicate(obj.vmaddr()?, &ctx)?
        .iter()
        .for_each(|symbol| println!("{symbol}"));

    Ok(())
}
