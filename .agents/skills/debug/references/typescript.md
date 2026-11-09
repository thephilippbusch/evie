# Debugging TypeScript

Tools for the SolidStart frontend and BFF on bun, written in Effect. Arguments after `--` in `moon run <project>:test -- …` go to the test runner. The commands below are for `bun test`; check the project's `moon.yml` if its `test` task runs something else.

moon caches a task's result by its inputs and replays it when they haven't changed. Environment variables aren't inputs, so add `--force` (`moon run --force <project>:test …`) whenever you rerun unchanged code.

## Run one test

| Goal                              | Command                                                        |
| --------------------------------- | -------------------------------------------------------------- |
| Tests whose name matches a regex  | `moon run <project>:test -- -t "joins words"`                  |
| One file                          | `moon run <project>:test -- src/lib/slug.test.ts`              |
| Stop at the first failure         | `moon run <project>:test -- --bail`                            |
| Flaky test: run each test n times | `moon run --force <project>:test -- -t "name" --rerun-each 50` |
| Slow or hanging test              | `moon run <project>:test -- -t "name" --timeout 30000`         |

`test.only` also narrows a run; remove it before committing.

## Step through with a debugger

```sh
bun test --inspect-brk src/lib/slug.test.ts
```

Run it in the project directory. bun prints a `debug.bun.sh` URL; open it, set breakpoints and continue. `--inspect-wait` waits for the debugger without stopping on the first line. A `debugger;` statement in the code stops there. The same flags work for a server: `bun --inspect <entry>`.

## Effect

The names below are Effect 3 APIs, matching the `Context.Tag` rule in the root `AGENTS.md`.

- A failed Effect carries a `Cause`. Print all of it, not just the message: `Effect.tapErrorCause((cause) => Effect.logError(Cause.pretty(cause)))`.
- In a test, run with `Effect.runPromiseExit(program)` and assert on the `Exit`, so a failure shows its tagged error instead of a rejected promise.
- Show debug logs for one program: `program.pipe(Logger.withMinimumLogLevel(LogLevel.Debug))`. Add `Effect.logDebug` or `Effect.tap` at the step you suspect.
- Name steps with `Effect.withSpan("name")` or `Effect.fn("name")`, so logs and errors show where they came from.
- Isolate a dependency by providing a test `Layer` for its `Context.Tag`, e.g. `Layer.succeed(BackendClient, fake)`. If the bug disappears, it is in the real service or in what it returns.
- A missing `Config` value fails with a `ConfigError` that names the key; check the variable is set, but never read `.env` files.

## BFF and backend

- Decide which side is wrong first: log the request a server function sends and the response it gets back. If the backend's response is already wrong, continue with [rust.md](rust.md).
- `BUN_CONFIG_VERBOSE_FETCH=curl` prints every `fetch` as a `curl` command. The output can contain cookies and tokens: never paste it into a commit, issue or PR.

## Browser and Solid

- Reactivity: a value that doesn't update is often a signal read outside a tracking scope, or destructured props. Log inside `createEffect(() => console.log(value()))` to see when it changes.
- Hydration errors: compare the server-rendered HTML (view source) with what the client renders; the first differing element is the place to look.
- Check the browser console and network tab before reading code.
