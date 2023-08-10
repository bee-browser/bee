use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use tracing_subscriber::filter::EnvFilter;

use bee_dfagen::automaton::Dfa;
use bee_dfagen::automaton::StateData;
use bee_dfagen::grammar::Grammar;
use bee_dfagen::grammar::Rule;
use bee_dfagen::unicode::CodePoint;
use bee_dfagen::unicode::UnicodeSet;
use bee_dfagen::unicode::UnicodeSpan;

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

    let states = dfa.export_states();
    let ascii_table = build_ascii_table(&unicode_sets);
    let non_ascii_list = build_non_ascii_list(&unicode_sets);

    serde_json::to_writer(
        std::io::stdout(),
        &DfaSpec {
            tokens: cl.tokens,
            unicode_sets,
            dfa,
            // for the backward-compatibility
            states,
            ascii_table,
            non_ascii_list,
        },
    )?;

    Ok(())
}

fn build_ascii_table(unicode_sets: &[UnicodeSet]) -> Vec<u8> {
    assert!(unicode_sets.len() < u8::MAX as usize);
    let table: Vec<u8> = ('\u{0000}'..='\u{007F}')
        .map(|ch| {
            unicode_sets
                .iter()
                .position(|unicode_set| unicode_set.contains_span(&ch.into()))
                .unwrap_or(unicode_sets.len()) as u8
        })
        .collect();
    assert_eq!(table.len(), 128);
    table
}

fn build_non_ascii_list(unicode_sets: &[UnicodeSet]) -> Vec<NonAsciiEntry> {
    let ascii = UnicodeSpan::from('\u{0000}'..='\u{007F}');
    let mut list = vec![];
    for (i, unicode_set) in unicode_sets.iter().enumerate() {
        let non_ascii = unicode_set.exclude_span(&ascii);
        if non_ascii.is_empty() {
            continue;
        }
        for span in non_ascii.spans() {
            if span.len() == 1 {
                list.push(NonAsciiEntry {
                    span: false,
                    first_code_point: span.first_code_point(),
                    last_code_point: span.last_code_point(),
                    unicode_set: i,
                });
            } else {
                list.push(NonAsciiEntry {
                    span: true,
                    first_code_point: span.first_code_point(),
                    last_code_point: span.last_code_point(),
                    unicode_set: i,
                });
            }
        }
    }
    list
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DfaSpec {
    pub tokens: Vec<String>,
    pub unicode_sets: Vec<UnicodeSet>,
    pub dfa: Dfa,
    // for the backward compatibility
    pub states: Vec<StateData>,
    pub ascii_table: Vec<u8>,
    pub non_ascii_list: Vec<NonAsciiEntry>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NonAsciiEntry {
    span: bool,
    first_code_point: CodePoint,
    last_code_point: CodePoint,
    unicode_set: usize,
}
