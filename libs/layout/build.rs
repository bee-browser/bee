use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use rayon::prelude::*;
use serde::Serialize;
use serde_json::json;

const LAYOUT_TEST_LAYOUT_FILES: &'static [&'static str] = &[
    "_absolutely_positioned_box.html.njk",
    "_containing_block.html.njk",
    "_layout.html.njk",
    "_z_index.html.njk",
];

const LAYOUT_TEST_NAMES: &'static [&'static str] = &[
    "absolutely_positioned_box_0000",
    "absolutely_positioned_box_0001",
    "containing_block_0000",
    "containing_block_0001",
    "containing_block_0002",
    "z_index_0000",
    "z_index_0001",
    "z_index_0002",
    "z_index_0003",
    "z_index_0004",
    "z_index_0005",
];

fn main() {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    codegen_layout_test();
}

fn codegen_layout_test() {
    #[derive(Serialize)]
    struct TestCase {
        name: &'static str,
        html_template_path: PathBuf,
        html_path: PathBuf,
        expected_path: PathBuf,
        scenario_path: PathBuf,
    }

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR");
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let tests_dir = Path::new(&src_dir).join("tests");

    let test_cases: Vec<TestCase> = LAYOUT_TEST_NAMES.iter()
        .map(|name| TestCase {
            name,
            html_template_path: Path::new(&tests_dir).join(&format!("{}.html.njk", name)),
            html_path: Path::new(&out_dir).join(&format!("{}.codegen.html", name)),
            scenario_path: Path::new(&out_dir).join(&format!("{}.scenario.codegen.json", name)),
            expected_path: Path::new(&tests_dir).join(&format!("{}.expected.yml", name)),
        })
        .collect();

    for filename in LAYOUT_TEST_LAYOUT_FILES.iter() {
        println!("cargo:rerun-if-changed={}",
                 Path::new(&tests_dir).join(filename).to_str().expect("PathBuf::to_str()"));
    }

    test_cases.par_iter().for_each(|test_case| {
        let status = Command::new("bee-codegen")
            .arg(&test_case.html_template_path)
            .current_dir(&tests_dir)
            .stdout(Stdio::from(File::create(&test_case.html_path).expect("File::create()")))
            .status()
            .expect("Failed to spawn `bee-codegen`");
        assert!(status.success());

        let status = Command::new("bee-lms-html")
            .arg("--viewport=1000x500")
            .arg("--no-sandbox")
            .arg(&test_case.html_path)
            .current_dir(&tests_dir)
            .stdout(Stdio::from(File::create(&test_case.scenario_path).expect("File::create()")))
            .status()
            .expect("Faild to spawn `bee-lms-html`");
        assert!(status.success());

        println!("cargo:rerun-if-changed={}",
                 test_case.html_template_path.to_str().expect("PathBuf::to_str()"));
    });

    let json_path = Path::new(&out_dir).join("layout_test.codegen.json");
    fs::write(&json_path, json!({ "tests": test_cases }).to_string()).expect("fs::write()");

    let template_path = Path::new(&tests_dir).join("_test.rs.njk");
    let test_path = Path::new(&out_dir).join("layout_test.codegen.rs");
    let status = Command::new("bee-codegen")
        .arg("--stdin")
        .arg(&template_path)
        .current_dir(&tests_dir)
        .stdin(Stdio::from(File::open(&json_path).unwrap()))
        .stdout(Stdio::from(File::create(&test_path).unwrap()))
        .status()
        .expect("Faild to spawn `bee-codegen`");
    assert!(status.success());

    // The following code
    //
    //   println!("cargo:rerun-if-changed={:?}", template_path);
    //
    // outputs an instruction with a "quoted" path string like below:
    //
    //   cargo:rerun-if-changed="/path/to/file"
    //
    // Such an instruction does NOT work with cargo.  Probably, cargo cannot interpret the quoted
    // string in the instruction correctly as we expected.
    println!("cargo:rerun-if-changed={}", template_path.to_str().expect("PathBuf::to_str()"));
}
