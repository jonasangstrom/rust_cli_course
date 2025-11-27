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

fn run_test_with_file(args: &[&str], expected_file_path: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file_path)?;

    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    run_test_with_file(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<()> {
    run_test_with_file(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello2n() -> Result<()> {
    run_test_with_file(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}

#[test]
fn hello1n() -> Result<()> {
    run_test_with_file(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}
