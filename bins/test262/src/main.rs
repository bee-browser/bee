mod driver;
mod launcher;
mod metadata;
mod report;
mod runner;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as _;
use is_executable::IsExecutable;

use driver::Driver;

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

    /// Shows progress.
    #[arg(long)]
    progress: bool,

    /// Enables the scope cleanup checker.
    #[arg(long)]
    scope_cleanup_checker: bool,

    /// Time limit in milliseconds.
    #[arg(long, default_value = "5000")]
    timeout: i64,

    /// Path to a launcher script to perform a test.
    ///
    /// The launcher script is executed on a separate process.  And the execution continues even
    /// if a runtime crashes in the test on the separate process.
    #[arg(long)]
    launcher: Option<PathBuf>,

    /// Tests to be run.
    ///
    /// All tests are run when no test is specified.
    #[arg()]
    tests: Vec<String>,
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
        if let Some(launcher) = self.launcher.as_deref() {
            anyhow::ensure!(
                launcher.is_executable(),
                "<launcher>: must be an executable file: {}",
                launcher.display()
            );
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let cl = CommandLine::parse();
    cl.validate()?;
    jsruntime::initialize();
    let mut driver = Driver::new(&cl);
    driver.load();
    let report = driver.run();
    serde_json::to_writer(std::io::stdout().lock(), &report)?;
    Ok(())
}
