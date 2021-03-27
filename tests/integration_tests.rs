use assert_cmd::prelude::*;
use glob::glob;
use predicates::boolean::PredicateBooleanExt;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[test]
fn runs_without_arguments() {
    let mut cmd = Command::cargo_bin("algo").unwrap();
    cmd.assert().success();
}

#[test]
fn fails_when_in_wrong_dir() {
    Command::cargo_bin("algo")
        .unwrap()
        .current_dir("tests/")
        .assert()
        .code(1);
}

#[test]
fn verify_all_success() {
    Command::cargo_bin("algo")
        .unwrap()
        .arg("v")
        .current_dir("tests/fixture/success")
        .assert()
        .success();
}

#[test]
fn verify_fails_if_some_fails() {
    Command::cargo_bin("algo")
        .unwrap()
        .arg("v")
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}

#[test]
fn run_single_compile_success() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "compSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_compile_failure() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "compFailure"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_success() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_test_failure() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "testFailure"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_not_passed() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "testNotPassed.rs"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_filename() {
    Command::cargo_bin("algo")
        .unwrap()
        .arg("r")
        .current_dir("tests/fixture/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_algorithm() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "compNoAlgorithm.rs"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}

#[test]
fn get_hint_for_single_test() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["h", "testFailure"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(0)
        .stdout("Hello!\n");
}

#[test]
fn all_algorithms_require_confirmation() {
    for algorithm in glob("algorithms/**/*.rs").unwrap() {
        let path = algorithm.unwrap();
        let source = {
            let mut file = File::open(&path).unwrap();
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            s
        };
        source.matches("// I AM NOT DONE").next().expect(&format!(
            "There should be an `I AM NOT DONE` annotation in {:?}",
            path
        ));
    }
}

#[test]
fn run_compile_algorithm_does_not_prompt() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "pending_algorithm"])
        .current_dir("tests/fixture/state")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("I AM NOT DONE").not());
}

#[test]
fn run_test_algorithm_does_not_prompt() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "pending_test_algorithm"])
        .current_dir("tests/fixture/state")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("I AM NOT DONE").not());
}

#[test]
fn run_single_test_success_with_output() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["--nocapture", "r", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("THIS TEST TOO SHALL PAS"));
}

#[test]
fn run_single_test_success_without_output() {
    Command::cargo_bin("algo")
        .unwrap()
        .args(&["r", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("THIS TEST TOO SHALL PAS").not());
}
