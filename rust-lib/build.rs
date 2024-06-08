extern crate cbindgen;

use std::env;

fn main() {
    let crate_name = env::var("CARGO_PKG_NAME").unwrap();
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("BINDGEN_OUT_DIR").unwrap();
    let header_file = format!("{}/{}.h", out_dir, crate_name);

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&header_file);
}
