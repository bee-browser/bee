mod driver;
mod launcher;
mod metadata;
mod report;
mod runner;

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use clap::Parser as _;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use is_executable::IsExecutable;

use driver::Driver;
use report::TestReport;
use report::TestStatus;

/// A test262 runner for jsruntime.
///
/// The test results will be output to STDOUT in the CTRF format.
///
/// This program uses the `rayon` crate in order to perform tests in parallel.  Run with
/// `RAYON_NUM_THREADS=1` if you want to perform tests one by one.
#[derive(clap::Parser)]
struct CommandLine {
    /// Path to tc39/test262.
    #[arg(long, env = "BEE_TEST262_DIR")]
    test262_dir: PathBuf,

    /// Show progress.
    #[arg(long)]
    progress: bool,

    /// Enable the scope cleanup checker.
    #[arg(long)]
    scope_cleanup_checker: bool,

    /// Tests to be run.
    ///
    /// All tests are run when no filter is specified.
    #[arg(long)]
    filters: Vec<String>,

    #[command(subcommand)]
    command: Command,
}

impl CommandLine {
    fn validate(&self) -> Result<()> {
        anyhow::ensure!(
            self.test262_dir.exists(),
            "<test262-dir>: no such directory: {}",
            self.test262_dir.display()
        );
        anyhow::ensure!(
            self.test262_dir.is_dir(),
            "<test262-dir>: not a directory: {}",
            self.test262_dir.display()
        );
        anyhow::ensure!(
            self.test262_dir.join("harness").is_dir(),
            "<test262-dir>: no harness folder contained: {}",
            self.test262_dir.display()
        );
        anyhow::ensure!(
            self.test262_dir.join("test").is_dir(),
            "<test262-dir>: no test folder contained: {}",
            self.test262_dir.display()
        );
        self.command.validate()
    }
}

#[derive(clap::Subcommand)]
enum Command {
    Run(Run),
    Launch(Launch),
}

impl Command {
    fn validate(&self) -> Result<()> {
        match self {
            Self::Run(run) => run.validate(),
            Self::Launch(launch) => launch.validate(),
        }
    }
}

/// Run the tests in threads.
#[derive(clap::Args)]
struct Run {}

impl Run {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Run the tests in processes.
#[derive(clap::Args)]
struct Launch {
    /// The time limit of each test case.
    #[arg(long, default_value = "30s", value_parser = humantime::parse_duration)]
    timeout: Duration,

    /// Path to a launcher program to perform a test.
    ///
    /// The launcher script is executed on a separate process.  And the execution continues even
    /// if a runtime crashes in the test on the separate process.
    #[arg(value_parser)]
    program: PathBuf,

    /// Optional arguments passed to the launcher program.
    #[arg()]
    args: Vec<String>,
}

impl Launch {
    fn validate(&self) -> Result<()> {
        anyhow::ensure!(
            self.program.is_executable(),
            "launch: <program>: must be an executable file: {}",
            self.program.display()
        );
        Ok(())
    }
}

fn main() -> Result<()> {
    let cl = CommandLine::parse();
    cl.validate()?;
    jsruntime::initialize();
    let mut driver = Driver::new(&cl);
    let num_test_cases = driver.load();
    let progress = if cl.progress {
        let style = ProgressStyle::with_template(
            "{spinner} [{elapsed_precise}] [{bar}] {pos}/{len}\n{msg}",
        )?
        .progress_chars("#>-");
        Some(ProgressBar::new(num_test_cases as u64).with_style(style))
    } else {
        None
    };
    let report = driver.run(progress.as_ref());
    let summary = summarize(&report);
    if let Some(progress) = progress {
        progress.finish_with_message(summary);
    }
    serde_json::to_writer(std::io::stdout().lock(), &report)?;
    Ok(())
}

fn summarize(report: &TestReport) -> String {
    let num_tests = report.results.len();
    let mut num_passed = 0;
    let mut num_failed = 0;
    let mut num_timed_out = 0;
    let mut num_panics = 0;

    for result in report.results.iter() {
        match result.status {
            TestStatus::Passed => num_passed += 1,
            TestStatus::Failed => num_failed += 1,
            TestStatus::TimedOut => num_timed_out += 1,
            TestStatus::Panic => num_panics += 1,
        }
    }

    format!(
        "{num_tests} tests: {num_passed} passed, {num_failed} failed, {num_timed_out} timed-out, {num_panics} panics"
    )
}
