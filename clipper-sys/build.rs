extern crate bindgen;
extern crate gcc;

fn main() {
    println!("cargo:rustc-link-lib=clipper");

    gcc::Build::new()
        .file("./clipper/cpp/clipper.cpp")
        .compile("clipper");

    bindgen::Builder::default()
        .enable_cxx_namespaces()
        .header("./clipper/cpp/clipper.hpp")
        .whitelist_type("ClipperLib::Clipper")
        .generate()
        .unwrap()
        .write_to_file("./src/bindings.rs")
        .unwrap();
}
