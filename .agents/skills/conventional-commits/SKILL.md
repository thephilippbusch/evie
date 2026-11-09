---
name: conventional-commits
description: Sets evie's Conventional Commit format for commit messages and PR titles, the allowed types and which of them release a version. Use when writing a commit message or PR title, picking a commit type or scope, or asking whether a change releases.
---

# Conventional Commits

Every commit message and PR title is a [Conventional Commit](https://www.conventionalcommits.org/en/v1.0.0/). `scripts/lint-commit-msg.sh` checks commit messages in the `commit-msg` hook and PR titles in CI. PRs are squash-merged, so the PR title becomes the commit on `main` that release-please reads.

Before v1, the PR title follows the `pull-request` skill.

## Format

```text
<type>[(scope)][!]: <description>

[body]

[footers]
```

- **Header:** at most 100 characters, followed by a blank line if anything comes after it.
- **Type:** one of the 12 types below.
- **Scope:** optional. A moon project ID, e.g. `root`; `moon query projects` lists them. Leave it out when the change spans several projects or none.
- **Description:** imperative, lowercase, no trailing period: `add the commit skill`, not `Added the commit skill.`
- **Body:** what changed and why, following the `writing-style` skill. Leave it out when the header says it all.

## Types

The release effect follows `release-please-config.json`.

| Type       | Use for                                                               | Release |
| ---------- | --------------------------------------------------------------------- | ------- |
| `feat`     | new or changed behaviour for people using evie                        | minor   |
| `fix`      | behaviour that doesn't work as documented or promised                 | patch   |
| `perf`     | the same behaviour, faster or leaner                                  | patch   |
| `revert`   | undoing an earlier commit                                             | patch   |
| `refactor` | code restructured without a change in behaviour                       | none    |
| `test`     | tests only                                                            | none    |
| `docs`     | docs for people, such as `docs/` and READMEs                          | none    |
| `ai`       | agent configuration: skills, subagents, `AGENTS.md` files             | none    |
| `ci`       | GitHub workflows, repository settings and release configuration       | none    |
| `build`    | build system, Dockerfiles, dependencies and the pinned toolchain      | none    |
| `style`    | formatting only, no change in meaning                                 | none    |
| `chore`    | other tooling and maintenance, such as scripts and moon configuration | none    |

- Types with no release are also hidden from `CHANGELOG.md`.
- A breaking change releases a major version: add `!` after the type or scope, or a `BREAKING CHANGE:` footer. Before v1 it releases a minor version, because the config sets `bump-minor-pre-major`.

## Pick a type

Take the first that fits:

1. It undoes an earlier commit: `revert`.
2. It changes what people using evie see or do: `feat` if it adds or changes behaviour, `fix` if it repairs it.
3. It keeps the behaviour and makes it faster or leaner: `perf`.
4. It touches only one kind of file: `test`, `docs`, `ai`, `ci`, `build` or `style`.
5. It restructures code without a change in behaviour: `refactor`.
6. Anything else: `chore`.

| Change                                               | Header                                           |
| ---------------------------------------------------- | ------------------------------------------------ |
| Hosts can offer a sofa as a place to sleep           | `feat(web): let hosts offer a sofa`              |
| Rescheduling an event drops its bookings             | `fix(backend): keep booked nights on reschedule` |
| A new skill and its link in `AGENTS.md`              | `ai: add the commit skill`                       |
| The CI workflow fetches the full history             | `ci: fetch the full history in CI`               |
| A booking service split into modules, same behaviour | `refactor(backend): split the booking service`   |
| A new moon task that runs a repo-wide script         | `chore: add the validate-agent-files task`       |
| An API change that breaks existing clients           | `feat(backend)!: drop the legacy booking API`    |

The issue's kind label hints at the usual type. The change decides; the label doesn't.

| Kind label      | Usual type                                 |
| --------------- | ------------------------------------------ |
| `enhancement`   | `feat`                                     |
| `bug`           | `fix`                                      |
| `chore`         | `chore`, `ci`, `ai`, `build` or `refactor` |
| `documentation` | `docs`                                     |

## Footers

Footers go after the body, separated by a blank line, one per line.

| Footer                    | When                                                                    |
| ------------------------- | ----------------------------------------------------------------------- |
| `Refs #N`                 | the commit relates to an issue it doesn't close                         |
| `BREAKING CHANGE: <what>` | the change breaks a public contract; say what breaks and how to migrate |
| `Release-As: X.Y.Z`       | only when the maintainer asks to set the next version; last line of all |

Closing an issue is the PR's job, with `Closes #N` in its body; see the `pull-request` skill.

## Rules

- Never use a type outside the table; the hook and CI reject it.
- Never add AI attribution trailers such as `Co-Authored-By`.
