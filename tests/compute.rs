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

#[test]
fn tetrahedron_can() {
    let output = Command::new(bin_path())
        .args(["0", "1", "2", "1", "3", "2", "0", "2", "3", "0", "3", "1"])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!("", str::from_utf8(&output.stderr).unwrap());
}

#[test]
fn hexahedron_can() {
    let output = Command::new(bin_path())
        .args(["0", "1", "2", "1", "3", "2", "2", "3", "7", "1", "7", "3"
            , "1", "5", "7", "0", "5", "1", "0", "4", "5", "0", "2", "4"
            , "2", "6", "4", "2", "7", "6", "4", "6", "7", "4", "7", "5"])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!("", str::from_utf8(&output.stderr).unwrap());
}

#[test]
fn octahedron_cannot() {
    let output = Command::new(bin_path())
        .args(["0", "1", "2", "0", "2", "3", "0", "3", "4", "0", "4", "1"
            , "1", "5", "2", "2", "5", "3", "3", "5", "4", "1", "4", "5"])
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn icosahedron_cannot() {
    let output = Command::new(bin_path())
        .args(["0", "1", "2", "0", "2", "4", "0", "4", "5", "0", "5", "3", "0", "3", "1"
            , "1", "6", "2", "2", "8", "4", "4", "10", "5", "3", "5", "9", "1", "3", "7"
            , "1", "7", "6", "2", "6", "8", "4", "8", "10", "5", "10", "9", "3", "9", "7"
            , "6", "11", "8", "8", "11", "10", "9", "10", "11", "7", "9", "11", "6", "7", "11"])
        .output()
        .unwrap();

    assert!(!output.status.success());
}

