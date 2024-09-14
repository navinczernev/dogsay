use std::process::Command;
use assert_cmd::prelude::*;


#[test]
fn run_with_defaults() {
    Command::cargo_bin("dogsay")
        .expect("binary exists")
        .assert().success();
}
