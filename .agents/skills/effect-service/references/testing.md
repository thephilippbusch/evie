# Testing with layers

Tests use `bun test` next to the source, as the `typescript-conventions` skill says. A test runs the effect with `Effect.runPromise` and provides the layers it needs: the live layer of the service under test, with test layers for its settings and the services it depends on. Never `mock.module` a service.

## The service under test

Provide its live layer, and settings through a `ConfigProvider`:

```ts
// src/invites/invites.test.ts
import { describe, expect, test } from "bun:test";
import { ConfigProvider, Effect } from "effect";
import { Invites, invitesLive } from "./invites";

const config = ConfigProvider.fromMap(new Map([["APP_URL", "https://evie.test"]]));

const run = <A, E>(effect: Effect.Effect<A, E, Invites>) =>
  Effect.runPromise(effect.pipe(Effect.provide(invitesLive), Effect.withConfigProvider(config)));

describe("Invites", () => {
  test("links to the invite page of the event", async () => {
    const eventId = "0190f5a4-7b1c-7cc2-9d2e-3f1a2b3c4d5e";

    const link = await run(Effect.map(Invites, (invites) => invites.linkFor(eventId)));

    expect(link.href).toBe(`https://evie.test/invites/${eventId}`);
  });

  test("rejects params without a valid event ID", async () => {
    const error = await run(Effect.flip(Effect.flatMap(Invites, (invites) => invites.parse({ eventId: "nope" }))));

    expect(error._tag).toBe("InvalidInvite");
  });
});
```

- `Effect.withConfigProvider` wraps the whole run, so the live layer reads its `Config` from the map.
- `Effect.flip` swaps the error and success channels, so a test can assert on the failure.
- Assert on `_tag` and fields, not on messages.

## A dependency

When the service under test needs another service, provide a test layer for that one. `Layer.succeed` with `Tag.of` builds it from plain values:

```ts
// src/events/event-page.test.ts
import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { Invites } from "~/invites/invites";
import { EventPage, eventPageLive } from "./event-page";

const invitesTest = Layer.succeed(
  Invites,
  Invites.of({
    linkFor: (eventId) => new URL(`https://evie.test/invites/${eventId}`),
    parse: () => Effect.die("not used in this test"),
  }),
);

const run = <A, E>(effect: Effect.Effect<A, E, EventPage>) =>
  Effect.runPromise(effect.pipe(Effect.provide(eventPageLive.pipe(Layer.provide(invitesTest)))));

describe("EventPage", () => {
  test("shares the invite link of the event", async () => {
    const link = await run(Effect.map(EventPage, (page) => page.shareLink("event-1")));

    expect(link.href).toBe("https://evie.test/invites/event-1");
  });
});
```

Give the test layer only the behaviour the test needs; a method the test never calls can `Effect.die`.

## Time

Code that waits or reads the time uses Effect's `Clock`, `Effect.sleep` or `Effect.timeout`, never `Date.now()` or `setTimeout`. Tests then move the time with `TestClock` instead of waiting:

```ts
import { expect, test } from "bun:test";
import { Duration, Effect, Fiber, TestClock, TestContext } from "effect";

test("gives up on the backend after 5 seconds", async () => {
  const result = await Effect.runPromise(
    Effect.gen(function* () {
      const fiber = yield* Effect.fork(Effect.never.pipe(Effect.timeoutOption(Duration.seconds(5))));
      yield* TestClock.adjust(Duration.seconds(5));
      return yield* Fiber.join(fiber);
    }).pipe(Effect.provide(TestContext.TestContext)),
  );

  expect(result._tag).toBe("None");
});
```

Fork the effect that waits, move the clock past its deadline, then join the fiber.

## Server functions

Don't test server functions through SolidStart. Keep their effect small and test the services it calls. When the mapping from failures to the view is worth a test, move the effect into the service file as an exported function and test it there.

## Running

`moon run frontend:test`, or one test with `moon run frontend:test -- -t "rejects params"`. Use the `moon-run` skill.
