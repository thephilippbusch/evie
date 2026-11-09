# PR descriptions

Follow `.github/pull_request_template.md`. PRs are squash-merged and the body becomes the commit body on `main`, so it follows the commit rules too. The title format belongs to the `conventional-commits` skill.

## Description: one short paragraph

Before:

```markdown
## Description

This PR is a really big step forward for our developer experience! In this PR I
have added a new skill which is called verify. The idea behind it is that agents
can use it to check their work. I think this will help a lot. Let me know what
you think!
```

After:

```markdown
## Description

Adds the `verify` skill: it checks the formatting, lint, build and tests of what
a change affects and reports only the failures. `ship` and the verifier subagent
build on it.
```

## Acceptance Criteria: copied, then ticked

Before:

```markdown
## Acceptance Criteria

Everything should work now, I tested it locally.
```

After:

```markdown
## Acceptance Criteria

- [x] An agent checks only what its change affects and reports failures clearly.
- [x] Docs, commits and PRs written by agents read the same way.
```

## Say what is left

When the PR doesn't meet a criterion, leave it unticked and say why in one line under the list, with the issue that takes it over:

```markdown
- [ ] Every app image is published on release.

Image publishing needs the Docker build from #19; tracked in #42.
```
