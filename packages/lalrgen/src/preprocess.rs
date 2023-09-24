use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;

use crate::grammar::Grammar;
use crate::grammar::Lookahead;
use crate::grammar::NonTerminal;
use crate::grammar::Rule;
use crate::grammar::Term;
use crate::phrase::MatchStatus;

/// Preprocesses a grammar.
///
/// This function resolves *inner* lookahead restrictions.  The resultant grammar has no inner
/// lookahead restrictions (but it may have *outer* lookahead restrictions).
///
/// Generally, the resultant grammar size is larger then the original grammar size.  As a result,
/// the number of states in the corresponding LR(0) automaton increases.
pub fn preprocess(grammar: Grammar) -> Grammar {
    preprocess_lookaheads(grammar)
}

#[tracing::instrument(level = "debug", skip_all)]
fn preprocess_lookaheads(grammar: Grammar) -> Grammar {
    let mut variant_table = VariantNameTable::new();
    match preprocess_non_tail_lookaheads(grammar, &mut variant_table) {
        PreprocessResult::Changed(grammar) => grammar,
        PreprocessResult::NotChanged(grammar) => grammar,
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn preprocess_non_tail_lookaheads(
    grammar: Grammar,
    variant_table: &mut VariantNameTable,
) -> PreprocessResult {
    let mut remaining = VecDeque::with_capacity(grammar.len());
    remaining.extend(grammar.rules().iter().cloned());

    let mut changed = false;
    let mut rules = vec![];
    while let Some(rule) = remaining.pop_front() {
        if rule.production.len() < 2 {
            rules.push(rule);
            continue;
        }

        let n = rule.production.len();

        let num_lookaheads = rule.production[0..n - 1]
            .iter()
            .filter(|term| term.is_lookahead())
            .count();
        if num_lookaheads == 0 {
            rules.push(rule);
            continue;
        }

        changed = true;

        let mut preprocessor = LookaheadPreprocessor::new(n, &grammar, variant_table);
        for term in rule.production.iter() {
            if !preprocessor.preprocess(&rule.name, term) {
                break;
            }
        }
        remaining.extend(preprocessor.variant_rules.iter().cloned());
        if preprocessor.is_invalid() {
            tracing::trace!(invalidated = %rule);
            continue;
        }

        // Add the modified rule which might be removed if a non-tail lookahead restriction
        // copied into a rule referred from the production doesn't meet.  See the next
        // `removal` loop for details.
        let modified = Arc::new(Rule::with_rule(
            rule.name.clone(),
            preprocessor.take_production(),
            rule.clone(),
        ));
        tracing::trace!(%modified, original = %rule);
        rules.push(modified);
    }

    tracing::debug!(changed);
    if changed {
        rules = remove_invalidated_rules(rules);
        PreprocessResult::Changed(Grammar::new(rules))
    } else {
        PreprocessResult::NotChanged(grammar)
    }
}

/// Remove rules containing non-terminal symbols which were invalidated.
fn remove_invalidated_rules(mut rules: Vec<Arc<Rule>>) -> Vec<Arc<Rule>> {
    loop {
        let mut non_terminals = HashSet::new();
        for rule in rules.iter() {
            non_terminals.insert(&rule.name);
        }
        let mut new_rules = Vec::with_capacity(rules.len());
        for rule in rules.iter() {
            let valid = rule
                .production
                .iter()
                .filter_map(|term| match term {
                    Term::NonTerminal(non_terminal) => Some(non_terminal),
                    _ => None,
                })
                .all(|non_terminal| non_terminals.contains(&non_terminal));
            if valid {
                new_rules.push(rule.clone());
            } else {
                tracing::trace!(invalidated = %rule);
            }
        }
        if new_rules.len() == rules.len() {
            break;
        }
        rules = new_rules;
    }

    rules
}

struct LookaheadPreprocessor<'g, 't> {
    grammar: &'g Grammar,
    table: &'t mut VariantNameTable,
    lookahead: Option<Arc<Lookahead>>,
    production: Vec<Term>,
    variant_rules: Vec<Arc<Rule>>,
    invalid_rule: bool,
}

impl<'g, 't> LookaheadPreprocessor<'g, 't> {
    fn new(n: usize, grammar: &'g Grammar, table: &'t mut VariantNameTable) -> Self {
        LookaheadPreprocessor {
            grammar,
            table,
            lookahead: None,
            production: Vec::with_capacity(n),
            variant_rules: vec![],
            invalid_rule: false,
        }
    }

    #[tracing::instrument(level = "trace", skip_all, fields(%non_terminal, %term))]
    fn preprocess(&mut self, non_terminal: &NonTerminal, term: &Term) -> bool {
        match (term, self.lookahead.take()) {
            (Term::NonTerminal(non_terminal), Some(lookahead)) => {
                tracing::trace!(%non_terminal, %lookahead);
                let variant_name = self
                    .table
                    .map
                    .entry((non_terminal.clone(), lookahead.clone()))
                    .or_insert_with(|| {
                        let variant_name = non_terminal.with_variant(self.table.next_variant_id);
                        self.table.next_variant_id += 1;

                        // Add variant rules.
                        for rule in self.grammar.non_terminal_rules(non_terminal) {
                            let mut variant_production = vec![Term::Lookahead(lookahead.clone())];
                            variant_production.extend(rule.production.iter().cloned());
                            let variant = Arc::new(Rule::with_rule(
                                variant_name.clone(),
                                variant_production,
                                rule.clone(),
                            ));
                            tracing::trace!(%variant, original = %rule);
                            self.variant_rules.push(variant);
                        }

                        variant_name
                    });
                self.production
                    .push(Term::NonTerminal(variant_name.clone()));
                true
            }
            (term, Some(lookahead)) => match lookahead.process_token(&format!("{term}")) {
                MatchStatus::Matched => {
                    tracing::trace!("matched");
                    self.production.push(term.clone());
                    true
                }
                MatchStatus::Unmatched => {
                    tracing::trace!("unmatched");
                    self.invalid_rule = true;
                    false
                }
                MatchStatus::Remaining(next_lookahead) => {
                    tracing::trace!(%next_lookahead);
                    self.production.push(term.clone());
                    self.lookahead = Some(next_lookahead);
                    true
                }
            },
            (term, None) => {
                match term {
                    Term::Lookahead(lookahead) => {
                        tracing::trace!(%lookahead);
                        self.lookahead = Some(lookahead.clone());
                    }
                    _ => {
                        self.production.push(term.clone());
                    }
                }
                true
            }
        }
    }

    fn is_invalid(&self) -> bool {
        self.invalid_rule
    }

    fn take_production(&mut self) -> Vec<Term> {
        let mut production = std::mem::replace(&mut self.production, vec![]);
        if let Some(lookahead) = self.lookahead.take() {
            production.push(Term::Lookahead(lookahead));
        }
        production
    }
}

struct VariantNameTable {
    next_variant_id: usize,
    map: HashMap<(NonTerminal, Arc<Lookahead>), NonTerminal>,
}

impl VariantNameTable {
    fn new() -> Self {
        VariantNameTable {
            next_variant_id: 1,
            map: Default::default(),
        }
    }
}

enum PreprocessResult {
    Changed(Grammar),
    NotChanged(Grammar),
}
