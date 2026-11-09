---
name: writing-style
description: Sets evie's plain, concise style for English prose in docs, code comments, commit bodies, PR descriptions and issues, with before and after examples. Use when writing or reviewing any of them, or when asked to shorten, simplify or reword text.
---

# Writing style

Write for a reader who is busy and new to the change. The root `AGENTS.md` and the `authoring-skills` skill show the style.

| Read                                                       | When you write                                 |
| ---------------------------------------------------------- | ---------------------------------------------- |
| [references/docs.md](references/docs.md)                   | `AGENTS.md`, skills, ADRs, READMEs, other docs |
| [references/comments.md](references/comments.md)           | code comments and doc comments                 |
| [references/commits.md](references/commits.md)             | a commit body                                  |
| [references/pull-requests.md](references/pull-requests.md) | a PR description                               |
| [references/issues.md](references/issues.md)               | an issue or a milestone description            |

## Rules

- **Outcome first.** Start with what changes or what the reader must do. Reasons and detail come after.
- **Short sentences.** One idea each. Split a sentence at "and", "which" or a semicolon when it carries two.
- **Structure over prose.** Use a table for anything with columns, bullets for rules and lists, numbered steps for an order.
- **Plain words.** Pick the common word: "use", not "leverage"; "start", not "kick off"; "check", not "validate the correctness of".
- **No filler.** Cut words that add nothing: "really", "actually", "real", "genuine", "honest", "simply", "just", "basically", "very", "in order to".
- **No hedging.** Say what is true: "X fails when Y", not "X might possibly fail in some cases". If you don't know, say what you don't know.
- **No meta talk.** Don't announce text ("In this section we…", "Note that…", "It's worth mentioning…"). Write the thing.
- **No invented jargon.** Use the names the code, moon and the stack already use. Define a new term once, where it first appears.
- **Link, don't repeat.** Say a thing once, in the file closest to where it applies, and link to it.
- **Concrete over abstract.** Name the file, the command, the role, the number.
- **Active voice, present tense.** "The hook formats the files", not "The files will be formatted".
- **No history in docs.** Docs describe the current state. History belongs in commits and PRs.

## Per kind of text

- **Docs:** imperative for instructions, present tense for facts. Mark what doesn't exist yet as _planned_.
- **Code comments:** say why, not what. The code shows what it does; a comment explains a choice, a constraint or a trap.
- **Commit bodies:** what changed and why, in one to three sentences. The title format belongs to the `conventional-commits` skill.
- **PR descriptions:** follow `.github/pull_request_template.md`. The body becomes the squash commit body, so it follows the commit rules. The title format belongs to the `conventional-commits` skill.
- **Issues:** follow the `issue` skill for structure; this skill covers the sentences.

## Scope

This skill covers English prose in the repository and on GitHub. Every user-facing text in the app exists in German and English, as the root `AGENTS.md` requires; this skill doesn't cover UI text.
