use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use tracing_subscriber::filter::EnvFilter;

use bee_dfagen::automaton::Dfa;
use bee_dfagen::grammar::Grammar;
use bee_dfagen::grammar::Rule;
use bee_dfagen::unicode::CodePoint;
use bee_dfagen::unicode::UnicodeSet;

#[derive(Parser)]
#[command(author, version, about)]
struct CommandLine {
    /// Logging format.
    #[arg(long, value_enum, env = "BEE_LOG_FORMAT", default_value = "text")]
    log_format: LogFormat,

    /// A path to an YAML file defining the syntactic grammar.
    ///
    /// Read from STDIN if not specified.
    #[arg(short, long)]
    grammar: Option<PathBuf>,

    /// Tokens that the generated DFA recognizes.
    #[arg()]
    tokens: Vec<String>,
}

#[derive(Clone, ValueEnum)]
enum LogFormat {
    Text,
    Json,
}

fn main() -> Result<()> {
    let cl = CommandLine::parse();

    match cl.log_format {
        LogFormat::Text => {
            tracing_subscriber::fmt()
                .with_writer(std::io::stderr)
                .with_env_filter(EnvFilter::from_default_env())
                .init();
        }
        LogFormat::Json => {
            tracing_subscriber::fmt()
                .json()
                .with_writer(std::io::stderr)
                .with_env_filter(EnvFilter::from_default_env())
                .init();
        }
    }

    let rules: Vec<Rule> = match cl.grammar {
        Some(grammar) => serde_yaml::from_reader(std::fs::File::open(grammar)?)?,
        None => serde_yaml::from_reader(std::io::stdin())?,
    };
    let grammar = Grammar::new(&rules, &cl.tokens);
    let dfa = grammar.compile();
    let unicode_sets = dfa.build_unicode_sets();

    tracing::info!(
        rules.size = rules.len(),
        tokens.size = cl.tokens.len(),
        unicode_sets.size = unicode_sets.len(),
        dfa.size = dfa.size(),
    );

    serde_json::to_writer(
        std::io::stdout(),
        &DfaSpec {
            tokens: cl.tokens,
            unicode_sets,
            dfa,
        },
    )?;

    Ok(())
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DfaSpec {
    pub tokens: Vec<String>,
    pub unicode_sets: Vec<UnicodeSet>,
    pub dfa: Dfa,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NonAsciiEntry {
    span: bool,
    first_code_point: CodePoint,
    last_code_point: CodePoint,
    unicode_set: usize,
}
