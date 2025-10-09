use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;

use serde::Serialize;

use crate::driver::TestCase;
use crate::metadata::Metadata;

/// A test report in the CTRF format.
#[derive(Debug, Serialize)]
pub struct TestReport {
    pub timestamp: Option<u128>,
    pub results: Vec<TestResult>,
}

impl TestReport {
    pub fn new(results: Vec<TestResult>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .map(|duration| duration.as_millis());
        Self { timestamp, results }
    }
}

#[derive(Debug, Serialize)]
pub struct TestResult {
    pub file: PathBuf,
    pub strict: bool,
    pub status: TestStatus,
    pub duration: Duration,
    pub metadata: Arc<Metadata>,
}

impl TestResult {
    fn new(base_dir: &Path, test_case: &TestCase, duration: Duration, status: TestStatus) -> Self {
        let file = test_case.path.strip_prefix(base_dir).unwrap().to_owned();
        Self {
            file,
            strict: test_case.strict,
            status,
            duration,
            metadata: test_case.metadata.clone(),
        }
    }

    pub fn passed(base_dir: &Path, test_case: &TestCase, duration: Duration) -> Self {
        Self::new(base_dir, test_case, duration, TestStatus::Passed)
    }

    pub fn failed(base_dir: &Path, test_case: &TestCase, duration: Duration) -> Self {
        Self::new(base_dir, test_case, duration, TestStatus::Failed)
    }

    pub fn timed_out(base_dir: &Path, test_case: &TestCase, duration: Duration) -> Self {
        Self::new(base_dir, test_case, duration, TestStatus::TimedOut)
    }

    pub fn panic(base_dir: &Path, test_case: &TestCase, duration: Duration) -> Self {
        Self::new(base_dir, test_case, duration, TestStatus::Panic)
    }
}

#[derive(Debug, Serialize)]
pub enum TestStatus {
    #[serde(rename = "passed")]
    Passed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "timed-out")]
    TimedOut,
    #[serde(rename = "panic")]
    Panic,
}
