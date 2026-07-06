# NTT 3SUM in [-U, U]

A Rust solver for the **3SUM decision problem**:

> Given integers $a_1, \dots, a_n$, are there three distinct indices $i$, $j$, $k$ with $a_i + a_j + a_k = 0$

## Implementation

`Solution::has_three_sum(nums)` decides bounded-integer 3SUM by exact convolution:

```rust
let result = Solution::has_three_sum(vec![-5, 2, 3, 7]);
assert_eq!(result, Some(true));
```

Three outcomes:

- `Some(true)` — a zero-sum triple exists.
- `Some(false)` — none does.
- `None` — the values are spread too far apart for this method, so the solver declines instead of guessing.

Count how many times each value shows up. That gives a frequency vector. Square it with a Number Theoretic Transform (NTT), and you learn, for every sum $s$, how many pairs $(a_i, a_j)$ add up to $s$.

Uses inclusion–exclusion self-pairs and reused indices.

- Let $U = \max(a_i) - \min(a_i) + 1$.
- Runtime is $O(U \log U)$, almost all of it the NTT. The only part that scales with $n$ is the single pass that tallies frequencies.
- So this is *weakly (pseudo-)polynomial*: great when $U$ is small next to $n$, useless as a general 3SUM algorithm. (General 3SUM has no known truly-subquadratic bound anyway.)
- When $U$ gets too big, the solver returns `None` rather than quietly limping along at $O(n^2)$.

## Benchmark

Both benchmarks race the convolution solver against the textbook $O(n^2)$ sorted two-pointer baseline, on generated mixed-sign inputs. They're for different jobs.

**`cargo bench`** runs the [Criterion](https://github.com/bheisler/criterion.rs) suite in `benches/three_sum.rs`, with outlier detection and confidence intervals, from three angles: growing `n` at fixed universe, growing the universe at fixed `n`, and a satisfiable instance where the baseline gets to bail out early.

**`cargo run --release --example benchmark`** prints the CSV behind the plot. It warms up first and reports the *median*, so one unlucky slow run doesn't wreck the number.

![Runtime comparison](docs/compute.svg)

Raw numbers are in `docs/compute.csv`.

## Usage

Run the tests:

```sh
cargo test
```

Run the benchmarks:

```sh
cargo bench                                 # rigorous Criterion suite
cargo run --release --example benchmark     # CSV table for the plot
```
