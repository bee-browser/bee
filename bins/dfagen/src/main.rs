#![doc = include_str!("../README.md")]

mod automaton;
mod grammar;
mod logger;
mod unicode;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

use automaton::Dfa;
use grammar::Grammar;
use grammar::Rule;
use unicode::CodePoint;
use unicode::UnicodeSet;

#[derive(Parser)]
#[command(author, version, about)]
struct CommandLine {
    /// A path to an YAML file defining the syntactic grammar.
    ///
    /// Read from STDIN if not specified.
    #[arg(short, long)]
    grammar: Option<PathBuf>,

    /// Tokens that the generated DFA recognizes.
    #[arg()]
    tokens: Vec<String>,
}

fn main() -> Result<()> {
    logging::init();

    let cl = CommandLine::parse();

    logger::info!("Loading rules...");
    let rules: Vec<Rule> = match cl.grammar {
        Some(grammar) => serde_yaml::from_reader(std::fs::File::open(grammar)?)?,
        None => serde_yaml::from_reader(std::io::stdin())?,
    };
    logger::info!(rules.size = rules.len());

    logger::info!("Trimming rules...");
    let rules = grammar::trim(&rules, &cl.tokens);
    logger::info!(rules = rules.len());

    let grammar = Grammar::new(&rules);

    logger::info!("Building an NFA from the lexical grammar in CFG...");
    let nfa = grammar.build_nfa(&cl.tokens);
    logger::info!("#States in NFA: {}", nfa.size());

    logger::info!("Building DFA from NFA...");
    let dfa = nfa.build_dfa();
    logger::info!("#States in DFA: {}", dfa.size());

    logger::info!("Minifying DFA...");
    let dfa = dfa.minify(&cl.tokens);
    logger::info!("#States in DFA (minified): {}", dfa.size());

    let unicode_sets = dfa.build_unicode_sets();
    let unicode_set_labels = unicode_sets
        .iter()
        .map(|unicode_set| format!("{unicode_set}"))
        .collect();

    logger::info!(
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
            unicode_set_labels,
            dfa,
        },
    )?;

    Ok(())
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DfaSpec {
    tokens: Vec<String>,
    unicode_sets: Vec<UnicodeSet>,
    unicode_set_labels: Vec<String>,
    dfa: Dfa,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct NonAsciiEntry {
    span: bool,
    first_code_point: CodePoint,
    last_code_point: CodePoint,
    unicode_set: usize,
}
