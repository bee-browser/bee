use std::path::Path;

use criterion::Criterion;

use htmlparser::*;
use toydom::NullBuilder;

fn htmlparser_benchmark(c: &mut Criterion) {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .unwrap()
        .join("vendor")
        .join("src")
        .join("bee-browser")
        .join("rust-html-parser-benchmark")
        .join("data");
    for entry in data_dir.read_dir().expect("read_dir call failed").flatten() {
        let filepath = entry.path();
        match filepath.extension() {
            Some(ext) if ext == "html" => run_bench(c, &filepath),
            _ => (),
        }
    }
}

fn run_bench(c: &mut Criterion, filepath: &Path) {
    let html = std::fs::read_to_string(filepath).expect("cannot read file");
    let data: Vec<u16> = html.encode_utf16().collect();
    let test_name = format!("htmlparser/parse/{}", filepath.file_name().unwrap().to_str().unwrap());
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
