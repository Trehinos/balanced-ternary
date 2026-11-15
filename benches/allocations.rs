mod common;

use common::Runner;
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::hint::black_box;

#[global_allocator]
static ALLOCATOR: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

struct Allocations;
fn count<T>(name: &str, operation: impl FnOnce() -> T) {
    let region = Region::new(ALLOCATOR);
    let output = black_box(operation());
    let stats = region.change();
    drop(output);

    println!("{name},{},{}", stats.allocations, stats.reallocations);
}

impl Runner for Allocations {
    fn measure<T>(&mut self, name: &str, operation: impl FnMut() -> T) {
        count(name, operation);
    }

    fn measure_owned<I, T>(
        &mut self,
        name: &str,
        mut setup: impl FnMut() -> I,
        mut operation: impl FnMut(I) -> T,
    ) {
        let input = setup();
        count(name, || operation(input));
    }
}

fn main() {
    println!("benchmark,allocations,reallocations");
    common::run(&mut Allocations);
}
