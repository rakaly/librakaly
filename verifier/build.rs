fn main() {
    if cfg!(not(windows)) {
        println!("cargo:rustc-link-lib=rakaly");
    }
    println!("cargo:rustc-link-search=.");
}
