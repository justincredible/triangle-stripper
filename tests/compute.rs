use std::process::Command;

mod support;
use support::bin_path;

#[test]
fn not_enough_input() {
    let output = Command::new(bin_path())
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn one_triangle() {
    let output = Command::new(bin_path())
        .args(["0", "1", "2"])
        .output()
        .unwrap();

    let expected = "[0, 1, 2]\n";
    assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
}

#[test]
fn two_triangles_easy() {
    let output = Command::new(bin_path())
        .args(["0", "1", "2", "2", "1", "3"])
        .output()
        .unwrap();

    let expected = "[0, 1, 2, 3]\n";
    assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
}

#[test]
fn two_triangles_hard() {
    let output = Command::new(bin_path())
        .args(["2", "0", "1", "1", "3", "2"])
        .output()
        .unwrap();

    let expected = "[0, 1, 2, 3]\n";
    assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
}

