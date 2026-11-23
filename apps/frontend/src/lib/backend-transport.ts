import type { Transport } from "@connectrpc/connect";
import { createGrpcTransport } from "@connectrpc/connect-node";
import { Config, Context, Effect, Layer } from "effect";

/** The gRPC connection to the backend, shared by every client service. */
export class BackendTransport extends Context.Service<BackendTransport, Transport>()(
  "BackendTransport",
) {}

// No default: a missing BACKEND_URL fails the first call with a ConfigError,
// instead of silently calling localhost.
export const backendTransportLive = Layer.effect(
  BackendTransport,
  Effect.map(Config.String("BACKEND_URL"), (baseUrl) => createGrpcTransport({ baseUrl })),
);
