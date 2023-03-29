mod cli;

use anyhow::Result;

#[tracing::instrument]
fn main() -> Result<()> {
    match cli::build().get_matches().subcommand() {
        Some(("run", args)) => Ok(()),
        Some(("check", _)) => {
            unimplemented!()
        }
        _ => unreachable!(),
    }
}
