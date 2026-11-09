# Debugging Rust

Tools for the Rust backend (Tokio, Tonic, SeaORM) and Rust crates. Arguments after the first `--` in `moon run <project>:test -- …` go to `cargo test`; flags after a second `--` go to the test binary. Check the project's `moon.yml` if its `test` task runs something else, such as `cargo nextest`.

moon caches a task's result by its inputs and replays it when they haven't changed. Environment variables such as `RUST_LOG` aren't inputs, so add `--force` (`moon run --force <project>:test …`) whenever you rerun unchanged code.

## Run one test

| Goal                          | Command                                                     |
| ----------------------------- | ----------------------------------------------------------- |
| Tests whose name contains `x` | `moon run <project>:test -- x`                              |
| Exactly one test              | `moon run <project>:test -- module::tests::name -- --exact` |
| See `println!` and `dbg!`     | `moon run <project>:test -- name -- --nocapture`            |
| Rule out shared state         | `moon run <project>:test -- name -- --test-threads=1`       |
| Optimisation-dependent bugs   | `moon run <project>:test -- --release name`                 |

For a flaky test, run it in a loop until it fails and count the runs:

```sh
for i in $(seq 50); do moon run --force <project>:test -- name || break; done
```

## Backtraces and panics

- `RUST_BACKTRACE=1 moon run --force <project>:test -- name` prints a backtrace on panic; `RUST_BACKTRACE=full` adds every frame.
- Replace an `unwrap()` or `expect()` you suspect with a `match` that prints the error, then put it back or handle the error properly in the fix.
- `dbg!(expr)` prints the expression, its value and the line, and returns the value. Remove it before committing.

## Logs and tracing

The backend logs through `tracing`. `RUST_LOG` filters by level and by target, the module path starting with the crate name:

```sh
RUST_LOG=debug moon run --force backend:test -- name -- --nocapture
RUST_LOG=<crate>=trace,h2=info,tower=info moon run --force backend:test -- name -- --nocapture
```

`RUST_LOG` only works if a subscriber with an `EnvFilter` is installed. Tests don't install one by themselves; add this at the start of the test while debugging (needs `tracing-subscriber` with the `env-filter` feature):

```rust
let _ = tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .with_test_writer()
    .try_init();
```

`#[tracing::instrument]` on a function logs its arguments and opens a span, so nested log lines show where they came from.

## Async and Tokio

- Reproduce in a `#[tokio::test]`. Use `#[tokio::test(flavor = "multi_thread")]` if the bug only shows with real parallelism.
- Turn a hang into a failure: wrap the suspect future in `tokio::time::timeout(Duration::from_secs(5), fut).await`.
- A hang is often a lock held across `.await`, or a channel whose sender is never dropped. Look for both before anything else.
- A panic in a spawned task doesn't fail the test unless the `JoinHandle` is awaited; await it and check the `Result`.

## gRPC (Tonic)

- Log the `tonic::Status` in full: `status.code()`, `status.message()` and `status.metadata()`.
- Reproduce in an integration test that starts the service in-process and calls it through the generated client. That rules out the BFF.
- Against the local stack (`docker compose up`), `grpcurl` can call a method directly if the server has reflection enabled.

## Database (SeaORM)

- Print the SQL a query builds: `Entity::find().filter(…).build(DbBackend::Postgres).to_string()`.
- Log every statement at run time: enable `ConnectOptions::sqlx_logging(true)` and run with `RUST_LOG=sqlx::query=info`.
- Run the printed SQL against the local PostgreSQL to see whether the query or the mapping is wrong.
- Check that the test database has the current migrations applied (`moon run :migrate`).
