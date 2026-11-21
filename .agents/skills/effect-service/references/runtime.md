# Runtime and server functions

## The app runtime

One `ManagedRuntime` for the whole server, built from every live layer. It builds each layer once and shares the services across requests.

```ts
// src/lib/runtime.ts
import { Layer, ManagedRuntime } from "effect";
import { invitesLive } from "~/invites/invites";

/** Every live service of the server. */
const appLayer = Layer.mergeAll(invitesLive);

/** Runs the server's effects. Only server functions use it. */
export const appRuntime = ManagedRuntime.make(appLayer);
```

- Add each new live layer to `Layer.mergeAll`.
- A layer that needs another service gets it with `Layer.provide`, e.g. `Layer.mergeAll(pingClientLive).pipe(Layer.provide(backendTransportLive))`.
- Effects run only through `appRuntime`, in server function bodies. Outside tests, nothing else calls `Effect.runPromise`.

## Server functions

A server function is the browser's way into server code. It runs one effect with `appRuntime` and returns plain, serialisable data: no class instances, no `Effect`, no errors to unpack. The example is public: anyone with the link may open an invite, so it checks no session.

```ts
// src/invites/get-invite.ts
import { query } from "@solidjs/router";
import { Effect } from "effect";
import { appRuntime } from "~/lib/runtime";
import { Invites } from "./invites";

/** What the invite page shows. */
export type InviteView =
  | { readonly _tag: "Valid"; readonly eventId: string }
  | { readonly _tag: "Invalid" };

export const getInvite = query(async (params: unknown): Promise<InviteView> => {
  "use server";
  const view = Effect.gen(function* () {
    const invites = yield* Invites;
    const invite = yield* invites.parse(params);
    return { _tag: "Valid", eventId: invite.eventId } satisfies InviteView;
  }).pipe(
    Effect.catchTag("InvalidInvite", () => Effect.succeed<InviteView>({ _tag: "Invalid" })),
    Effect.tapCause(Effect.logError),
  );
  return appRuntime.runPromise(view);
}, "invite");
```

- `"use server"` is the first statement of the function body. SolidStart runs the body on the server and gives the browser a stub that calls it.
- Use `query` from `@solidjs/router` for reads, with a unique key; routes load them with `createAsync`. Use `action` for writes.
- Take arguments as `unknown` and decode them first: anyone can call a server function with any input.
- A server function that reads or changes someone's data gets the BetterAuth session first and fails without one, then checks that the user may act on the event, activity, place or booking. Sign-in comes with #30 and passing the verified user to the backend with #34 (_planned_); until then, server functions serve only public data.
- Return a `_tag` union with one variant per outcome the page shows, as the `typescript-conventions` skill models variants. The page switches on `_tag`.
- Handle every expected failure inside the effect. A failure that is left rejects the promise and reaches the nearest Solid `ErrorBoundary`.
- Keep the server function in its own file. Import services and `appRuntime` only in server function files, and use them only inside the `"use server"` body, so they stay out of the browser bundle. After `moon run frontend:build`, check that the gRPC client stayed out of the browser bundle: `grep -rlE "http2|__vite-browser-external" apps/frontend/.output/public` prints nothing. `@connectrpc/connect-node` imports `node:http2`, and Vite leaves one of the two strings in any browser chunk that pulls it in.

## Using it in a route

```tsx
// src/routes/invite.tsx, opened as /invite?eventId=…
import { createAsync, useSearchParams } from "@solidjs/router";
import { Show } from "solid-js";
import { getInvite } from "~/invites/get-invite";
import { InviteCard } from "~/invites/invite-card";

const InvitePage = () => {
  const [params] = useSearchParams();
  const invite = createAsync(() => getInvite({ ...params }));
  return <Show when={invite()}>{(view) => <InviteCard view={view()} />}</Show>;
};

export default InvitePage;
```

The app shell already wraps every page in `Suspense`, so the route needs no loading state of its own.
