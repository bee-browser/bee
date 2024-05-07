use std::path::Path;
use std::path::PathBuf;

use duct::cmd;

static LLVM_COMPONENTS: &[&str] = &["core", "orcjit", "x86"];

static LLVMIR_SOURCE_FILES: &[&str] = &[
    "src/llvmir/bridge.cc",
    "src/llvmir/bridge.hh",
    "src/llvmir/compiler.cc",
    "src/llvmir/compiler.hh",
    "src/llvmir/executor.cc",
    "src/llvmir/executor.codegen.cc",
    "src/llvmir/executor.hh",
    "src/llvmir/macros.hh",
    "src/llvmir/module.hh",
    "src/llvmir/runtime.hh",
    "src/llvmir/type_holder.hh",
    "src/llvmir/type_holder.cc",
];

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Generate bindings for Rust.
    let input_file = "src/llvmir/bridge.hh";
    let output_file = out_dir.join("bridge.rs");
    bindgen::Builder::default()
        .header(input_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for Rust")
        .write_to_file(output_file)
        .expect("Couldn't write bindings for Rust");

    // Build LLVM-IR glue.
    let llvm_config = LlvmConfig::new();
    let cc_files = LLVMIR_SOURCE_FILES
        .iter()
        .filter(|src| src.ends_with(".cc"));
    let mut build = cc::Build::default();
    let mut build = build.cpp(true).files(cc_files).include(&out_dir);
    for flag in llvm_config.cxxflags().iter() {
        build = build.flag(flag);
    }
    build.compile("llvmir");

    // Rebuild when any of LLVMIR_SOURCE_FILES change.
    for src in LLVMIR_SOURCE_FILES {
        println!("cargo::rerun-if-changed={src}");
    }

    // Link against LLVM.
    println!("cargo::rustc-link-search=native={}", llvm_config.libdir());
    for lib in llvm_config.libs(LLVM_COMPONENTS).iter() {
        println!("cargo::rustc-link-lib=static={}", lib);
    }
    for lib in llvm_config.system_libs(LLVM_COMPONENTS).iter() {
        println!("cargo::rustc-link-lib={}", lib);
    }
}

struct LlvmConfig(PathBuf);

impl LlvmConfig {
    const LINK_TYPE: &'static str = "--link-static";

    fn new() -> Self {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let proj_dir = crate_dir.join("..").join("..").canonicalize().unwrap();
        let vendor_dir = proj_dir.join("vendor");
        Self(vendor_dir.join("bin").join("llvm-config"))
    }

    fn libdir(&self) -> String {
        cmd!(&self.0, Self::LINK_TYPE, "--libdir").read().unwrap()
    }

    fn libs(&self, components: &[&str]) -> Vec<String> {
        let mut args = vec![Self::LINK_TYPE, "--libs"];
        args.extend(components);
        self.list_libs(&args)
    }

    fn system_libs(&self, components: &[&str]) -> Vec<String> {
        let mut args = vec![Self::LINK_TYPE, "--system-libs"];
        args.extend(components);
        self.list_libs(&args)
    }

    fn list_libs(&self, args: &[&str]) -> Vec<String> {
        cmd(&self.0, args)
            .read()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s[2..].to_owned()) // remove '-l'
            .collect()
    }

    fn cxxflags(&self) -> Vec<String> {
        cmd!(&self.0, Self::LINK_TYPE, "--cxxflags")
            .read()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect()
    }
}
