---
name: branch
description: Creates a branch off the latest main for one issue, with evie's branch name, and sets up git worktrees for parallel work. Use when starting work on an issue, or when asked to create, name or switch to a branch or a worktree.
---

# Branch

Every change lands through a PR from its own branch. Never commit on `main`.

## Name

`<type>/<issue_nr>/<slug>`, e.g. `chore/8/git-delivery-skills`.

- **type:** the type the PR title will have. Pick it with the `pull-request` skill, which follows the `conventional-commits` skill.
- **issue_nr:** the number of the issue the branch closes.
- **slug:** 2–5 lowercase words joined by `-`, taken from the issue title.

One issue per branch. If the work grows past its issue, split the issue with the `issue` skill instead of widening the branch.

## Steps

1. Check that the working tree is clean: `git status --short` prints nothing. Commit or set aside open changes first.
2. Fetch the latest `main`: `git fetch origin`.
3. Create the branch from it and switch to it: `git switch -c <name> origin/main`.

## Worktrees for parallel work

A worktree is a second checkout with its own branch, so several tasks can run at once without sharing a working tree.

1. `git fetch origin`
2. `git worktree add <path> -b <name> origin/main`
3. Work and commit inside `<path>`.
4. Once the branch is pushed and you no longer need the checkout, remove it: `git worktree remove <path>`.
5. Run `moon sync hooks` in the main checkout.

All worktrees share one `core.hooksPath`, and moon may point it into a worktree. After a worktree is removed, the hooks would be gone for every checkout until `moon sync hooks` points them back.

## Output

The branch name and the commit it starts from: `git log --oneline -1`.
