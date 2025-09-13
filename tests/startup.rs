use std::process::Command;

mod support;
use support::{bin_path, triangle_output};

#[test]
fn no_arg() {
    let output = Command::new(bin_path())
        .output()
        .unwrap();

    let expected = "error: the following required arguments were not provided:";
    assert_eq!(expected, str::from_utf8(&output.stderr).unwrap().split('\n').next().unwrap());
}

#[test]
fn float_arg() {
    let output = Command::new(bin_path())
        .arg("0.0")
        .output()
        .unwrap();

    let expected = "error: invalid value '0.0' for '<INDEX_LIST>...': invalid digit found in string";
    assert_eq!(expected, str::from_utf8(&output.stderr).unwrap().split('\n').next().unwrap());
}

#[test]
fn non_number_arg() {
    let output = Command::new(bin_path())
        .arg("hi")
        .output()
        .unwrap();

    let expected = "error: invalid value 'hi' for '<INDEX_LIST>...': invalid digit found in string";
    assert_eq!(expected, str::from_utf8(&output.stderr).unwrap().split('\n').next().unwrap());
}

#[test]
fn degenerate_input() {
    let output = Command::new(bin_path())
        .args(["--echo-input", "0", "0", "0", "0", "1", "1"])
        .output()
        .unwrap();

    let err_expected = "Skipping degenerate input: (0, 0, 0)\nSkipping degenerate input: (0, 1, 1)\n";
    let out_expected = "[]\n";
    assert_eq!(err_expected, str::from_utf8(&output.stderr).unwrap());
    assert_eq!(out_expected, str::from_utf8(&output.stdout).unwrap());
}

#[test]
fn one_triangle() {
    let output = Command::new(bin_path())
        .args(["--echo-input", "0", "1", "2"])
        .output()
        .unwrap();

    let expected = format!("[{}]\n", triangle_output(0, 1, 2));
    assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
}

#[test]
fn one_triangle_extra() {
    let output = Command::new(bin_path())
        .args(["--echo-input", "0", "1", "2", "2", "1"])
        .output()
        .unwrap();

    let expected = format!("[{}]\n", triangle_output(0, 1, 2));
    assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
}

#[test]
fn two_triangles() {
    let output = Command::new(bin_path())
        .args(["--echo-input", "0", "1", "2", "2", "1", "3"])
        .output()
        .unwrap();

    let expected = format!("[{}, {}]\n", triangle_output(0, 1, 2), triangle_output(2, 1, 3));
    assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
}

