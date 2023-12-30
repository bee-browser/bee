//<exclude:coverage>
use htmlparser::*;

pub struct NullBuilder {
    next_id: usize,
}

impl NullBuilder {
    pub fn new() -> Self {
        NullBuilder { next_id: 1 }
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl DomTreeBuilder for NullBuilder {
    type NodeId = usize;

    fn get_document(&mut self) -> Self::NodeId {
        0
    }

    fn create_doctype(&mut self, _doctype: &Doctype<'_>) -> Self::NodeId {
        self.next_id()
    }

    fn create_element(&mut self, _name: &str, _ns: Namespace) -> Self::NodeId {
        self.next_id()
    }

    fn create_text(&mut self, _data: &str) -> Self::NodeId {
        self.next_id()
    }

    fn create_comment(&mut self, _data: &str) -> Self::NodeId {
        self.next_id()
    }

    fn set_attributes<'a, I>(&mut self, _node: Self::NodeId, _attrs: I, _overwrite: bool)
    where
        I: Iterator<Item = (&'a str, &'a str)>,
    {
    }

    fn clone_node(&mut self, _node: Self::NodeId) -> Self::NodeId {
        self.next_id()
    }

    fn append_child(&mut self, _parent: Self::NodeId, _node: Self::NodeId) {}

    fn insert_before(
        &mut self,
        _parent: Self::NodeId,
        _node: Self::NodeId,
        _sibling: Self::NodeId,
    ) {
    }

    fn remove_child(&mut self, _parent: Self::NodeId, _node: Self::NodeId) {}

    fn move_child_nodes(&mut self, _node: Self::NodeId, _new_parent: Self::NodeId) {}

    fn end(&mut self) {}

    fn print_tree(&self) {}
}
//</exclude:coverage>
