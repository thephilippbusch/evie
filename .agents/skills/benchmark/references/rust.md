# Benchmarking Rust with criterion

Run every command in the crate's directory.

## Set up

1. Add the dependency: `cargo add --dev criterion`. For async code, add the `async_tokio` feature: `cargo add --dev criterion --features async_tokio`.
2. Register each benchmark in `Cargo.toml`. `harness = false` hands `main` to criterion:

   ```toml
   [[bench]]
   name = "slugify"
   harness = false
   ```

3. Write it in `benches/<name>.rs`:

   ```rust
   use std::hint::black_box;

   use criterion::{criterion_group, criterion_main, Criterion};
   use my_crate::slugify;

   fn bench_slugify(c: &mut Criterion) {
       c.bench_function("slugify", |b| {
           b.iter(|| slugify(black_box("Summer Party 2026")))
       });
   }

   criterion_group!(benches, bench_slugify);
   criterion_main!(benches);
   ```

Wrap inputs in `std::hint::black_box`, so the compiler can't compute the result at build time.

## Patterns

```rust
// Several input sizes, reported as slugify_len/1, slugify_len/10, …
let mut group = c.benchmark_group("slugify_len");
for words in [1, 10, 100] {
    let input = vec!["Party"; words].join(" ");
    group.bench_with_input(BenchmarkId::from_parameter(words), &input, |b, input| {
        b.iter(|| slugify(black_box(input)))
    });
}
group.finish();

// Setup that mustn't be measured: a fresh input for every batch.
c.bench_function("slugify_owned", |b| {
    b.iter_batched(
        || "Summer Party 2026".repeat(10),
        |input| slugify(&input),
        BatchSize::SmallInput,
    )
});

// An async function on Tokio (needs the async_tokio feature).
let runtime = tokio::runtime::Runtime::new().unwrap();
c.bench_function("slugify_async", |b| {
    b.to_async(&runtime)
        .iter(|| slugify_async(black_box("Summer Party 2026")))
});
```

Import `BatchSize` and `BenchmarkId` from `criterion` too. Build connections, runtimes and test data outside `b.iter`. Benchmark pure logic in-process; a benchmark against PostgreSQL or a gRPC server mostly measures the network and the database, and varies more.

## Run and compare

Always pass `--bench <name>`. Without it, cargo also runs the library's test harness, which rejects criterion's flags (`error: Unrecognized option`).

```sh
# Baseline, on the code without the change:
cargo bench --bench slugify -- --save-baseline before
# The change, compared against it; repeat, the baseline isn't overwritten:
cargo bench --bench slugify -- --baseline before
```

- Run only matching benchmarks: `cargo bench --bench slugify -- slugify_len`.
- Check the noise first: run `--baseline before` on the unchanged code. It should report "No change in performance detected"; if it doesn't, the machine is too noisy to measure.
- Criterion takes 100 samples per run. Its output per benchmark:

  ```text
  slugify   time:   [95.539 ns 95.661 ns 95.800 ns]
            change: [−0.1808% −0.0696% +0.0436%] (p = 0.24 > 0.05)
            No change in performance detected.
  ```

  `time` is the lower bound, estimate and upper bound of the time per iteration; `change` is the same for the difference to the baseline. Copy both lines into the report.

- Results and an HTML report go to `target/criterion/`. Don't commit them.

## Baseline in another worktree

If the change is already written, measure the baseline in a separate worktree, and share criterion's output directory through `CRITERION_HOME` so both sides see the same baselines. The baseline commit is the last one without the change: the commit that adds the benchmark, or `git merge-base origin/main HEAD`. If it lacks the benchmark, copy in the bench file, its `[[bench]]` entry and the `criterion` dev-dependency.

```sh
git worktree add "$TMPDIR/evie-baseline" <baseline-commit>
# In $TMPDIR/evie-baseline/<crate-dir>:
CRITERION_HOME="$TMPDIR/criterion" cargo bench --bench slugify -- --save-baseline before
# In <crate-dir> of your worktree:
CRITERION_HOME="$TMPDIR/criterion" cargo bench --bench slugify -- --baseline before
# When done:
git worktree remove --force "$TMPDIR/evie-baseline"
```

To alternate runs, save each baseline run under its own name (`before-1`, `before-2`, …) and compare the change against each.
