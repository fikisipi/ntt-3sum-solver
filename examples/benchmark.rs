//! Emits the CSV table behind `docs/compute.svg`.
//!
//! This is a lightweight, dependency-free harness intended for regenerating
//! the documentation plot. For rigorous, statistically-sound measurements
//! (outlier detection, confidence intervals) use the Criterion benchmark:
//! `cargo bench`.
//!
//! Reports the *median* of several timed runs after a warmup pass. Median is
//! used rather than the mean because it is far less sensitive to the
//! occasional slow run (scheduler noise, allocator, turbo clocking).

use std::time::Instant;

#[path = "../benches/support.rs"]
mod support;
use support::{has_three_sum_quadratic, make_input, universe, Instance};

use three_sum::Solution;

const WARMUP: usize = 2;
const SAMPLES: usize = 11;

/// Median wall-clock milliseconds of `runs` timed invocations of `f`, after
/// `WARMUP` untimed passes.
fn median_ms(mut f: impl FnMut()) -> f64 {
    for _ in 0..WARMUP {
        f();
    }

    let mut samples = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let started = Instant::now();
        f();
        samples.push(started.elapsed().as_secs_f64() * 1_000.0);
    }

    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    samples[samples.len() / 2]
}

fn run_case(n: usize, span: i32) {
    let nums = make_input(n, span, Instance::No);
    let u = universe(&nums);

    let mut convolution_result = None;
    let convolution_ms = median_ms(|| convolution_result = Solution::has_three_sum(nums.clone()));

    let mut quadratic_result = false;
    let quadratic_ms = median_ms(|| quadratic_result = has_three_sum_quadratic(nums.clone()));

    println!(
        "{n},{u},{convolution_ms:.3},{quadratic_ms:.3},{convolution_result:?},{quadratic_result}"
    );
}

fn main() {
    println!("n,universe,convolution_median_ms,quadratic_median_ms,convolution_result,quadratic_result");

    for (n, span) in [(2_000, 500), (10_000, 500), (50_000, 500), (50_000, 12_500)] {
        run_case(n, span);
    }
}
