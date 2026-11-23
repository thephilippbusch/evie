import { describe, expect, test } from "bun:test";
import { Code, ConnectError, createRouterTransport } from "@connectrpc/connect";
import { PingService } from "@evie/proto/evie/ping/v1/ping_pb";
import { Cause, Effect, Exit, Layer } from "effect";
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

const backendFailingWith = (code: Code) =>
  backendAnswering(() => {
    throw new ConnectError("the backend failed", code);
  });

const ping = PingClient.use((client) => client.ping);

describe("PingClient", () => {
  test("returns the backend's version", async () => {
    const backend = backendAnswering(() => ({ version: "0.1.0" }));

    const result = await Effect.runPromise(ping.pipe(Effect.provide(backend)));

    expect(result.version).toBe("0.1.0");
  });

  test("fails with BackendUnavailable when the backend is down", async () => {
    const backend = backendFailingWith(Code.Unavailable);

    const error = await Effect.runPromise(Effect.flip(ping).pipe(Effect.provide(backend)));

    expect(error._tag).toBe("BackendUnavailable");
  });

  test("fails with BackendUnavailable when the backend doesn't answer in time", async () => {
    const backend = backendFailingWith(Code.DeadlineExceeded);

    const error = await Effect.runPromise(Effect.flip(ping).pipe(Effect.provide(backend)));

    expect(error._tag).toBe("BackendUnavailable");
  });

  test("fails with BackendError and the code for any other status", async () => {
    const backend = backendFailingWith(Code.Internal);

    const error = await Effect.runPromise(Effect.flip(ping).pipe(Effect.provide(backend)));

    expect(error).toMatchObject({ _tag: "BackendError", code: Code.Internal });
  });

  test("dies when the response has no version", async () => {
    const backend = backendAnswering(() => ({ version: " " }));

    const exit = await Effect.runPromiseExit(ping.pipe(Effect.provide(backend)));

    expect(Exit.isFailure(exit) && Cause.hasDies(exit.cause)).toBe(true);
  });
});
