use std::path::Path;
use std::path::PathBuf;

use duct::cmd;

const CBINDGEN_TOML: &str = "src/backend/llvm/cbindgen.toml";

const BRIDGE_SOURCE_FILES: &[&str] = &[
    "src/backend/bridge.rs",
    "src/backend/llvm/bridge.rs",
    "src/backend/llvm/compiler/bridge.rs",
    "src/backend/llvm/executor/bridge.rs",
    "src/backend/llvm/module/bridge.rs",
    "src/types.rs",
];

const BACKEND_LLVM_COMPONENTS: &[&str] = &["core", "orcjit", "x86"];

const BACKEND_LLVM_SOURCE_FILES: &[&str] = &[
    "src/backend/llvm/bridge.hh",
    "src/backend/llvm/compiler/impl.hh",
    "src/backend/llvm/compiler/peer.cc",
    "src/backend/llvm/compiler/type_holder.hh",
    "src/backend/llvm/compiler/type_holder.cc",
    "src/backend/llvm/executor/impl.codegen.hh",
    "src/backend/llvm/executor/impl.hh",
    "src/backend/llvm/executor/peer.cc",
    "src/backend/llvm/module/impl.hh",
    "src/backend/llvm/module/peer.cc",
    "src/backend/llvm/peer.cc",
];

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Generate bridge.hh for LLVM IR bridge.

    let config = cbindgen::Config::from_file(CBINDGEN_TOML).expect("Unable to load cbindgen.toml");

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/backend/llvm/bridge.hh");

    // Rebuild when cbindgen.toml change.
    println!("cargo::rerun-if-changed={CBINDGEN_TOML}");

    // Rebuild when any of BRIDGE_SOURCE_FILES change.
    for src in BRIDGE_SOURCE_FILES {
        println!("cargo::rerun-if-changed={src}");
    }

    // Build LLVM IR bridge.

    let llvm_config = LlvmConfig::new();
    let cc_files = BACKEND_LLVM_SOURCE_FILES
        .iter()
        .filter(|src| src.ends_with(".cc"));
    let mut build = cc::Build::default();
    let mut build = build.cpp(true).files(cc_files).include(&out_dir);
    for flag in llvm_config.cxxflags().iter() {
        build = build.flag(flag);
    }
    if profile == "debug" {
        build.define("BEE_BUILD_DEBUG", "1");
    }
    build.compile("backend-llvm");

    // Rebuild when any of BACKEND_LLVM_SOURCE_FILES change.
    for src in BACKEND_LLVM_SOURCE_FILES {
        println!("cargo::rerun-if-changed={src}");
    }

    // Link against LLVM.
    println!("cargo::rustc-link-search=native={}", llvm_config.libdir());
    for lib in llvm_config.libs(BACKEND_LLVM_COMPONENTS).iter() {
        println!("cargo::rustc-link-lib=static={lib}");
    }
    for lib in llvm_config.system_libs(BACKEND_LLVM_COMPONENTS).iter() {
        println!("cargo::rustc-link-lib={lib}");
    }
}

struct LlvmConfig(PathBuf);

impl LlvmConfig {
    const LINK_TYPE: &str = "--link-static";

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
