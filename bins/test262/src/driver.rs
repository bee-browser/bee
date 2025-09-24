use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;

use itertools::Itertools;
use pathdiff::diff_paths;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::CommandLine;
use crate::metadata;
use crate::metadata::Metadata;
use crate::report::TestReport;
use crate::report::TestResult;
use crate::runner;
use crate::runner::Error;

pub struct Driver {
    cl: CommandLine,
    harnesses: FxHashMap<String, Arc<Harness>>,
    test_cases: Vec<TestCase>,
}

impl Driver {
    pub fn new(cl: CommandLine) -> Self {
        Self {
            cl,
            harnesses: Default::default(),
            test_cases: Default::default(),
        }
    }

    /// Loads.
    pub fn load(&mut self) {
        self.load_harness();
        self.load_tests();
    }

    fn load_harness(&mut self) {
        let harness_dir = self.cl.test262_dir.join("harness");
        for entry in JsFiles::new(&self.cl.test262_dir, "harness", &[]) {
            let path_diff = diff_paths(entry.path(), &harness_dir).unwrap();
            let name = path_diff.to_str().unwrap().to_owned();
            let harness = Arc::new(Harness {
                path: entry.path().to_owned(),
                content: match std::fs::read_to_string(entry.path()) {
                    Ok(content) => content,
                    Err(err) => panic!("{}: {err:?}", entry.path().display()),
                },
            });
            self.harnesses.insert(name, harness);
        }
        eprintln!("{} harnesses", self.harnesses.len());
    }

    fn load_tests(&mut self) {
        let test_dir = self.cl.test262_dir.join("test");
        for entry in JsFiles::new(&self.cl.test262_dir, "test", &self.cl.tests) {
            let path_diff = diff_paths(entry.path(), &test_dir).unwrap();
            let name = path_diff.to_str().unwrap().to_owned();
            if let Some(metadata) = Metadata::extract(entry.path()) {
                let mut includes = vec![
                    self.harnesses.get("assert.js").unwrap().clone(),
                    self.harnesses.get("sta.js").unwrap().clone(),
                ];
                assert!(metadata.includes.iter().all_unique());
                for include in metadata.includes.iter() {
                    if include == "assert.js" {
                        continue;
                    }
                    if include == "sta.js" {
                        continue;
                    }
                    let harness = match self.harnesses.get(include) {
                        Some(harness) => harness.clone(),
                        None => panic!("{include}: no such harness file"),
                    };
                    includes.push(harness);
                }
                if metadata.flags.contains(&metadata::Flag::Module) {
                    self.test_cases.push(TestCase {
                        name,
                        includes,
                        path: entry.path().to_owned(),
                        strict: true,
                        metadata: metadata.clone(),
                    });
                } else if metadata.flags.contains(&metadata::Flag::Raw) {
                    self.test_cases.push(TestCase {
                        name,
                        includes: vec![],
                        path: entry.path().to_owned(),
                        strict: true,
                        metadata: metadata.clone(),
                    });
                } else {
                    if !metadata.flags.contains(&metadata::Flag::NoStrict) {
                        self.test_cases.push(TestCase {
                            name: format!("{name}#strict"),
                            includes: includes.clone(),
                            path: entry.path().to_owned(),
                            strict: true,
                            metadata: metadata.clone(),
                        });
                    }
                    if !metadata.flags.contains(&metadata::Flag::OnlyStrict) {
                        self.test_cases.push(TestCase {
                            name,
                            includes,
                            path: entry.path().to_owned(),
                            strict: false,
                            metadata,
                        });
                    }
                }
            }
        }
        eprintln!("{} test cases", self.test_cases.len());
    }

    pub fn run(&mut self) -> TestReport {
        fn now() -> u128 {
            SystemTime::elapsed(&SystemTime::UNIX_EPOCH).unwrap().as_millis()
        }

        let start = now();
        let results = self
            .test_cases
            .par_iter()
            .map(|test_case| (test_case, runner::run(test_case)))
            .map(TestResult::from)
            .collect::<Vec<_>>();
        let end = now();
        TestReport::new(start, end, results)
    }
}

impl From<(&TestCase, Result<Duration, Error>)> for TestResult {
    fn from((test_case, result): (&TestCase, Result<Duration, Error>)) -> Self {
        match result {
            Ok(duration) => {
                if let Some(ref _negative) = test_case.metadata.negative {
                    TestResult::failed(test_case, duration)
                } else {
                    TestResult::passed(test_case, duration)
                }
            }
            Err(Error::Harness { duration, .. }) => TestResult::other(test_case, duration),
            Err(Error::Parse { duration, .. }) => {
                if test_case.should_be_syntax_error() {
                    TestResult::passed(test_case, duration)
                } else {
                    TestResult::failed(test_case, duration)
                }
            }
            Err(Error::Runtime { duration, .. }) => {
                // TODO: check error type
                if test_case.should_be_runtime_error() {
                    TestResult::passed(test_case, duration)
                } else {
                    TestResult::failed(test_case, duration)
                }
            }
        }
    }
}

struct JsFiles<'a> {
    walk: walkdir::IntoIter,
    test262_dir: PathBuf,
    tests: &'a [String],
}

impl<'a> JsFiles<'a> {
    fn new(test262_dir: &Path, holder: &str, tests: &'a [String]) -> Self {
        let base_dir = test262_dir.join(holder);
        let walk = WalkDir::new(&base_dir).into_iter();
        Self {
            walk,
            test262_dir: test262_dir.to_owned(),
            tests,
        }
    }
}

impl<'a> Iterator for JsFiles<'a> {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        for entry in self.walk.by_ref() {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {
                continue;
            }
            if !matches!(entry.path().extension(), Some(ext) if ext == "js") {
                continue;
            }
            if entry.file_name().to_str().unwrap().ends_with("_FIXTURE.js") {
                continue;
            }
            let path_diff = diff_paths(entry.path(), &self.test262_dir).unwrap();
            let matched =
                self.tests.is_empty() || self.tests.iter().any(|test| path_diff.starts_with(test));
            if !matched {
                continue;
            }
            return Some(entry);
        }
        None
    }
}

pub struct Harness {
    #[allow(unused)]
    path: PathBuf,
    content: String,
}

impl Harness {
    #[allow(unused)]
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct TestCase {
    pub includes: Vec<Arc<Harness>>,
    pub path: PathBuf,
    pub name: String,
    pub strict: bool,
    pub metadata: Arc<Metadata>,
}

impl TestCase {
    fn should_be_syntax_error(&self) -> bool {
        if let Some(ref negative) = self.metadata.negative {
            negative.is_syntax_error_in_parse()
        } else {
            false
        }
    }

    fn should_be_runtime_error(&self) -> bool {
        if let Some(ref negative) = self.metadata.negative {
            negative.is_runtime_error()
        } else {
            false
        }
    }
}
