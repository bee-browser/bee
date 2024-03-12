use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use jsruntime::Runtime;

fn fib(c: &mut Criterion) {
    const SRC: &str = include_str!("dataset/fib.js");
    Runtime::initialize();
    c.bench_function("fib", |b| {
        b.iter(|| {
            let mut runtime = Runtime::new();
            let module = runtime.compile_script(&SRC).unwrap();
            runtime.eval(module);
        })
    });
}

criterion_group! {
    benches,
    fib,
}

criterion_main! {
    benches,
}
