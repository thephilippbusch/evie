---
name: review-tests
description: Reviews a diff for missing or weak tests in evie, such as untested behaviour, bug fixes without a regression test, tests of internals, skipped tests, module mocks and flaky timing, and returns JSON findings. Use when reviewing a PR, branch or uncommitted change for tests.
---

# Review: tests

Find behaviour a change leaves untested and tests that won't catch a regression. The `review` workflow runs this skill with the other `review-*` skills and merges the results.

This skill reads tests; it doesn't run them. Whether they pass is the `verify` skill's job.

## Input

The caller names the target. Without one, review the branch.

| Target           | Diff                                                                                       |
| ---------------- | ------------------------------------------------------------------------------------------ |
| PR `<n>`         | `gh pr diff <n>`; the title and body with `gh pr view <n> --json title,body,headRefName`   |
| Branch (default) | `git diff origin/main...HEAD`, plus `git diff HEAD` for uncommitted changes                |
| Uncommitted      | `git diff HEAD`; `git status --short` lists untracked files, which `git diff` doesn't show |

For a PR, the working tree may not match the PR. Read a changed file with `git show origin/<headRefName>:<path>`.

## Steps

1. Get the diff for the target. If the command fails or the diff is empty, return `blocked` with the reason.
2. Load the testing conventions for the languages in the diff: the `rust-conventions` skill for Rust, the `typescript-conventions` skill for TypeScript. Skip this if they are already loaded.
3. List the behaviour the change adds or changes. For a PR, the title, body and linked issue say what it should do.
4. For each item, find its tests: in the diff, or beside the source (`<name>.test.ts`, `#[cfg(test)] mod tests`, `tests/<feature>.rs`).
5. Go through the [checklist](#checklist) for the changed code and the changed tests.
6. Report only problems in the change or caused by it. Code the change doesn't touch is out of scope.
7. Return the result in the [output](#output) format.

## Checklist

**Coverage**

- Each new or changed behaviour has a test, including its error paths and edge cases (empty, full, past dates, someone else's booking).
- Changes to docs, config or agent files need no tests; don't report them.

**Bug fixes**

- A fix (`fix:` title, a linked bug issue, or a change that corrects behaviour) has a regression test, as the `debug` skill requires.
- The test shows the cause at the lowest level that can and is named after the behaviour, not the issue number.

**Behaviour, not internals**

- Tests go through a module's public API or exports. No helper exported only for a test.
- Assertions check results and errors, not calls on internals. Rust asserts error variants with `matches!`, not message strings.
- Test names state the behaviour: `book_rejects_a_full_place`, `test("returns the places left")`.

**Skipped tests**

- No `test.skip`, `test.todo`, `describe.skip`, `.only` or `#[ignore]` added, and no test deleted or weakened to make the suite pass.
- An existing test changed together with the code it tests needs a reason in the change.

**Effect and mocks**

- Effect code is tested with a test `Layer` that provides the services, not with `mock.module`.
- No `mock.module` on the project's own code; dependencies are passed in.

**Flaky timing**

- No real sleeps or waits: Rust uses `#[tokio::test(start_paused = true)]`, Effect uses `TestClock`, plain TypeScript takes a clock as a parameter.
- No `Date.now()`, `new Date()` or `SystemTime::now()` in an assertion's path without a fixed clock.
- No dependence on test order, shared mutable state, random values without a seed, or a real network.

## Severity

| Severity | Meaning                                                                                     |
| -------- | ------------------------------------------------------------------------------------------- |
| critical | Exploitable, loses or leaks data, or breaks a non-negotiable rule in the root `AGENTS.md`   |
| high     | A defect users or the next change will hit; fix before merging                              |
| medium   | Makes the code harder to change, check or trust; fix in this change unless there's a reason |
| low      | A small improvement or nit; fix if cheap                                                    |

## Output

Return only this JSON object: no code fence, no text before or after it.

```json
{
  "focus": "tests",
  "status": "fail",
  "findings": [
    {
      "severity": "high",
      "file": "apps/backend/src/booking.rs",
      "line": 57,
      "problem": "`book` now rejects bookings for past activities, but no test covers it.",
      "fix": "Add `book_rejects_a_past_activity` to the tests in `booking.rs`."
    }
  ]
}
```

| Field      | Value                                                                                        |
| ---------- | -------------------------------------------------------------------------------------------- |
| `focus`    | `tests`                                                                                      |
| `status`   | `pass`: no findings. `fail`: one or more findings. `blocked`: no diff to review              |
| `reason`   | Only when `blocked`: why, e.g. the command and its error                                     |
| `findings` | Sorted by severity, critical first. An empty array on `pass` and `blocked`                   |
| `file`     | Repo-root path, or `PR` for the PR title or body                                             |
| `line`     | Line in the new version of the file; `null` when the finding is about a whole file or the PR |
| `problem`  | What is wrong and why it matters, in one or two sentences                                    |
| `fix`      | The change that removes the problem, in one or two sentences                                 |

For missing tests, point `file` and `line` at the untested code. State assumptions in `problem`, e.g. "Assuming this branch is reachable, …".

## Rules

- Never edit files. Never run a command that changes files, the index, branches or GitHub.
- Never run the tests; read them.
- One finding per problem. Report a repeated problem once, at its first line, and say where else it occurs.
