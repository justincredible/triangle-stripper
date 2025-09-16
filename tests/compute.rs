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
fn dodecahedron_can() {
    let output = Command::new(bin_path())
        .args(["0", "1", "3", "0", "3", "4", "0", "4", "2", "0", "5", "10", "0", "10", "6", "0", "6", "1", "0", "2", "7", "0", "7", "11", "0", "11", "5", "1", "6", "12", "1", "12", "3", "12", "8", "3", "2", "4", "7", "4", "9", "7", "9", "13", "7", "3", "8", "4", "8", "14", "9", "8", "9", "4", "5", "11", "10", "11", "16", "10", "16", "15", "10", "6", "10", "15", "6", "15", "17", "6", "17", "12", "7", "13", "11", "13", "18", "16", "13", "16", "11", "8", "12", "19", "8", "19", "14", "12", "17", "19", "9", "14", "19", "9", "19", "13", "19", "18", "13", "15", "16", "19", "15", "19", "17", "16", "18", "19"])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!("", str::from_utf8(&output.stderr).unwrap());
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

fn pentagonal_tessellator(v: [u32; 5]) -> [[u32; 9]; 5] {
    [
        [v[0], v[1], v[2], v[0], v[2], v[3], v[0], v[3], v[4]],
        [v[0], v[1], v[2], v[0], v[2], v[4], v[2], v[3], v[4]],
        [v[0], v[1], v[3], v[0], v[3], v[4], v[1], v[2], v[3]],
        [v[0], v[1], v[4], v[1], v[2], v[3], v[1], v[3], v[4]],
        [v[0], v[1], v[4], v[1], v[2], v[4], v[2], v[3], v[4]],
    ]
}

#[test]
#[ignore]
fn dodecahedron_can_long() {
    // duodecuply-nested loop, indentation removed for readability
    for triangles01 in pentagonal_tessellator([0, 1, 3, 4, 2]) {
    for triangles02 in pentagonal_tessellator([0, 5, 10, 6, 1]) {
    for triangles03 in pentagonal_tessellator([0, 2, 7, 11, 5]) {
    for triangles04 in pentagonal_tessellator([1, 6, 12, 8, 3]) {
    for triangles05 in pentagonal_tessellator([2, 4, 9, 13, 7]) {
    for triangles06 in pentagonal_tessellator([3, 8, 14, 9, 4]) {
    for triangles07 in pentagonal_tessellator([5, 11, 16, 15, 10]) {
    for triangles08 in pentagonal_tessellator([6, 10, 15, 17, 12]) {
    for triangles09 in pentagonal_tessellator([7, 13, 18, 16, 11]) {
    for triangles10 in pentagonal_tessellator([8, 12, 17, 19, 14]) {
    for triangles11 in pentagonal_tessellator([9, 14, 19, 18, 13]) {
    for triangles12 in pentagonal_tessellator([15, 16, 18, 19, 17]) {
        let args: Vec<_> = triangles01.iter().map(|&i| format!("{i}"))
            .chain(triangles02.iter().map(|&i| format!("{i}")))
            .chain(triangles03.iter().map(|&i| format!("{i}")))
            .chain(triangles04.iter().map(|&i| format!("{i}")))
            .chain(triangles05.iter().map(|&i| format!("{i}")))
            .chain(triangles06.iter().map(|&i| format!("{i}")))
            .chain(triangles07.iter().map(|&i| format!("{i}")))
            .chain(triangles08.iter().map(|&i| format!("{i}")))
            .chain(triangles09.iter().map(|&i| format!("{i}")))
            .chain(triangles10.iter().map(|&i| format!("{i}")))
            .chain(triangles11.iter().map(|&i| format!("{i}")))
            .chain(triangles12.iter().map(|&i| format!("{i}")))
            .collect();

        let output = Command::new(bin_path())
            .args(args.clone())
            .output()
            .unwrap();

        if output.status.success() {
            println!("{:?}", args);
            return;
        };
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }

    panic!("No solution");
}

