use itertools::Itertools;

use htmlparser::*;
use toydom::ToyBuilder;
use toydom::ToyNode;
use toydom::delegate;

pub fn parse(test: Test) {
    match test.scripting {
        Scripting::Off => do_parse(&test, false),
        Scripting::On => do_parse(&test, true),
        Scripting::Both => {
            do_parse(&test, false);
            do_parse(&test, true);
        }
    }
}

fn do_parse(test: &Test, scripting: bool) {
    let mut parser = Parser::new(TreeValidator::new(test));
    parser.set_scripting(scripting);
    if let Some((namespace, local_name)) = test.context_element {
        let namespace = match namespace {
            "html" => Namespace::Html,
            "math" => Namespace::MathMl,
            "svg" => Namespace::Svg,
            _ => panic!(),
        };
        parser.set_context_element(local_name, namespace, 0)
    }
    let data: Vec<u16> = test.data.encode_utf16().collect();
    parser.feed_data(&data);
    parser.feed_end();
    parser.parse();
}

struct TreeValidator<'a> {
    test: &'a Test,
    builder: ToyBuilder,
}

impl<'a> TreeValidator<'a> {
    fn new(test: &'a Test) -> Self {
        TreeValidator {
            test,
            builder: ToyBuilder::new(),
        }
    }

    fn flatten(&self, depth: usize, index: usize, v: &mut Vec<LinearNode>) {
        match self.builder.get_node(index) {
            ToyNode::Document { child_nodes, .. } => {
                let child_nodes = if self.test.for_html_fragment_parsing() {
                    self.builder.child_nodes(1)
                } else {
                    child_nodes
                };
                for &child_index in child_nodes.iter() {
                    self.flatten(depth, child_index, v);
                }
            }
            ToyNode::DocumentType {
                name,
                public_id,
                system_id,
                ..
            } => {
                let name = name.as_deref().unwrap_or_default();
                let repr = if public_id.is_none() && system_id.is_none() {
                    format!("<!DOCTYPE {name}>")
                } else {
                    let public_id = public_id.as_deref().unwrap_or_default();
                    let system_id = system_id.as_deref().unwrap_or_default();
                    format!(r#"<!DOCTYPE {name} "{public_id}" "{system_id}">"#)
                };
                v.push(LinearNode { depth, repr });
            }
            ToyNode::Element {
                name,
                attrs,
                child_nodes,
                namespace,
                ..
            } => {
                v.push(LinearNode {
                    depth,
                    repr: match namespace {
                        Namespace::Html => format!("<{name}>"),
                        Namespace::MathMl => format!("<math {name}>"),
                        Namespace::Svg => format!("<svg {name}>"),
                    },
                });
                let depth = if *namespace == Namespace::Html && name == "template" {
                    v.push(LinearNode {
                        depth: depth + 1,
                        repr: "content".to_string(),
                    });
                    depth + 1
                } else {
                    depth
                };
                let mut attr_nodes = attrs
                    .iter()
                    .map(|(name, value)| match namespace {
                        Namespace::Html => format!(r#"{name}="{value}""#),
                        _ => {
                            // Adjust foreign attributes.
                            static XLINK_ATTRS: [&str; 7] = [
                                "xlink:actuate",
                                "xlink:arcrole",
                                "xlink:href",
                                "xlink:role",
                                "xlink:show",
                                "xlink:title",
                                "xlink:type",
                            ];
                            static XML_ATTRS: [&str; 2] = ["xml:lang", "xml:space"];
                            if name == "xmlns:xlink"
                                || XLINK_ATTRS.contains(&name.as_str())
                                || XML_ATTRS.contains(&name.as_str())
                            {
                                let (prefix, local_name) = name.split_once(':').unwrap();
                                format!(r#"{prefix} {local_name}="{value}""#)
                            } else {
                                format!(r#"{name}="{value}""#)
                            }
                        }
                    })
                    .sorted()
                    .map(|repr| LinearNode {
                        depth: depth + 1,
                        repr,
                    });
                v.extend(&mut attr_nodes);
                for &child_index in child_nodes.iter() {
                    self.flatten(depth + 1, child_index, v);
                }
            }
            ToyNode::Text { data, .. } => {
                if let Some(node) = v.last_mut() {
                    if node.depth == depth && node.repr.starts_with('"') {
                        // Concatenate characters.
                        node.repr.pop();
                        node.repr += data;
                        node.repr += "\"";
                        return;
                    }
                }
                v.push(LinearNode {
                    depth,
                    repr: format!(r#""{data}""#),
                });
            }
            ToyNode::Comment { data, .. } => {
                v.push(LinearNode {
                    depth,
                    repr: format!(r#"<!-- {data} -->"#),
                });
            }
        }
    }
}

impl DomTreeBuilder for TreeValidator<'_> {
    type NodeId = usize;

    delegate! {
        builder;
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
    }

    fn end(&mut self) {
        let mut v = vec![];
        self.flatten(0, 0, &mut v);
        assert_eq!(
            v,
            self.test.document,
            "{escaped}",
            escaped = self.test.data.escape_debug()
        );
    }

    fn print_tree(&self) {
        let mut v = vec![];
        self.flatten(0, 0, &mut v);
        logging::debug!(target: "bee.tests", "{v:?}");
    }
}

struct LinearNode {
    depth: usize,
    repr: String,
}

impl PartialEq<(usize, &'static str)> for LinearNode {
    fn eq(&self, &(depth, repr): &(usize, &'static str)) -> bool {
        self.depth == depth && self.repr == repr
    }
}

impl std::fmt::Debug for LinearNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"({depth}, "{repr}")"#,
            depth = self.depth,
            repr = self.repr
        )
    }
}

pub struct Test {
    pub data: &'static str,
    pub document: Vec<(usize, &'static str)>,
    pub context_element: Option<(&'static str, &'static str)>, // (ns, name)
    pub scripting: Scripting,
}

impl Test {
    fn for_html_fragment_parsing(&self) -> bool {
        self.context_element.is_some()
    }
}

#[allow(dead_code)]
pub enum Scripting {
    Off,
    On,
    Both,
}
