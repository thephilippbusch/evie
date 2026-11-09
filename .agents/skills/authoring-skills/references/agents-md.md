# Writing AGENTS.md files

`AGENTS.md` is the entry point every agent reads first. Claude Code reads `CLAUDE.md`, so each `AGENTS.md` has a `CLAUDE.md` symlink next to it:

```sh
ln -s AGENTS.md CLAUDE.md
```

Write and edit only `AGENTS.md`, never `CLAUDE.md`.

## Root and project files

| File                  | Contains                                                                                                        |
| --------------------- | --------------------------------------------------------------------------------------------------------------- |
| Root `AGENTS.md`      | What evie is, repo map, stack, toolchain and commands, repo-wide rules, workflow, index of skills and subagents |
| `<project>/AGENTS.md` | One per moon project: purpose, structure, commands, local rules                                                 |

A project file has these sections, in this order:

1. **Purpose**: what the project is and who uses it, in one or two sentences.
2. **Structure**: the main directories and files, as a table.
3. **Commands**: the project's moon tasks, as `moon run <project>:<task>`.
4. **Local rules**: rules that hold only here, or that differ from the root.

Where a project file differs from the root, the project file wins. Say so explicitly ("Unlike the root rule, …") so the override is deliberate.

## Link, don't repeat

- Say a thing once, in the file closest to where it applies. Link to it from everywhere else.
- Project files don't repeat root rules or the stack.
- Link to skills, ADRs and docs for detail instead of copying it in.
- Mark anything that doesn't exist yet as _planned_ and drop the marker when it lands.

## Size

Every `AGENTS.md` stays under 200 lines (see the budgets in [SKILL.md](../SKILL.md#budgets)). When a file grows past it, move detail to a skill, an ADR or a project file and link to it.

## Skills and subagents index

The root `AGENTS.md` lists every skill and subagent, grouped by family:

- A planned skill is a plain name in backticks: `` `verify` ``.
- A skill that exists is a link to its `SKILL.md`, like the `authoring-skills` entry under Meta.
- A workflow is listed after its building blocks with `· workflow`.

## Keep them current

A change that adds or changes a workflow updates the matching `AGENTS.md` and skills in the same change. That includes:

- a new or renamed skill or subagent → the root index
- a new or changed moon task → the project's Commands section
- a new or changed rule → the file it applies to, root or project

## Style

Short, plain and scannable. Tables for maps and commands, bullets for rules, one idea per sentence. No history, no changelog, no references to internal planning documents; point to GitHub milestones and issues instead.
