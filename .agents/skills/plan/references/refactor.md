# Planning a refactor

A chore that restructures code without changing its behaviour. If anything a user, a caller or a test sees changes, it isn't a refactor.

## Find out first

1. Run the tests of the affected projects: `moon run <project>:test`. They pass before you start. If they fail, fix that first or file a bug with the `issue` skill.
2. Check that tests cover the code you move. Where they don't, plan characterisation tests that pin the current behaviour, including its odd cases.
3. Find every caller of the code, with `git grep` or the editor, so no path is left out.

## Plan sections

Add these under Approach:

- **Before and after:** the structure now and the structure you want, e.g. module layout, types or function boundaries.
- **Steps:** small steps, each one leaving the code building and the tests passing: move, rename, extract, inline, then delete the old path.
- **Kept out:** features, fixes and behaviour changes you notice on the way. Each one goes to a new issue with the `issue` skill.

Under Order, one `refactor:` commit per step. Characterisation tests come first, in a `test:` commit.

## Verify

- Every commit passes the `verify` skill at the uncommitted scope.
- No test changes, except the new characterisation tests and renamed imports or paths.
- Public interfaces stay the same: gRPC services, server functions, moon tasks. If one must change, it isn't a refactor; plan it as a feature.
