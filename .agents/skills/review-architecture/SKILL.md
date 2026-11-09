---
name: review-architecture
description: Reviews a diff against evie's architecture and boundaries, such as the browser-to-gRPC rule, schema ownership, apps and libraries, idiomatic Effect and generated code, and returns JSON findings. Use when reviewing a PR, branch or uncommitted change for architecture or boundaries.
---

# Review: architecture

Find the places where a change crosses evie's boundaries or breaks its structure. The rules come from the root `AGENTS.md` and the touched projects' `AGENTS.md` files. The `review` workflow runs this skill with the other `review-*` skills and merges the results.

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
2. Read the `AGENTS.md` of every moon project the diff touches: the file in the project's root directory, such as `apps/<name>/AGENTS.md`.
3. For each hunk, read the code around it in the changed file: the function, the imports, the module it sits in. Don't read the whole repo.
4. Go through the [checklist](#checklist) for every hunk.
5. Report only problems in the change or caused by it. A problem that existed before and that the change doesn't touch is out of scope.
6. Return the result in the [output](#output) format.

## Checklist

**Browser and backend**

- The browser never talks gRPC. gRPC clients live in server-only code; client components call SolidStart server functions, and those call the backend.
- The BFF reaches domain data only through the backend, never through the domain database.

**Schemas**

- SeaORM owns the domain schema: domain entities and migrations live in the backend.
- Drizzle owns the `auth` schema and nothing else.
- No foreign keys between the two schemas. The domain stores the BetterAuth user ID as an opaque value, with no join to `auth` tables.

**Apps and libraries**

- Deployable code lives in `apps/`, shared code in `libs/`. A library never depends on an app.
- `moon.yml` sets `layer: 'application'` for apps and `layer: 'library'` for libraries.
- `dependsOn` lists every library the project uses, so moon sees the change as affecting it.

**Effect** (frontend server and shared code)

- Services are a `Context.Tag` with a `Layer`; code gets them from the context, not by importing an instance.
- Configuration comes from `Config`, never `process.env`.
- Data at a boundary has a `Schema`; errors are `Data.TaggedError` classes, not `throw` or plain `Error`.
- Effects run at the edge, in the server function or route; no `Effect.runPromise` inside services.

**Generated code**

- No hand edits to protobuf output or the BetterAuth Drizzle schema. A change there comes with its source change (the `.proto` file or the BetterAuth config) and the `codegen` output.
- The generated paths are listed in `.claude/hooks/protected-paths.txt`.

**New apps**

- A new directory in `apps/` follows the `moon-new-app` checklist: `Dockerfile`, Docker Compose service, `release-please-config.json` entry, image publishing, and the `apps/` row of the repo map in the root `AGENTS.md`.
- A new project has an `AGENTS.md` and a `CLAUDE.md` symlink beside it.

**Project rules**

- The change follows the local rules in the touched projects' `AGENTS.md` files. Where a project file differs from the root, the project file wins.

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
  "focus": "architecture",
  "status": "fail",
  "findings": [
    {
      "severity": "critical",
      "file": "apps/web/src/events/event-list.tsx",
      "line": 8,
      "problem": "A client component imports the gRPC client, so the browser would talk gRPC.",
      "fix": "Move the call into a server function and call that from the component."
    }
  ]
}
```

| Field      | Value                                                                                        |
| ---------- | -------------------------------------------------------------------------------------------- |
| `focus`    | `architecture`                                                                               |
| `status`   | `pass`: no findings. `fail`: one or more findings. `blocked`: no diff to review              |
| `reason`   | Only when `blocked`: why, e.g. the command and its error                                     |
| `findings` | Sorted by severity, critical first. An empty array on `pass` and `blocked`                   |
| `file`     | Repo-root path, or `PR` for the PR title or body                                             |
| `line`     | Line in the new version of the file; `null` when the finding is about a whole file or the PR |
| `problem`  | What is wrong and why it matters, in one or two sentences                                    |
| `fix`      | The change that removes the problem, in one or two sentences                                 |

State assumptions in `problem`, e.g. "Assuming this module runs in the browser, …".

## Rules

- Never edit files. Never run a command that changes files, the index, branches or GitHub.
- One finding per problem. Report a repeated problem once, at its first line, and say where else it occurs.
