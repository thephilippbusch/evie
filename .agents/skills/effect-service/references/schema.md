# Schema

Data that comes from outside the server code enters as `unknown` and is decoded with a `Schema` at the point where it enters. Inside, trust the types.

| Enters through                   | Decode                                                 |
| -------------------------------- | ------------------------------------------------------ |
| server function arguments        | in the server function, before anything else uses them |
| URL and search params, form data | in the service method that takes them, as `unknown`    |
| backend responses                | in the client service, into the feature's own type     |
| third-party HTTP APIs, storage   | in the service that calls them                         |

Server function arguments come from the browser, so anyone can send anything: decode them even when the caller is your own component.

## Define

The schema is a value; its type has the same name.

```ts
import { Schema } from "effect";

const NewBooking = Schema.Struct({
  placeId: Schema.UUID,
  guests: Schema.Int.pipe(Schema.between(1, 8)),
  note: Schema.optional(Schema.Trim.pipe(Schema.maxLength(500))),
});
type NewBooking = typeof NewBooking.Type;
```

| Need                            | Schema                                        |
| ------------------------------- | --------------------------------------------- |
| ID                              | `Schema.UUID`                                 |
| text a person types             | `Schema.Trim`, `Schema.NonEmptyTrimmedString` |
| whole number                    | `Schema.Int`, with `Schema.between(min, max)` |
| number from a URL or form field | `Schema.NumberFromString`                     |
| date from an ISO string         | `Schema.Date`                                 |
| one of a few values             | `Schema.Literal("pending", "confirmed")`      |
| optional field                  | `Schema.optional(…)`                          |
| list                            | `Schema.Array(…)`                             |

## Decode

`Schema.decodeUnknown(schema)(input)` is an `Effect` that fails with a `ParseError`. Map it to the feature's tagged error where the caller can act on it:

```ts
const parseBooking = (input: unknown) =>
  Schema.decodeUnknown(NewBooking)(input).pipe(
    Effect.mapError((cause) => new InvalidBooking({ cause })),
  );
```

A `ParseError` on data the backend sent means the BFF and the backend disagree: a bug, not a case to handle. Turn it into a defect with `Effect.catchTag("ParseError", Effect.die)`, as the client services of the `grpc-rpc` skill do.

## Rules

- Never `as` external data into a type; decode it.
- Decode once, at the edge. Don't decode the same value again further in.
- Return the feature's own type from a service, not the protobuf message or the raw response.
- `Schema.Struct` drops unknown fields when it decodes, so the result holds only what the schema names.
