---
name: commit
description: Commits work in small, logical commits with Conventional Commit messages, through evie's git hooks. Use when committing changes, splitting work into commits, or when asked to commit, stage or write a commit message.
---

# Commit

## When to commit

- One logical change per commit. Each commit passes the `verify` skill at the uncommitted scope.
- A test and the fix it guards go in the same commit, so no commit has a failing test.
- Refactors and cleanups go in their own commit, before or after the change that needs them.

## Steps

1. Look at what changed: `git status` and `git diff`.
2. Stage the files of one logical change by name: `git add <path>…`. Never `git add -A` or `git add .`.
3. Check what is staged: `git diff --cached --stat`.
4. Write the header with the `conventional-commits` skill. Write the body with the `writing-style` skill; leave it out when the header says it all.
5. Commit: `git commit -m "<header>"`, or with a body, `git commit -F <file>`.
6. Repeat for the next logical change.

## Hooks

| Hook         | What it does                                                                 |
| ------------ | ---------------------------------------------------------------------------- |
| `pre-commit` | formats the staged Markdown, JSON and YAML files with dprint and stages them |
| `commit-msg` | rejects a message that isn't a Conventional Commit                           |

If a hook fails, the commit doesn't happen. Read its message, fix the cause and commit again. A file with both staged and unstaged changes is only checked, not formatted: run `dprint fmt <file>`, stage it and commit again.

## Rules

- Never pass `--no-verify`.
- Never amend, rebase or otherwise rewrite a commit that is pushed. Force-push is denied.
- No AI attribution trailers, such as `Co-Authored-By`.
- Never commit secrets or `.env` files.
- Never commit hand-edited generated code; change the source and run `codegen`.
- Never commit changes to `CHANGELOG.md` or `.release-please-manifest.json`; release-please owns them.

## Output

One line per commit: the short hash and the header, as `git log --oneline` prints them.
