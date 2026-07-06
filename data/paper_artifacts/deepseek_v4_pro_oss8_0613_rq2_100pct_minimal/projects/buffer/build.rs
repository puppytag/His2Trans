fn main() {
    cc::Build::new()
        .file("native/buffer_appendf_impl.c")
        .include("native/include")
        .compile("native_shims");
    println!("cargo:rerun-if-changed=native/buffer_appendf_impl.c");
    println!("cargo:rerun-if-changed=native/include/buffer.h");
    println!("cargo:rerun-if-changed=build.rs");
}
