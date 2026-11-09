---
name: authoring-skills
description: Conventions for writing evie's agent skills, Claude Code subagents and AGENTS.md files, with size budgets and a checklist. Use when adding, changing or reviewing a skill in .agents/skills/, a subagent in .claude/agents/, or any AGENTS.md or CLAUDE.md file.
---

# Authoring skills, subagents and AGENTS.md files

Skills follow the [Agent Skills spec](https://agentskills.io/specification). This skill adds evie's conventions on top; it doesn't repeat the spec.

| Read                                               | When you write                                         |
| -------------------------------------------------- | ------------------------------------------------------ |
| [references/skills.md](references/skills.md)       | a skill: structure, naming, building blocks, workflows |
| [references/subagents.md](references/subagents.md) | a subagent in `.claude/agents/` that wraps skills      |
| [references/agents-md.md](references/agents-md.md) | the root or a project `AGENTS.md`                      |

## Where things live

| What               | Path                                   | Read by                             |
| ------------------ | -------------------------------------- | ----------------------------------- |
| Skills             | `.agents/skills/<name>/SKILL.md`       | Claude Code, OpenCode, Codex        |
| Skills (Claude)    | `.claude/skills` → `../.agents/skills` | symlink; never add files through it |
| Subagents          | `.claude/agents/<name>.md`             | Claude Code only                    |
| Agent instructions | `AGENTS.md`, `CLAUDE.md` → `AGENTS.md` | root and every moon project         |

There is one skills directory, at the repo root. No per-project skill directories, no `.opencode/skills/`.

## Budgets

The source of truth for the CI validator. Lines are counted with `wc -l` over the whole file, frontmatter included. "Guide" rows are advice; CI doesn't check them.

| What                                                | Limit                                                                                             | CI    |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ----- |
| `SKILL.md`                                          | under 500 lines                                                                                   | yes   |
| Any `AGENTS.md`                                     | under 200 lines                                                                                   | yes   |
| Subagent file in `.claude/agents/`                  | under 100 lines                                                                                   | yes   |
| Skill `name`                                        | 1–64 chars of `a-z`, `0-9` and `-`; no leading, trailing or double `-`; equals the directory name | yes   |
| Skill `description`                                 | 1–1024 chars                                                                                      | yes   |
| Skill `compatibility`                               | 1–500 chars, if present                                                                           | yes   |
| Skill frontmatter keys                              | spec fields only: `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools`  | yes   |
| Subagent `name`                                     | same rules as a skill `name`, equals the file name without `.md`                                  | yes   |
| Subagent `description`                              | non-empty                                                                                         | yes   |
| Relative links in skills, subagents and `AGENTS.md` | resolve to an existing file                                                                       | yes   |
| Symlinks (`CLAUDE.md`, `.claude/skills`)            | resolve; `CLAUDE.md` points to `AGENTS.md` in the same directory                                  | yes   |
| Skill `description`                                 | aim for under 300 chars                                                                           | guide |
| `SKILL.md`                                          | aim for under 150 lines; move detail to `references/`                                             | guide |
| Reference file                                      | under 300 lines, one topic each                                                                   | guide |

## Building blocks and workflows

- A _building block_ does one task and is reused by other skills, e.g. `commit`, `verify`, `moon-run`.
- A _workflow_ chains building blocks as numbered steps and adds no task logic of its own, e.g. `ship` = `verify` → `commit` → `pull-request`.

Details in [references/skills.md](references/skills.md#building-blocks-and-workflows).

## Skills referencing each other

Skills can't call each other. A skill names another one in backticks and tells the agent to use it: "Use the `verify` skill." A subagent can also preload skills with its `skills` field. Never link into another skill's files.

## Checklist: add a skill

1. Check the Skills list in the root `AGENTS.md`. Reuse a family prefix (`moon-`, `review-`, `-conventions`) if one fits.
2. Create `.agents/skills/<name>/SKILL.md` with `name` and `description` only (see [description](references/skills.md#description)).
3. Write a short body: when to use, steps, output. Move detail to `references/`, scripts to `scripts/`, templates to `assets/`.
4. Refer to other skills by name; refer to repo files by their repo-root path in backticks.
5. If a subagent wraps the skill, add it to `.claude/agents/` (see [references/subagents.md](references/subagents.md)).
6. In the root `AGENTS.md`, link the skill in the Skills list, drop its _planned_ marker, and update the Workflow table if a step changes.
7. Update any project `AGENTS.md` whose workflow the skill changes.
8. Validate:

   ```sh
   uvx --from "git+https://github.com/agentskills/agentskills#subdirectory=skills-ref" skills-ref validate .agents/skills/<name>
   wc -l .agents/skills/<name>/SKILL.md AGENTS.md
   ```

   Once CI validation exists, run its moon task instead.

9. Try it: start a fresh agent session, ask with one of the trigger phrases, and check the skill loads.
