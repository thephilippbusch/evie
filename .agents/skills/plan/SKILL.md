---
name: plan
description: Plans the work for one GitHub issue from the entry point that fits its kind, such as bug, feature, performance, refactor or CI, and writes a plan in fixed sections. Use before implementing an issue, or when asked to plan, scope or break down a bug fix, feature, refactor or other change.
---

# Plan

A plan says what changes, where, in which order, and how to prove it works. It changes no files; the next step is the `branch` skill.

## Steps

1. **Read** the issue and what it builds on:
   - the issue: `gh issue view <n> --json title,body,labels,milestone`
   - its milestone's targets: the `# Targets` section of `.milestone.description`, and the target the issue serves
   - the issues it links, such as blocked-by, sub-issues and Out of scope items: `gh issue view <n>`
   - the `AGENTS.md` of every project it touches; find the projects with the `moon-query` skill

   If the issue fails the refining checklist of the `issue` skill, refine it with the `issue` skill first.

2. **Pick the entry point** by kind. Take the first row that matches and read its reference:

   | Kind        | When                                                       | Read                                                   |
   | ----------- | ---------------------------------------------------------- | ------------------------------------------------------ |
   | bug         | the `bug` label                                            | [references/bug.md](references/bug.md)                 |
   | performance | the issue is about speed, memory or size                   | [references/performance.md](references/performance.md) |
   | refactor    | a chore that restructures code without changing behaviour  | [references/refactor.md](references/refactor.md)       |
   | CI          | the `ci` label, workflows, repository settings or releases | [references/ci.md](references/ci.md)                   |
   | feature     | everything else, including agent files and tooling         | [references/feature.md](references/feature.md)         |

   Each reference says what to find out first, which plan sections the kind adds, and how to verify.

3. **Find out** what the reference asks for first, such as a reproduction or a baseline. Read the code the issue touches.

4. **Write the plan** in the [output](#output) format. Put the kind's own sections under Approach.

5. **Check** that every acceptance criterion of the issue maps to a step under Order and a check under Verification. Add what is missing, or list it under Open questions.

## Output

The plan, in these sections and this order. No text before or after it.

```markdown
## Plan: #<n> <issue title>

**Kind:** <bug|feature|performance|refactor|CI>

### Context

The issue, the milestone target it serves, and why it matters now.

### Approach

How the change works, in a few bullets, plus the sections the kind's reference adds.

### Files

- `<path>`: what changes there.

### Order

1. `<type>: <commit header>`: what the commit holds.

### Verification

- `<command>` or `<test>`: what it proves, and the acceptance criterion it covers.

### Open questions

- <question>, and the assumption the plan makes until it is answered. "None" if there are none.
```

- **Files:** every path the plan creates, changes or deletes. Mark new files with "(new)".
- **Order:** one line per commit, in the order of the `commit` skill: refactors before the change that needs them, a test together with the fix it guards. Headers follow the `conventional-commits` skill.
- **Verification:** the `verify` skill runs at the end in every plan. List only what it doesn't cover, such as new tests, a manual check or a command that shows the result.

## Rules

- Stay inside the issue's scope. Work that doesn't fit goes to a new issue with the `issue` skill, named under Open questions.
- One issue, one branch, one PR. If the plan needs more than one PR, split the issue with the `issue` skill instead.
- Name real paths, commands and skills; check that they exist.
- Follow the `writing-style` skill.
