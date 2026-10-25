# 00 — Development setup

**Kind:** horizontal · **Status:** pending

## Requirements

- None. This is the root stage.

## Desired outcome

Agents and humans work in the repo with shared, documented conventions: any coding agent (Claude Code, Codex, Cursor, …) finds the same instructions, skills and guardrails. evie is released with one repository-wide SemVer version derived from Conventional Commits, with a changelog, git tag and GitHub release for every version.

## Scope

### In scope

- **Root `AGENTS.md`** (with `CLAUDE.md` as a symlink to it, under 200 lines). It points to details instead of repeating them:
  - what evie is, with a link to the gameplan;
  - a repo map (`apps/`, `libs/`, `docs/`, agent folders), noting that every project has its own `AGENTS.md` (stage 01);
  - toolchain and common commands: moon as the entry point (run, query, ci), proto, bun, cargo, docker compose;
  - non-negotiable rules as one-liners with links (e.g. schema ownership, the browser never talks gRPC, never hand-edit generated code, `CHANGELOG.md` or the release manifest);
  - the standard workflow (plan → branch → implement → verify → commit → PR → review), naming the skill for each step;
  - an index of skills and subagents.
- **Skills** in `.agents/skills/`, following the [Agent Skills](https://agentskills.io/specification) spec with portable frontmatter; `.claude/skills` is a symlink to it. Each skill does one task, keeps its body brief and moves details into `references/`. There are two kinds: _building blocks_, which other skills and subagents reuse, and _workflows_, entry points that chain building blocks. Initial set:
  - **Git & delivery:** `conventional-commits` (types and their release impact), `branch`, `commit` (how and when), `pull-request`; workflow `ship` (verify → commit → PR).
  - **Issues & project management:** writing and refining GitHub issues per type, labels, GitHub milestones and projects, following the [release goals](../GAMEPLAN.md#release-goals) and the [milestone](../templates/github-milestone.md) and [issue](../templates/github-issue.md) templates.
  - **moon:** `moon-query` (project and task graph, affected projects), `moon-run` (tasks, `moon ci`), `moon-new-library`, `moon-new-app` (includes the "Adding an app" checklist), `moon-add-task`.
  - **Quality:** `verify` (format, lint and test the affected projects), `writing-style`, and Rust and TypeScript/bun conventions that linters and compilers do not cover.
  - **Planning:** `plan` with one entry point per kind of work (bug, feature, performance, refactor, CI), each in its own reference file; `gameplan` (start and finish a stage, keep the status table and open questions current).
  - **Investigation:** `debug` (reproduce → isolate → fix → regression test), `benchmark` (baseline vs. change, tools per language).
  - **Review:** one skill per focus (security, architecture & boundaries, code quality, tests, project rules such as i18n, accessibility and writing style) and the `review` workflow that merges their findings.
  - **Meta:** `authoring-skills`, the conventions for writing skills, subagents and `AGENTS.md` files.
- **Subagents** in `.claude/agents/`. They are thin wrappers that preload skills and restrict tools, so other agents can still run the same skills one after another:
  - **Review fan-out:** one read-only reviewer per review focus, run in parallel.
  - **Verifier:** runs the affected checks in the background and reports only failures.
  - **Milestone auditor:** checks a stage's "done when" criteria against the repo and lists the gaps.
  - **Implementer:** works in its own git worktree and preloads `commit` and `verify`, for independent tasks in parallel.
  - All subagents run non-interactively with bounded turns and structured output, so stage 10 can run them headless in CI.
- **Guardrails & tools:**
  - Agent permissions: allow common moon, bun, cargo and git commands; deny force-pushes, pushes to `main`, `drizzle-kit push` outside local dev, and reading `.env` files and secrets.
  - Hooks that block edits to generated code, `CHANGELOG.md` and the release manifest.
  - moon VCS hooks for commit-message linting and formatting, so the same rules apply to humans and every agent.
  - GitHub issue and PR templates.
  - The moon MCP server (`moon mcp`) registered for agents.
- **Validation:** a moon task, run in CI, checks skills (`skills-ref validate`), file references, symlinks and size budgets.
- **Commit conventions:** Conventional Commits. PR titles are linted in CI and PRs are squash-merged only, so the PR title becomes the commit that release-please reads.
- **Release management with release-please** (`googleapis/release-please-action`):
  - One repository-wide version: a single root component with `release-type: simple`. One release PR collects changes until it is merged.
  - Only user-facing commits (`feat`, `fix`, `perf`, breaking `!`) trigger a release; `chore`, `docs`, `ci` and `refactor` do not.
  - Each release creates a `vX.Y.Z` tag, a GitHub release and an entry in the root `CHANGELOG.md`. Versions start at `0.1.0` (`bump-minor-pre-major`).
  - Before v1, versions follow the [release goals](../GAMEPLAN.md#release-goals): every PR is a `fix`, and only the PR that completes a goal is a `feat` (minor bump). v1 and v2 are set with a `Release-As: X.0.0` footer.
  - The version is written via `extra-files` to the root `Cargo.toml` (`[workspace.package].version`, inherited by every crate with `version.workspace = true`) and to each app's `package.json`. `Cargo.lock` is updated in the release PR too, so `--locked` builds keep working.
- **Apps and libraries:** apps produce a deployable Docker image; libraries are consumed by apps and never deployed. Every app is released with the repository version, even if it did not change. Libraries have no version of their own.
- **Release contract for stage 10:** the release workflow's outputs (`release_created`, `tag_name`, `version`) are documented and used to publish images.
- **"Adding an app" checklist:** add its manifest to `extra-files`, add a Dockerfile, add it to the image publish list.

### Out of scope

- Per-project `AGENTS.md` files and stack-specific skills (stage 01).
- Automatically triggered agentic workflows such as issue refinement, PR review on open, nightly improvement PRs and vulnerability screening (stage 10).
- Publishing images and deploying (stage 10).
- Building the apps and libraries themselves (stage 01).
- App-store build numbers for a future mobile app.

## Milestones

The agentic milestones come first, so the release milestones are built with the new skills and review.

1. **Agent entry point**: done when the root `AGENTS.md` exists within its size budget, `CLAUDE.md` and `.claude/skills` are symlinks, and Claude Code and at least one other agent load the instructions and skills.
2. **Guardrails & tools**: done when the denied actions are blocked for Claude Code, every local commit runs the moon VCS hooks, and agents can query moon through MCP.
3. **Skill conventions & validation**: done when `authoring-skills` is written and CI fails on an invalid skill, a broken reference or a broken symlink.
4. **Core workflow skills**: done when the git & delivery, moon, quality, `plan` and `gameplan` skills exist, and an agent took a stage 00 milestone from plan to PR using them.
5. **Issues, investigation & conventions**: done when the issue and project-management skills, `debug`, `benchmark` and the Rust and TypeScript conventions skills exist, and the issue templates match the issue skill.
6. **Subagents**: done when `review` runs the focus reviewers in parallel and returns one ranked list of findings, also headless with structured output, and the verifier, milestone auditor and implementer have each completed a real task.
7. **Commit conventions**: done when CI rejects PRs whose title is not a Conventional Commit and the repository allows squash merges only.
8. **Release pipeline**: done when a merged `feat:` PR opens or updates the release PR, and merging it creates the `vX.Y.Z` tag, changelog entry and GitHub release, with the version written to every registered manifest.
9. **Adding an app**: done when the checklist is documented and was followed for both initial apps (SolidStart frontend, Rust backend).

## Open questions

- Which agents besides Claude Code are tested, and do they all read `.agents/skills/`?
- Which model and effort does each subagent use, and what is the budget for parallel reviews?
- Do skills get evals (triggering and output quality), now or later?
- How is `Cargo.lock` refreshed in the release PR: a `toml` extra-file entry, or a workflow step that runs `cargo update --workspace` on the release branch?
