use std::hint::black_box;
use std::time::Instant;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use jsruntime::BasicRuntime;

const SAMPLE_SIZE: usize = 200;

const DATA_SET: &[(&str, &str)] = &[
    ("fib16.js", include_str!("dataset/fib16.js")),
];

macro_rules! elapsed {
    ($target:expr) => {
        {
            let start = Instant::now();
            $target;
            start.elapsed()
        }
    };
}

fn init(c: &mut Criterion) {
    jsruntime::initialize();
    c.bench_function("jsruntime/init", |b| b.iter(|| black_box(BasicRuntime::new())));
}

fn parse(c: &mut Criterion) {
    jsruntime::initialize();
    let mut group = c.benchmark_group("jsruntime/parse");
    group.sample_size(SAMPLE_SIZE);
    for data in DATA_SET.iter() {
        group.bench_function(data.0, |b| {
            b.iter_custom(|iters| {
                let mut total = Default::default();
                for _i in 0..iters {
                    let mut runtime = BasicRuntime::new();
                    total += elapsed! {
                        black_box(runtime.parse_script(black_box(data.1)).unwrap())
                    };
                }
                total
            })
        });
    }
    group.finish();
}

// NOTE: The `compile` time does NOT include the link time.
fn compile(c: &mut Criterion) {
    jsruntime::initialize();
    let mut group = c.benchmark_group("jsruntime/compile");
    group.sample_size(SAMPLE_SIZE);
    for data in DATA_SET.iter() {
        group.bench_function(data.0, |b| {
            b.iter_custom(|iters| {
                let mut total = Default::default();
                for _i in 0..iters {
                    let mut runtime = BasicRuntime::new();
                    let program = runtime.parse_script(data.1).unwrap();
                    total += elapsed! {
                        black_box(runtime.compile(black_box(&program), black_box(true)).unwrap())
                    };
                }
                total
            })
        });
    }
    group.finish();
}

fn link(c: &mut Criterion) {
    jsruntime::initialize();
    let mut group = c.benchmark_group("jsruntime/link");
    group.sample_size(SAMPLE_SIZE);
    for data in DATA_SET.iter() {
        group.bench_function(data.0, |b| {
            b.iter_custom(|iters| {
                let mut total = Default::default();
                for _i in 0..iters {
                    let mut runtime = BasicRuntime::new();
                    let program = runtime.parse_script(data.1).unwrap();
                    let module = runtime.compile(&program, true).unwrap();
                    total += elapsed! {
                        black_box(runtime.link(module))
                    };
                }
                total
            })
        });
    }
    group.finish();
}

fn evaluate(c: &mut Criterion) {
    jsruntime::initialize();
    let mut group = c.benchmark_group("jsruntime/evaluate");
    group.sample_size(SAMPLE_SIZE);
    for data in DATA_SET.iter() {
        group.bench_function(data.0, |b| {
            b.iter_custom(|iters| {
                let mut total = Default::default();
                for _i in 0..iters {
                    let mut runtime = BasicRuntime::new();
                    let program = runtime.parse_script(data.1).unwrap();
                    let module = runtime.compile(&program, true).unwrap();
                    runtime.link(module);
                    total += elapsed! {
                        black_box(runtime.evaluate(black_box(&program)).unwrap())
                    };
                }
                total
            })
        });
    }
    group.finish();
}

fn full(c: &mut Criterion) {
    jsruntime::initialize();
    let mut group = c.benchmark_group("jsruntime/full");
    group.sample_size(SAMPLE_SIZE);
    for data in DATA_SET.iter() {
        group.bench_function(data.0, |b| {
            b.iter(|| {
                let mut runtime = BasicRuntime::new();
                let program = runtime.parse_script(data.1).unwrap();
                let module = runtime.compile(&program, true).unwrap();
                runtime.link(module);
                black_box(runtime.evaluate(black_box(&program)).unwrap());
            })
        });
    }
    group.finish();
}

criterion_group! {
    benches,
    init,
    parse,
    compile,
    link,
    evaluate,
    full,
}

criterion_main! {
    benches,
}
