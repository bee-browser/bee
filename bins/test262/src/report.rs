use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;

use crate::driver::TestCase;
use crate::metadata::Metadata;

/// A test report in the CTRF format.
#[derive(Debug, Serialize)]
pub struct TestReport {
    pub report_format: &'static str,
    pub spec_version: &'static str,
    pub results: TestResults,
}

impl TestReport {
    pub fn new(start: u128, stop: u128, tests: Vec<TestResult>) -> Self {
        Self {
            report_format: "CTRF",
            spec_version: "0.0.0",
            results: TestResults::new(start, stop, tests),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TestResults {
    pub tool: TestTool,
    pub summary: TestSummary,
    pub tests: Vec<TestResult>,
}

impl TestResults {
    fn new(start: u128, stop: u128, tests: Vec<TestResult>) -> Self {
        let summary = TestSummary::new(start, stop, &tests);
        Self {
            tool: TestTool {
                name: "bee-browser/bee;bins/test262",
            },
            summary,
            tests,
        }
    }
}


#[derive(Debug, Default, Serialize)]
pub struct TestSummary {
    tests: u32,
    passed: u32,
    failed: u32,
    pending: u32,
    other: u32,
    start: u32,
    stop: u32,
}

impl TestSummary {
    fn new(start: u128, stop: u128, tests: &[TestResult]) -> Self {
        // Ensure that tests.len() < Number.MAX_SAFE_INTEGER.
        assert!(tests.len() <= u32::MAX as usize);

        macro_rules! count {
            ($status:pat) => {
                tests.iter().filter(|r| matches!(r.status, $status)).count() as u32
            };
        }

        Self {
            tests: tests.len() as u32,
            passed: count!(TestResultStatus::Passed),
            failed: count!(TestResultStatus::Failed),
            pending: count!(TestResultStatus::Pending),
            other: count!(TestResultStatus::Other),
            start: start as u32,
            stop: stop as u32,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TestResult {
    pub name: String,
    pub status: TestResultStatus,
    pub duration: Duration,
    pub extra: TestResultExtra,
}

impl TestResult {
    fn new(test_case: &TestCase, duration: Duration, status: TestResultStatus) -> Self {
        Self {
            name: test_case.name.clone(),
            status,
            duration,
            extra: TestResultExtra {
                metadata: test_case.metadata.clone(),
            },
        }
    }

    pub fn passed(test_case: &TestCase, duration: Duration) -> Self {
        Self::new(test_case, duration, TestResultStatus::Passed)
    }

    pub fn failed(test_case: &TestCase, duration: Duration) -> Self {
        Self::new(test_case, duration, TestResultStatus::Failed)
    }

    pub fn other(test_case: &TestCase, duration: Duration) -> Self {
        Self::new(test_case, duration, TestResultStatus::Other)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TestResultStatus {
    Passed,
    Failed,
    #[allow(unused)]
    Skipped,
    #[allow(unused)]
    Pending,
    Other,
}

#[derive(Debug, Serialize)]
pub struct TestResultExtra {
    pub metadata: Arc<Metadata>,
}

#[derive(Debug, Serialize)]
pub struct TestTool {
    pub name: &'static str,
}
