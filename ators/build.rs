use anyhow::{anyhow, Result};
use memmap2::Mmap;
use object::Object;
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

const TEST_DWARF: &str = "test.dwarf";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../fixtures/test_addrs.txt");

    test_addrs_path()?
        .exists()
        .eq(&false)
        .then(generate_test_addrs_file);

    Ok(())
}

fn pwd() -> Result<String, env::VarError> {
    env::var("CARGO_MANIFEST_DIR")
}

fn test_addrs_path() -> Result<PathBuf, env::VarError> {
    Ok([&pwd()?, "..", "fixtures", "test_addrs.txt"]
        .iter()
        .collect())
}

fn generate_test_addrs_file() -> Result<()> {
    let path = [&pwd()?, "..", "fixtures", "objects", TEST_DWARF]
        .iter()
        .collect::<PathBuf>();

    if !path.exists() {
        Err(anyhow!("Can't generate test addrs without test.dwarf"))?;
    }

    let mmap = unsafe { Mmap::map(&fs::File::open(path)?) }?;
    let object = object::File::parse(&*mmap)?;
    let mut addrs_file = io::BufWriter::new(fs::File::create(test_addrs_path()?)?);

    for symbol in object.symbol_map().symbols() {
        write!(addrs_file, "{:#018x} ", symbol.address())?;
    }

    Ok(addrs_file.flush()?)
}
