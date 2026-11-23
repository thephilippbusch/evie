import { describe, expect, test } from "bun:test";
import { ConfigProvider, Effect } from "effect";
import { BackendTransport, backendTransportLive } from "./backend-transport";

const transportWith = (env: Record<string, string>) =>
  Effect.gen(function* () {
    return yield* BackendTransport;
  }).pipe(
    Effect.provide(backendTransportLive),
    Effect.provide(ConfigProvider.layer(ConfigProvider.fromUnknown(env))),
  );

describe("backendTransportLive", () => {
  test("builds the transport from BACKEND_URL", async () => {
    const transport = await Effect.runPromise(
      transportWith({ BACKEND_URL: "http://backend.test:50051" }),
    );

    expect(typeof transport.unary).toBe("function");
  });

  test("fails with a ConfigError without BACKEND_URL", async () => {
    const error = await Effect.runPromise(Effect.flip(transportWith({})));

    expect(error._tag).toBe("ConfigError");
  });
});
