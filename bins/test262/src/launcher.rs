use std::process::Stdio;
use std::time::Duration;
use std::process::Command;
use std::time::Instant;

use crate::driver::Error;
use crate::driver::TestCase;
use crate::Launch;

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
        args.push(format!("{}", harness.path().display()));
    }
    args.push(format!("{}", test_case.path.display()));
    let output = Command::new("timeout")
        .arg(launch.timeout.as_secs_f64().to_string())
        .arg(&launch.program)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    let duration = Instant::elapsed(&start);
    let result = match output.status.code() {
        Some(0) => Ok(()),
        Some(1) => Err(Error::Runtime),
        Some(2) => Err(Error::Parse),
        Some(124) => Err(Error::TimedOut),
        None | Some(101) => Err(Error::Panic),
        _ => unreachable!(),
    };
    (result, duration)
}
