# Bounded 3-SUM solver

A Rust solver for the **3SUM decision problem**:

> Given integers $a_1, \dots, a_n$, are there three distinct indices $i$, $j$, $k$ with $a_i + a_j + a_k = 0$?

Note the question mark. This is the *decision* variant — it answers yes or no. It doesn't list the triples (that's *reporting*) or count them (that's *counting*).

That distinction buys something. Once you only care whether a triple exists, you can stop the moment you find one. And you never have to build the full set of matches, which can be $\Theta(n^3)$ large.

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

The trick is to stop thinking about the list and start thinking about the values.

Count how many times each value shows up. That gives a frequency vector. Square it with a Number Theoretic Transform (NTT), and you learn, for every sum $s$, how many pairs $(a_i, a_j)$ add up to $s$.

Now a triple exists whenever the value $-s$ is somewhere in the input. One lookup per sum.

There's one catch: the convolution happily pairs an element with itself. A quick inclusion–exclusion pass subtracts those self-pairs and reused indices, so "distinct indices" means what it should — not just distinct values.

## Complexity and scope

What matters here isn't $n$. It's how wide the values spread — the *universe*.

- Let $U = \max(a_i) - \min(a_i) + 1$.
- Runtime is $O(U \log U)$, almost all of it the NTT. The only part that scales with $n$ is the single pass that tallies frequencies.
- So this is *weakly (pseudo-)polynomial*: great when $U$ is small next to $n$, useless as a general 3SUM algorithm. (General 3SUM has no known truly-subquadratic bound anyway.)
- When $U$ gets too big, the solver returns `None` rather than quietly limping along at $O(n^2)$.

## Compute

Both benchmarks race the convolution solver against the textbook $O(n^2)$ sorted two-pointer baseline, on generated mixed-sign inputs. They're for different jobs.

**`cargo bench`** is the one to trust. It runs the [Criterion](https://github.com/bheisler/criterion.rs) suite in `benches/three_sum.rs`, with outlier detection and confidence intervals, from three angles: growing `n` at fixed universe, growing the universe at fixed `n`, and a satisfiable instance where the baseline gets to bail out early.

**`cargo run --release --example benchmark`** is the quick-and-dirty one. No dependencies; it just prints the CSV behind the plot. It warms up first and reports the *median*, so one unlucky slow run doesn't wreck the number.

The plot is local release-build data. Read it as directional, not gospel.

![Runtime comparison](docs/compute.svg)

Raw numbers are in `docs/compute.csv`.

> **Note:** `docs/compute.svg` is hand-drawn — it won't regenerate on its own. After re-running the example, redraw it to match the fresh `docs/compute.csv`.

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
- `benches/three_sum.rs` - Criterion benchmark suite (`cargo bench`)
- `benches/support.rs` - shared input generator and quadratic baseline
- `examples/benchmark.rs` - CSV-emitting harness for the compute plot
- `docs/compute.svg` - runtime comparison plot
