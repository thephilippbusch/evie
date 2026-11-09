# Benchmarking TypeScript on bun with mitata

Run every command in the project's directory.

## Set up

1. Add the dependency: `bun add -d mitata`.
2. Put benchmarks in `bench/<subject>.bench.ts`. `bun test` only collects `*.test.*`, `*_test.*`, `*.spec.*` and `*_spec.*` files, so benchmarks never run with the tests.
3. Write the benchmark. `BENCH_FORMAT=json` switches the output to JSON for comparing runs:

   ```ts
   import { bench, do_not_optimize, run, summary } from "mitata";

   import { slugify } from "../src/lib/slug";

   summary(() => {
     bench("slugify", () => do_not_optimize(slugify("Summer Party 2026")));
   });

   await run({ format: process.env.BENCH_FORMAT === "json" ? "json" : "mitata" });
   ```

4. Run it: `bun run bench/slugify.bench.ts`.

## Patterns

- `do_not_optimize(value)` keeps the JIT from removing a call whose result is unused. Wrap every result in it.
- Async code: `bench("name", async () => do_not_optimize(await fn()))`.
- Effect code: build the layers once, outside the benchmark, with a `ManagedRuntime`, and run the program inside it: `const runtime = ManagedRuntime.make(AppLayer)`, then `bench("name", () => runtime.runPromise(program))`. For synchronous programs, `runtime.runSync(program)`.
- Input sizes: `bench("Array.from($size)", function* (state) { const size = state.get("size"); yield () => Array.from({ length: size }); }).range("size", 1, 1024)`. Code before `yield` is setup and isn't measured.
- `summary(() => { … })` around several benchmarks prints how they compare, e.g. `2.21x faster than slugify (split/join)`. Use it to compare two implementations in one file; it doesn't replace the before-and-after runs.
- Run a subset: `await run({ filter: /slugify/ })`.

## Run and compare

mitata doesn't save baselines, so save each run's JSON and compare the numbers yourself:

```sh
# On the code without the change, then on the change; alternate, three times each:
BENCH_FORMAT=json bun run bench/slugify.bench.ts > "$TMPDIR/bench-before-1.json"
BENCH_FORMAT=json bun run bench/slugify.bench.ts > "$TMPDIR/bench-after-1.json"
# Time per iteration in nanoseconds, per benchmark:
jq -c '.benchmarks[].runs[] | {file: input_filename, name, avg: .stats.avg, p99: .stats.p99}' "$TMPDIR"/bench-*.json
```

- Report the median of the `avg` values per side, with their range across runs, and the `p99` if tail latency matters.
- The spread between runs of the same code is the noise. A difference smaller than that is no change. On a laptop, identical code can differ by 30 % or more between two runs, so one run proves nothing.
- If the change is already written, run the baseline in a separate worktree at the last commit without the change: `git worktree add "$TMPDIR/evie-baseline" <baseline-commit>`, then `bun install` there. Copy the benchmark in if that commit lacks it. Remove the worktree when done.
- Check that both sides run the same bun version: `bun --version`.
