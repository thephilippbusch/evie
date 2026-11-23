import { Layer, ManagedRuntime } from "effect";
import { pingClientLive } from "~/ping/ping-client";
import { backendTransportLive } from "./backend-transport";

/** Every live service of the server. The client services share one backend transport. */
const appLayer = Layer.mergeAll(pingClientLive).pipe(Layer.provide(backendTransportLive));

/** Runs the server's effects. Only server functions use it. */
export const appRuntime = ManagedRuntime.make(appLayer);
