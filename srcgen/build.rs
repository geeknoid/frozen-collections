//! Builds source generation tests

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

include!("./includes/make_collections.rs");

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");
    let mut file = BufWriter::new(File::create(dest_path).unwrap());

    for coll in &make_static_collections() {
        _ = writeln!(file, "{coll}");
    }

    println!("cargo::rerun-if-changed=build.rs");
}
