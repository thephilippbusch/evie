---
name: debug
description: Finds and fixes the cause of a bug by reproducing it, isolating it and fixing it, and leaves a regression test that guards it. Use when something is broken, failing, flaky or behaves unexpectedly, or when asked to debug, investigate or fix a bug.
---

# Debug

Fix the cause, not the symptom. No fix without a reproduction, and no fix without a regression test that failed before it.

| Read                                                 | When the bug is in                                             |
| ---------------------------------------------------- | -------------------------------------------------------------- |
| [references/rust.md](references/rust.md)             | the Rust backend or a Rust crate: logs, backtraces, async, SQL |
| [references/typescript.md](references/typescript.md) | the SolidStart frontend or BFF, Effect code, bun               |
| [references/git-bisect.md](references/git-bisect.md) | something that used to work: find the commit that broke it     |

## Steps

1. **Reproduce.** Write down the expected and the actual behaviour. Find the moon project that owns the code (use the `moon-query` skill). Turn the bug into the smallest failing test in that project and run it with `moon run <project>:test` (see the `moon-run` skill). If a test isn't possible yet (environment, UI, timing), write a minimal reproduction instead: a command, a script in `$TMPDIR` or numbered steps, and watch it fail. For a flaky bug, run it repeatedly and note how often it fails.
2. **Isolate.** Narrow it down until you can name the line and say why it fails. Change one thing at a time: one hypothesis, one experiment, one observation. Run only the failing test, shrink the input, log at the boundaries (BFF ↔ backend, backend ↔ database) to find the side at fault, and bisect if it is a regression. The language references list the tools.
3. **Write the regression test.** If step 1 produced a test, it is the regression test; otherwise write one now, at the lowest level that shows the cause. Name it after the behaviour, not the issue number. Run it: it must fail, and fail for the bug's reason (a wrong value or error, not a compile error or broken setup).
4. **Fix.** Make the smallest change that removes the cause. Remove temporary logs, `dbg!`, `console.log` and `.only`.
5. **Confirm.** The regression test passes, the project's whole `test` task passes, and the original reproduction no longer fails. Check the affected projects with the `verify` skill.
6. **Commit.** Commit the test and the fix together with the `commit` skill, so no commit has a failing test. Put the output below in the PR description with the `pull-request` skill.

## Output

```markdown
**Cause:** what was wrong and why, in one or two sentences.
**Fix:** what changed, with file paths.
**Regression test:** `<project>`: `<test file>` › `<test name>`. Failed before the fix, passes now.
**Reproduction:** the command or steps from step 1.
```

If no automated test can catch the bug (for example a config or infrastructure fault), say why under **Regression test** and give the reproduction steps instead. Never skip it silently.

## Rules

- Never change an existing test to make it pass, unless the test itself is wrong; then say so in the output.
- Never hide the cause: no catch-all error handling, retries, sleeps or longer timeouts in place of a fix.
- Keep unrelated refactors out of the fix.
