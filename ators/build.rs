use anyhow::{anyhow, Result};
use memmap2::Mmap;
use object::Object;
use std::{
    env, fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    process::Command,
};

const SYMBOLICATE: &str = "/usr/bin/atos";
const TEST_DWARF: &str = "test.dwarf";
const TEST_ADDRS: &str = "test_addrs.txt";
const TEST_SYMR: &str = "test_symr.txt";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={}", path(TEST_ADDRS)?.display());
    println!("cargo:rerun-if-changed={}", path(TEST_SYMR)?.display());

    path(TEST_ADDRS)?
        .exists()
        .eq(&false)
        .then(generate_test_addrs_file);

    path(TEST_SYMR)?
        .exists()
        .eq(&false)
        .then(generate_test_symr_file);

    Ok(())
}

fn path(file: &str) -> Result<PathBuf> {
    Ok([&env::var("CARGO_MANIFEST_DIR")?, "..", "fixtures", file]
        .iter()
        .collect())
}

fn generate_test_addrs_file() -> Result<()> {
    if !path(TEST_DWARF)?.exists() {
        Err(anyhow!("Can't generate test addrs without test.dwarf"))?;
    }

    let mmap = unsafe { Mmap::map(&fs::File::open(path(TEST_DWARF)?)?) }?;
    let object = object::File::parse(&*mmap)?;
    let mut addrs_file = io::BufWriter::new(fs::File::create(path(TEST_ADDRS)?)?);

    for symbol in object.symbol_map().symbols() {
        write!(addrs_file, "{:#018x} ", symbol.address())?;
    }

    Ok(addrs_file.flush()?)
}

fn generate_test_symr_file() -> Result<()> {
    let output = Command::new(SYMBOLICATE)
        .args([
            "-i",
            "-o",
            &path(TEST_DWARF)?.to_string_lossy(),
            "-s",
            "0",
            "-f",
            &path(TEST_ADDRS)?.to_string_lossy(),
        ])
        .output()?;

    let mut symr_file = io::BufWriter::new(fs::File::create(path(TEST_SYMR)?)?);

    for symbol in output.stdout.lines() {
        writeln!(symr_file, "{}", symbol?)?;
    }

    Ok(symr_file.flush()?)
}
