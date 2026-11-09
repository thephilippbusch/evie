# evie

evie is a web app for private "family & friends" events. An event holds one or more activities, hosts offer places to sleep, and attendees book into them.
Work is tracked in [GitHub milestones](https://github.com/thephilippbusch/evie/milestones), one per version, and their issues. Read the issue and its milestone before you start.

> Early setup: anything marked _planned_ doesn't exist yet.

## Repo map

| Path              | Contents                                                                                                     |
| ----------------- | ------------------------------------------------------------------------------------------------------------ |
| `apps/`           | Deployable apps, one Docker image each: SolidStart frontend + BFF, Rust gRPC backend (_planned_)             |
| `libs/`           | Libraries used by apps, never deployed: protobuf definitions, shared crates and packages (_planned_)         |
| `docs/adr/`       | Architecture Decision Records: the reasons behind the rules below (_planned_)                                |
| `docs/templates/` | GitHub [issue](docs/templates/github-issue.md) and [milestone](docs/templates/github-milestone.md) templates |
| `.agents/skills/` | Agent Skills shared by every agent; `.claude/skills` is a symlink to it                                      |
| `.claude/agents/` | Claude Code subagents (_planned_)                                                                            |
| `.moon/`          | moon workspace config. The root [`moon.yml`](moon.yml) is the `root` project, for repo-wide tasks            |
| `scripts/`        | Scripts run by `root` tasks, such as the commit hooks                                                        |

Every moon project has its own `AGENTS.md` (with a `CLAUDE.md` symlink) covering its purpose, structure, commands and local rules (_planned_).
Read it before you change that project. Where it differs from this file, the project file wins.

## Stack

- **Frontend:** SolidStart + shadcn/ui (Solid port), written in idiomatic Effect. The SolidStart server is the BFF.
- **Backend:** Rust, Tokio, Tonic (gRPC), SeaORM.
- **Database:** PostgreSQL.
- **Auth:** BetterAuth in the BFF, persisted through Drizzle.

## Toolchain and commands

proto pins the tool versions; moon is the entry point for every task. Prefer `moon run` over calling bun or cargo directly, so caching and task dependencies apply.

```sh
proto trust && proto install      # install the pinned toolchain (moon, dprint, bun, rust, …)
moon query projects               # list projects
moon query projects --affected    # projects touched by your changes
moon run <project>:<task>         # one task in one project, e.g. moon run backend:test
moon run :<task>                  # a task in every project that has it, e.g. moon run :lint
moon ci                           # the affected tasks, as CI runs them
docker compose up                 # local stack: PostgreSQL, Mailpit, backend, frontend
```

Standard tasks: `build`, `lint`, `format`, `test`, `codegen`, `migrate`.
Git hooks install themselves on the first `moon run` in a checkout, or with `moon sync hooks`. `pre-commit` formats the staged Markdown, JSON and YAML files with dprint and stages the result; `commit-msg` rejects messages that aren't Conventional Commits.
Add dependencies with the native tool inside the project: `bun add <pkg>` or `cargo add <crate>`.
Releases: a `feat`, `fix`, `perf` or `revert` squash commit on `main` makes release-please open or update the release PR; merging it tags `vX.Y.Z`, writes `CHANGELOG.md` and creates the GitHub release.
Agents can also query moon over MCP: [`.mcp.json`](.mcp.json) registers `moon mcp` for Claude Code, and `.claude/settings.json` allows its read-only tools.

- `get_projects`, `get_project`, `get_tasks`, `get_task`: the project and task graph.
- `get_changed_files`: changed files; pass `remote: false` for uncommitted changes. A file belongs to the project with the deepest `source` that contains it; those projects are the affected ones.
- `get_templates`, `get_template`: code generator templates. `generate`, `sync_projects` and `sync_workspace` write files, so they ask first.
- Not registered for OpenCode and Codex yet: moon 2.6 speaks only the stateless MCP `2026-07-28` protocol. OpenCode 2.2 can't connect to it; Codex is untested.

## Rules

Non-negotiable. Each rule will link to its ADR once it exists.

- Every user-facing text exists in German and English.
- The browser never talks gRPC; SolidStart server functions call the backend.
- SeaORM owns the domain schema, Drizzle owns the `auth` schema. No foreign keys between them; the domain stores the BetterAuth user ID as an opaque value.
- `drizzle-kit push` runs in local dev only. Everywhere else, generate and review migrations and apply them with `moon run :migrate`.
- Frontend server and shared code is idiomatic Effect: `Context.Tag` + `Layer`, `Config`, `Schema`, `Data.TaggedError`.
- Never hand-edit generated code (protobuf output, the BetterAuth Drizzle schema). Change the source and run `codegen`.
- Never edit `CHANGELOG.md` or the release manifest; release-please owns them.
- Never push to `main` or force-push. Work on a branch and open a PR.
- Never read `.env` files or other secrets.
- Claude Code enforces the rules above with permissions in `.claude/settings.json` and a hook in `.claude/hooks/` (needs `jq`); list new generated or release-owned paths in [`protected-paths.txt`](.claude/hooks/protected-paths.txt).
- PR titles are Conventional Commits, checked in CI. PRs are squash-merged, and the commit takes the PR title and description. Before v1, PR titles use a type that doesn't release (`chore`, `ci`, `docs`, `ai`, …); only the PR that closes the last issue of a milestone is a `feat`.
- From v0.5 on, participation features support acting for household members and managed profiles, and record who acted.
- New apps are added with the `moon-new-app` skill, which registers them for releases and image builds.
- A change that adds or changes a workflow updates the matching `AGENTS.md` and skills.

## Workflow

| Step         | What to do                                                  | Skill                                        |
| ------------ | ----------------------------------------------------------- | -------------------------------------------- |
| 1. Plan      | Read the issue and its milestone; plan by kind of work      | `plan`                                       |
| 2. Branch    | Branch off `main`                                           | `branch`                                     |
| 3. Implement | Follow the project's `AGENTS.md` and the conventions skills | `rust-conventions`, `typescript-conventions` |
| 4. Verify    | Format, lint and test the affected projects                 | `verify`                                     |
| 5. Commit    | Small commits with Conventional Commit messages             | `commit`, `conventional-commits`             |
| 6. PR        | Conventional Commit title, link the issue                   | `pull-request`                               |
| 7. Review    | Run the focus reviewers and address the findings            | `review`                                     |

`ship` chains steps 4–6. `check-milestone` shows what is still missing in a milestone. Until a skill exists, do the step by hand.

## Skills

In `.agents/skills/`, following the [Agent Skills spec](https://agentskills.io/specification). Claude Code reads them through the `.claude/skills` symlink; OpenCode and Codex read `.agents/skills/` directly. Add skills only in `.agents/skills/`, never in `.claude/skills/` or `.opencode/skills/`.
Check skills, subagents and every `AGENTS.md` with `moon run root:validate-agent-files`; CI runs it on every PR.
_Building blocks_ do one task and are reused by other skills; _workflows_ chain them. Unlinked names are _planned_; link each one here when it lands.

- **Git & delivery:** [`conventional-commits`](.agents/skills/conventional-commits/SKILL.md), [`branch`](.agents/skills/branch/SKILL.md), [`commit`](.agents/skills/commit/SKILL.md), [`pull-request`](.agents/skills/pull-request/SKILL.md) · workflow [`ship`](.agents/skills/ship/SKILL.md)
- **Issues & milestones:** [`issue`](.agents/skills/issue/SKILL.md), `check-milestone`
- **moon:** [`moon-query`](.agents/skills/moon-query/SKILL.md), [`moon-run`](.agents/skills/moon-run/SKILL.md), [`moon-new-library`](.agents/skills/moon-new-library/SKILL.md), [`moon-new-app`](.agents/skills/moon-new-app/SKILL.md), [`moon-add-task`](.agents/skills/moon-add-task/SKILL.md)
- **Quality:** [`verify`](.agents/skills/verify/SKILL.md), [`writing-style`](.agents/skills/writing-style/SKILL.md), [`rust-conventions`](.agents/skills/rust-conventions/SKILL.md), [`typescript-conventions`](.agents/skills/typescript-conventions/SKILL.md)
- **Planning:** `plan`
- **Investigation:** [`debug`](.agents/skills/debug/SKILL.md), [`benchmark`](.agents/skills/benchmark/SKILL.md)
- **Review:** `review-security`, `review-architecture`, `review-code-quality`, `review-tests`, `review-project-rules` · workflow `review`
- **Meta:** [`authoring-skills`](.agents/skills/authoring-skills/SKILL.md)

## Subagents

In `.claude/agents/`. Thin wrappers that preload skills and restrict tools; they run non-interactively with structured output. All _planned_.

- **Reviewers:** one read-only reviewer per review focus, run in parallel by `review`
- **Verifier:** runs the affected checks in the background and reports only failures
- **Milestone auditor:** runs `check-milestone` against a GitHub milestone and lists the gaps
- **Implementer:** works in its own git worktree with `commit` and `verify`, for parallel independent tasks

## Editing this file

Keep it under 200 lines and link to details instead of repeating them. Project-specific notes go in that project's `AGENTS.md`. Conventions for skills, subagents and `AGENTS.md` files live in [`authoring-skills`](.agents/skills/authoring-skills/SKILL.md).
