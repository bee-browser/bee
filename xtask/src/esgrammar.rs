use std::io::Read;

use anyhow::Result;
use bee_htmlparser::*;
use bee_toydom::delegate;
use bee_toydom::ToyBuilder;
use bee_toydom::ToyNode;
use clap::Args;

#[derive(Args)]
pub struct Opt {}

pub fn main(_opt: Opt) -> Result<()> {
    let mut html = String::new();
    std::io::stdin().read_to_string(&mut html)?;
    let data: Vec<u16> = html.encode_utf16().collect();

    let mut parser = Parser::new(GrammarPrinter::new());
    parser.feed_data(&data);
    parser.feed_end();
    parser.parse();
    Ok(())
}

struct GrammarPrinter {
    inner: ToyBuilder,
    sec_id: &'static str,
}

impl GrammarPrinter {
    fn new() -> Self {
        GrammarPrinter {
            inner: ToyBuilder::new(),
            sec_id: "sec-ecmascript-language-lexical-grammar",
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
                    if id != self.sec_id {
                        return;
                    }
                    for &child_id in child_nodes.iter() {
                        self.search_grammar(child_id);
                    }
                }
                return;
            }
            _ => (),
        }
    }

    fn search_grammar(&self, node_id: usize) {
        match self.inner.get_node(node_id) {
            ToyNode::Element {
                name,
                attrs,
                child_nodes,
                ..
            } => {
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
            _ => (),
        }
    }

    fn print_grammar(&self, node_id: usize) {
        match self.inner.get_node(node_id) {
            ToyNode::Text { data, .. } => {
                let mut indent = false;
                for line in data.split('\n') {
                    let line = line.trim();
                    if line.starts_with("//") {
                        continue;
                    }
                    if line.is_empty() {
                        if indent {
                            println!("");
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
                    println!("");
                }
            }
            _ => (),
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
