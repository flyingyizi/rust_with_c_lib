use std::{env, path::PathBuf};

fn main() {
    compile_();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
}

/// return compiled path
fn compile_() {
    // let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    // let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    // let target_os = get_os_from_triple(target.as_str()).unwrap();



    let mut cfg = cmake::Config::new("clibsample"); //cmake::Config::new(source_path);
    let dest = cfg.profile("release").no_build_target(true).build();//.build_target("ALL_BUILD").build();

    println!("-----debug: dest:{:?}", dest.display());
    let dest = dest.join("build/Release");

    println!("cargo:rustc-link-search=native={}", dest.display());
    println!("cargo:rustc-link-lib=static=clib");
}
