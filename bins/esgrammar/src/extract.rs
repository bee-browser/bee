use std::io::Read;

use anyhow::Result;
use clap::Args;
use clap::ValueEnum;

use htmlparser::*;
use toydom::delegate;
use toydom::ToyBuilder;
use toydom::ToyNode;

#[derive(Args)]
pub struct Opt {
    /// A type of grammar to extract.
    #[arg(value_enum)]
    grammar: GrammarKind,
}

#[derive(Clone, ValueEnum)]
enum GrammarKind {
    LexicalGrammar,
    SyntacticGrammar,
}

pub fn main(opt: Opt) -> Result<()> {
    let mut html = String::new();
    std::io::stdin().read_to_string(&mut html)?;
    let data: Vec<u16> = html.encode_utf16().collect();

    let mut parser = Parser::new(GrammarPrinter::new(opt.grammar));
    parser.feed_data(&data);
    parser.feed_end();
    parser.parse();
    Ok(())
}

struct GrammarPrinter {
    inner: ToyBuilder,
    sec_ids: &'static [&'static str],
}

impl GrammarPrinter {
    fn new(grammar: GrammarKind) -> Self {
        GrammarPrinter {
            inner: ToyBuilder::new(),
            sec_ids: match grammar {
                GrammarKind::LexicalGrammar => &["sec-ecmascript-language-lexical-grammar"],
                GrammarKind::SyntacticGrammar => &[
                    "sec-ecmascript-language-expressions",
                    "sec-ecmascript-language-statements-and-declarations",
                    "sec-ecmascript-language-functions-and-classes",
                    "sec-ecmascript-language-scripts-and-modules",
                ],
            },
        }
    }

    fn search_section(&self, node_id: usize) {
        match self.inner.get_node(node_id) {
            ToyNode::Document { child_nodes, .. } => {
                for &child_id in child_nodes.iter() {
                    self.search_section(child_id);
                }
            }
            ToyNode::Element {
                name,
                attrs,
                child_nodes,
                ..
            } => {
                if name == "html" || name == "body" {
                    for &child_id in child_nodes.iter() {
                        self.search_section(child_id);
                    }
                } else if name == "emu-clause" {
                    let id = match attrs.get("id") {
                        Some(id) => id,
                        None => return,
                    };
                    if !self.sec_ids.contains(&id.as_str()) {
                        return;
                    }
                    for &child_id in child_nodes.iter() {
                        self.search_grammar(child_id);
                    }
                }
            }
            _ => (),
        }
    }

    fn search_grammar(&self, node_id: usize) {
        if let ToyNode::Element {
            name,
            attrs,
            child_nodes,
            ..
        } = self.inner.get_node(node_id)
        {
            if name == "emu-grammar" {
                let type_ = match attrs.get("type") {
                    Some(type_) => type_,
                    None => return,
                };
                if type_ != "definition" {
                    return;
                }
                for &child_id in child_nodes.iter() {
                    self.print_grammar(child_id);
                }
                return;
            }
            for &child_id in child_nodes.iter() {
                self.search_grammar(child_id);
            }
        }
    }

    fn print_grammar(&self, node_id: usize) {
        if let ToyNode::Text { data, .. } = self.inner.get_node(node_id) {
            let mut indent = false;
            for line in data.split('\n') {
                let line = line.trim();
                if line.starts_with("//") {
                    continue;
                }
                if line.is_empty() {
                    if indent {
                        println!();
                    }
                    indent = false;
                } else if indent {
                    println!("  {line}");
                } else {
                    println!("{line}");
                    indent = true;
                }
            }
            if indent {
                println!();
            }
        }
    }
}

impl DomTreeBuilder for GrammarPrinter {
    type NodeId = usize;

    delegate! {
        inner;
        get_document,
        create_doctype,
        create_element,
        create_text,
        create_comment,
        set_attributes,
        clone_node,
        append_child,
        insert_before,
        remove_child,
        move_child_nodes,
        print_tree,
    }

    fn end(&mut self) {
        self.search_section(0);
    }
}
