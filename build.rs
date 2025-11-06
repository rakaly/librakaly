use std::path::Path;

fn main() {
    std::fs::create_dir_all("assets/tokens").expect("to create tokens directory");
    for game in ["ck3", "hoi4", "eu4", "imperator", "vic3", "eu5"] {
        let fp = format!("assets/tokens/{game}.txt");
        let p = std::path::Path::new(&fp);
        if !p.exists() {
            std::fs::write(p, "").expect("to write file");
        }
    }

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = Path::new(&crate_dir).join("target").join("rakaly.h");

    cbindgen::Builder::new()
        .with_cpp_compat(true)
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .with_include("stddef.h")
        .with_trailer(include_str!("./src/cpp_helper.h"))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path);
}
