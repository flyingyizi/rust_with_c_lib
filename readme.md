
demo how to binding c with your rust program. to simplest implement, this project guess that cmake command is in the %PATH%. and use visual studio as the generator.

# how to binding non-rust code overview

how to binding non-rust code is descripted in [cargo-build-scripts-documents](https://doc.rust-lang.org/cargo/reference/build-scripts.html). 

below records some key points that I recommend.

## where to locate build script

Create build.rs file in the root of the project. or specify in  Cargo.toml file, e.g. `build = "<path to build.rs>`

## binding name appoint

`*-sys` is a [naming convention](https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages) for crates that help Rust programs use C ("system") libraries, e.g. libz-sys, kernel32-sys, lcms2-sys. The task of the sys crates is expose a minimal low-level C interface to [Rust (FFI)](https://doc.rust-lang.org/book/ffi.html) and to tell Cargo how to link with the library. Adding higher-level, more Rust-friendly interfaces for the libraries is left to "wrapper" crates built as a layer on top of the sys crates (e.g. a "rusty-image-app" may depend on high-level "png-rs", which depends on low-level "libpng-sys", which depends on "libz-sys").

## Inputs to the Build Script
When the build script is run, there are a number of inputs to the build script, all passed in the form of [environment variables](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/cargo/reference/environment-variables.html).


In addition to environment variables, the build script’s current directory is the source directory of the build script’s package.

## build script how to commuicate with cargo

The script may communicate with Cargo by printing specially formatted commands prefixed with `cargo:` to stdout.

full list see [outputs-of-the-build-script](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script).

notes: "`cargo:KEY=VALUE` — Metadata, used by links scripts." This metadata is passed to the build scripts of dependent packages. details refer to [the-links-manifest-key](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key). for example, the [sdl2-sys crate](https://github.com/Rust-SDL2/rust-sdl2) add  `DEP_SDL2_INCLUDE` in `sdl2-sys` through `println!("cargo:include={}", include_paths.join(":"));`


## notes for cmake-rs

if using visual studio generator in cmake, defaultly not generate "install" build target. only when using `"install(TARGETS xxx DESTINATION .)"` statement in cmakefile.txt, then the "install" build target will be existed.

for the [cmake-rs](https://github.com/rust-lang/cmake-rs) crate, if not set the build target, the default is  "install", "install" build target is defaultly suppoted by by make and ninja. but visual studio not support it. 

so if the generator is visual studio:

- you need set the appropriate build target, e.g. `cfg.build_target("ALL_BUILD").build();`
- or using install statement in cmakefile.txt

