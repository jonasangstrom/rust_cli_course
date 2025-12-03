use std::fs;
use anyhow::Result;

use assert_cmd::cargo;
// use pretty_assertions::assert_eq;
use predicates::prelude::predicate;


fn run_test_with_file(args: &[&str], expected_file_path: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file_path)?;

    let mut cmd = cargo::cargo_bin_cmd!("catr");
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
// Unlike the real cat this should not fail if file does not exist according
// to my understanding of the book.
fn dies_non_existing_file() -> Result<()> {
    let mut cmd = cargo::cargo_bin_cmd!("catr");
    cmd.args(["non_exissting.txt"]).assert()
        .success()
        .stderr(predicate::str::contains("Failed to open"));
    Ok(())
}

#[test]
fn empty_file() -> Result<()> {
    run_test_with_file(&["tests/inputs/empty.txt"], "tests/expected/empty.txt")
}

#[test]
fn empty_file_b() -> Result<()> {
    run_test_with_file(&["tests/inputs/empty.txt", "-b"], "tests/expected/empty-b.txt")
}

#[test]
fn empty_file_n() -> Result<()> {
    run_test_with_file(&["tests/inputs/empty.txt", "-b"], "tests/expected/empty-b.txt")
}

#[test]
fn the_bustle() -> Result<()> {
    run_test_with_file(&["tests/inputs/the-bustle.txt"], "tests/expected/the-bustle.txt")
}

#[test]
fn the_bustle_b() -> Result<()> {
    run_test_with_file(&["tests/inputs/the-bustle.txt", "-b"], "tests/expected/the-bustle-b.txt")
}

#[test]
fn the_bustle_n() -> Result<()> {
    run_test_with_file(&["tests/inputs/the-bustle.txt", "-n"], "tests/expected/the-bustle-n.txt")
}

#[test]
fn the_bustle_spiders_n() -> Result<()> {
    run_test_with_file(&["tests/inputs/the-bustle.txt", "tests/inputs/spiders.txt", "-n"], "tests/expected/the-bustle-spiders-n.txt")
}

#[test]
fn the_bustle_spiders_b() -> Result<()> {
    run_test_with_file(&["tests/inputs/the-bustle.txt", "tests/inputs/spiders.txt", "-b"], "tests/expected/the-bustle-spiders-b.txt")
}
