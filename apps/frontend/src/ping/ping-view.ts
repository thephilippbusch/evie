import { Effect } from "effect";
import { PingClient } from "./ping-client";

/** What the page shows about the backend. */
export type PingView =
  | { readonly _tag: "Reachable"; readonly version: string }
  | { readonly _tag: "Unreachable" };

/**
 * Pings the backend for the page. A backend that is down is an outcome the page
 * shows; any other failure is left for the caller.
 */
export const pingView = Effect.gen(function* () {
  const client = yield* PingClient;
  const { version } = yield* client.ping;
  return { _tag: "Reachable", version } satisfies PingView;
}).pipe(
  Effect.catchTag("BackendUnavailable", (error) =>
    Effect.logWarning("the backend can't be reached", error).pipe(
      Effect.as<PingView>({ _tag: "Unreachable" }),
    ),
  ),
);
