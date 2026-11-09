---
name: review-security
description: Reviews a diff for security problems in evie, such as missing auth checks, unchecked input, injection, XSS, leaked secrets, unsafe GitHub workflows and risky dependencies, and returns JSON findings. Use when reviewing a PR, branch or uncommitted change for security.
---

# Review: security

Find the security problems a change adds. The `review` workflow runs this skill with the other `review-*` skills and merges the results.

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
2. For each hunk, read the code around it in the changed file: the function, its callers in the same file, the imports. Don't read the whole repo.
3. Go through the [checklist](#checklist) for every hunk.
4. Report only problems in the change or caused by it. A problem that existed before and that the change doesn't touch is out of scope.
5. Return the result in the [output](#output) format.

## Checklist

**Auth and access**

- Every SolidStart server function and API route that reads or changes user data gets the BetterAuth session first and fails without one.
- It also checks that the user may act on that event, activity, place or booking. A valid session alone isn't enough.
- Session cookies keep BetterAuth's defaults (`httpOnly`, `secure`, `sameSite`); a change that loosens them is a finding.

**Input**

- Untrusted input is decoded with Effect `Schema` where it enters: form data, URL and query parameters, request bodies, headers, webhook payloads.
- No `as` cast on external data. No `JSON.parse` result used without a `Schema` decode.
- Rust handlers check gRPC request fields before they reach the domain.

**Injection**

- No SQL built by string interpolation or concatenation. Use SeaORM's query builder or bound values (`Statement::from_sql_and_values`), and Drizzle's query builder or its `sql` template tag; never `sql.raw` with input.
- No shell commands, file paths or URLs built from user input without a check.

**XSS**

- No `innerHTML` with user content in Solid components. Render text as children.
- `href` and `src` values from users only allow `https:` or relative URLs, never `javascript:`.

**Secrets**

- No keys, tokens, passwords or `.env` files committed, including in tests, fixtures and docs.
- Secrets are read through Effect `Config` (`Config.redacted`) or the Rust config at startup, never hard-coded.
- Logs and error messages never contain secrets, session tokens or passwords. Internal errors reach the client as a generic message.

**GitHub workflows** (`.github/workflows/`)

- Untrusted input (`github.event.*` titles, bodies, branch names, commit messages) goes through `env` and is used as `"$VAR"`; never `${{ … }}` inside `run`.
- Every `uses:` is pinned to a full 40-character commit SHA with a version comment.
- `permissions` grants the least: `contents: read` at the top, write scopes only on the job that needs them.
- `actions/checkout` sets `persist-credentials: false` unless a later step pushes. `pull_request_target` never checks out the PR's code.

**Dependencies**

- A new dependency is needed: the stack or a few lines of code don't already cover it.
- It is maintained (recent releases, open issues answered) and widely used. Its lockfile entry is part of the change.

**gRPC boundary**

- No gRPC client is imported in code that runs in the browser; only server functions call the backend.
- The backend's gRPC port isn't exposed outside the internal network in Docker Compose, Dockerfiles or deployment files.

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
  "focus": "security",
  "status": "fail",
  "findings": [
    {
      "severity": "high",
      "file": "apps/web/src/bookings/book.ts",
      "line": 12,
      "problem": "The server function books a place without checking the session.",
      "fix": "Get the BetterAuth session first and fail with an unauthorized error without one."
    }
  ]
}
```

| Field      | Value                                                                                        |
| ---------- | -------------------------------------------------------------------------------------------- |
| `focus`    | `security`                                                                                   |
| `status`   | `pass`: no findings. `fail`: one or more findings. `blocked`: no diff to review              |
| `reason`   | Only when `blocked`: why, e.g. the command and its error                                     |
| `findings` | Sorted by severity, critical first. An empty array on `pass` and `blocked`                   |
| `file`     | Repo-root path, or `PR` for the PR title or body                                             |
| `line`     | Line in the new version of the file; `null` when the finding is about a whole file or the PR |
| `problem`  | What is wrong and why it matters, in one or two sentences                                    |
| `fix`      | The change that removes the problem, in one or two sentences                                 |

State assumptions in `problem`, e.g. "Assuming this route is public, …".

## Rules

- Never edit files. Never run a command that changes files, the index, branches or GitHub.
- One finding per problem. Report a repeated problem once, at its first line, and say where else it occurs.
