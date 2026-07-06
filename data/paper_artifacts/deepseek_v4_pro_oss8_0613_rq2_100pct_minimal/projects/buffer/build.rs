fn main() {
    cc::Build::new()
        .file("native/buffer_appendf_impl.c")
        .include("native/include")
        .include("/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/buffer/src")
        .compile("native_shims");
    println!("cargo:rerun-if-changed=native/buffer_appendf_impl.c");
    println!("cargo:rerun-if-changed=build.rs");
}
