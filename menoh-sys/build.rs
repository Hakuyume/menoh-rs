extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path;

fn main() {
    let library = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("menoh")
        .unwrap();

    let bindings = bindgen::Builder::default()
        .clang_args(library
                        .include_paths
                        .iter()
                        .map(|p| "-I".to_owned() + p.to_str().unwrap()))
        .header("wrapper.h")
        .whitelist_type("menoh_.*")
        .whitelist_function("menoh_.*")
        .rustfmt_bindings(false)
        .generate()
        .unwrap();
    bindings
        .write_to_file(path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
