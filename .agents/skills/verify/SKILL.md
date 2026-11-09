---
name: verify
description: Checks the formatting, lint, build and tests of what a change affects, through moon and dprint, and reports only the failures. Use when checking work before a commit or PR, or when asked to verify, check or test changes.
---

# Verify

Check only what the change affects, and change no files. Use the `moon-query` skill to see which projects and tasks are affected, and the `moon-run` skill for how moon picks the tasks it runs.

## Pick the scope

| Scope            | When                                | Checks                                                           |
| ---------------- | ----------------------------------- | ---------------------------------------------------------------- |
| Uncommitted work | before a commit                     | the changes since the last commit                                |
| Whole branch     | after the last commit               | every change on the branch compared with `main`, uncommitted too |
| CI               | before a PR, once the branch passes | the whole branch, with the tasks CI runs                         |

Uncommitted work:

```sh
moon check --all --affected --output-style buffer-only-failure
dprint check --dirty --allow-no-files --list-different
```

Whole branch:

```sh
moon check --all --affected remote --output-style buffer-only-failure
moon query changed-files --remote | jq -r '.files[]' | dprint check --stdin-files --allow-no-files --list-different
```

CI: the whole-branch dprint command, then:

```sh
moon ci --output-style buffer-only-failure
```

- `moon check` runs the affected `build` and `test` tasks. `lint` tasks are `test` tasks, so it runs them too.
- `moon ci` also runs the tasks CI runs on every PR, such as `root:validate-agent-files`.
- `--output-style buffer-only-failure` prints the output of failing tasks only.
- dprint checks Markdown, JSON and YAML. It lists the files that aren't formatted and skips deleted ones.
- `format` tasks, such as `root:format`, are `run` tasks that rewrite files, so verify never runs them. A project checks the formatting of its code in its `lint` task, e.g. with `cargo fmt --check`.

## Steps

1. Pick the scope.
2. Run its moon command. "No tasks affected" means there is nothing to check, which counts as a pass.
3. Run its dprint command.
4. Rerun each failing task alone for its full output: `moon run <project>:<task> --output-style stream`.
5. Report in the format below. Don't fix anything; the caller fixes and runs verify again.

If moon or dprint can't start, for example because a tool is missing or a `moon.yml` is invalid, report `blocked` with the error. `proto install` installs the pinned tools.

## Output

The status first: `pass`, `fail` or `blocked`, and the scope. Then one item per failing task or file, with the error lines that matter and the command to rerun it. The checks that passed fit on one line.

````markdown
**verify:** fail (whole branch)

- `web:lint`: rerun with `moon run web:lint --output-style stream`
  ```text
  src/routes/index.tsx:12:7 error 'event' is never reassigned. Use 'const' instead.
  ```
- `docs/templates/github-issue.md`: not formatted. Rerun with `dprint check docs/templates/github-issue.md`; fix with `dprint fmt docs/templates/github-issue.md`.

**Passed:** 5 tasks (`web:build`, `web:test`, `backend:build`, `backend:lint`, `backend:test`) and the dprint check.
````

- Keep the lines that name the file, the line and the cause. Drop progress output, test-runner stack frames and repeated errors.
- A pass is the status line and the **Passed** line.

## Rules

- Never pass `--force`, skip a failing task or change a test to get a pass.
- Never run `dprint fmt` or a `format` task while verifying.
