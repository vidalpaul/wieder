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

// Helper test fn
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("wieder")?
    .args(args)
    .assert()
    .success()
    .stdout(expected);
    Ok(())
   }

#[test]
fn hello1() -> TestResult {
 run(&["Hello there"], "tests/expected/hello1.txt")
}
#[test]
fn hello2() -> TestResult {
 run(&["Hello", "there"], "tests/expected/hello2.txt")
}
#[test]
fn hello1_no_newline() -> TestResult {
 run(&["-n", "Hello there"], "tests/expected/hello1.n.txt")
}
#[test]
fn hello2_no_newline() -> TestResult {
 run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}