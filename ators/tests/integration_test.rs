mod common;

use common::*;
use pretty_assertions::assert_str_eq;
use std::{
    fs,
    io::{self, BufRead},
    process::Command,
    str,
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

    str::from_utf8(&output.stdout)
        .expect("ators output to be utf8")
        .lines()
        .map(|line| line.to_string())
        .zip(
            io::BufReader::new(
                fs::File::open(path(TEST_SYMR).expect("test symr to exist"))
                    .expect("test symr to be opened"),
            )
            .lines()
            .map(|line| line.expect("test symr line to be ok")),
        )
        .for_each(|(expected, actual)| assert_str_eq!(expected, actual));
}
