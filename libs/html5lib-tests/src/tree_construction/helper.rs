//<coverage:exclude>
use bee_htmlparser::*;

#[ctor::ctor]
fn init() {
    tracing_subscriber::fmt::init();
}

pub fn parse(test: Test) {
    let mut parser = Parser::new(TreeValidator::new(&test));
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
                child_nodes: vec![],
            }],
        }
    }

    fn flatten(&self, depth: usize, index: usize, v: &mut Vec<LinearNode>) {
        match self.nodes.get(index).unwrap() {
            Node::Document { child_nodes } => {
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
            Node::Text(s) => {
                v.push(LinearNode {
                    depth,
                    repr: format!(r#""{}""#, s),
                });
            }
            Node::Comment(s) => {
                v.push(LinearNode {
                    depth,
                    repr: format!(r#"<!-- {} -->"#, s),
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
    type Node = usize;

    #[tracing::instrument(level = "debug", skip_all)]
    fn get_root(&mut self) -> Self::Node {
        0
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn create_doctype(&mut self, doctype: &Doctype<'_>) -> Self::Node {
        let node = self.nodes.len();
        tracing::debug!(node, ?doctype);
        self.nodes.push(Node::DocumentType {
            name: doctype.name.map(str::to_string),
            public_id: doctype.public_id.map(str::to_string),
            system_id: doctype.system_id.map(str::to_string),
            force_quirks: doctype.force_quirks,
        });
        node
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn create_element(&mut self, name: &str, ns: Namespace) -> Self::Node {
        let node = self.nodes.len();
        tracing::debug!(node, ?name, ?ns);
        self.nodes.push(Node::Element {
            name: name.to_string(),
            attrs: vec![],
            child_nodes: vec![],
            namespace: ns,
        });
        node
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn create_text(&mut self, data: &str) -> Self::Node {
        let node = self.nodes.len();
        tracing::debug!(node, ?data);
        self.nodes.push(Node::Text(data.to_string()));
        node
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn create_comment(&mut self, data: &str) -> Self::Node {
        let node = self.nodes.len();
        tracing::debug!(node, ?data);
        self.nodes.push(Node::Comment(data.to_string()));
        node
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn set_attribute(&mut self, node: Self::Node, name: &str, value: &str) {
        tracing::debug!(node, name, value);
        debug_assert!(self.nodes.get(node).is_some());
        let attrs = match self.nodes.get_mut(node).unwrap() {
            Node::Element { ref mut attrs, .. } => attrs,
            _ => unreachable!(),
        };
        attrs.push((name.to_string(), value.to_string()));
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn clone_node(&mut self, node: Self::Node) -> Self::Node {
        debug_assert!(self.nodes.get(node).is_some());
        let vnode = match self.nodes[node] {
            Node::Element {
                ref name,
                ref attrs,
                namespace,
                ..
            } => Node::Element {
                name: name.clone(),
                attrs: attrs.clone(),
                namespace,
                child_nodes: vec![],
            },
            _ => unreachable!(),
        };
        let cloned = self.nodes.len();
        self.nodes.push(vnode);
        tracing::debug!(
            node.index = node,
            node.node = ?self.nodes[node],
            cloned.index = cloned,
            cloned.node = ?self.nodes[cloned],
        );
        cloned
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_child(&mut self, parent: Self::Node, node: Self::Node) {
        debug_assert!(self.nodes.get(parent).is_some());
        debug_assert!(self.nodes.get(node).is_some());
        self.child_nodes_mut(parent).push(node);
        tracing::debug!(
            parent.index = parent,
            parent.node = ?self.nodes[parent],
            node.index = node,
            node.node = ?self.nodes[node],
        );
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn insert_before(&mut self, parent: Self::Node, node: Self::Node, sibling: Self::Node) {
        debug_assert!(self.nodes.get(parent).is_some());
        debug_assert!(self.nodes.get(node).is_some());
        debug_assert!(self.nodes.get(sibling).is_some());
        let child_nodes = self.child_nodes_mut(parent);
        debug_assert!(child_nodes.contains(&sibling));
        let pos = child_nodes
            .iter()
            .position(|&child| child == sibling)
            .unwrap();
        child_nodes.insert(pos, node);
        tracing::debug!(
            parent.index = parent,
            parent.node = ?self.nodes[parent],
            node.index = node,
            node.node = ?self.nodes[node],
            sibling.index = sibling,
            sibling.node = ?self.nodes[sibling],
        );
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn remove_child(&mut self, parent: Self::Node, node: Self::Node) {
        debug_assert!(self.nodes.get(parent).is_some());
        debug_assert!(self.nodes.get(node).is_some());
        let child_nodes = self.child_nodes_mut(parent);
        debug_assert!(child_nodes.contains(&node));
        child_nodes.retain(|&n| n != node);
        tracing::debug!(
            parent.index = parent,
            parent.node = ?self.nodes[parent],
            node.index = node,
            node.node = ?self.nodes[node],
        );
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn move_child_nodes(&mut self, node: Self::Node, new_parent: Self::Node) {
        debug_assert!(self.nodes.get(node).is_some());
        debug_assert!(self.nodes.get(new_parent).is_some());
        let mut child_nodes = self.take_child_nodes(node);
        let new_child_nodes = self.child_nodes_mut(new_parent);
        new_child_nodes.append(&mut child_nodes);
        tracing::debug!(
            node.index = node,
            node.node = ?self.nodes[node],
            new_parent.index = new_parent,
            new_parent.node = ?self.nodes[new_parent],
        );
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
}

#[derive(Clone, Debug)]
enum Node {
    Document {
        child_nodes: Vec<usize>,
    },
    DocumentType {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
        #[allow(unused)]
        force_quirks: bool,
    },
    Element {
        name: String,
        attrs: Vec<(String, String)>,
        child_nodes: Vec<usize>,
        namespace: Namespace,
    },
    Text(String),
    Comment(String),
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
}
//</coverage:exclude>
