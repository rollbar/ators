mod cli;
mod data;

use anyhow::Result;

fn main() -> Result<()> {
    let options = data::Options::from(cli::build().get_matches());
    println!("{:#?}", options);
    Ok(())
}
