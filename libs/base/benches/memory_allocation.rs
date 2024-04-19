use criterion::measurement::Measurement;
use criterion::Criterion;
use criterion::{black_box, criterion_group};
use criterion_cycles_per_byte::CyclesPerByte;

#[derive(Clone, Copy)]
struct Data {
    _buf: [u8; 256],
}

impl Default for Data {
    fn default() -> Self {
        Data { _buf: [0u8; 256] }
    }
}

fn box_new<M: Measurement + 'static>(c: &mut Criterion<M>) {
    c.bench_function("Box::new", |b| {
        b.iter(|| {
            let d = Box::<Data>::default();
            black_box(d);
        })
    });
}

fn vec_push<M: Measurement + 'static>(c: &mut Criterion<M>) {
    c.bench_function("vec_push", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for _ in 0..1000 {
                v.push(Data::default());
            }
            black_box(v);
        })
    });
}

fn vec_push_with_capacity<M: Measurement + 'static>(c: &mut Criterion<M>) {
    c.bench_function("vec_push_with_capacity", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(1000);
            for _ in 0..1000 {
                v.push(Data::default());
            }
            black_box(v);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = box_new, vec_push, vec_push_with_capacity
}
