use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;
use itertools::Itertools;
use tracing_subscriber::filter::EnvFilter;

use bee_lalrgen::Grammar;

#[derive(Parser)]
#[command(author, version, about)]
struct Opt {
    /// Logging format.
    #[arg(long, value_enum, env = "BEE_LOG_FORMAT", default_value = "text")]
    log_format: LogFormat,

    /// A path to an YAML file defining the syntactic grammar.
    #[arg()]
    grammar: PathBuf,

    /// A symbol in the syntactic grammar used as the goal symbol.
    #[arg()]
    goal_symbol: String,
}

#[derive(Clone, ValueEnum)]
enum LogFormat {
    Text,
    Json,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    match opt.log_format {
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

    let now = Instant::now();

    tracing::info!("Loading the grammar...");
    let production_rules = serde_yaml::from_reader(File::open(&opt.grammar)?)?;
    let grammar = Grammar::new(production_rules);
    grammar.validate();

    // We must create the augmented grammar before preprocessing.
    let grammar = grammar.create_augmented_grammar(&opt.goal_symbol);
    grammar.validate();

    // Preprocess the syntactic grammar for making subsequent translations easier.
    // The ECMA-262 specification uses non-tail lookahead notations.
    tracing::info!("Preprocessing the grammar...");
    let grammar = grammar.preprocess();
    grammar.validate();

    // Check the maximum number of lookahead tokens in the grammar.
    let max_lookahead_tokens = grammar.max_lookahead_tokens();
    if max_lookahead_tokens > 1 {
        tracing::error!(max_lookahead_tokens, "The grammar is not LALR(1)");
        std::process::exit(1);
    }

    tracing::info!("Collecting the first set of each non-terminal symbol...");
    let first_set = bee_lalrgen::firstset::collect(&grammar, 1);
    // The collected sets will be used in computation of closure of an LR item set.

    tracing::info!("Building LR(0) states...");
    let lr0_states = bee_lalrgen::state::build_lr0_states(&grammar, &first_set);
    tracing::info!("The number of the LR(0) states: {}", lr0_states.len());

    tracing::info!("Building a lookahead table for each LR(0) state...");
    let lookahead_tables =
        bee_lalrgen::lalr::build_lookahead_tables(&grammar, &first_set, &lr0_states);

    tracing::info!("Building LALR(1) states...");
    let lalr1_states = bee_lalrgen::lalr::build_states(&lr0_states, &lookahead_tables);

    tracing::info!(elapsed = %humantime::format_duration(now.elapsed()), "Done");

    serde_json::to_writer(
        std::io::stdout(),
        &bee_lalrgen::LalrSpec {
            goal_symbol: opt.goal_symbol,
            non_terminals: grammar
                .non_terminals()
                .filter(|non_terminal| !non_terminal.is_goal_of_augmented_grammar())
                .map(|non_terminal| non_terminal.symbol())
                .unique()
                .map(|symbol| symbol.to_owned())
                .sorted()
                .collect(),
            states: lalr1_states,
        }
    )?;

    Ok(())
}
