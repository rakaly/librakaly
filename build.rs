use std::env;
use std::path::Path;

fn main() {
    if env::var("EU4_IRONMAN_TOKENS").is_err() && env::var("CI").is_err() {
        panic!("librakaly needs to be built with EU4_IRONMAN_TOKENS specified")
    }

    if env::var("CK3_IRONMAN_TOKENS").is_err() && env::var("CI").is_err() {
        panic!("librakaly needs to be built with CK3_IRONMAN_TOKENS specified")
    }

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = Path::new(&crate_dir).join("target").join("rakaly.h");

    let config = cbindgen::Config {
        cpp_compat: true,
        ..Default::default()
    };

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .with_include("stddef.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path);
}
