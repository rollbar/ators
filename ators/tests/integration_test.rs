mod common;

use common::*;
use pretty_assertions::assert_eq;
use std::{
    fs,
    io::{self, BufRead},
    process::Command,
};

#[test]
fn test() {
    let output = Command::new("../target/debug/ators")
        .args([
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

    io::BufReader::new(
        fs::File::open(path(TEST_SYMR).expect("test symr to exist"))
            .expect("test symr to be opened"),
    )
    .lines()
    .map(|line| line.expect("test symr line to be ok"))
    .zip(
        String::from_utf8(output.stdout)
            .expect("ators output to be utf8")
            .lines()
            .map(|line| line.to_string()),
    )
    .for_each(|(expected, actual)| assert_eq!(expected, actual));
}
