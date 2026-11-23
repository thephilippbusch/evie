# Step 5: the server function and the page

The page gets the RPC's result through a server function that runs the client service with `appRuntime`. Write it in the shape the `effect-service` skill gives for server functions. Build the page from `@evie/ui` components; use the `ui-component` skill for any that's missing.

## View and server function

The effect that turns the RPC's result into what the page shows lives in `src/<feature>/<feature>-view.ts`, so `bun test` can test it: bun can't load the server function file, because `@solidjs/router` throws outside the browser.

```ts
// src/ping/ping-view.ts
import { Effect } from "effect";
import { PingClient } from "./ping-client";

/** What the page shows about the backend. */
export type PingView =
  | { readonly _tag: "Reachable"; readonly version: string }
  | { readonly _tag: "Unreachable" };

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
```

```ts
// src/ping/get-ping.ts
import { query } from "@solidjs/router";
import { Effect } from "effect";
import { appRuntime } from "~/lib/runtime";
import { type PingView, pingView } from "./ping-view";

// Public data: ping needs no session.
export const getPing = query(async (): Promise<PingView> => {
  "use server";
  return appRuntime.runPromise(pingView.pipe(Effect.tapCause(Effect.logError)));
}, "ping");
```

- One `_tag` variant per outcome the page shows. A backend that is down is an outcome, not an error page, and it's logged as a warning.
- Test the view with a test layer for the client service, as `ping-view.test.ts` does: one test per variant, and one that `BackendError` still fails.
- `BackendError` stays unhandled unless the page has something to say about it: it rejects and reaches the nearest `ErrorBoundary`.
- An RPC with arguments takes them as `unknown` and decodes them with `Schema` first.

## Component and route

The component switches on `_tag` and closes the switch with `satisfies never`, so a new variant fails the build until the page handles it. The route stays thin.

```tsx
// src/ping/ping-status.tsx
import { Card, CardDescription, CardHeader, CardTitle } from "@evie/ui/card";
import type { PingView } from "./ping-view";

type PingStatusProps = {
  readonly view: PingView;
  readonly texts: PingStatusTexts;
};

/** The page's texts, in the reader's language. */
export type PingStatusTexts = {
  readonly title: string;
  readonly version: (version: string) => string;
  readonly unreachable: string;
};

export const PingStatus = (props: PingStatusProps) => {
  const description = () => {
    const view = props.view;
    switch (view._tag) {
      case "Reachable":
        return props.texts.version(view.version);
      case "Unreachable":
        return props.texts.unreachable;
      default:
        return view satisfies never;
    }
  };
  return (
    <Card>
      <CardHeader>
        <CardTitle as="h1">{props.texts.title}</CardTitle>
        <CardDescription>{description()}</CardDescription>
      </CardHeader>
    </Card>
  );
};
```

```ts
// src/ping/ping-status-texts.ts
import type { PingStatusTexts } from "./ping-status";

export const pingStatusTexts = {
  de: {
    title: "Backend",
    version: (version) => `Version ${version}`,
    unreachable: "Das Backend ist gerade nicht erreichbar.",
  },
  en: {
    title: "Backend",
    version: (version) => `Version ${version}`,
    unreachable: "The backend can't be reached right now.",
  },
} satisfies Record<"de" | "en", PingStatusTexts>;
```

```tsx
// src/routes/index.tsx
import { createAsync, type RouteDefinition } from "@solidjs/router";
import { Show } from "solid-js";
import { getPing } from "~/ping/get-ping";
import { PingStatus } from "~/ping/ping-status";
import { pingStatusTexts } from "~/ping/ping-status-texts";

export const route = { preload: () => getPing() } satisfies RouteDefinition;

const Home = () => {
  const ping = createAsync(() => getPing());
  return <Show when={ping()}>{(view) => <PingStatus view={view()} texts={pingStatusTexts.en} />}</Show>;
};

export default Home;
```

`preload` starts the call while the route's code loads. The app shell's `Suspense` shows nothing until it returns.

## Texts

Every text the page shows exists in German and English. Components take their texts as props; `@evie/ui` holds none.
The app has no i18n setup yet; #31 adds it with locale detection (_planned_). Until then, keep a `de` and an `en` object per component, as the `ping-status-texts.ts` example above does. The page renders `en` only until #31 adds locale detection, which then picks the object.

## Check in the running app

1. Apply the migrations and start the backend: `moon run domain-migrations:migrate`, then `cargo run` in `apps/backend/`, both with `DATABASE_URL` set.
2. Start the frontend: `BACKEND_URL=http://127.0.0.1:50051 bun run dev` in `apps/frontend/`, and open the URL it prints. Or run the whole stack with `docker compose up` at the repository root and open http://127.0.0.1:3000.
3. Check the page shows the result. Stop the backend, reload, and check the page shows the unreachable state, not an error page.
4. Check it on a phone-sized window and in light and dark mode.
