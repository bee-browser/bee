use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use anyhow::bail;
use anyhow::ensure;
use anyhow::Result;
use assert_json_diff::assert_json_matches_no_panic;
use assert_json_diff::CompareMode;
use assert_json_diff::Config;
use clap::Parser;
use serde::Deserialize;
use serde_json::Value;

/// Compare two JSON files.
#[derive(Parser)]
pub struct CommandLine {
    /// LHS.
    #[arg()]
    lhs: PathBuf,

    /// RHS.
    #[arg()]
    rhs: PathBuf,
}

fn main() -> Result<()> {
    let cl = CommandLine::parse();
    ensure!(
        cl.lhs.as_os_str() != "-" || cl.rhs.as_os_str() != "-",
        "No JSON file specified"
    );
    let lhs = load_json(&cl.lhs)?;
    let rhs = load_json(&cl.rhs)?;
    let config = Config::new(CompareMode::Strict);
    if let Err(diff) = assert_json_matches_no_panic(&lhs, &rhs, config) {
        println!("{diff}");
        bail!("Check differences");
    }
    Ok(())
}

fn load_json(path: &Path) -> Result<Value> {
    let value = if path.as_os_str() == "-" {
        let mut de = serde_json::Deserializer::from_reader(std::io::stdin());
        de.disable_recursion_limit();
        Value::deserialize(&mut de)?
    } else {
        let mut de = serde_json::Deserializer::from_reader(File::open(path)?);
        de.disable_recursion_limit();
        Value::deserialize(&mut de)?
    };
    Ok(value)
}
