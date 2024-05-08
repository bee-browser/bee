mod memory_allocation;

use criterion::criterion_main;

criterion_main! {
    memory_allocation::benches,
}
