use anyhow::Result;
use std::{env, path::PathBuf};

pub const TEST_DWARF: &str = "test.dwarf";
pub const TEST_ADDRS: &str = "test_addrs.txt";
pub const TEST_SYMR: &str = "test_symr.txt";

pub fn path(file: &str) -> Result<PathBuf> {
    Ok([&env::var("CARGO_MANIFEST_DIR")?, "..", "fixtures", file]
        .iter()
        .collect())
}
