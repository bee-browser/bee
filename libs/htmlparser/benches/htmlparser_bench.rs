//<exclude:coverage>
use std::path::PathBuf;

use bee_htmlparser::*;
use criterion::Criterion;

fn htmlparser_benchmark(c: &mut Criterion) {
    run_bench(c, "lipsum.html");
    run_bench(c, "medium-fragment.html");
    run_bench(c, "wikipedia.html");
}

fn run_bench(c: &mut Criterion, name: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("benches")
        .join("data")
        .join(name);
    let html = std::fs::read_to_string(&path).expect("cannot read file");
    let data: Vec<u16> = html.encode_utf16().collect();
    let test_name = format!("parsing {}", name);
    c.bench_function(&test_name, |b| {
        b.iter(|| {
            let mut parser = Parser::new(DummyBuilder::new());
            parser.feed_data(&data);
            parser.feed_end();
            parser.parse();
        })
    });
}

struct DummyBuilder {
    next_id: usize,
}

impl DummyBuilder {
    fn new() -> Self {
        DummyBuilder { next_id: 1 }
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl DomTreeBuilder for DummyBuilder {
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

criterion::criterion_group!(benches, htmlparser_benchmark);
criterion::criterion_main!(benches);
//</exclude:coverage>
