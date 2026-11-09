---
name: review-code-quality
description: Reviews a diff for code quality in evie, against the Rust and TypeScript conventions and for what linters miss, such as duplication, dead code, naming, error handling and needless complexity, and returns JSON findings. Use when reviewing a PR, branch or uncommitted change for code quality.
---

# Review: code quality

Find what makes a change harder to read, change or trust than it needs to be. The `review` workflow runs this skill with the other `review-*` skills and merges the results.

Formatting, lint and compiler errors belong to the `verify` skill; don't report what rustfmt, clippy, the TypeScript compiler, the linter or dprint already catch.

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
2. Load the conventions for the languages in the diff: the `rust-conventions` skill for `.rs` files, the `typescript-conventions` skill for `.ts` and `.tsx` files. Skip this if they are already loaded.
3. For each hunk, read the code around it in the changed file: the function, its callers in the same file, the imports. Search the repo only to check for duplication.
4. Go through the [checklist](#checklist) for every hunk.
5. Report only problems in the change or caused by it. A problem that existed before and that the change doesn't touch is out of scope.
6. Return the result in the [output](#output) format.

## Checklist

**Conventions**

- Rust code follows `rust-conventions`: one `thiserror` enum per domain module, no `unwrap` outside tests and startup, `name.rs` instead of `mod.rs`, no blocking calls in async code, timeouts on outbound calls.
- TypeScript code follows `typescript-conventions`: code grouped by feature, named exports, no barrel files, kebab-case files, string literal unions instead of `enum`, exhaustive `switch`, no `as` on external data.
- Cite the convention in `problem`, e.g. "`rust-conventions`, Errors: …".

**Duplication**

- No copy of logic that already exists in the repo; reuse or extract it.
- No near-identical blocks within the change that differ in one value.

**Dead code**

- No unused functions, exports, parameters, imports or feature flags.
- No commented-out code, no leftover `dbg!`, `console.log`, `.only` or debug output.
- No `TODO` without an issue number.

**Naming**

- Names say what a thing is or does, in the domain's words: event, activity, place, booking, host, attendee.
- One name per concept across the change; no `data`, `info`, `helper`, `manager` or `utils` without a reason.

**Error handling**

- Errors are handled or passed on, never swallowed: no empty `catch`, no `let _ =` on a `Result`, no `Effect.orDie` or `Effect.catchAll` that hides a failure.
- Each error carries what the caller needs to act on it, and is logged once, at the edge.

**Complexity**

- No abstraction with one user and no second one in sight: generic types, traits, config options or layers that the change doesn't need.
- Functions do one thing. Deep nesting, long parameter lists and boolean flag parameters are a sign to split.
- Comments say why, not what, as the `writing-style` skill says for code comments.

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
  "focus": "code-quality",
  "status": "fail",
  "findings": [
    {
      "severity": "medium",
      "file": "apps/backend/src/booking.rs",
      "line": 42,
      "problem": "`rust-conventions`, Errors: `book` returns `Result<_, String>`, so callers can't match on the failure.",
      "fix": "Return a `BookingError` variant, e.g. `BookingError::PlaceFull`."
    }
  ]
}
```

| Field      | Value                                                                                        |
| ---------- | -------------------------------------------------------------------------------------------- |
| `focus`    | `code-quality`                                                                               |
| `status`   | `pass`: no findings. `fail`: one or more findings. `blocked`: no diff to review              |
| `reason`   | Only when `blocked`: why, e.g. the command and its error                                     |
| `findings` | Sorted by severity, critical first. An empty array on `pass` and `blocked`                   |
| `file`     | Repo-root path, or `PR` for the PR title or body                                             |
| `line`     | Line in the new version of the file; `null` when the finding is about a whole file or the PR |
| `problem`  | What is wrong and why it matters, in one or two sentences                                    |
| `fix`      | The change that removes the problem, in one or two sentences                                 |

State assumptions in `problem`, e.g. "Assuming nothing outside this crate calls it, …".

## Rules

- Never edit files. Never run a command that changes files, the index, branches or GitHub.
- One finding per problem. Report a repeated problem once, at its first line, and say where else it occurs.
- Report style preferences that no convention or checklist item covers as `low`, or not at all.
