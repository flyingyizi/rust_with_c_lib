use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let source_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("clibsample");

    compile_(source_path.as_path());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=clibsample");
}

/// return compiled path
fn compile_(source_path: &Path) {
    // let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    // let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    // let target_os = get_os_from_triple(target.as_str()).unwrap();

    let mut cfg = cmake::Config::new(source_path);
    let dest = cfg.profile("release").no_build_target(true).build(); //.build_target("ALL_BUILD").build();

    println!("-----debug: dest:{:?}", dest.display());
    let dest = dest.join("build/Release");

    println!("cargo:rustc-link-search=native={}", dest.display());
    println!("cargo:rustc-link-lib=static=clib");

    generate_bindings(source_path);
}

// headers_path is a list of directories where the clibsample headers are
fn generate_bindings(source_path: &Path) {
    let includes_ = source_path.join("inc").to_str().unwrap().to_string();

    // expected to be found by bindgen (should point to the include/ directories)
    let include_paths: Vec<String> = vec![includes_];

    let mut bindings = bindgen::Builder::default()
        // enable no_std-friendly output by only using core definitions
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .constified_enum_module("game")
        .bitfield_enum("animal")
        .newtype_enum("planet")
        .rustified_enum("color")
        .ctypes_prefix("libc");

    for headers_path in include_paths {
        bindings = bindings.clang_arg(format!("-I{}", headers_path));
    }

    let wrapper_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("wrapper.h");
    let wrapper_path = wrapper_path.to_str().unwrap();

    let bindings = bindings
        .header(wrapper_path)
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("clibsample_bindings.rs"))
        .expect("Couldn't write bindings!");
}
