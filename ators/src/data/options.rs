use crate::data::Address;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Options {
    pub object: PathBuf,
    pub load_address: Address,
    pub addresses: Vec<Address>,
    pub architecture: Option<String>,
    pub expand_inline: bool,
    pub verbose: bool,
}
