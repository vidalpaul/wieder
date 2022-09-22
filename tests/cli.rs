use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    Command::cargo_bin("wieder")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn dies_no_text() {
    Command::cargo_bin("wieder")
        .unwrap()
        .arg("-r")
        .arg("2")
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn dies_invalid_repeat() {
    Command::cargo_bin("wieder")
        .unwrap()
        .arg("-r")
        .arg("foo")
        .arg("bar")
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
 let mut cmd = Command::cargo_bin("wieder").unwrap();
 cmd.arg("hello").assert().success();
}