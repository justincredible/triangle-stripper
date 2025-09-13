use std::env;
use std::path::PathBuf;

pub fn bin_path() -> PathBuf {
    #[cfg(debug_assertions)]
    let build_config = "debug";
    #[cfg(not(debug_assertions))]
    let build_config = "release";

    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push(format!("target/{build_config}/triangle_stripper"));

    path
}

pub fn triangle_output(first: u32, second: u32, third: u32) -> String {
    // default formatter
    format!("({first},{second},{third})")
    // Debug formatter
    //format!("Triangle {{ first: {first}, second: {second}, third: {third} }}")
}

