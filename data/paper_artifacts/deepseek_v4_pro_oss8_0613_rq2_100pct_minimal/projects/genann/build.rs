fn main() {
    // Compile native C accessor shims so that functions like
    // c2r_field_ptr_genann__weight are available at link time.
    cc::Build::new()
        .file("native/c2r_accessors.c")
        .include("native/include")
        .compile("c2r_accessors");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=native/c2r_accessors.c");
}
