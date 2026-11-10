# Planning a performance change

The issue is about speed, memory or size. A speed-up exists only if numbers show it, so measure before you change anything.

## Find out first

1. Name the operation, the metric and realistic inputs, with the `benchmark` skill's step 1.
2. Measure the baseline with the `benchmark` skill, on the code before any change.
3. Set a target number with the issue, e.g. "median under 5 ms for 500 bookings". Without a target, the plan has no end.
4. Profile or log to find where the time or memory goes. Don't guess.

## Plan sections

Add these under Approach:

- **Baseline:** the numbers from the `benchmark` skill, with the command.
- **Target:** the number the change must reach.
- **Hypothesis:** what costs the time or memory, why, and the evidence from profiling.
- **Change:** what makes it cheaper. Behaviour stays the same: same outputs, errors and side effects.
- **Behaviour guard:** the existing tests that cover the code, and the ones to add first if none do.

Under Order:

1. Tests that pin the current behaviour, if missing.
2. The benchmark, on its own, before the change.
3. The change, in a `perf:` commit.

## Verify

- The behaviour tests pass before and after the change.
- Measure after the change with the `benchmark` skill, against the same baseline. The result goes in the PR description.
- If the difference isn't larger than the variance, report "no measurable change" and drop or rethink the change.
