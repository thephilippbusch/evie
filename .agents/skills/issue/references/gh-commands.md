# gh commands for issues

Run them from the repo root: `gh api` fills in `{owner}/{repo}` from the checkout. In Claude Code, `gh issue view`, `gh issue list` and `gh label list` run without asking; `gh api` and every command that changes GitHub ask first.

## Read

```sh
gh api repos/{owner}/{repo}/milestones --jq '.[] | "\(.number)\t\(.title)"'   # open milestones
gh api repos/{owner}/{repo}/milestones/<number> --jq .description          # a milestone's targets
gh issue list --milestone "v0.1 - Development setup" --state all          # its issues
gh issue view <n>                                                          # body, labels, parent, blocked-by
gh label list                                                              # live labels
```

## Create

Pass the body on stdin with `--body-file -` and a quoted heredoc, so no temporary file is needed and nothing in the body is expanded:

```sh
gh issue create \
  --title "Add the issue skill" \
  --milestone "v0.1 - Development setup" \
  --label chore --label agents \
  --project "Evie Planner App" \
  --body-file - <<'EOF'
# Description

Teach agents to write and refine GitHub issues the way this repo does.

# Scope

## In scope

- Writing and refining issues per kind of work, using the issue template

## Out of scope

- Automatic issue refinement on new issues (#44)

# Acceptance Criteria

- An agent can turn a milestone's targets into well-formed issues in that milestone.

# Technical Details

- Template: `docs/templates/github-issue.md`
EOF
```

`gh issue create` prints the new issue's URL. Add `--assignee <login>` (or `@me`) only for whoever starts on it now. `--project` needs the `project` token scope (`gh auth refresh -s project`).

## Edit

```sh
gh issue edit <n> --body-file - <<'EOF'
# Description
…
EOF
gh issue edit <n> --title "<new title>"
gh issue edit <n> --milestone "v0.2 - Monorepo setup"
gh issue edit <n> --add-label agents --remove-label documentation
gh issue edit <n> --add-assignee <login>
```

`--body-file` replaces the whole body, so start from the current one: `gh issue view <n> --json body --jq .body`.

## Database IDs

The sub-issue and dependency endpoints take an issue's database ID, not its number:

```sh
gh api repos/{owner}/{repo}/issues/<n> --jq .id
```

## Sub-issues

Only while planning a single issue (see the `SKILL.md` Sub-issues section).

```sh
gh api -X POST repos/{owner}/{repo}/issues/<parent>/sub_issues -F sub_issue_id=<database id of the child>
gh api repos/{owner}/{repo}/issues/<parent>/sub_issues --jq '.[] | "#\(.number) \(.title)"'
gh api -X DELETE repos/{owner}/{repo}/issues/<parent>/sub_issue -F sub_issue_id=<database id of the child>
```

The remove path is singular: `sub_issue`.

## Blocked-by dependencies

Record only direct dependencies: if #3 waits for #2 and #2 for #1, don't also mark #3 blocked by #1.

```sh
gh api -X POST repos/{owner}/{repo}/issues/<n>/dependencies/blocked_by -F issue_id=<database id of the blocker>
gh api repos/{owner}/{repo}/issues/<n>/dependencies/blocked_by --jq '.[] | "#\(.number) \(.title)"'
gh api repos/{owner}/{repo}/issues/<n>/dependencies/blocking --jq '.[] | "#\(.number) \(.title)"'
gh api -X DELETE repos/{owner}/{repo}/issues/<n>/dependencies/blocked_by/<database id of the blocker>
```

Recent `gh` versions do the same by issue number: `gh issue create --blocked-by 9 --parent 12` and `gh issue edit <n> --add-blocked-by 9 --add-sub-issue 13`. Check `gh issue edit --help` before you rely on them; `gh` isn't pinned by proto.
