#![feature(result_option_inspect)]

mod cli;
mod data;
mod lookup;
mod prelude;

use anyhow::Result;
use lookup::lookup;
use prelude::void;

fn print(xs: &Vec<String>) -> () {
    xs.into_iter().for_each(|x| println!("{x}"))
}

fn main() -> Result<()> {
    lookup(
        cli::build()
            .try_get_matches()
            .map(data::Options::from)?,
    )
    .inspect(print)
    .map(void)
}
