use crate::data::Address;
use std::path::PathBuf;

/// The program's context, defines its behavior.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Context {
    pub object_path: PathBuf,
    pub load_address: Address,
    pub addresses: Vec<Address>,
    pub architecture: Option<String>,
    pub expand_inline: bool,
    pub verbose: bool,
}
