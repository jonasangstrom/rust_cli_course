use assert_cmd::cargo;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {
    let mut cmd = cargo::cargo_bin_cmd!("hello");
    cmd.assert().success();
}
