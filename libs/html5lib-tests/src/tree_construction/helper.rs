//<coverage:exclude>
use std::{collections::BTreeMap, fmt::Debug};

use bee_htmlparser::*;

#[ctor::ctor]
fn init() {
    tracing_subscriber::fmt::init();
}

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
            "mathml" => Namespace::MathMl,
            "svg" => Namespace::Svg,
            _ => panic!(),
        };
        parser.set_context_element(local_name, namespace, 0)
    }
    parser.feed_data(test.data.encode_utf16().collect());
    parser.feed_end();
    parser.parse();
}

struct TreeValidator<'a> {
    test: &'a Test,
    nodes: Vec<Node>,
}

impl<'a> TreeValidator<'a> {
    fn new(test: &'a Test) -> Self {
        TreeValidator {
            test,
            nodes: vec![Node::Document {
                id: 0,
                child_nodes: vec![],
            }],
        }
    }

    fn flatten(&self, depth: usize, index: usize, v: &mut Vec<LinearNode>) {
        match &self.nodes[index] {
            Node::Document { child_nodes, .. } => {
                for &child_index in child_nodes.iter() {
                    self.flatten(depth, child_index, v);
                }
            }
            Node::DocumentType {
                name,
                public_id,
                system_id,
                ..
            } => {
                let name = name.as_deref().unwrap_or_default();
                let repr = if public_id.is_none() && system_id.is_none() {
                    format!("<!DOCTYPE {}>", name)
                } else {
                    let public_id = public_id.as_deref().unwrap_or_default();
                    let system_id = system_id.as_deref().unwrap_or_default();
                    format!(r#"<!DOCTYPE {} "{}" "{}">"#, name, public_id, system_id)
                };
                v.push(LinearNode { depth, repr });
            }
            Node::Element {
                name,
                attrs,
                child_nodes,
                namespace,
                ..
            } => {
                v.push(LinearNode {
                    depth,
                    repr: match namespace {
                        Namespace::Html => format!("<{}>", name),
                        Namespace::MathMl => format!("<math {}>", name),
                        Namespace::Svg => format!("<svg {}>", name),
                    },
                });
                for (name, value) in attrs.iter() {
                    v.push(LinearNode {
                        depth: depth + 1,
                        repr: format!(r#"{}="{}""#, name, value),
                    });
                }
                for &child_index in child_nodes.iter() {
                    self.flatten(depth + 1, child_index, v);
                }
            }
            Node::Text { data, .. } => {
                v.push(LinearNode {
                    depth,
                    repr: format!(r#""{}""#, data),
                });
            }
            Node::Comment { data, .. } => {
                v.push(LinearNode {
                    depth,
                    repr: format!(r#"<!-- {} -->"#, data),
                });
            }
        }
    }
}

impl<'a> TreeValidator<'a> {
    fn take_child_nodes(&mut self, node: usize) -> Vec<usize> {
        debug_assert!(self.nodes.get(node).is_some());
        let child_nodes = match self.nodes.get_mut(node) {
            Some(Node::Document {
                ref mut child_nodes,
                ..
            }) => child_nodes,
            Some(Node::Element {
                ref mut child_nodes,
                ..
            }) => child_nodes,
            _ => unreachable!(),
        };
        std::mem::replace(child_nodes, vec![])
    }

    fn child_nodes_mut(&mut self, node: usize) -> &mut Vec<usize> {
        debug_assert!(self.nodes.get(node).is_some());
        match self.nodes.get_mut(node) {
            Some(Node::Document {
                ref mut child_nodes,
                ..
            }) => child_nodes,
            Some(Node::Element {
                ref mut child_nodes,
                ..
            }) => child_nodes,
            _ => unreachable!(),
        }
    }
}

impl<'a> DomTreeBuilder for TreeValidator<'a> {
    type NodeId = usize;

    #[tracing::instrument(level = "debug", skip_all)]
    fn get_document(&mut self) -> Self::NodeId {
        0
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn get_root(&mut self) -> Self::NodeId {
        // In the HTML5 specification, an 'html' element is created and appended
        // to the document, but the expected document tree of each test cases in
        // html5lib-tests for the HTML fragment parsing algorithm has no root
        // 'html' element.
        0
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn create_doctype(&mut self, doctype: &Doctype<'_>) -> Self::NodeId {
        let id = self.nodes.len();
        let node = Node::DocumentType {
            id,
            name: doctype.name.map(str::to_string),
            public_id: doctype.public_id.map(str::to_string),
            system_id: doctype.system_id.map(str::to_string),
            force_quirks: doctype.force_quirks,
        };
        tracing::debug!(?node);
        self.nodes.push(node);
        id
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn create_element(&mut self, name: &str, ns: Namespace) -> Self::NodeId {
        let id = self.nodes.len();
        let node = Node::Element {
            id,
            name: name.to_string(),
            attrs: Default::default(),
            child_nodes: vec![],
            namespace: ns,
        };
        tracing::debug!(?node);
        self.nodes.push(node);
        id
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn create_text(&mut self, data: &str) -> Self::NodeId {
        let id = self.nodes.len();
        let node = Node::Text {
            id,
            data: data.to_string(),
        };
        tracing::debug!(?node);
        self.nodes.push(node);
        id
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn create_comment(&mut self, data: &str) -> Self::NodeId {
        let id = self.nodes.len();
        let node = Node::Comment {
            id,
            data: data.to_string(),
        };
        tracing::debug!(?node);
        self.nodes.push(node);
        id
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn set_attribute<'b, I>(&mut self, node_id: Self::NodeId, attrs: I, overwrite: bool)
    where
        I: Iterator<Item = (&'b str, &'b str)>,
    {
        debug_assert!(self.nodes.get(node_id).is_some());
        let element_attrs = match self.nodes[node_id] {
            Node::Element { ref mut attrs, .. } => attrs,
            _ => unreachable!(),
        };
        if overwrite {
            for (name, value) in attrs {
                tracing::debug!(node_id, attr.name = name, attr.value = value);
                element_attrs.insert(name.to_string(), value.to_string());
            }
        } else {
            for (name, value) in attrs {
                if element_attrs.contains_key(name) {
                    continue;
                }
                tracing::debug!(node_id, attr.name = name, attr.value = value);
                element_attrs.insert(name.to_string(), value.to_string());
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn clone_node(&mut self, node_id: Self::NodeId) -> Self::NodeId {
        debug_assert!(self.nodes.get(node_id).is_some());
        let id = self.nodes.len();
        let node = match &self.nodes[node_id] {
            Node::Element {
                name,
                attrs,
                namespace,
                ..
            } => Node::Element {
                id,
                name: name.clone(),
                attrs: attrs.clone(),
                namespace: namespace.clone(),
                child_nodes: vec![],
            },
            _ => unreachable!(),
        };
        tracing::debug!(node = ?self.nodes[node_id], clone = ?node);
        self.nodes.push(node);
        id
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_child(&mut self, parent_id: Self::NodeId, node_id: Self::NodeId) {
        debug_assert!(self.nodes.get(parent_id).is_some());
        debug_assert!(self.nodes.get(node_id).is_some());
        let child_nodes = self.child_nodes_mut(parent_id);
        child_nodes.push(node_id);
        tracing::debug!(parent = ?self.nodes[parent_id], node = ?self.nodes[node_id]);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn insert_before(
        &mut self,
        parent_id: Self::NodeId,
        node_id: Self::NodeId,
        sibling_id: Self::NodeId,
    ) {
        debug_assert!(self.nodes.get(parent_id).is_some());
        debug_assert!(self.nodes.get(node_id).is_some());
        debug_assert!(self.nodes.get(sibling_id).is_some());
        let child_nodes = self.child_nodes_mut(parent_id);
        debug_assert!(child_nodes.contains(&sibling_id));
        let pos = child_nodes
            .iter()
            .position(|&child_id| child_id == sibling_id)
            .unwrap();
        child_nodes.insert(pos, node_id);
        tracing::debug!(parent = ?self.nodes[parent_id], node = ?self.nodes[node_id], sibling = ?self.nodes[sibling_id]);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn remove_child(&mut self, parent_id: Self::NodeId, node_id: Self::NodeId) {
        debug_assert!(self.nodes.get(parent_id).is_some());
        debug_assert!(self.nodes.get(node_id).is_some());
        let child_nodes = self.child_nodes_mut(parent_id);
        debug_assert!(child_nodes.contains(&node_id));
        child_nodes.retain(|&child_id| child_id != node_id);
        tracing::debug!(parent = ?self.nodes[parent_id], node = ?self.nodes[node_id]);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn move_child_nodes(&mut self, src_id: Self::NodeId, dst_id: Self::NodeId) {
        debug_assert!(self.nodes.get(src_id).is_some());
        debug_assert!(self.nodes.get(dst_id).is_some());
        let mut src_child_nodes = self.take_child_nodes(src_id);
        let dst_child_nodes = self.child_nodes_mut(dst_id);
        dst_child_nodes.append(&mut src_child_nodes);
        tracing::debug!(src = ?self.nodes[src_id], dst = ?self.nodes[dst_id]);
    }

    fn end(&mut self) {
        let mut v = vec![];
        self.flatten(0, 0, &mut v);
        assert_eq!(v, self.test.document, "{}", self.test.data);
    }

    fn print_tree(&self) {
        let mut v = vec![];
        self.flatten(0, 0, &mut v);
        tracing::debug!("{:?}", v);
    }

    fn has_same_name(&mut self, node_id: Self::NodeId, name: &str) -> bool {
        match self.nodes[node_id] {
            Node::Element { name: ref v, .. } => v.eq_ignore_ascii_case(name),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
enum Node {
    Document {
        id: usize,
        child_nodes: Vec<usize>,
    },
    DocumentType {
        id: usize,
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
        #[allow(unused)]
        force_quirks: bool,
    },
    Element {
        id: usize,
        name: String,
        attrs: BTreeMap<String, String>,
        child_nodes: Vec<usize>,
        namespace: Namespace,
    },
    Text {
        id: usize,
        data: String,
    },
    Comment {
        id: usize,
        data: String,
    },
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Document { id, .. } => {
                write!(f, "{}:#document", id)
            }
            Self::DocumentType { id, name, .. } => {
                write!(f, "{}:<!DOCTYPE", id)?;
                if let Some(name) = name {
                    write!(f, " {}", name)?;
                }
                write!(f, ">")
            }
            Self::Element {
                id,
                name,
                namespace,
                ..
            } => {
                write!(f, "{}:<{:?}:{}>", id, namespace, name)
            }
            Self::Text { id, data } => {
                write!(f, "{}:#text:{}", id, data.escape_debug())
            }
            Self::Comment { id, data } => {
                write!(f, "{}:#comment:{}", id, data.escape_debug())
            }
        }
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
        write!(f, "({}, {:?})", self.depth, self.repr)
    }
}

pub struct Test {
    pub data: &'static str,
    pub document: Vec<(usize, &'static str)>,
    pub context_element: Option<(&'static str, &'static str)>, // (ns, name)
    pub scripting: Scripting,
}

pub enum Scripting {
    Off,
    On,
    Both,
}
//</coverage:exclude>
