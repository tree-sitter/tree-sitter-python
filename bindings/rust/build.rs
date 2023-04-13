use std::path::Path;
extern crate cc;

fn main() {
    let src_dir = Path::new("src");
    let parser_path = src_dir.join("parser.c");
    let mut compiler = cc::Build::new();

    // set minimal C sysroot if wasm32-unknown-unknown
    if std::env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        let sysroot_dir = Path::new("bindings/rust/wasm-sysroot");
        compiler
            .archiver("llvm-ar")
            .include(&sysroot_dir);
    }

    compiler
        .include(&src_dir)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        .file(&parser_path)
        .compile("parser");

    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
}
