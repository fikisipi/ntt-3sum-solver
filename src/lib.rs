pub struct Solution;

impl Solution {
    const MAX_SUBQUADRATIC_UNIVERSE: usize = 1 << 20;

    /// Returns whether any three distinct entries sum to zero.
    ///
    /// Returns `None` when the input value range is too large for the
    /// bounded-integer convolution method used here.
    pub fn has_three_sum(nums: Vec<i32>) -> Option<bool> {
        Self::has_three_sum_bounded_integer(nums, Self::MAX_SUBQUADRATIC_UNIVERSE)
    }

    /// Same solver with an explicit caller-supplied value-range limit.
    pub fn has_three_sum_bounded_integer(nums: Vec<i32>, max_universe: usize) -> Option<bool> {
        if nums.len() < 3 {
            return Some(false);
        }

        let min = *nums.iter().min()? as i64;
        let max = *nums.iter().max()? as i64;
        let universe_size = max.checked_sub(min)?.checked_add(1)?;

        if universe_size <= 0
            || universe_size as u128 > max_universe as u128
            || universe_size as u128 > Self::MAX_SUBQUADRATIC_UNIVERSE as u128
        {
            return None;
        }

        let universe = universe_size as usize;
        let mut counts = vec![0_i64; universe];
        for num in nums {
            counts[(num as i64 - min) as usize] += 1;
        }

        let pair_counts = convolution_square(&counts)?;

        for (c_index, &c_count) in counts.iter().enumerate() {
            if c_count == 0 {
                continue;
            }

            let c = min + c_index as i64;
            let target_pair_sum = -c;
            let target_pair_index = target_pair_sum - 2 * min;

            if target_pair_index < 0 || target_pair_index as usize >= pair_counts.len() {
                continue;
            }

            let pair_count = pair_counts[target_pair_index as usize];
            if pair_count == 0 {
                continue;
            }

            if count_distinct_index_triples(pair_count, c, c_count, min, &counts) > 0 {
                return Some(true);
            }
        }

        Some(false)
    }
}

fn count_distinct_index_triples(
    pair_count: i128,
    c: i64,
    c_count: i64,
    min: i64,
    counts: &[i64],
) -> i128 {
    let mut invalid = 0_i128;

    if (-c) % 2 == 0 {
        invalid += count_for_value((-c) / 2, min, counts) * c_count as i128;
    }

    let b_count = count_for_value(-2 * c, min, counts);
    invalid += 2 * c_count as i128 * b_count;

    let repeated_all_same = if c == 0 { 2 * c_count as i128 } else { 0 };

    pair_count * c_count as i128 - invalid + repeated_all_same
}

fn count_for_value(value: i64, min: i64, counts: &[i64]) -> i128 {
    let index = value - min;

    if index < 0 || index as usize >= counts.len() {
        0
    } else {
        counts[index as usize] as i128
    }
}

fn convolution_square(values: &[i64]) -> Option<Vec<i128>> {
    const MODS: [(i64, i64); 3] = [(998_244_353, 3), (1_004_535_809, 3), (469_762_049, 3)];

    if values.is_empty() {
        return Some(Vec::new());
    }

    let result_len = 2 * values.len() - 1;
    let conv_len = result_len.next_power_of_two();

    if conv_len > (1 << 21) {
        return None;
    }

    let first = convolution_square_mod(values, conv_len, MODS[0].0, MODS[0].1);
    let second = convolution_square_mod(values, conv_len, MODS[1].0, MODS[1].1);
    let third = convolution_square_mod(values, conv_len, MODS[2].0, MODS[2].1);

    let m1 = MODS[0].0 as i128;
    let m2 = MODS[1].0 as i128;
    let m3 = MODS[2].0 as i128;
    let m1_inv_m2 = mod_inverse_i128(m1 % m2, m2);
    let m12 = m1 * m2;
    let m12_inv_m3 = mod_inverse_i128(m12 % m3, m3);

    let mut result = Vec::with_capacity(result_len);

    for i in 0..result_len {
        let a1 = first[i] as i128;
        let a2 = second[i] as i128;
        let a3 = third[i] as i128;

        let t2 = positive_mod(a2 - a1, m2) * m1_inv_m2 % m2;
        let x12 = a1 + m1 * t2;
        let t3 = positive_mod(a3 - x12, m3) * m12_inv_m3 % m3;

        result.push(x12 + m12 * t3);
    }

    Some(result)
}

fn convolution_square_mod(
    values: &[i64],
    len: usize,
    modulus: i64,
    primitive_root: i64,
) -> Vec<i64> {
    let mut data = vec![0_i64; len];

    for (i, &value) in values.iter().enumerate() {
        data[i] = value.rem_euclid(modulus);
    }

    ntt(&mut data, false, modulus, primitive_root);

    for value in &mut data {
        *value = multiply_mod(*value, *value, modulus);
    }

    ntt(&mut data, true, modulus, primitive_root);
    data
}

fn ntt(values: &mut [i64], invert: bool, modulus: i64, primitive_root: i64) {
    let n = values.len();
    let mut j = 0;

    for i in 1..n {
        let mut bit = n >> 1;

        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }

        j ^= bit;

        if i < j {
            values.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let mut root = mod_pow(primitive_root, (modulus - 1) / len as i64, modulus);

        if invert {
            root = mod_pow(root, modulus - 2, modulus);
        }

        for chunk in values.chunks_mut(len) {
            let mut current_root = 1_i64;

            for i in 0..len / 2 {
                let even = chunk[i];
                let odd = multiply_mod(chunk[i + len / 2], current_root, modulus);

                chunk[i] = (even + odd) % modulus;
                chunk[i + len / 2] = (even - odd).rem_euclid(modulus);
                current_root = multiply_mod(current_root, root, modulus);
            }
        }

        len <<= 1;
    }

    if invert {
        let n_inverse = mod_pow(n as i64, modulus - 2, modulus);

        for value in values {
            *value = multiply_mod(*value, n_inverse, modulus);
        }
    }
}

fn mod_pow(mut base: i64, mut exponent: i64, modulus: i64) -> i64 {
    let mut result = 1_i64;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result = multiply_mod(result, base, modulus);
        }

        base = multiply_mod(base, base, modulus);
        exponent >>= 1;
    }

    result
}

fn multiply_mod(left: i64, right: i64, modulus: i64) -> i64 {
    (left as i128 * right as i128 % modulus as i128) as i64
}

fn mod_inverse_i128(value: i128, modulus: i128) -> i128 {
    let (mut old_r, mut r) = (value, modulus);
    let (mut old_s, mut s) = (1_i128, 0_i128);

    while r != 0 {
        let quotient = old_r / r;

        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    positive_mod(old_s, modulus)
}

fn positive_mod(value: i128, modulus: i128) -> i128 {
    value.rem_euclid(modulus)
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn brute_force(nums: &[i32]) -> bool {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[k] as i64;

                    if sum == 0 {
                        return true;
                    }
                }
            }
        }

        false
    }

    #[test]
    fn finds_zero_sum_triple() {
        let nums = vec![-5, 2, 3, 7];

        assert_eq!(Solution::has_three_sum(nums), Some(true));
    }

    #[test]
    fn rejects_input_without_zero_sum_triple() {
        let nums = vec![1, 2, 4, 8];

        assert_eq!(Solution::has_three_sum(nums), Some(false));
    }

    #[test]
    fn requires_three_entries() {
        let nums = vec![0, 0];

        assert_eq!(Solution::has_three_sum(nums), Some(false));
    }

    #[test]
    fn respects_multiplicity() {
        assert_eq!(Solution::has_three_sum(vec![0, 0]), Some(false));
        assert_eq!(Solution::has_three_sum(vec![1, -2]), Some(false));
        assert_eq!(Solution::has_three_sum(vec![1, 1, -2]), Some(true));
        assert_eq!(Solution::has_three_sum(vec![0, 0, 0]), Some(true));
    }

    #[test]
    fn matches_brute_force_on_examples() {
        let examples = vec![
            vec![-1, 0, 1, 2, -1, -4],
            vec![1, 2, -2, -1],
            vec![-10, -4, -1, 1, 2, 5, 8],
            vec![3, 3, -6, 4],
            vec![3, -6, 4],
        ];

        for nums in examples {
            assert_eq!(
                Solution::has_three_sum(nums.clone()),
                Some(brute_force(&nums))
            );
        }
    }

    #[test]
    fn matches_brute_force_exhaustively_for_small_inputs() {
        fn check_all(current: &mut Vec<i32>, remaining: usize) {
            if remaining == 0 {
                assert_eq!(
                    Solution::has_three_sum(current.clone()),
                    Some(brute_force(current))
                );
                return;
            }

            for value in -2..=2 {
                current.push(value);
                check_all(current, remaining - 1);
                current.pop();
            }
        }

        for len in 0..=5 {
            check_all(&mut Vec::new(), len);
        }
    }

    #[test]
    fn declines_large_universe() {
        let nums = vec![i32::MIN, 0, i32::MAX];

        assert_eq!(Solution::has_three_sum_bounded_integer(nums, 16), None);
    }

    #[test]
    fn matches_brute_force_on_generated_inputs() {
        let mut seed = 0x1234_5678_9abc_def0_u64;

        for _ in 0..10_000 {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            let len = (seed % 13) as usize;
            let mut nums = Vec::with_capacity(len);

            for _ in 0..len {
                seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                nums.push((seed % 41) as i32 - 20);
            }

            assert_eq!(Solution::has_three_sum(nums.clone()), Some(brute_force(&nums)));
        }
    }
}
