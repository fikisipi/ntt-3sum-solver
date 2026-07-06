//! Criterion benchmarks for the bounded 3SUM decision solver.
//!
//! Three views:
//!   * `scale_n`       — hold the universe fixed, grow `n`; shows the
//!                       convolution solver is (nearly) flat in `n` while the
//!                       quadratic baseline is not.
//!   * `scale_universe`— hold `n` fixed, grow `U`; shows the convolution
//!                       solver's `O(U log U)` dependence on the value domain.
//!   * `yes_instance`  — a satisfiable instance, so the short-circuiting
//!                       baseline can return early.
//!
//! Run with `cargo bench`.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

use three_sum::Solution;

mod support;
use support::{has_three_sum_quadratic, make_input, universe, Instance};

/// Benchmark both solvers on one input under a shared group.
fn compare(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>, id: u64, nums: &[i32]) {
    group.bench_with_input(BenchmarkId::new("convolution", id), nums, |b, nums| {
        b.iter_batched(
            || nums.to_vec(),
            |input| black_box(Solution::has_three_sum(black_box(input))),
            BatchSize::LargeInput,
        );
    });
    group.bench_with_input(BenchmarkId::new("quadratic", id), nums, |b, nums| {
        b.iter_batched(
            || nums.to_vec(),
            |input| black_box(has_three_sum_quadratic(black_box(input))),
            BatchSize::LargeInput,
        );
    });
}

fn scale_n(c: &mut Criterion) {
    let mut group = c.benchmark_group("scale_n");
    group.sample_size(20);
    for &n in &[1_000usize, 5_000, 20_000, 50_000] {
        let nums = make_input(n, 500, Instance::No);
        group.throughput(Throughput::Elements(n as u64));
        compare(&mut group, n as u64, &nums);
    }
    group.finish();
}

fn scale_universe(c: &mut Criterion) {
    let mut group = c.benchmark_group("scale_universe");
    group.sample_size(20);
    for &span in &[125i32, 500, 2_000, 12_500] {
        let nums = make_input(20_000, span, Instance::No);
        let u = universe(&nums) as u64;
        group.throughput(Throughput::Elements(u));
        compare(&mut group, u, &nums);
    }
    group.finish();
}

fn yes_instance(c: &mut Criterion) {
    let mut group = c.benchmark_group("yes_instance");
    group.sample_size(20);
    for &n in &[5_000usize, 50_000] {
        let nums = make_input(n, 500, Instance::Yes);
        group.throughput(Throughput::Elements(n as u64));
        compare(&mut group, n as u64, &nums);
    }
    group.finish();
}

criterion_group!(benches, scale_n, scale_universe, yes_instance);
criterion_main!(benches);
