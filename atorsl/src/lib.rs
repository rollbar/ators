pub mod data;
pub mod load;
pub mod read;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error reading DWARF")]
    Gimli(#[from] gimli::Error),

    #[error("Error reading binary image object")]
    Object(#[from] object::read::Error),
}
