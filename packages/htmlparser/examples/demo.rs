use std::io::Read;

use anyhow::Result;

use bee_htmlparser::*;
use bee_toydom::delegate;
use bee_toydom::ToyBuilder;
use bee_toydom::ToyNode;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut html = String::new();
    std::io::stdin().read_to_string(&mut html)?;
    let data: Vec<u16> = html.encode_utf16().collect();

    let mut parser = Parser::new(DemoBuilder::new());
    parser.feed_data(&data);
    parser.feed_end();
    parser.parse();
    Ok(())
}

struct DemoBuilder {
    inner: ToyBuilder,
}

impl DemoBuilder {
    fn new() -> Self {
        DemoBuilder {
            inner: ToyBuilder::new(),
        }
    }

    fn print(&self, depth: usize, node_id: usize) {
        print!("{:depth$}", "");
        match self.inner.get_node(node_id) {
            ToyNode::Document { child_nodes, .. } => {
                for &child_id in child_nodes.iter() {
                    self.print(depth, child_id);
                }
            }
            ToyNode::DocumentType {
                name,
                public_id,
                system_id,
                ..
            } => {
                let name = name.as_deref().unwrap_or_default();
                if public_id.is_none() && system_id.is_none() {
                    println!("<!DOCTYPE {}>", name);
                } else {
                    let public_id = public_id.as_deref().unwrap_or_default();
                    let system_id = system_id.as_deref().unwrap_or_default();
                    println!(r#"<!DOCTYPE {} "{}" "{}">"#, name, public_id, system_id)
                }
            }
            ToyNode::Element {
                name,
                attrs,
                child_nodes,
                namespace,
                ..
            } => {
                match namespace {
                    Namespace::Html => {
                        print!("<html:{}", name);
                    }
                    Namespace::MathMl => {
                        print!("<math:{}", name);
                    }
                    Namespace::Svg => {
                        print!("<svg:{}", name);
                    }
                }
                for (name, value) in attrs.iter() {
                    print!(r#" {}="{}""#, name, value.escape_debug());
                }
                println!(">");
                for &child_id in child_nodes.iter() {
                    self.print(depth + 1, child_id);
                }
            }
            ToyNode::Text { data, .. } => {
                println!(r#""{}""#, data.escape_debug());
            }
            ToyNode::Comment { data, .. } => {
                println!("<!-- {} -->", data);
            }
        }
    }
}

impl DomTreeBuilder for DemoBuilder {
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
        self.print(0, 0);
    }
}
