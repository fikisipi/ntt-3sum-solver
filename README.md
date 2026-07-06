# Bounded 3-SUM solver

This repo contains a Rust implementation of the **3SUM decision problem**:

> Given a sequence of integers $a_1, \dots, a_n$, decide whether there exist distinct indices $i$, $j$, $k$ with $a_i + a_j + a_k = 0$.

This is the decision (yes/no) variant: the solver returns a single boolean rather than enumerating witnesses (the *reporting* variant) or counting them (the *counting* variant). Because it only decides existence, it can short-circuit and avoids materializing the potentially $\Theta(n^3)$ set of matching triples.

## What Is Implemented

`Solution::has_three_sum(nums)` solves bounded-integer 3SUM with exact convolution:

```rust
let result = Solution::has_three_sum(vec![-5, 2, 3, 7]);
assert_eq!(result, Some(true));
```

Return values:

- `Some(true)` — a zero-sum triple exists (the instance is a yes-instance).
- `Some(false)` — no zero-sum triple exists (no-instance).
- `None` — the integer universe is too large for this bounded-integer method; the instance is undecided by this solver.

The implementation reduces the decision problem to a convolution over the value domain. It builds a frequency (indicator/multiplicity) vector over the input value range, squares it via Number Theoretic Transform (NTT) convolution to obtain, for every attainable pair sum $s$, the number of ordered index pairs with $a_i + a_j = s$, then checks whether some value $a_k = -s$ is present. An inclusion–exclusion correction removes pairs and triples that reuse an index, so the answer respects the *distinct indices* requirement rather than merely distinct values.

## Complexity and scope

This implementation is limited to bounded integers, and its complexity is parameterized by the size of the value domain (the *universe*) rather than by $n$ alone:

- Let $U = \max(a_i) - \min(a_i) + 1$ be the size of the integer universe.
- Runtime is $O(U \log U)$, dominated by the NTT convolution; this is independent of $n$ beyond the $O(n)$ pass that builds the frequency vector.
- It is therefore *weakly polynomial* (pseudo-polynomial in the input values): fast when $U$ is bounded relative to $n$, but not a subquadratic algorithm for the general 3SUM problem, whose best known bounds remain mildly subquadratic.
- It intentionally returns `None` for very large universes instead of silently falling back to the standard $O(n^2)$ approach.

## Compute

The benchmark compares this convolution solver against the standard $O(n^2)$ sorted two-pointer baseline on generated mixed-sign inputs. These are five-run local release-build averages, so treat them as directional rather than absolute.

![Runtime comparison](docs/compute.svg)

Raw numbers are in `docs/compute.csv`.

## Usage

Run the tests:

```sh
cargo test
```

Run the benchmark:

```sh
cargo run --release --example benchmark
```

Use as a library:

```rust
use three_sum::Solution;

fn main() {
    let nums = vec![-1, 0, 1, 2];
    println!("{:?}", Solution::has_three_sum(nums));
}
```

## Files

- `src/lib.rs` - library implementation and tests
- `examples/benchmark.rs` - benchmark used for the compute plot
- `docs/compute.svg` - runtime comparison plot
