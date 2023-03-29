//<exclude:coverage>
use std::path::PathBuf;

use bee_htmlparser::*;
use bee_toydom::NullBuilder;
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
            let mut parser = Parser::new(NullBuilder::new());
            parser.feed_data(&data);
            parser.feed_end();
            parser.parse();
        })
    });
}
criterion::criterion_group!(benches, htmlparser_benchmark);
criterion::criterion_main!(benches);
//</exclude:coverage>
