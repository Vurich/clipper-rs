extern crate bindgen;
extern crate gcc;

fn main() {
    println!("cargo:rustc-link-lib=clipper");

    gcc::Build::new()
        .cpp(true)
        .file("./wrapper.cpp")
        .compile("clipper");

    bindgen::Builder::default()
        .enable_cxx_namespaces()
        .header("./wrapper.hpp")
        .whitelist_type("path")
        .whitelist_type("paths")
        .whitelist_type("clipper")
        .whitelist_function("path_new")
        .whitelist_function("path_clear")
        .whitelist_function("path_size")
        .whitelist_function("path_addPoint")
        .whitelist_function("path_getPointX")
        .whitelist_function("path_getPointY")
        .whitelist_function("path_free")
        .whitelist_function("path_getArea")
        .whitelist_function("paths_new")
        .whitelist_function("paths_clear")
        .whitelist_function("paths_size")
        .whitelist_function("paths_getPath")
        .whitelist_function("paths_addPath")
        .whitelist_function("paths_free")
        .whitelist_function("clipper_new")
        .whitelist_function("clipper_addPath")
        .whitelist_function("clipper_addPaths")
        .whitelist_function("clipper_execute")
        .whitelist_function("clipper_free")
        .whitelist_function("clipperoffset_new")
        .whitelist_function("clipperoffset_setMiterLimit")
        .whitelist_function("clipperoffset_setArcTolerance")
        .whitelist_function("clipperoffset_addPath")
        .whitelist_function("clipperoffset_addPaths")
        .whitelist_function("clipperoffset_execute")
        .whitelist_function("clipperoffset_free")
        .generate()
        .unwrap()
        .write_to_file("./src/bindings.rs")
        .unwrap();
}
