# Step 4: the BFF client service

The frontend server calls the backend with [Connect-ES](https://connectrpc.com/docs/node/using-clients) v2 over gRPC, using the protobuf-es v2 code in `@evie/proto`. Each area gets a client service built with the `effect-service` skill: it calls the RPC, decodes the response into the feature's own type with `Schema`, and fails with tagged errors.

## First RPC

1. Follow the First service only steps of the `effect-service` skill: they install `effect` and create `src/lib/runtime.ts`.
2. In `apps/frontend/`: `bun add @connectrpc/connect@^2 @connectrpc/connect-node@^2`.
3. Add `"@evie/proto": "workspace:*"` to `dependencies` in `apps/frontend/package.json`, next to `@evie/ui`, and run `bun install`.
4. Add `'proto'` to `dependsOn` in `apps/frontend/moon.yml`.
5. Create the two shared files below.
6. Document `BACKEND_URL` under Configuration in `apps/frontend/AGENTS.md`, and set it on the `frontend` service in `compose.yaml`: `BACKEND_URL: http://backend:50051`. For `bun run dev` next to `cargo run`, set `BACKEND_URL=http://127.0.0.1:50051`.

### `src/lib/backend-transport.ts`

```ts
import type { Transport } from "@connectrpc/connect";
import { createGrpcTransport } from "@connectrpc/connect-node";
import { Config, Context, Effect, Layer } from "effect";

/** The gRPC connection to the backend, shared by every client service. */
export class BackendTransport extends Context.Service<BackendTransport, Transport>()("BackendTransport") {}

export const backendTransportLive = Layer.effect(
  BackendTransport,
  Effect.map(Config.String("BACKEND_URL"), (baseUrl) => createGrpcTransport({ baseUrl })),
);
```

- `BACKEND_URL` has no default: a missing value fails the layer with a `ConfigError` on the first call, instead of silently calling localhost.
- The backend serves plaintext gRPC with no auth of its own, so it stays on the internal network: no host port in `compose.yaml` or a deployment, and `BACKEND_URL` always points to an internal address.

### `src/lib/call-backend.ts`

```ts
import { type CallOptions, Code, ConnectError } from "@connectrpc/connect";
import { Data, Effect } from "effect";

/** How long one backend call may take. */
const TIMEOUT_MS = 5000;

/** The backend can't be reached or didn't answer in time. */
export class BackendUnavailable extends Data.TaggedError("BackendUnavailable")<{
  readonly cause: ConnectError;
}> {}

/** The backend answered with a status code that the caller didn't turn into an error of its own. */
export class BackendError extends Data.TaggedError("BackendError")<{
  readonly code: Code;
  readonly cause: ConnectError;
}> {}

const toFailure = (cause: unknown): BackendUnavailable | BackendError => {
  const error = ConnectError.from(cause);
  if (error.code === Code.Unavailable || error.code === Code.DeadlineExceeded) {
    return new BackendUnavailable({ cause: error });
  }
  return new BackendError({ code: error.code, cause: error });
};

/** Runs one RPC with a deadline. Interrupting the effect cancels the call. */
export const callBackend = <A>(call: (options: CallOptions) => Promise<A>) =>
  Effect.tryPromise({
    try: (signal) => call({ signal, timeoutMs: TIMEOUT_MS }),
    catch: toFailure,
  });
```

## The client service

`src/<feature>/<area>-client.ts`. The response is decoded into the feature's own type, so pages never see protobuf messages, and proto3's empty defaults can't slip through.

```ts
// src/ping/ping-client.ts
import { createClient } from "@connectrpc/connect";
import { PingService } from "@evie/proto/evie/ping/v1/ping_pb";
import { Context, Effect, Layer, Schema } from "effect";
import { BackendTransport } from "~/lib/backend-transport";
import { type BackendError, type BackendUnavailable, callBackend } from "~/lib/call-backend";

const Ping = Schema.Struct({ version: Schema.Trim.check(Schema.isNonEmpty()) });
export type Ping = typeof Ping.Type;

/** Checks that the backend answers. */
export class PingClient extends Context.Service<
  PingClient,
  { readonly ping: Effect.Effect<Ping, BackendUnavailable | BackendError> }
>()("PingClient") {}

export const pingClientLive = Layer.effect(
  PingClient,
  Effect.gen(function* () {
    const client = createClient(PingService, yield* BackendTransport);
    return PingClient.of({
      ping: callBackend((options) => client.ping({}, options)).pipe(
        Effect.flatMap(Schema.decodeUnknownEffect(Ping)),
        // Both sides share the definitions, so a response that doesn't decode is a bug.
        Effect.catchTag("SchemaError", Effect.die),
      ),
    });
  }),
);
```

Turn a status code the page handles into the feature's own error; leave the rest as `BackendError`:

```ts
getPlace: (placeId) =>
  callBackend((options) => client.getPlace({ placeId }, options)).pipe(
    Effect.catchTag("BackendError", (error) =>
      Effect.fail(error.code === Code.NotFound ? new PlaceNotFound({ placeId }) : error),
    ),
    Effect.flatMap((response) => Schema.decodeUnknownEffect(Place)(response.place)),
    Effect.catchTag("SchemaError", Effect.die),
  ),
```

Request fields are camelCase in TypeScript: `place_id` becomes `placeId`.

## Runtime

Add the client to `appLayer` and give the clients the transport:

```ts
// src/lib/runtime.ts
const appLayer = Layer.mergeAll(pingClientLive).pipe(Layer.provide(backendTransportLive));
```

## Test

Test the client against an in-memory backend: `createRouterTransport` from `@connectrpc/connect` serves the service from plain functions, so the test needs no server and no mocks.

```ts
// src/ping/ping-client.test.ts
import { Code, ConnectError, createRouterTransport } from "@connectrpc/connect";
import { PingService } from "@evie/proto/evie/ping/v1/ping_pb";
import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { BackendTransport } from "~/lib/backend-transport";
import { PingClient, pingClientLive } from "./ping-client";

const backendAnswering = (ping: () => { version: string }) =>
  pingClientLive.pipe(
    Layer.provide(
      Layer.succeed(
        BackendTransport,
        createRouterTransport(({ service }) => {
          service(PingService, { ping });
        }),
      ),
    ),
  );

const ping = PingClient.use((client) => client.ping);

describe("PingClient", () => {
  test("returns the backend's version", async () => {
    const backend = backendAnswering(() => ({ version: "0.1.0" }));

    const result = await Effect.runPromise(ping.pipe(Effect.provide(backend)));

    expect(result.version).toBe("0.1.0");
  });

  test("fails with BackendUnavailable when the backend is down", async () => {
    const backend = backendAnswering(() => {
      throw new ConnectError("connection refused", Code.Unavailable);
    });

    const error = await Effect.runPromise(Effect.flip(ping).pipe(Effect.provide(backend)));

    expect(error._tag).toBe("BackendUnavailable");
  });
});
```

Cover each tagged error the client returns, and a response that doesn't decode (`Effect.runPromiseExit` and `Exit.isFailure`).
