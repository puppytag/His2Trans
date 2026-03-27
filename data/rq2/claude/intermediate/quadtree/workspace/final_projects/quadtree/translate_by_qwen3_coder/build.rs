//! Build script to compile C accessor shims
fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    cc::Build::new()
        .file("native/c2r_accessors.c")
        .include(manifest_dir.join("native/include"))
        .compile("c2r_accessors");
    println!("cargo:rerun-if-changed=native/c2r_accessors.c");
}
