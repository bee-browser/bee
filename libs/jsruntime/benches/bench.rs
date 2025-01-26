use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use jsruntime::BasicRuntime;

fn fib(c: &mut Criterion) {
    const FIB16: &str = include_str!("dataset/fib16.js");
    const FIB24: &str = include_str!("dataset/fib24.js");
    //const FIB32: &str = include_str!("dataset/fib32.js");

    jsruntime::initialize();

    let mut group = c.benchmark_group("jsruntime/fib");
    group.sample_size(1_000);

    macro_rules! fib {
        ($label:literal, $src:expr) => {
            group.bench_function($label, |b| {
                b.iter(|| {
                    let mut runtime = BasicRuntime::new();
                    let program = runtime.parse_script($src).unwrap();
                    let module = runtime.compile(&program, true).unwrap();
                    runtime.evaluate(module).unwrap();
                })
            });
        };
    }

    fib!("16", FIB16);
    fib!("24", FIB24);
    //fib!("32", FIB32); // removed because it's very slow...

    group.finish();
}

fn parse(c: &mut Criterion) {
    const FIB32: &str = include_str!("dataset/fib32.js");

    jsruntime::initialize();

    let mut group = c.benchmark_group("jsruntime/parse");

    macro_rules! parse {
        ($label:literal, $src:expr) => {
            group.bench_function($label, |b| {
                let mut runtime = BasicRuntime::new();
                b.iter(|| {
                    black_box(runtime.parse_script(black_box($src)).unwrap());
                })
            });
        };
    }

    parse!("fib32.js", FIB32);

    group.finish();
}

fn compile(c: &mut Criterion) {
    const FIB32: &str = include_str!("dataset/fib32.js");

    jsruntime::initialize();

    let mut group = c.benchmark_group("jsruntime/compile");

    macro_rules! compile {
        ($label:literal, $src:expr) => {
            group.bench_function($label, |b| {
                let mut runtime = BasicRuntime::new();
                let program = runtime.parse_script($src).unwrap();
                b.iter(|| {
                    black_box(
                        runtime
                            .compile(black_box(&program), black_box(true))
                            .unwrap(),
                    );
                })
            });
        };
    }

    compile!("fib32.js", FIB32);

    group.finish();
}

criterion_group! {
    benches,
    fib,
    parse,
    compile,
}

criterion_main! {
    benches,
}
