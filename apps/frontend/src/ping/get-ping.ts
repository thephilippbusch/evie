import { query } from "@solidjs/router";
import { Effect } from "effect";
import { appRuntime } from "~/lib/runtime";
import { PingClient } from "./ping-client";

/** What the page shows about the backend. */
export type PingView =
  | { readonly _tag: "Reachable"; readonly version: string }
  | { readonly _tag: "Unreachable" };

// Public data: ping needs no session.
export const getPing = query(async (): Promise<PingView> => {
  "use server";
  const view = Effect.gen(function* () {
    const client = yield* PingClient;
    const { version } = yield* client.ping;
    return { _tag: "Reachable", version } satisfies PingView;
  }).pipe(
    Effect.catchTag("BackendUnavailable", () => Effect.succeed<PingView>({ _tag: "Unreachable" })),
    Effect.tapCause(Effect.logError),
  );
  return appRuntime.runPromise(view);
}, "ping");
