# Testing

## Unit tests in the module

At the bottom of the file under test, so they can reach private items.

```rust
// src/booking/capacity.rs
pub(crate) fn free_places(capacity: u32, booked: u32) -> u32 {
    capacity.saturating_sub(booked)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_places_never_goes_below_zero() {
        assert_eq!(free_places(2, 3), 0);
    }
}
```

## Integration tests in `tests/`

One file per feature, `tests/<feature>.rs`, using only the crate's public API. Shared helpers live in `tests/common/mod.rs`: a `tests/common.rs` would be compiled as a test crate of its own.

```rust
// tests/common/mod.rs
use backend::booking::BookingService;

pub fn booking_service() -> BookingService {
    BookingService::in_memory()
}
```

```rust
// tests/booking.rs
mod common;

use backend::booking::BookingError;
use uuid::Uuid;

#[tokio::test]
async fn book_rejects_a_full_place() {
    let service = common::booking_service();
    let place_id = service.add_place(1).await;
    service.book(place_id, Uuid::new_v4()).await.unwrap();

    let err = service.book(place_id, Uuid::new_v4()).await.unwrap_err();

    assert!(matches!(err, BookingError::PlaceFull { .. }));
}
```

## Async and time

`#[tokio::test]` for async tests, never a hand-built runtime. For code that waits, pause time instead of sleeping: the clock jumps ahead when every task is idle. Needs Tokio's `test-util` feature in `[dev-dependencies]`.

```rust
#[tokio::test(start_paused = true)]
async fn reminder_fires_after_one_hour() {
    let fired = tokio::spawn(wait_for_reminder(Duration::from_secs(3600)));
    tokio::time::advance(Duration::from_secs(3600)).await;
    assert!(fired.await.unwrap());
}
```

## Naming and assertions

- Name the behaviour, without a `test_` prefix: `free_places_never_goes_below_zero`, not `test_free_places`.
- One behaviour per test; arrange, act and assert separated by blank lines.
- `unwrap` and `expect` are fine in tests.
- Match error variants, not messages: `assert!(matches!(err, BookingError::PlaceFull { .. }))`, not `assert_eq!(err.to_string(), "place … is full")`.

## Running tests

Through moon, so caching and task dependencies apply:

```sh
moon run backend:test                                # one project
moon run :test                                       # every project with a test task
moon run backend:test -- book_rejects_a_full_place   # one test, by name filter
```

Use the `moon-run` skill (once it exists) for the details.
