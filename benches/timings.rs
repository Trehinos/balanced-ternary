mod common;

use common::Runner;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

struct Timings<'a>(&'a mut Criterion);

impl Runner for Timings<'_> {
    fn measure<T>(&mut self, name: &str, mut operation: impl FnMut() -> T) {
        self.0.bench_function(name, |bencher| {
            bencher.iter(&mut operation);
        });
    }

    fn measure_owned<I, T>(
        &mut self,
        name: &str,
        mut setup: impl FnMut() -> I,
        mut operation: impl FnMut(I) -> T,
    ) {
        self.0.bench_function(name, |bencher| {
            bencher.iter_batched(&mut setup, &mut operation, BatchSize::SmallInput);
        });
    }
}

fn benchmarks(criterion: &mut Criterion) {
    common::run(&mut Timings(criterion));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
