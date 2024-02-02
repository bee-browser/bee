use std::path::Path;
use std::path::PathBuf;

use duct::cmd;

static LLVM_COMPONENTS: &[&str] = &["core", "orcjit", "x86"];

static LLVMIR_SOURCE_FILES: &[&str] = &[
    "llvmir/bridge.cc",
    "llvmir/bridge.hh",
    "llvmir/compiler.cc",
    "llvmir/compiler.hh",
    "llvmir/evaluator.hh",
    "llvmir/runtime.cc",
    "llvmir/runtime.hh",
];

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Generate bindings for C++.
    let output_file = out_dir.join("bindings.hh");
    cbindgen::generate(env!("CARGO_MANIFEST_DIR"))
        .expect("Unable to generate bindings for C++")
        .write_to_file(&output_file);

    // Generate bindings for Rust.
    let input_file = "llvmir/bridge.hh";
    let output_file = out_dir.join("bridge.rs");
    bindgen::Builder::default()
        .header(input_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for Rust")
        .write_to_file(&output_file)
        .expect("Couldn't write bindings for Rust");

    // Build LLVM-IR glue.
    let llvm_config = LlvmConfig::new();
    let mut build = cc::Build::default();
    let mut build = build.files(
        LLVMIR_SOURCE_FILES
            .iter()
            .filter(|src| src.ends_with(".cc")),
    );
    build = build.cpp(true);
    build = build.include(&out_dir);
    for flag in llvm_config.cxxflags().iter() {
        build = build.flag(flag);
    }
    build.compile("llvmir");

    // Rebuild when any of LLVMIR_SOURCE_FILES change.
    for src in LLVMIR_SOURCE_FILES {
        println!("cargo:rerun-if-changed={src}");
    }

    // Link against LLVM.
    println!("cargo:rustc-link-search=native={}", llvm_config.libdir());
    for lib in llvm_config.libs(LLVM_COMPONENTS).iter() {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
    for lib in llvm_config.system_libs(LLVM_COMPONENTS).iter() {
        println!("cargo:rustc-link-lib={}", lib);
    }
}

struct LlvmConfig(PathBuf);

impl LlvmConfig {
    fn new() -> Self {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let proj_dir = crate_dir.join("..").join("..").canonicalize().unwrap();
        let vendor_dir = proj_dir.join("vendor");
        Self(vendor_dir.join("bin").join("llvm-config"))
    }

    fn libdir(&self) -> String {
        cmd!(&self.0, "--libdir").read().unwrap()
    }

    fn libs(&self, components: &[&str]) -> Vec<String> {
        let mut args = vec!["--libs"];
        args.extend(components);
        self.vec(&args)
    }

    fn system_libs(&self, components: &[&str]) -> Vec<String> {
        let mut args = vec!["--system-libs"];
        args.extend(components);
        self.vec(&args)
    }

    fn vec(&self, args: &[&str]) -> Vec<String> {
        cmd(&self.0, args)
            .read()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s[2..].to_owned()) // remove '-l'
            .collect()
    }

    fn cxxflags(&self) -> Vec<String> {
        cmd!(&self.0, "--cxxflags")
            .read()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect()
    }
}
