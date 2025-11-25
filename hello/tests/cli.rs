use assert_cmd::cargo;
use pretty_assertions::assert_eq;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {
    let mut cmd = cargo::cargo_bin_cmd!("hello");
    cmd.assert().success();
    let output = cmd.output().expect("fail");
    let stdout = String::from_utf8(output.stdout).expect("fail");
    assert_eq!(stdout, "Hello, world!\n");
}

