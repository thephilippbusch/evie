---
name: issue
description: Writes and refines evie's GitHub issues with the issue template, labels, milestone, sub-issues and blocked-by links. Use when turning a milestone's targets into issues, refining or splitting a rough issue before implementation, or when asked to create, file or label an issue.
---

# Writing and refining issues

`docs/templates/github-issue.md` is the source of truth for an issue's title and body; `docs/templates/github-milestone.md` for a milestone's. Read both before you write. The GitHub UI templates in `.github/ISSUE_TEMPLATE/` mirror the issue template, so issues from the UI and from agents share one structure.

| Read                                                   | When                                                       |
| ------------------------------------------------------ | ---------------------------------------------------------- |
| [references/kinds.md](references/kinds.md)             | writing any section: what it holds per kind of work        |
| [references/labels.md](references/labels.md)           | picking labels, or adding a new label                      |
| [references/gh-commands.md](references/gh-commands.md) | running `gh`: read, create, edit, sub-issues, dependencies |

## Turn a milestone's targets into issues

1. Read the milestone's description and its existing issues (see [gh-commands.md](references/gh-commands.md#read)).
2. For each target, plan the issues that together meet its "done when". One issue is one PR's worth of work; skip what an existing issue already covers.
3. Write each issue as a rough outline: title, Description, Scope and Acceptance Criteria. Technical Details may stay short until refinement.
4. Find the order: which issue needs another one done first. Record only those direct blocked-by links.
5. Show the drafts to the user before you create them, unless you run unattended.
6. Create each issue in the milestone, with its labels, in the `Evie Planner App` project; then add the blocked-by links. Assign it only to whoever starts on it now.

## Refine an issue before implementation

1. Read the issue, its milestone, the issues it links, and the code it touches.
2. Rewrite each section for its kind of work, with [references/kinds.md](references/kinds.md). Keep the template's headings and order; add no sections.
3. Move every technical detail out of the Acceptance Criteria into Technical Details.
4. For every Out of scope item, link the issue that owns it, e.g. `Review skills (#15)`. Items no issue owns stay unlinked.
5. Check labels, milestone and blocked-by links against the new scope.
6. If the issue is still more than one PR's worth, split it into sub-issues (see below).
7. Update the issue with `gh issue edit <n> --body-file -`.

## Sub-issues

Create sub-issues only while planning the implementation of a single issue, never to group a milestone's issues: the milestone does that. Each sub-issue uses the same template, milestone and labels, and gets its own PR. The parent keeps the overall Acceptance Criteria; close it when its last sub-issue closes.

## Rules

- Title: the intent in sentence case, starting with a verb, e.g. `Add the issue skill`. No type prefix such as `Chore |`; the kind is a label.
- Description: at most two sentences.
- Acceptance Criteria: domain-level and testable by a person, without file names or code, unless they are part of the domain (a CI check, a moon task).
- Refer to other work by issue number (`#15`) or milestone title only, never by names from notes outside GitHub.
- Only direct blocked-by links; GitHub shows the chain.
- When `docs/templates/github-issue.md` or a kind's hints in [references/kinds.md](references/kinds.md) change, update `.github/ISSUE_TEMPLATE/` in the same change.
- Follow the `writing-style` skill (once it exists).

## Output

The issue numbers and URLs you created or changed, each with its milestone, labels and blocked-by links.
