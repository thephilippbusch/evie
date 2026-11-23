import { describe, expect, test } from "bun:test";
import { type CallOptions, Code } from "@connectrpc/connect";
import { Effect, Fiber } from "effect";
import { callBackend } from "./call-backend";

describe("callBackend", () => {
  test("gives the call a 5 second deadline", async () => {
    const calls: Array<CallOptions> = [];

    await Effect.runPromise(
      callBackend((options) => {
        calls.push(options);
        return Promise.resolve("answer");
      }),
    );

    expect(calls.map((options) => options.timeoutMs)).toEqual([5000]);
  });

  test("cancels the call when the effect is interrupted", async () => {
    const signals: Array<AbortSignal> = [];

    await Effect.runPromise(
      Effect.gen(function* () {
        const fiber = yield* Effect.forkChild(
          callBackend((options) => {
            if (options.signal) signals.push(options.signal);
            return new Promise<never>(() => {});
          }),
        );
        yield* Effect.yieldNow;
        yield* Fiber.interrupt(fiber);
      }),
    );

    expect(signals.map((signal) => signal.aborted)).toEqual([true]);
  });

  test("fails with BackendError and code Unknown when the call rejects with a plain error", async () => {
    const error = await Effect.runPromise(
      Effect.flip(callBackend(() => Promise.reject(new Error("socket hang up")))),
    );

    expect(error).toMatchObject({ _tag: "BackendError", code: Code.Unknown });
  });
});
