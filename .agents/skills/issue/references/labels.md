# Labels

This file owns evie's label set. Check the live list with `gh label list`; if it differs from the tables below, fix whichever is wrong.

## Kind labels

Every open issue has exactly one. The kinds are explained in [kinds.md](kinds.md).

| Label           | Use for                                                               |
| --------------- | --------------------------------------------------------------------- |
| `enhancement`   | New or changed behaviour for people using evie                        |
| `bug`           | Something that doesn't work as documented or promised                 |
| `chore`         | Tooling, CI, agent files, refactoring and setup; most v0.x setup work |
| `documentation` | Docs for people in `docs/`                                            |

## Area labels

Zero or more, next to the kind label.

| Label    | Description on GitHub                              |
| -------- | -------------------------------------------------- |
| `agents` | Agent skills, subagents and AGENTS.md files        |
| `ci`     | GitHub workflows, repository settings and releases |

## Triage labels

`duplicate` and `invalid` are GitHub's defaults. Use them only when you close an issue, with a comment that links the duplicate or says why.

## Labels and commit types

A label describes the issue, not the release. The PR title's Conventional Commit type decides the release; see the `conventional-commits` skill (once it exists).

## Add a label

1. Check that no existing label fits.
2. Create it with a description and colour:

   ```sh
   gh label create <name> --description "<what it marks>" --color <hex without #>
   ```

3. Add it to the matching table above, in the same change.
4. If it is a kind label, add an issue template for it in `.github/ISSUE_TEMPLATE/` and a section in [kinds.md](kinds.md).
