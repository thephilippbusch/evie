# Writing a subagent

Subagents live in `.claude/agents/<name>.md` and run only in Claude Code. See the [Claude Code subagent docs](https://code.claude.com/docs/en/sub-agents) for every frontmatter field.

## Thin wrappers

A subagent wraps one or more skills. The skills hold the knowledge; the subagent adds what only Claude Code can do: preload skills, restrict tools, run in parallel, in the background or in its own worktree.

- Put task logic in a skill, never only in a subagent. OpenCode and Codex run the same skills one after another, without subagents.
- The body says which preloaded skill to follow, the limits of the role and the output. It doesn't repeat the skill.
- One role per subagent. Several reviewers run in parallel; one reviewer doesn't cover every focus.

## Naming

- Lowercase, digits and single hyphens; the file name is `<name>.md`.
- Name the role, not the task: `verifier`, `implementer`, `milestone-auditor`.
- A family follows its skills: the `review-security` skill gets the `reviewer-security` subagent.

## Frontmatter

```yaml
---
name: reviewer-security
description: Read-only security reviewer for a diff. Use when the review workflow runs its focus reviewers.
tools: Read, Grep, Glob, Bash
skills:
  - review-security
maxTurns: 30
---
```

| Field                 | Convention                                                                               |
| --------------------- | ---------------------------------------------------------------------------------------- |
| `name`                | Required. Equals the file name.                                                          |
| `description`         | Required. When the main agent should delegate to it, in one or two sentences.            |
| `tools`               | Always set. The smallest allowlist the role needs; reviewers and auditors are read-only. |
| `skills`              | The skills the subagent follows, preloaded in full at startup.                           |
| `maxTurns`            | Always set, so a stuck run ends with partial output.                                     |
| `isolation: worktree` | For agents that edit files in parallel with others, e.g. the implementer.                |
| `background: true`    | For agents whose result can wait, e.g. the verifier.                                     |
| `model`, `effort`     | Only when the role needs a different one than the main session.                          |

`tools` takes whole tools only; `Bash(git diff *)`-style specifiers don't narrow it. A read-only role that needs `git diff` keeps `Bash`, and its body forbids commands that change anything. Blocking specific commands for every agent is done in `.claude/settings.json`, not here.

Preloading doesn't restrict which skills a subagent can use. To stop it from loading other skills, leave `Skill` out of `tools`.

## Non-interactive

Subagents can't ask the user. The body says so and says what to do instead:

- Don't ask questions. If something is missing, make the safest assumption, state it in the output and go on.
- If the task can't be done, stop and return the reason in the output format.
- Stay inside the role: a reviewer doesn't fix, an auditor doesn't edit.

## Structured output

The output format belongs to the skill, so it is the same whether the skill runs in a subagent, headless with `claude -p`, or inline in another agent. The subagent body only says "Return only the result, in the format the skill defines."

A good format is fixed and easy to merge:

- A short status first: `pass`, `fail` or `blocked`.
- Then a list of items with the same fields each, e.g. for findings: severity, file and line, problem, suggested fix.
- No prose before or after it.

Use JSON when a script or workflow parses the result; use a fixed Markdown list when a person reads it first.

## Body template

```markdown
You are the security reviewer for evie. Follow the preloaded `review-security` skill.

- Review only the diff you are given. Don't edit files.
- Don't ask questions. State assumptions in your output.
- Return only the findings, in the format the skill defines.
```
