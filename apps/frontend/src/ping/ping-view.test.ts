import { describe, expect, test } from "bun:test";
import { Code, ConnectError } from "@connectrpc/connect";
import { Effect, Layer, Logger } from "effect";
import { BackendError, BackendUnavailable } from "~/lib/call-backend";
import { PingClient } from "./ping-client";
import { pingView } from "./ping-view";

const backendDoing = (
  ping: Effect.Effect<{ readonly version: string }, BackendUnavailable | BackendError>,
) => Layer.succeed(PingClient, PingClient.of({ ping }));

const run = <A, E>(effect: Effect.Effect<A, E, PingClient>, backend: Layer.Layer<PingClient>) =>
  Effect.runPromise(effect.pipe(Effect.provide(backend)));

describe("pingView", () => {
  test("shows the version of a reachable backend", async () => {
    const backend = backendDoing(Effect.succeed({ version: "0.1.0" }));

    const view = await run(pingView, backend);

    expect(view).toEqual({ _tag: "Reachable", version: "0.1.0" });
  });

  test("shows an unreachable backend and logs a warning", async () => {
    const unavailable = new BackendUnavailable({
      cause: new ConnectError("down", Code.Unavailable),
    });
    const backend = backendDoing(Effect.fail(unavailable));
    const levels: Array<string> = [];
    const recordLevels = Logger.layer([
      Logger.make(({ logLevel }) => {
        levels.push(logLevel);
      }),
    ]);

    const view = await run(pingView.pipe(Effect.provide(recordLevels)), backend);

    expect(view).toEqual({ _tag: "Unreachable" });
    expect(levels).toEqual(["Warn"]);
  });

  test("leaves any other backend failure to the caller", async () => {
    const failure = new BackendError({
      code: Code.Internal,
      cause: new ConnectError("boom", Code.Internal),
    });
    const backend = backendDoing(Effect.fail(failure));

    const error = await run(Effect.flip(pingView), backend);

    expect(error).toMatchObject({ _tag: "BackendError", code: Code.Internal });
  });
});
