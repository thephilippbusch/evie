# Async

## One runtime

Tokio only. `#[tokio::main]` sits on `main` in `main.rs`; library code never builds a runtime or calls `block_on`.

## No blocking in async code

A blocking call stalls a runtime worker and every task scheduled on it. Use the async API, or move the work to the blocking pool.

```rust
// wrong: blocks the worker
let template = std::fs::read_to_string(&path)?;
std::thread::sleep(Duration::from_millis(100));

// right
let template = tokio::fs::read_to_string(&path).await?;
tokio::time::sleep(Duration::from_millis(100)).await;
```

CPU-heavy or sync-only work goes to `spawn_blocking`:

```rust
pub async fn export_calendar(events: Vec<Event>) -> Result<String, ExportError> {
    let ics = tokio::task::spawn_blocking(move || render_ics(&events)).await?;
    Ok(ics)
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("calendar export task failed")]
    Task(#[from] tokio::task::JoinError),
}
```

## Locks and `.await`

Prefer `std::sync::Mutex` for short critical sections, and drop the guard before the next `.await`. A block scope makes that visible:

```rust
pub async fn record_hit(&self, key: &str) -> u32 {
    let count = {
        let mut hits = self.hits.lock().expect("rate limiter lock poisoned");
        let count = hits.entry(key.to_owned()).or_default();
        *count += 1;
        *count
    }; // guard dropped here

    self.audit.write(key, count).await;
    count
}
```

Use `tokio::sync::Mutex` only when the lock must be held across an `.await`, e.g. to serialise access to one connection.

## Cancellation

When a client disconnects or its deadline passes, tonic drops the handler's future at whatever `.await` it is on. Code after that point never runs.

- Multi-step writes go in one database transaction, which rolls back if dropped.
- Work that must finish, such as sending a confirmation mail, goes to a spawned task or a queue.

```rust
// wrong: if the request is cancelled after the insert, no mail is sent
repo.insert(&booking).await?;
mailer.send_confirmation(&booking).await?;

// right: the mail no longer depends on the request staying alive
repo.insert(&booking).await?;
let mailer = mailer.clone();
tasks.spawn(async move { mailer.send_confirmation(&booking).await });
```

In `tokio::select!`, every branch that loses is dropped. Only race cancel-safe futures, such as `mpsc::Receiver::recv`, `CancellationToken::cancelled` or `tokio::time::sleep`. Not `AsyncReadExt::read_exact` or a future halfway through a write.

## Spawned tasks and shutdown

Keep every spawned task's `JoinHandle`, or put it in a `JoinSet`, and look at its result. Long-running tasks take a `CancellationToken` (from `tokio-util`) and stop when it fires.

```rust
use tokio::{sync::mpsc, task::JoinSet};
use tokio_util::sync::CancellationToken;

pub async fn run_reminders(shutdown: CancellationToken, mut jobs: mpsc::Receiver<Reminder>) {
    loop {
        tokio::select! {
            () = shutdown.cancelled() => break,
            job = jobs.recv() => match job {
                Some(job) => send_reminder(job).await,
                None => break,
            },
        }
    }
}

pub async fn run(shutdown: CancellationToken, jobs: mpsc::Receiver<Reminder>) {
    let mut tasks = JoinSet::new();
    tasks.spawn(run_reminders(shutdown.clone(), jobs));

    shutdown.cancelled().await;
    while let Some(result) = tasks.join_next().await {
        if let Err(err) = result {
            tracing::error!(error = ?err, "background task failed");
        }
    }
}
```

## Timeouts

Every call that leaves the process gets a deadline, so a slow dependency can't hold a request forever.

```rust
let reply = tokio::time::timeout(Duration::from_secs(5), mailer.send(message))
    .await
    .map_err(|_| MailError::Timeout)??;
```
