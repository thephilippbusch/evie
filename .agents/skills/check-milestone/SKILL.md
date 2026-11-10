---
name: check-milestone
description: Checks each target of a GitHub milestone against the repo and GitHub, and reports which targets are done, partly done, missing or need a person to check, with evidence and the open issues that would finish them. Use when asked what a milestone still needs, before closing a milestone, or before its last PR.
---

# Check milestone

Read-only: it reads the milestone, the repo and GitHub, and reports. It never closes issues or edits milestones, issues, PRs or files.

Use only commands that match the `Bash(...)` allow list in `.claude/settings.json`, so the `milestone-auditor` subagent runs without prompts. The commands below all match it. Read files with the agent's file tools (Read, Grep, Glob) or `git ls-files` and `git grep`, not `ls`, `cat` or `find`. Filter JSON with `gh --jq` or `jq`. `gh api` isn't allowed.

## Steps

1. **Pick the milestone.**
   - The title the caller gives, e.g. `v0.1 - Development setup`.
   - Otherwise the lowest open version: `gh issue list --state open --limit 1000 --json milestone --jq '[.[].milestone.title | select(.)] | unique'`. Compare versions as numbers, so `v0.2` < `v0.10` < `v1`.
2. **Read the targets.**
   - List the milestone's issues: `gh issue list --milestone "<m>" --state all --limit 1000 --json number,title,state`.
   - Read the description through any of them: `gh issue view <n> --json milestone --jq .milestone.description`.
   - Each bullet under `# Targets` is one target: `- **<target>**: done when <outcome>`. The format is in `docs/templates/github-milestone.md`.
3. **Check each target.** Split its "done when" into single checks, one per claim. Check each against the first source that can answer it:

   | Source   | Commands                                                                                                                                                                                      |
   | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
   | the repo | file tools, `git ls-files`, `git grep`, `git log`, `moon query projects`, `moon query tasks`, the workflows in `.github/workflows/`                                                           |
   | GitHub   | `gh repo view --json <fields>` for settings, e.g. `squashMergeAllowed,mergeCommitAllowed,rebaseMergeAllowed`; `gh pr list --state merged --search "<text>"`; `gh run list`; `gh release list` |
   | a person | anything no allowed command can show, such as rulesets, secrets, or "an agent loads the skills"                                                                                               |

   A check is `pass` with evidence, `fail` with the gap, or `unknown` with what to check by hand.
4. **Rate each target** from its checks:

   | Status    | When                                         |
   | --------- | -------------------------------------------- |
   | `done`    | every check passes                           |
   | `partly`  | some checks pass and at least one fails      |
   | `missing` | no check passes and at least one fails       |
   | `unknown` | no check fails and at least one is `unknown` |

5. **Map open issues** to the targets they would finish, from their titles; read the body with `gh issue view <n> --json title,body` when the title isn't enough. Closed issues aren't evidence: a target is done only when the checks pass.

## Output

A fixed Markdown list, with no text before or after it:

1. The status line: `**check-milestone:** <m>: N of M targets done, N partly, N missing, N unknown`.
2. One line per target, in the order of the description: `- **done|partly|missing|unknown** · <target>: <evidence or gap> (<open issues>)`. Leave out the brackets when no open issue maps to the target.
3. If you made assumptions, such as which milestone you picked: `**Assumptions:** <list>`.

```markdown
**check-milestone:** v0.1 - Development setup: 1 of 3 targets done, 1 partly, 0 missing, 1 unknown

- **done** · Commit conventions: `.github/workflows/pr-title.yml` checks PR titles; `gh repo view` shows squash merges only
- **partly** · Core workflow skills: `.agents/skills/verify/SKILL.md` and five more exist; `plan` is missing (#11)
- **unknown** · Agent entry point: `AGENTS.md` is 120 lines and the symlinks resolve; check by hand that OpenCode loads the skills

**Assumptions:** no milestone given; picked the lowest open version.
```

If the milestone can't be read, return only `**check-milestone:** blocked: <reason>`.

## Rules

- Never close or edit issues, milestones, PRs or files.
- Every `done` cites evidence: a path, a PR, a workflow run or a release.
- Every `partly`, `missing` and `unknown` names the gap or the check left for a person.
- Never guess a check: if no allowed command can answer it, it is `unknown`.
