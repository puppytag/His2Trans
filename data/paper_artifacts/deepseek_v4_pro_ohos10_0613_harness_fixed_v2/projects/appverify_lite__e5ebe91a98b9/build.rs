//! Build script: compiles C shim that provides global variables.

fn main() {
    // Compile the C shim
    cc::Build::new()
        .file("native/globals.c")
        .compile("globals");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=native/globals.c");
}
