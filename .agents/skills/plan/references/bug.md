# Planning a bug fix

Something doesn't work as documented or as an earlier issue promised. No plan without a reproduction.

## Find out first

1. Reproduce the bug with the `debug` skill, steps 1 and 2: the expected and the actual behaviour, the moon project that owns the code, and a failing test or a minimal reproduction.
2. Check whether it used to work. If so, find the commit or PR that broke it; the `debug` skill shows how with `git bisect`.
3. Read the reproduction steps, logs and suspected cause in the issue's Technical Details. Write down where your findings differ.

If you can't reproduce it, stop planning. Say what you tried under Open questions and ask for the missing environment or steps.

## Plan sections

Add these under Approach:

- **Reproduction:** the command, test or steps that fail now.
- **Suspected cause:** the file and line, and why it fails there. Mark it "confirmed" once a test or an experiment shows it.
- **Regression test:** the project, the test file and the test name, at the lowest level that shows the cause. Name it after the behaviour, not the issue number.
- **Fix:** the smallest change that removes the cause. No catch-all error handling, retries or longer timeouts in its place.
- **Neighbouring cases:** related inputs and paths that must keep working, from the issue's Acceptance Criteria and from reading the code around the fix. Each gets a test if none covers it yet.

Under Order, the regression test and the fix are one commit, a `fix:` commit. Unrelated cleanups go in their own commit or a new issue.

## Verify

- The regression test fails before the fix, for the bug's reason, and passes after it.
- The project's whole `test` task passes: `moon run <project>:test`.
- The reproduction no longer fails.
- The `debug` skill's output goes in the PR description.
