import { createAsync, type RouteDefinition } from "@solidjs/router";
import { Show } from "solid-js";
import { getPing } from "~/ping/get-ping";
import { PingStatus } from "~/ping/ping-status";
import { pingStatusTexts } from "~/ping/ping-status-texts";

export const route = { preload: () => getPing() } satisfies RouteDefinition;

// Renders `en` until #31 adds locale detection.
const Home = () => {
  const ping = createAsync(() => getPing());
  return (
    <Show when={ping()}>{(view) => <PingStatus view={view()} texts={pingStatusTexts.en} />}</Show>
  );
};

export default Home;
