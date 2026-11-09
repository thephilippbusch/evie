---
name: review-project-rules
description: Reviews a diff against evie's project rules, such as German and English texts, accessibility, writing style, the PR title, release files, migrations and matching AGENTS.md updates, and returns JSON findings. Use when reviewing a PR, branch or uncommitted change for project rules.
---

# Review: project rules

Find where a change breaks the rules in the root `AGENTS.md` that no other review focus covers. The `review` workflow runs this skill with the other `review-*` skills and merges the results.

## Input

The caller names the target. Without one, review the branch.

| Target           | Diff                                                                                       |
| ---------------- | ------------------------------------------------------------------------------------------ |
| PR `<n>`         | `gh pr diff <n>`                                                                           |
| Branch (default) | `git diff origin/main...HEAD`, plus `git diff HEAD` for uncommitted changes                |
| Uncommitted      | `git diff HEAD`; `git status --short` lists untracked files, which `git diff` doesn't show |

Get the PR's title, body and linked issues with `gh pr view <n> --json title,body,headRefName,closingIssuesReferences`. For the branch, `gh pr view` without a number finds its PR, if there is one. Without a PR, skip the PR title and body checks.

For a PR, the working tree may not match the PR. Read a changed file with `git show origin/<headRefName>:<path>`.

## Steps

1. Get the diff for the target. If the command fails or the diff is empty, return `blocked` with the reason.
2. Load the `writing-style` skill if it isn't loaded yet.
3. Find the version: `.release-please-manifest.json` holds the current one, the milestone of the linked issue (`gh issue view <i> --json milestone`) the next one.
4. For each hunk, read the code or text around it in the changed file. Don't read the whole repo.
5. Go through the [checklist](#checklist) for every hunk, and for the PR title and body.
6. Report only problems in the change or caused by it. A problem that existed before and that the change doesn't touch is out of scope.
7. Return the result in the [output](#output) format.

## Checklist

**German and English**

- Every new or changed user-facing text exists in German and English: UI labels, messages, errors shown to users, emails, page titles.
- No user-facing text is hard-coded in a component; both languages have the same keys.

**Accessibility**

- Every form control has a label; icon-only buttons have an accessible name.
- Images have `alt` text, or `alt=""` when they are decoration.
- Everything clickable works with the keyboard: a `<button>` or `<a>`, not a `<div>` with `onClick`. Focus stays visible.
- Semantic HTML first (`<nav>`, `<main>`, headings in order, `<ul>` for lists); ARIA only where HTML has no element for it.

**Writing style**

- Docs, code comments, commit bodies and the PR body follow the `writing-style` skill: outcome first, short sentences, plain words, no filler, no history in docs.
- The PR body follows `.github/pull_request_template.md` and links its issue.

**PR title**

- The title is a Conventional Commit, as the `pull-request` skill describes.
- Before v1, the title uses a type that doesn't release: `chore`, `ci`, `docs`, `ai`, `build`, `refactor`, `style` or `test`.
- Only the PR that closes the last open issue of a milestone is a `feat`. Check with `gh issue list --milestone "<title>" --state open`.

**Release files**

- `CHANGELOG.md` and `.release-please-manifest.json` are untouched; release-please owns them. Any change to them is critical.

**Migrations**

- A schema change comes with a generated migration in the same change: SeaORM for the domain, Drizzle for the `auth` schema.
- The migration matches the schema change and loses no data, or says why it may.
- `drizzle-kit push` appears only in local dev setup, never in a moon task, script, Dockerfile or workflow that runs elsewhere.

**Docs follow workflows**

- A change that adds or changes a workflow updates the matching `AGENTS.md` and skills: a new moon task in the project's Commands section, a new skill or subagent in the root index, a new rule in the file it applies to.

**Acting for others** (from v0.5 on)

- Participation features, such as booking a place or joining an activity, let a user act for household members and managed profiles.
- They record who acted, beside whom the action is for.

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
  "focus": "project-rules",
  "status": "fail",
  "findings": [
    {
      "severity": "critical",
      "file": "apps/web/src/bookings/booking-form.tsx",
      "line": 23,
      "problem": "The button label \"Book place\" exists only in English.",
      "fix": "Add the German and English texts for the label and use them in the button."
    }
  ]
}
```

| Field      | Value                                                                                        |
| ---------- | -------------------------------------------------------------------------------------------- |
| `focus`    | `project-rules`                                                                              |
| `status`   | `pass`: no findings. `fail`: one or more findings. `blocked`: no diff to review              |
| `reason`   | Only when `blocked`: why, e.g. the command and its error                                     |
| `findings` | Sorted by severity, critical first. An empty array on `pass` and `blocked`                   |
| `file`     | Repo-root path, or `PR` for the PR title or body                                             |
| `line`     | Line in the new version of the file; `null` when the finding is about a whole file or the PR |
| `problem`  | What is wrong and why it matters, in one or two sentences                                    |
| `fix`      | The change that removes the problem, in one or two sentences                                 |

State assumptions in `problem`, e.g. "Assuming this text is shown to users, …".

## Rules

- Never edit files. Never run a command that changes files, the index, branches or GitHub.
- One finding per problem. Report a repeated problem once, at its first line, and say where else it occurs.
