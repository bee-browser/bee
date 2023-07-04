use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
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
struct Opt {
    /// Tokens that the generated DFA recognizes.
    #[arg()]
    tokens: Vec<String>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let opt = Opt::parse();

    let rules: HashMap<String, Rule> = serde_yaml::from_reader(std::io::stdin())?;
    let grammar = Grammar::new(&rules, &opt.tokens);
    let dfa = grammar.compile();
    let unicode_sets = dfa.build_unicode_sets();

    tracing::info!(
        rules.size = rules.len(),
        tokens.size = opt.tokens.len(),
        unicode_sets.size = unicode_sets.len(),
        dfa.size = dfa.size(),
    );

    let states = dfa.export_states();
    let ascii_table = build_ascii_table(&unicode_sets);
    let non_ascii_list = build_non_ascii_list(&unicode_sets);

    serde_json::to_writer(
        std::io::stdout(),
        &DfaSpec {
            tokens: opt.tokens,
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
