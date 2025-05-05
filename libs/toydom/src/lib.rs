logging::define_logger! {"bee::toydom"}

mod null;
mod toy;

pub use null::NullBuilder;
pub use toy::Builder as ToyBuilder;
pub use toy::Node as ToyNode;

#[macro_export]
macro_rules! delegate {
    ($delegate:ident; get_document) => {
        fn get_document(&mut self) -> Self::NodeId {
            self.$delegate.get_document()
        }
    };
    ($delegate:ident; create_doctype) => {
        fn create_doctype(&mut self, doctype: &Doctype<'_>) -> Self::NodeId {
            self.$delegate.create_doctype(doctype)
        }
    };
    ($delegate:ident; create_element) => {
        fn create_element(&mut self, name: &str, ns: Namespace) -> Self::NodeId {
            self.$delegate.create_element(name, ns)
        }
    };
    ($delegate:ident; create_text) => {
        fn create_text(&mut self, data: &str) -> Self::NodeId {
            self.$delegate.create_text(data)
        }
    };
    ($delegate:ident; create_comment) => {
        fn create_comment(&mut self, data: &str) -> Self::NodeId {
            self.$delegate.create_comment(data)
        }
    };
    ($delegate:ident; set_attributes) => {
        fn set_attributes<'lifetime, I>(&mut self, node: Self::NodeId, attrs: I, overwrite: bool)
        where
            I: Iterator<Item = (&'lifetime str, &'lifetime str)>,
        {
            self.$delegate.set_attributes(node, attrs, overwrite)
        }
    };
    ($delegate:ident; clone_node) => {
        fn clone_node(&mut self, node: Self::NodeId) -> Self::NodeId {
            self.$delegate.clone_node(node)
        }
    };
    ($delegate:ident; append_child) => {
        fn append_child(&mut self, parent: Self::NodeId, node: Self::NodeId) {
            self.$delegate.append_child(parent, node)
        }
    };
    ($delegate:ident; insert_before) => {
        fn insert_before(&mut self, parent: Self::NodeId, node: Self::NodeId, sibling: Self::NodeId) {
            self.$delegate.insert_before(parent, node, sibling)
        }
    };
    ($delegate:ident; remove_child) => {
        fn remove_child(&mut self, parent: Self::NodeId, node: Self::NodeId) {
            self.$delegate.remove_child(parent, node)
        }
    };
    ($delegate:ident; move_child_nodes) => {
        fn move_child_nodes(&mut self, node: Self::NodeId, new_parent: Self::NodeId) {
            self.$delegate.move_child_nodes(node, new_parent)
        }
    };
    ($delegate:ident; end) => {
        fn end(&mut self) {
            self.$delegate.end()
        }
    };
    ($delegate:ident; print_tree) => {
        fn print_tree(&self) {
            self.$delegate.print_tree()
        }
    };
    ($delegate:ident; $method:ident, $($more:ident),+) => {
        delegate!{$delegate; $method}
        delegate!{$delegate; $($more),+}
    };
    ($delegate:ident; $($more:ident,)+) => {
        delegate!{$delegate; $($more),+}
    };
}
