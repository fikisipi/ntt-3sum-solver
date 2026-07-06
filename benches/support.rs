//! Shared benchmark helpers: deterministic input generation and the
//! standard sorted two-pointer baseline the convolution solver is compared
//! against. Included by both the Criterion benchmark (`benches/three_sum.rs`)
//! and the CSV-emitting example (`examples/benchmark.rs`).
//!
//! Some items are used by only one of the two consumers, so silence the
//! per-target dead-code warnings.
#![allow(dead_code)]

/// Whether a generated instance should contain a zero-sum triple.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Instance {
    /// No three entries sum to zero (a no-instance).
    No,
    /// At least one zero-sum triple exists (a yes-instance).
    Yes,
}

/// Deterministically generate `n` values within a universe of roughly
/// `8 * span` centered on zero.
///
/// Base values are `4 * bucket + 1`, i.e. all congruent to 1 (mod 4). Any
/// three such values sum to 3 (mod 4), which is never 0, so the base instance
/// is guaranteed to be a no-instance regardless of `n`. For [`Instance::Yes`]
/// three entries are overwritten with `0`, which lies inside the universe and
/// therefore leaves `U` unchanged while injecting a zero-sum triple.
pub fn make_input(n: usize, span: i32, instance: Instance) -> Vec<i32> {
    let mut seed = 0x1234_5678_9abc_def0_u64;
    let mut nums = Vec::with_capacity(n);

    for _ in 0..n {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let bucket = (seed % (2 * span) as u64) as i32 - span;
        nums.push(4 * bucket + 1);
    }

    if instance == Instance::Yes && n >= 3 {
        nums[0] = 0;
        nums[1] = 0;
        nums[2] = 0;
    }

    nums
}

/// Size of the integer universe `U = max - min + 1` for an input.
pub fn universe(nums: &[i32]) -> i64 {
    let min = *nums.iter().min().unwrap() as i64;
    let max = *nums.iter().max().unwrap() as i64;
    max - min + 1
}

/// Standard `O(n^2)` sorted two-pointer 3SUM decision baseline.
pub fn has_three_sum_quadratic(mut nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }

    nums.sort_unstable();

    for i in 0..nums.len() - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        if nums[i] > 0 {
            break;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] as i64 + nums[left] as i64 + nums[right] as i64;

            if sum == 0 {
                return true;
            }

            if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    false
}
