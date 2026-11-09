# Types

## `type` over `interface`

`type` for object shapes and unions. `interface` only when another declaration must extend or merge it, e.g. augmenting a library's types.

```ts
type Activity = {
  readonly id: string;
  readonly title: string;
};
```

## Literal unions, not `enum`

```ts
type BookingStatus = "pending" | "confirmed" | "cancelled";
// not: enum BookingStatus { Pending, Confirmed, Cancelled }
```

## Discriminated unions on `_tag`

Model variants as a union with a `_tag` field, the discriminant Effect uses for `Data.TaggedEnum` and `Data.TaggedError`. Not as one type with optional fields.

```ts
type Host =
  | { readonly _tag: "Person"; readonly userId: string }
  | { readonly _tag: "Household"; readonly householdId: string; readonly memberIds: ReadonlyArray<string> };

// not:
// type Host = { userId?: string; householdId?: string; memberIds?: string[] };
```

## Exhaustive `switch`

End every `switch` over a union with a `default` that only compiles when every case is handled. Adding a variant then fails the build where it needs handling.

```ts
function canCancel(status: BookingStatus): boolean {
  switch (status) {
    case "pending":
    case "confirmed":
      return true;
    case "cancelled":
      return false;
    default:
      return status satisfies never;
  }
}
```

## Decode external data, never cast it

Request bodies, responses, storage, URL params and environment values enter as `unknown`. Decode them with a `Schema` where they enter; inside, trust the types. How to use `Schema` is part of the `effect-service` skill (once it exists).

```ts
import { Schema } from "effect";

const Booking = Schema.Struct({
  id: Schema.String,
  guests: Schema.Number,
});
type Booking = typeof Booking.Type;

// wrong: claims a type nobody checked
const cast = (await response.json()) as Booking;

// right: an Effect that fails with a ParseError on bad data
const decoded = Schema.decodeUnknown(Booking)(await response.json());
```

`as const` is fine; it narrows a literal you wrote yourself.

## Immutable data

`readonly` fields and `ReadonlyArray` in hand-written types. Return a new value instead of changing an argument.

```ts
type Place = { readonly name: string; readonly guestIds: ReadonlyArray<string> };

function addGuest(place: Place, guestId: string): Place {
  return { ...place, guestIds: [...place.guestIds, guestId] };
}

// not: place.guestIds.push(guestId);
```
