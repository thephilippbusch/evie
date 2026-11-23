import { query } from "@solidjs/router";
import { Effect } from "effect";
import { appRuntime } from "~/lib/runtime";
import { type PingView, pingView } from "./ping-view";

// Public data: ping needs no session.
export const getPing = query(async (): Promise<PingView> => {
  "use server";
  return appRuntime.runPromise(pingView.pipe(Effect.tapCause(Effect.logError)));
}, "ping");
