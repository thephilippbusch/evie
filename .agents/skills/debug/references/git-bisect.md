# Finding a regression with git bisect

Use it when something used to work: `git bisect` binary-searches the history for the first commit where the reproduction fails. PRs are squash-merged, so on `main` every commit is one PR, and the result names the PR that broke it.

## Steps

1. Find a good commit where the bug doesn't happen, e.g. an older release tag (`git tag --list 'v*'`) or a commit from before the feature changed. The bad commit is usually `HEAD`.
2. Write the reproduction as a script outside the repo, e.g. `$TMPDIR/repro.sh`, so it exists in every commit bisect checks out. Exit codes: `0` good, `1` to `124` bad, `125` skip (the commit can't be tested, e.g. it doesn't build).

   ```sh
   #!/bin/sh
   # Skip commits that don't build; fail on the bug.
   moon run --force backend:build || exit 125
   moon run --force backend:test -- slug_keeps_umlauts
   ```

   A new regression test doesn't exist in old commits. Copy it in from the script, or reproduce through something old commits already have, such as a command or an HTTP call.

3. Bisect in a separate worktree, so your own work stays untouched:

   ```sh
   git worktree add "$TMPDIR/evie-bisect" HEAD
   cd "$TMPDIR/evie-bisect"
   git bisect start --first-parent HEAD <good-commit>
   git bisect run sh "$TMPDIR/repro.sh"
   ```

4. Read the first bad commit it prints, and its diff: that's where to isolate. Then clean up:

   ```sh
   git bisect reset
   cd - && git worktree remove "$TMPDIR/evie-bisect"
   ```

## Tips

- Check the script by hand first: it must exit `0` on the good commit and non-zero on `HEAD`, or bisect gives a wrong answer.
- `--force` makes moon run the task in every commit instead of replaying a cached result.
- A new worktree has no installed dependencies or build output, so the first step installs and builds everything and takes longer.
- If the reproduction is manual, run `git bisect good` or `git bisect bad` yourself after each checkout.
- `git bisect log` shows the steps so far; `git bisect skip` skips one commit.
