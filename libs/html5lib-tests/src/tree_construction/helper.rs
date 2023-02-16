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
    stack: Vec<usize>,
}

impl<'a> TreeValidator<'a> {
    fn new(test: &'a Test) -> Self {
        TreeValidator {
            test,
            nodes: vec![Node::Document {
                child_nodes: vec![],
            }],
            stack: vec![0],
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
            } => {
                v.push(LinearNode {
                    depth,
                    repr: format!("<{}>", name),
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
    fn append(&mut self, node_index: usize) {
        let parent_index = *self.stack.last().unwrap();
        match self.nodes.get_mut(parent_index).unwrap() {
            Node::Document {
                ref mut child_nodes,
            } => {
                child_nodes.push(node_index);
            }
            Node::Element {
                ref mut child_nodes,
                ..
            } => {
                child_nodes.push(node_index);
            }
            t => unreachable!("{:?}", t),
        }
    }
}

impl<'a> DocumentWriter for TreeValidator<'a> {
    fn append_doctype(&mut self, doctype: &Doctype<'_>) {
        tracing::debug!(?doctype);
        let index = self.nodes.len();
        self.nodes.push(Node::DocumentType {
            name: doctype.name.map(str::to_string),
            public_id: doctype.public_id.map(str::to_string),
            system_id: doctype.system_id.map(str::to_string),
            force_quirks: doctype.force_quirks,
        });
        self.append(index);
    }

    fn push_element(&mut self, tag: &Tag<'_>) {
        tracing::debug!(?tag);
        let index = self.nodes.len();
        self.nodes.push(Node::Element {
            name: tag.name().into(),
            attrs: tag
                .attrs()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect(),
            child_nodes: vec![],
        });
        self.append(index);
        self.stack.push(index);
    }

    fn pop(&mut self) {
        let index = self.stack.pop().unwrap();
        let node = self.nodes.get(index).unwrap();
        tracing::debug!(?node);
    }

    fn append_text(&mut self, text: &str) {
        tracing::debug!(?text);
        let index = self.nodes.len();
        self.nodes.push(Node::Text(text.to_string()));
        self.append(index);
    }

    fn append_comment(&mut self, comment: &Comment<'_>) {
        tracing::debug!(?comment);
        let index = self.nodes.len();
        self.nodes.push(Node::Comment(comment.data.into()));
        self.append(index);
    }

    fn end(&mut self) {
        let mut v = vec![];
        self.flatten(0, 0, &mut v);
        assert_eq!(v, self.test.document, "{}", self.test.data);
    }
}

#[derive(Debug)]
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
