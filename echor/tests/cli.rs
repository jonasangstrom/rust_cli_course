use std::fs;
use anyhow::Result;

use assert_cmd::cargo;
// use pretty_assertions::assert_eq;
use predicates::prelude::predicate;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> Result<()> {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.arg("Hello, world!").assert().success();
    let output = cmd.output().expect("fail");
    let stdout = String::from_utf8(output.stdout).expect("fail");
    assert_eq!(stdout, "Hello, world!\n");
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;

    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}
