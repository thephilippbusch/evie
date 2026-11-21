# Errors

A failure the caller can act on is a `Data.TaggedError` in the error channel of the `Effect`. A bug is a defect. Nothing throws.

## Define

One class per failure, named for what went wrong. Its fields hold what a caller needs to react, and the `cause` when there is one.

```ts
import { Data, type ParseResult } from "effect";

/** The place has no room for the guests. */
export class PlaceFull extends Data.TaggedError("PlaceFull")<{
  readonly placeId: string;
  readonly freePlaces: number;
}> {}

/** The booking request doesn't decode. */
export class InvalidBooking extends Data.TaggedError("InvalidBooking")<{
  readonly cause: ParseResult.ParseError;
}> {}
```

- The tag equals the class name; both read as a noun phrase: `PlaceFull`, `EventNotFound`, `BackendUnavailable`.
- Errors that only one service raises live in that service's file. Errors that several features share, such as `BackendUnavailable`, live in their own file in `src/lib/`.

## Failures and defects

| Kind    | Example                                                 | How                                             |
| ------- | ------------------------------------------------------- | ----------------------------------------------- |
| failure | the backend is down, a place is full, bad input         | `Effect.fail(new PlaceFull({ … }))`, typed      |
| defect  | a response that breaks the contract, a broken invariant | `Effect.die(…)`, or `Effect.orDie` on an effect |

Wrap promise-based code with `Effect.tryPromise({ try, catch })`, so a rejection becomes a typed failure:

```ts
const fetchHolidays = (year: number) =>
  Effect.tryPromise({
    try: (signal) => fetch(`https://holidays.example/${year}`, { signal }).then((res) => res.json()),
    catch: (cause) => new HolidaysUnavailable({ cause }),
  });
```

`try` gets an `AbortSignal` that fires when the effect is interrupted; pass it on.

## Handle

Handle a failure where the code knows what to do instead. Narrow by tag:

```ts
const freePlaceCount = bookings.freePlaces(placeId).pipe(
  Effect.catchTag("PlaceNotFound", () => Effect.succeed(0)),
);

const view = loadEvent(eventId).pipe(
  Effect.catchTags({
    EventNotFound: () => Effect.succeed({ _tag: "NotFound" } as const),
    BackendUnavailable: () => Effect.succeed({ _tag: "Unavailable" } as const),
  }),
);
```

A server function handles every failure the page shows to people and returns it as a variant of its result (see [runtime.md](runtime.md)). What's left is unexpected: log it once at that boundary with `Effect.tapErrorCause(Effect.logError)` and let it reject.

## Rules

- Never `throw` in Effect code, and never `catch` with `try`/`catch`; use `Effect.try` or `Effect.tryPromise`.
- Never put a message for people into an error. The page picks the text from the tag, in German and English.
- Don't widen errors to `unknown` or `Error`; keep the union of tags in the type.
