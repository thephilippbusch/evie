---
name: effect-service
description: Builds a service for evie's frontend server (BFF) in idiomatic Effect, with a Context.Tag, a Layer, Config, Schema, tagged errors, a server function and test layers. Use when adding server-side logic, settings or a server function to apps/frontend, or when asked to write Effect code.
---

# Effect service

Server and shared code in `apps/frontend` is idiomatic Effect 3. A service is a `Context.Tag` that says what it offers, a `Layer` that builds it, and tests that swap the layer. Server functions run effects with one app runtime and return plain data; components never import Effect.
For a client of a backend RPC, use the `grpc-e2e` skill, which builds on this one.

| Read                                           | When you                                                     |
| ---------------------------------------------- | ------------------------------------------------------------ |
| [references/schema.md](references/schema.md)   | take in data from outside: arguments, responses, params      |
| [references/errors.md](references/errors.md)   | define, handle or map a failure                              |
| [references/runtime.md](references/runtime.md) | add a layer to the runtime or write a server function        |
| [references/testing.md](references/testing.md) | test a service, or a service that depends on another service |

## First service only

1. In `apps/frontend/`: `bun add effect@^3`.
2. Create `src/lib/runtime.ts` with the app runtime, as [references/runtime.md](references/runtime.md) shows.

## Files

| File                              | Holds                                                                    |
| --------------------------------- | ------------------------------------------------------------------------ |
| `src/<feature>/<service>.ts`      | the tag, its live layer, its errors and the schemas of what it takes in  |
| `src/<feature>/<service>.test.ts` | tests with test layers                                                   |
| `src/<feature>/get-<thing>.ts`    | a server function, apart from the service so tests never load the router |
| `src/lib/runtime.ts`              | `appRuntime`: the one `ManagedRuntime` over every live layer             |

## Steps

1. Name the service for what it offers: `Invites`, `PingClient`. The file is kebab-case: `src/invites/invites.ts`.
2. Declare the tag with the service's shape. A method that fails or does I/O returns an `Effect` with its errors in the type; a pure one returns a plain value.
3. Write the live layer, named in camelCase with a `Live` suffix: `invitesLive`. Read settings with `Config` and other services with `yield*` inside `Effect.gen`.
4. Decode everything that comes from outside with `Schema` where it enters ([references/schema.md](references/schema.md)).
5. Give each failure a `Data.TaggedError` ([references/errors.md](references/errors.md)).
6. Add the live layer to `appLayer` in `src/lib/runtime.ts` ([references/runtime.md](references/runtime.md)).
7. Write the tests with test layers ([references/testing.md](references/testing.md)).
8. Document each new environment variable under Configuration in `apps/frontend/AGENTS.md`, in the table format `apps/backend/AGENTS.md` uses. Add the section if it doesn't exist yet.
9. Check with `moon run frontend:check frontend:test`. Use the `moon-run` skill.

## Example

```ts
// src/invites/invites.ts
import { Config, Context, Data, Effect, Layer, type ParseResult, Schema } from "effect";

const InviteParams = Schema.Struct({ eventId: Schema.UUID });
export type InviteParams = typeof InviteParams.Type;

/** The invite link's parameters are missing or malformed. */
export class InvalidInvite extends Data.TaggedError("InvalidInvite")<{
  readonly cause: ParseResult.ParseError;
}> {}

/** Builds invite links for an event and reads them back. */
export class Invites extends Context.Tag("Invites")<
  Invites,
  {
    readonly linkFor: (eventId: string) => URL;
    readonly parse: (params: unknown) => Effect.Effect<InviteParams, InvalidInvite>;
  }
>() {}

export const invitesLive = Layer.effect(
  Invites,
  Effect.gen(function* () {
    const appUrl = yield* Config.string("APP_URL");
    return Invites.of({
      linkFor: (eventId) => new URL(`/invites/${eventId}`, appUrl),
      parse: (params) =>
        Schema.decodeUnknown(InviteParams)(params).pipe(
          Effect.mapError((cause) => new InvalidInvite({ cause })),
        ),
    });
  }),
);
```

## Config

| Setting               | Read it with                                                                            |
| --------------------- | --------------------------------------------------------------------------------------- |
| text                  | `Config.string("NAME")`                                                                 |
| number or flag        | `Config.integer("NAME")`, `Config.boolean("NAME")`                                      |
| secret                | `Config.redacted("NAME")`; unwrap with `Redacted.value` only where it is used           |
| optional with default | `.pipe(Config.withDefault(value))`, only when the default is right in every environment |

A missing or invalid variable fails the layer, and with it the server function, with a `ConfigError` that names the variable.

## Rules

- Tags and errors are classes, because Effect declares them that way; everything else follows the `typescript-conventions` skill. `Effect.gen` takes a `function*` expression, since arrow functions can't be generators.
- Never read `process.env` and never `throw`. Effects run only through `appRuntime`, in server function bodies; outside tests, never call `Effect.runPromise` elsewhere.
- Only server code imports services, layers and `appRuntime`. `@evie/ui` and components stay free of Effect.
- One concept per module: a service and its own errors and schemas share a file; anything a second module needs moves to its own file.
- Never read or write `.env` files.

## Output

The service, its tests, its line in `src/lib/runtime.ts` and any server function, with `moon run frontend:check frontend:test` passing.
