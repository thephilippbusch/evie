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
