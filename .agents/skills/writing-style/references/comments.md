# Code comments

A comment says why: a choice, a constraint, a trap or a link to the reason. The code already says what it does. Doc comments on public items say what the item is for and what the caller must know, such as errors and panics.

## Why, not what

Before:

```rust
// Loop over the bookings and add up the nights.
let nights: u32 = bookings.iter().map(|b| b.nights).sum();
```

After: no comment. The code says it.

Before:

```ts
// Set the timeout to 30 seconds.
const timeout = Duration.seconds(30);
```

After:

```ts
// The backend's slowest RPC, the occupancy export, takes up to 20s.
const timeout = Duration.seconds(30);
```

## Name the constraint

Before:

```yaml
options:
  # Don't cache this.
  cache: false
```

After:

```yaml
options:
  # Links may point to any file in the repo, which the inputs can't list
  # without hashing the whole repo.
  cache: false
```

## No history, no author notes

Before:

```ts
// Changed this after the bug last week, see Slack. Don't touch!!
const user = yield* Schema.decodeUnknown(User)(body);
```

After:

```ts
// Decode at the boundary: the body comes from the browser and is untrusted.
const user = yield* Schema.decodeUnknown(User)(body);
```

## TODOs name an issue

Before:

```rust
// TODO: fix this later
```

After:

```rust
// TODO(#42): retry once the outbox worker exists.
```
