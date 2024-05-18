use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use jsruntime::Runtime;

fn fib(c: &mut Criterion) {
    const FIB16: &str = include_str!("dataset/fib16.js");
    const FIB24: &str = include_str!("dataset/fib24.js");
    const FIB32: &str = include_str!("dataset/fib32.js");

    let mut group = c.benchmark_group("fib");

    macro_rules! fib {
        ($label:literal, $src:expr) => {
            group.bench_function($label, |b| {
                b.iter(|| {
                    let mut runtime = Runtime::new();
                    let module = runtime.compile_script($src, true).unwrap();
                    runtime.eval(module);
                })
            });
        };
    }

    Runtime::initialize();
    fib!("16", FIB16);
    fib!("24", FIB24);
    group.measurement_time(Duration::from_secs(10));
    fib!("32", FIB32);
    group.finish();
}

criterion_group! {
    benches,
    fib,
}

criterion_main! {
    benches,
}
