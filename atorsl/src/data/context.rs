use crate::data::Address;
use std::path::PathBuf;

/// The program's context, defines its behavior.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Context {
    pub objpath: PathBuf,
    pub loadaddr: Address,
    pub addrs: Vec<Address>,
    pub arch: Option<String>,
    pub inline: bool,
    pub verbose: bool,
}
