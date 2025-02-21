use std::collections::BTreeMap;

use htmlparser::*;

use crate::logger;

pub struct Builder {
    nodes: Vec<Node>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            nodes: vec![Node::Document {
                id: 0,
                child_nodes: vec![],
            }],
        }
    }

    pub fn get_node(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    pub fn nodes(&self) -> &[Node] {
        self.nodes.as_slice()
    }

    pub fn take_child_nodes(&mut self, node: usize) -> Vec<usize> {
        debug_assert!(self.nodes.get(node).is_some());
        let child_nodes = match self.nodes.get_mut(node) {
            Some(Node::Document { child_nodes, .. }) => child_nodes,
            Some(Node::Element { child_nodes, .. }) => child_nodes,
            _ => unreachable!(),
        };
        std::mem::take(child_nodes)
    }

    pub fn child_nodes(&self, node_id: usize) -> &Vec<usize> {
        debug_assert!(self.nodes.get(node_id).is_some());
        match self.nodes.get(node_id) {
            Some(Node::Document { child_nodes, .. }) => child_nodes,
            Some(Node::Element { child_nodes, .. }) => child_nodes,
            _ => unreachable!(),
        }
    }

    pub fn child_nodes_mut(&mut self, node: usize) -> &mut Vec<usize> {
        debug_assert!(self.nodes.get(node).is_some());
        match self.nodes.get_mut(node) {
            Some(Node::Document { child_nodes, .. }) => child_nodes,
            Some(Node::Element { child_nodes, .. }) => child_nodes,
            _ => unreachable!(),
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl DomTreeBuilder for Builder {
    type NodeId = usize;

    fn get_document(&mut self) -> Self::NodeId {
        0
    }

    fn create_doctype(&mut self, doctype: &Doctype<'_>) -> Self::NodeId {
        let id = self.nodes.len();
        let node = Node::DocumentType {
            id,
            name: doctype.name.map(str::to_string),
            public_id: doctype.public_id.map(str::to_string),
            system_id: doctype.system_id.map(str::to_string),
            force_quirks: doctype.force_quirks,
        };
        logger::debug!(?node);
        self.nodes.push(node);
        id
    }

    fn create_element(&mut self, name: &str, ns: Namespace) -> Self::NodeId {
        let id = self.nodes.len();
        let node = Node::Element {
            id,
            name: name.to_string(),
            attrs: Default::default(),
            child_nodes: vec![],
            namespace: ns,
        };
        logger::debug!(?node);
        self.nodes.push(node);
        id
    }

    fn create_text(&mut self, data: &str) -> Self::NodeId {
        let id = self.nodes.len();
        let node = Node::Text {
            id,
            data: data.to_string(),
        };
        logger::debug!(?node);
        self.nodes.push(node);
        id
    }

    fn create_comment(&mut self, data: &str) -> Self::NodeId {
        let id = self.nodes.len();
        let node = Node::Comment {
            id,
            data: data.to_string(),
        };
        logger::debug!(?node);
        self.nodes.push(node);
        id
    }

    fn set_attributes<'b, I>(&mut self, node_id: Self::NodeId, attrs: I, overwrite: bool)
    where
        I: Iterator<Item = (&'b str, &'b str)>,
    {
        debug_assert!(self.nodes.get(node_id).is_some());
        let element_attrs = match &mut self.nodes[node_id] {
            Node::Element { attrs, .. } => attrs,
            _ => unreachable!(),
        };
        if overwrite {
            for (name, value) in attrs {
                logger::debug!(node_id, attr.name = name, attr.value = value);
                element_attrs.insert(name.to_string(), value.to_string());
            }
        } else {
            for (name, value) in attrs {
                if element_attrs.contains_key(name) {
                    continue;
                }
                logger::debug!(node_id, attr.name = name, attr.value = value);
                element_attrs.insert(name.to_string(), value.to_string());
            }
        }
    }

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
                namespace: *namespace,
                child_nodes: vec![],
            },
            _ => unreachable!(),
        };
        logger::debug!(node = ?self.nodes[node_id], clone = ?node);
        self.nodes.push(node);
        id
    }

    fn append_child(&mut self, parent_id: Self::NodeId, node_id: Self::NodeId) {
        debug_assert!(self.nodes.get(parent_id).is_some());
        debug_assert!(self.nodes.get(node_id).is_some());
        let child_nodes = self.child_nodes_mut(parent_id);
        child_nodes.push(node_id);
        logger::debug!(parent = ?self.nodes[parent_id], node = ?self.nodes[node_id]);
    }

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
        logger::debug!(parent = ?self.nodes[parent_id], node = ?self.nodes[node_id], sibling = ?self.nodes[sibling_id]);
    }

    fn remove_child(&mut self, parent_id: Self::NodeId, node_id: Self::NodeId) {
        debug_assert!(self.nodes.get(parent_id).is_some());
        debug_assert!(self.nodes.get(node_id).is_some());
        let child_nodes = self.child_nodes_mut(parent_id);
        debug_assert!(child_nodes.contains(&node_id));
        child_nodes.retain(|&child_id| child_id != node_id);
        logger::debug!(parent = ?self.nodes[parent_id], node = ?self.nodes[node_id]);
    }

    fn move_child_nodes(&mut self, src_id: Self::NodeId, dst_id: Self::NodeId) {
        debug_assert!(self.nodes.get(src_id).is_some());
        debug_assert!(self.nodes.get(dst_id).is_some());
        let mut src_child_nodes = self.take_child_nodes(src_id);
        let dst_child_nodes = self.child_nodes_mut(dst_id);
        dst_child_nodes.append(&mut src_child_nodes);
        logger::debug!(src = ?self.nodes[src_id], dst = ?self.nodes[dst_id]);
    }

    fn end(&mut self) {}

    fn print_tree(&self) {}
}

#[derive(Clone)]
pub enum Node {
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

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Document { id, .. } => {
                write!(f, "{id}:#document")
            }
            Self::DocumentType { id, name, .. } => {
                write!(f, "{id}:<!DOCTYPE")?;
                if let Some(name) = name {
                    write!(f, " {name}")?;
                }
                write!(f, ">")
            }
            Self::Element {
                id,
                name,
                namespace,
                ..
            } => {
                write!(f, "{id}:<{namespace:?}:{name}>")
            }
            Self::Text { id, data } => {
                write!(f, "{id}:#text:{escaped}", escaped = data.escape_debug())
            }
            Self::Comment { id, data } => {
                write!(f, "{id}:#comment:{escaped}", escaped = data.escape_debug())
            }
        }
    }
}
