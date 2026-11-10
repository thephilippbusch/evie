# Planning a CI change

GitHub workflows, moon tasks that CI runs, repository settings and the release pipeline. A broken workflow blocks every PR, so plan how to test it before it merges.

## Find out first

1. Read the workflows in `.github/workflows/` and the moon tasks they run: `moon query tasks`.
2. Check which repository settings the issue touches: `gh repo view --json <fields>` for merge settings. Rulesets and secrets need the GitHub web UI or an admin.
3. For releases, read `release-please-config.json` and `.release-please-manifest.json`. Never edit the manifest or `CHANGELOG.md`; release-please owns them.

## Plan sections

Add these under Approach:

- **Workflows:** each file in `.github/workflows/` and its triggers, jobs and steps.
- **moon tasks:** checks run as moon tasks, so `moon ci` runs them locally and in CI the same way. Add them with the `moon-add-task` skill.
- **Workflow security:**
  - pin every action to a full commit SHA, with the version in a comment
  - set the least `permissions` per workflow or job
  - pass untrusted input, such as PR titles, branch names and issue bodies, through `env`, never straight into `run`
  - `pull_request_target` and secrets only with a reason written next to them
- **Repository settings:** each setting, its value, and who changes it. Agents can't change them; list them for a person under Open questions.
- **Agent files:** the `AGENTS.md` files and skills that describe the changed workflow, such as `verify`, `moon-run` or `pull-request`, change in the same PR.

## Verify

- `moon ci` passes locally, with the `verify` skill at the CI scope.
- The workflow runs on the PR: `gh run list --branch <branch>`. For a workflow that only runs on `main` or on a release, say how it will be checked after the merge.
- Settings changes: `gh repo view --json <fields>` shows the new values, or a person confirms them.
