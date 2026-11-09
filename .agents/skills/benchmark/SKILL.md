---
name: benchmark
description: Measures a change against a baseline on the same machine and reports the before and after numbers with their variance. Use when claiming or checking that a change is faster, slower or leaner, or when asked to benchmark, measure or compare performance.
---

# Benchmark

A speed-up exists only if before-and-after numbers show it. This skill measures; deciding what to optimise and doing it is not part of it.

## Tools

| Language       | Tool        | Why                                                                                                                                      |
| -------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Rust           | `criterion` | Saves named baselines and reports each change with a confidence interval and p-value; `divan` is lighter but can't compare baselines.    |
| TypeScript/bun | `mitata`    | What bun uses for its own benchmarks; guards against dead-code elimination and prints JSON to compare runs. `tinybench` would also work. |

| Read                                                 | When you benchmark                                    |
| ---------------------------------------------------- | ----------------------------------------------------- |
| [references/rust.md](references/rust.md)             | Rust code: setup, `cargo bench`, saved baselines      |
| [references/typescript.md](references/typescript.md) | TypeScript on bun: setup, mitata, comparing JSON runs |

There is no `bench` moon task yet, so run the commands in the project directory. If the project has one, use `moon run <project>:bench`.

## Steps

1. **Define the claim.** Name the operation, the metric (time per operation, throughput, allocations) and realistic inputs, including sizes.
2. **Write the benchmark** in the project that owns the code, or reuse an existing one. Measure the smallest unit that contains the change. Commit it on its own with the `commit` skill, before the change, so both sides run the same benchmark.
3. **Measure the baseline** on the code without the change. If the change is already written, check out the baseline in a separate worktree; the references show how.
4. **Measure the change** on the same machine, with the same build profile, toolchain version and inputs. Keep the machine quiet: no builds, tests or other benchmarks running alongside.
5. **Repeat.** Run each side at least three times, alternating baseline and change, so drift (heat, background load) shows up as variance instead of as a difference.
6. **Judge.** The difference is real only if it is larger than the variance between runs of the same code. Otherwise the result is "no measurable change". Report slowdowns too.
7. **Report** the result in the PR description with the `pull-request` skill.

## Output

```markdown
### Benchmark

- Machine: <CPU>, <OS>; toolchain: <rustc or bun version>; profile: <bench or release>
- Command: `<exact command>`
- Runs: <n> per side, alternating

| Benchmark | Before                 | After                  | Change                         |
| --------- | ---------------------- | ---------------------- | ------------------------------ |
| <name>    | <median> (<min>–<max>) | <median> (<min>–<max>) | <−x %> [<low> <high>], p = <p> |
```

Before and After are in the metric from step 1, usually time per operation: the median of the runs, with the range across runs. Change is the tool's change interval and p-value (criterion), or the difference of the medians (mitata).

## Rules

- Never claim a speed-up, or the absence of a slowdown, without before and after numbers from this procedure.
- Never compare numbers across machines, build profiles, toolchain versions or with CI.
- Never time debug builds or single runs with `time` or `Date.now()`.
- Commit benchmark code; never commit results (`target/criterion/`, JSON output).
