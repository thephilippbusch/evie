# Writing a skill

The [Agent Skills spec](https://agentskills.io/specification) defines the format. This file covers what evie adds.

## Structure

```text
.agents/skills/<name>/
├── SKILL.md       # frontmatter + short body, always loaded on use
├── references/    # detail the agent reads only when a step needs it
├── scripts/       # code the agent runs instead of reading
└── assets/        # templates and data files the agent copies or fills
```

Only `SKILL.md` is required. Add the other directories when you need them, not up front.

## Frontmatter

Use spec fields only: `name`, `description`, and rarely `compatibility`, `metadata`, `license` or `allowed-tools`. The spec validator rejects anything else, and OpenCode and Codex ignore Claude Code-only fields such as `context` or `disable-model-invocation`. If a skill needs Claude Code-only behaviour, put it in a subagent.

### Name

- Lowercase, digits and single hyphens, equal to the directory name.
- A building block is named for its task: a verb or a short noun (`commit`, `verify`, `plan`, `debug`).
- Related skills share a prefix: `moon-run`, `moon-query`; `review-security`, `review-tests`.
- Coding conventions end in `-conventions`: `rust-conventions`, `typescript-conventions`.
- A workflow is named for its outcome (`ship`, `review`). A workflow and its building blocks may share a prefix: `review` runs `review-*`.

### Description

The description is all an agent sees before it picks a skill. Write it for that choice:

1. One sentence on what the skill does, starting with a verb or noun phrase, in the third person.
2. One sentence starting with "Use when", listing the situations and the words a user would say.

```yaml
description: Formats, lints and tests the moon projects a change affects and reports only failures. Use when checking work before a commit or PR, or when asked to verify, check or test changes.
```

Avoid vague verbs ("helps with"), repo history, and step details. Aim for under 300 characters.

## Body

The body loads in full when the skill is used, so keep it short and imperative:

- **When to use**: only if the description isn't enough, e.g. when not to use it.
- **Steps**: numbered, one action each, naming the command or skill to use.
- **Output**: what the agent returns or leaves behind. If a subagent or workflow consumes it, give a fixed format (see [subagents.md](subagents.md#structured-output)).
- **Rules**: the few things that must never happen, if any.

Write like the root `AGENTS.md`: short sentences, tables and bullets, no filler. Follow the `writing-style` skill once it exists.

## Progressive disclosure

Put in `SKILL.md` what every use needs. Move the rest out:

- `references/<topic>.md` for detail only some uses need: one topic per file, e.g. the `plan` skill keeps one file per kind of work.
- `scripts/` when a step is mechanical and the same every time. Scripts state their dependencies and fail with a clear message.
- `assets/` for templates the agent fills in.

Link each file from `SKILL.md` with a relative path from the skill root and say when to read it, as the `authoring-skills` `SKILL.md` does in its first table. Keep references one level deep: link every file the agent may need from `SKILL.md`, so no file is reachable only through another reference file.

## Building blocks and workflows

A **building block** does one task end to end and is useful on its own. It doesn't chain other skills. It may tell the agent to use another skill for a prerequisite, e.g. `verify` uses `moon-query` to find affected projects.

A **workflow** chains building blocks. Its body is a numbered list of steps, each naming the skill it uses, plus what to do when a step fails:

```markdown
## Steps

1. Use the `verify` skill. Stop and report if it fails.
2. Use the `commit` skill.
3. Use the `pull-request` skill.
```

A workflow doesn't repeat a building block's instructions. If two workflows need the same logic, make it a building block.

## Referencing other skills and files

- **Other skills**: name them in backticks, "Use the `verify` skill". Skills can't call each other, and paths into another skill's directory differ between agents. Never link to another skill's files.
- **Preloading**: a subagent lists skills in its `skills` field to load them up front (see [subagents.md](subagents.md)).
- **Files in the skill**: relative Markdown links from the skill root, e.g. `references/bug.md` in the `plan` skill. CI checks that they resolve.
- **Repo files**: their repo-root path in backticks, e.g. `docs/templates/github-issue.md`. Agents run from the repo root, so no `../../` links.
- **Planned skills**: name them anyway and add "(once it exists)", so the link is obvious when the skill lands.

## Changing a skill

- Keep the name stable; other skills and subagents refer to it.
- When you rename or remove a skill, search `.agents/`, `.claude/agents/` and every `AGENTS.md` for its name and update them.
- When a skill changes how work is done, update the matching `AGENTS.md` in the same change.
