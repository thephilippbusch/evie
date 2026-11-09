---
name: review
description: Reviews a PR, branch or uncommitted change for security, architecture, code quality, tests and project rules, and merges the findings into one ranked list. Use when asked to review a change, a branch or a PR, or before handing a PR over for merging.
---

# Review

A workflow: it runs the five `review-*` skills on one target and merges their JSON results into one ranked list. The focus skills hold the checklists; this skill adds none.

| Focus           | Skill                  | Claude Code subagent     |
| --------------- | ---------------------- | ------------------------ |
| `security`      | `review-security`      | `reviewer-security`      |
| `architecture`  | `review-architecture`  | `reviewer-architecture`  |
| `code-quality`  | `review-code-quality`  | `reviewer-code-quality`  |
| `tests`         | `review-tests`         | `reviewer-tests`         |
| `project-rules` | `review-project-rules` | `reviewer-project-rules` |

## Steps

1. **Pick the target.** Run `git fetch origin` first, so `origin/main` and PR branches are current.
   - A PR number, when the caller gives one: `PR #<n>`.
   - The branch, by default: every change on it compared with `origin/main`, uncommitted changes included.
   - Uncommitted changes only, when the caller asks for them.
2. **Run the five reviews.**
   - In Claude Code: start the five `reviewer-*` subagents in parallel, in one message, and pass each the target, e.g. "Review PR #12." or "Review the branch."
   - In other agents, such as OpenCode and Codex: use the five `review-*` skills one after another on the same target, and keep each JSON result.
3. **Merge the results.**
   - Parse the five JSON objects. A result that isn't valid JSON counts as `blocked`, with the reason "returned no valid JSON".
   - Merge duplicates: findings with the same file, line and problem become one. Keep the higher severity and list both focuses.
   - Rank by severity (critical, high, medium, low), then by focus in the order of the table above.
4. **Report** in the [output](#output) format. Then stop: the caller addresses the findings.

## Output

The same fixed Markdown, whether the review runs in a session or headless with `claude -p`. No text before or after it.

1. The status line: `**review:** <status> (<target>): N critical, N high, N medium, N low`.
   - `<status>` is `fail` if there is any finding, `blocked` if a focus blocked and there are no findings, and `pass` otherwise.
   - `<target>` is `PR #<n>`, `branch <name>` or `uncommitted`.
2. One numbered item per finding, in rank order: ``1. **<severity>** · <focus>[, <focus>] · `<file>:<line>`: <problem> Fix: <fix>``. Leave out `:<line>` when the line is `null`; write `PR` without backticks for the PR title or body.
3. For each focus that returned `blocked`: `**Blocked:** <focus>: <reason>`.

A pass is the status line alone.

```markdown
**review:** fail (PR #12): 1 critical, 1 high, 1 medium, 0 low

1. **critical** · project-rules · `apps/web/src/bookings/booking-form.tsx:23`: The button label "Book place" exists only in English. Fix: Add the German and English texts and use them in the button.
2. **high** · security, architecture · `apps/web/src/bookings/book.ts:12`: The server function books a place without checking the session. Fix: Get the BetterAuth session first and fail without one.
3. **medium** · code-quality · `apps/backend/src/booking.rs:42`: `book` returns `Result<_, String>`, so callers can't match on the failure. Fix: Return a `BookingError` variant.

**Blocked:** tests: `gh pr diff 12` failed: no pull request found.
```

## Rules

- Never edit files, and never fix a finding. The caller addresses the findings, as step 7 of the Workflow table in the root `AGENTS.md` says.
- Never drop or soften a finding while merging. Only exact duplicates merge.
- Run all five focuses on the same target, even if one blocks.
