---
name: pull-request
description: Opens a GitHub PR for the current branch with a Conventional Commit title, including the pre-v1 rule, and a body from evie's PR template. Use when work on a branch is ready for review, or when asked to open, create or title a PR.
---

# Pull request

PRs are squash-merged: the PR title becomes the commit header on `main` and the body its commit body. So the title decides whether `main` releases.

## Steps

1. **Read the issue:** `gh issue view <n> --json title,milestone,labels,body`. You need its milestone, labels and Acceptance Criteria.
2. **Title:** a Conventional Commit, checked in CI.
   - Run `git fetch --tags origin`, then `git tag -l 'v[1-9]*'`. No output means evie is before v1.
   - **Before v1:** use a type that doesn't release: `chore`, `ci`, `docs`, `ai`, `build`, `refactor`, `style` or `test`. Use `feat` only for the PR that closes the last open issue of its milestone: `gh issue list --milestone "<milestone>" --state open` lists only this PR's issue. Never use `fix`, `perf`, `revert` or `!` before v1; they release.
   - **From v1 on:** pick the type with the `conventional-commits` skill.
   - The description says what the PR does for the reader, e.g. `ai: add the git and delivery skills`.
3. **Body:** copy `.github/pull_request_template.md` to a temporary file and fill it in:
   - **Description:** one short paragraph on what changed and why.
   - **Issue Reference:** `Closes #<n>`.
   - **Acceptance Criteria:** the issue's criteria, copied as a checklist. Tick each one the PR meets. Leave an unmet one unticked and add one line under the list: why, and which issue takes it over.
   - Delete the template's comments.
   - Add a `Release-As: X.Y.Z` footer, as the last line after a blank line, only when the maintainer asks for it.
   - Follow the `writing-style` skill. No AI attribution lines.
4. **Push:** `git push -u origin <branch>`. Never push `main`, never force-push.
5. **Open:** with the issue's labels, comma-separated:

   ```sh
   gh pr create --base main --title "<title>" --body-file <file> --label <labels> --assignee @me
   ```

6. **Stop.** Don't merge; the maintainer squash-merges.

If the title must change later, for example because the milestone's other issues closed first and this PR now closes the last one, run `gh pr edit <n> --title "<title>"`.

## Output

The PR URL that `gh pr create` prints.
