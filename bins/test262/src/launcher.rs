use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;

use crate::Launch;
use crate::driver::Error;
use crate::driver::TestCase;

pub fn run(test_case: &TestCase, launch: &Launch) -> (Result<(), Error>, Duration) {
    let start = Instant::now();
    let mut args = launch.args.clone();
    if test_case.strict {
        args.push("--strict".to_string());
    }
    if test_case.metadata.is_module() {
        args.push("--module".to_string());
    } else {
        args.push("--script".to_string());
    }
    for harness in test_case.includes.iter() {
        args.push("--harness".to_string());
        args.push(harness.path().to_string_lossy().into_owned());
    }
    args.push(test_case.path.to_string_lossy().into_owned());
    let output = Command::new("timeout")
        .arg(launch.timeout.as_secs_f64().to_string())
        .arg(&launch.program)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    let duration = start.elapsed();
    let result = match output.status.code() {
        Some(0) => Ok(()),
        Some(1) => Err(Error::Runtime),
        Some(2) => Err(Error::Parse),
        Some(124) => Err(Error::TimedOut),
        Some(137) => Err(Error::TimedOut),
        Some(101) | Some(132) | Some(134) | Some(139) | None => Err(Error::Panic),
        Some(_code) => Err(Error::Panic),
    };
    (result, duration)
}
