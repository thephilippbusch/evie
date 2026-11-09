# What each section holds, per kind of work

Every issue uses the headings of `docs/templates/github-issue.md`, in that order. The kind of work decides what goes under them and which kind label the issue gets (see [labels.md](labels.md)). The matching GitHub UI template in `.github/ISSUE_TEMPLATE/` carries the same hints.

| Kind          | Label           | Example title                                             |
| ------------- | --------------- | --------------------------------------------------------- |
| Feature       | `enhancement`   | `Create and edit draft events`                            |
| Bug           | `bug`           | `Keep booked nights when an event is rescheduled`         |
| Chore         | `chore`         | `Enforce Conventional Commit PR titles and squash merges` |
| Documentation | `documentation` | `Document the release process`                            |

## All kinds

- **Description:** at most two sentences: who gets what, and why.
- **In scope:** short bullets, each one a capability, rule or deliverable.
- **Out of scope:** what a reader could expect here but won't get. Link the issue that owns it: `Inviting people (#71, #72)`. Leave an item unlinked only when no issue owns it.
- **Acceptance Criteria:** one sentence each, true or false for a person who tries it. Name roles, not code: "An organizer can …", "A PR with an invalid title can't be merged."
- **Technical Details:** anything an agent needs to start: files, services, data model, settings, links to docs, dependencies (`Depends on #20`). Short bullets, no prose.

## Feature

New or changed behaviour for people using evie.

- **Description:** the user and what they can now do, e.g. "Organizers can create an event as a draft and edit it before publishing."
- **In scope:** fields, rules, visibility and states the feature adds.
- **Acceptance Criteria:** what a user sees or can do, per role. Include the permission edge cases: who must not see or do it.
- **Technical Details:** affected apps, gRPC services, tables and migrations, BFF server functions. Note German and English texts if the issue adds user-facing text. From v0.5 on, say how acting for household members and managed profiles works, and how the actor is recorded.

## Bug

Something that doesn't work as documented or as an earlier issue promised.

- **Title:** the behaviour you want back, not the symptom: `Keep booked nights when an event is rescheduled`.
- **Description:** what happens and what should happen instead.
- **In scope:** the fix and a regression test.
- **Out of scope:** related improvements, with their issues.
- **Acceptance Criteria:** the expected behaviour in the reported case, plus any neighbouring cases that must keep working.
- **Technical Details:** steps to reproduce, environment and version, logs or error messages, the suspected cause, and the issue or PR that introduced it if known. The template has no reproduction section; this is where it goes.

## Chore

Tooling, CI, agent files, refactoring and setup: no change in behaviour for people using evie. Most v0.x setup work is a chore.

- **Description:** what contributors or agents get, e.g. "Teach agents to write and refine GitHub issues the way this repo does."
- **In scope:** the tools, skills, settings or workflows the issue adds.
- **Acceptance Criteria:** what a contributor or agent can now do, or what CI now prevents. Tools such as moon or GitHub checks may appear when they are the subject; file paths may not.
- **Technical Details:** file paths, config keys, candidate tools and actions, templates to follow.

## Documentation

Docs for people: ADRs, templates, guides and checklists in `docs/`. Agent files (`AGENTS.md`, skills) are chores with the `agents` label.

- **Acceptance Criteria:** what a reader can find out or do with the docs alone, e.g. "Adding a third app needs only the checklist."
- **Technical Details:** where the docs live and which skills or `AGENTS.md` files must link them.

## Refining checklist

An issue is ready to implement when:

- [ ] Every section follows its kind above, and the headings match the template.
- [ ] Each acceptance criterion is domain-level and can be checked by a person.
- [ ] Every owned Out of scope item links its issue.
- [ ] It fits one PR, or it has sub-issues that each do.
- [ ] It has a milestone, one kind label, any area labels, and its direct blocked-by links.
