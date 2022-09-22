use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("wieder")
        ?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn dies_no_text() -> TestResult {
    Command::cargo_bin("wieder")
        ?
        .arg("-r")
        .arg("2")
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn dies_invalid_repeat() -> TestResult {
    Command::cargo_bin("wieder")
        ?
        .arg("-r")
        .arg("foo")
        .arg("bar")
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
 let mut cmd = Command::cargo_bin("wieder")?;
 cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn runs_repeat() -> TestResult {
    let mut cmd = Command::cargo_bin("wieder")?;
    cmd.arg("-r=2")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello\nhello"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
 let outfile = "tests/expected/hello1.txt";
 let expected = fs::read_to_string(outfile)?;
 let mut cmd = Command::cargo_bin("wieder")?;
 cmd.arg("Hello there").assert().success().stdout(expected);
 Ok(())
}

#[test]
fn hello2() -> TestResult {
 let expected = fs::read_to_string("tests/expected/hello2.txt")?;
 let mut cmd = Command::cargo_bin("wieder")?;
 cmd.args(vec!["Hello", "there"])
 .assert()
 .success()
 .stdout(expected);
 Ok(())
}