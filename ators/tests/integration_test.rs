use anyhow::Result;
use pretty_assertions::assert_str_eq;
use std::{
    env, fs,
    io::{self, BufRead},
    path::PathBuf,
    process::Command,
    str,
};

pub const TEST_DWARF: &str = "test.dwarf";
pub const TEST_ADDRS: &str = "test_addrs.txt";
pub const TEST_SYMR: &str = "test_symr.txt";

pub fn path(file: &str) -> Result<PathBuf> {
    Ok([&env::var("CARGO_MANIFEST_DIR")?, "..", "fixtures", file]
        .iter()
        .collect())
}

#[test]
fn test() {
    let output = Command::new("../target/debug/ators")
        .args([
            "--prefixAddr",
            "-i",
            "-o",
            &path(TEST_DWARF)
                .expect("test dwarf to exist")
                .to_string_lossy(),
            "-s",
            "0",
            "-f",
            &path(TEST_ADDRS)
                .expect("test addrs to exist")
                .to_string_lossy(),
        ])
        .output()
        .expect("ators to run");

    str::from_utf8(&output.stdout)
        .expect("ators output to be utf8")
        .lines()
        .map(|line| line.split_once(": ").unwrap_or(("?", line)))
        .zip(
            io::BufReader::new(
                fs::File::open(path(TEST_SYMR).expect("test symr to exist"))
                    .expect("test symr to be opened"),
            )
            .lines()
            .map(|line| line.expect("test symr line to be ok")),
        )
        .enumerate()
        .for_each(|(count, ((addr, actual), expected))| {
            assert_str_eq!(actual, expected, "in line {} for {}", count + 1, addr)
        });
}
