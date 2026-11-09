# Testing

## `bun test` next to the source

Tests use bun's runner and import from `bun:test`. A test file sits beside the file it tests and adds `.test`.

```text
src/bookings/free-places.ts
src/bookings/free-places.test.ts
```

```ts
// src/bookings/free-places.test.ts
import { describe, expect, test } from "bun:test";
import { freePlaces } from "./free-places";

describe("freePlaces", () => {
  test("returns the places left", () => {
    expect(freePlaces({ places: 4, booked: 1 })).toBe(3);
  });

  test("never returns a negative number", () => {
    expect(freePlaces({ places: 2, booked: 3 })).toBe(0);
  });
});
```

## Names

- `describe` names the unit: the function, component or service.
- `test` states the behaviour as a sentence, so a failure reads as a broken promise: "freePlaces never returns a negative number".
- Use `test`, not `it`; one behaviour per test.

## Test through the exports

Test what a module exports. If a helper needs its own tests, it is a concept of its own: move it to its own module and export it there. Don't export a helper only for a test.

## Pass dependencies in, don't mock modules

Don't replace your own modules with `mock.module`. Take the dependency as a parameter and pass a test version. In Effect code, that is a test `Layer`; see the `effect-service` skill (once it exists).

```ts
// src/bookings/reminder.ts
type Clock = { readonly now: () => Date };

export function isReminderDue(startsAt: Date, clock: Clock): boolean {
  const oneDay = 24 * 60 * 60 * 1000;
  return startsAt.getTime() - clock.now().getTime() <= oneDay;
}
```

```ts
// src/bookings/reminder.test.ts
import { describe, expect, test } from "bun:test";
import { isReminderDue } from "./reminder";

describe("isReminderDue", () => {
  test("is due within a day of the start", () => {
    const clock = { now: () => new Date("2026-06-01T10:00:00Z") };
    expect(isReminderDue(new Date("2026-06-02T09:00:00Z"), clock)).toBe(true);
  });
});
```

Mocking third-party modules that have no seam is fine, e.g. `spyOn` on a browser API.

## Running tests

Through moon, so caching and task dependencies apply:

```sh
moon run frontend:test                              # one project
moon run :test                                      # every project with a test task
moon run frontend:test -- -t "never returns"        # tests whose name matches
moon run frontend:test -- src/bookings              # tests under a path
```

Use the `moon-run` skill (once it exists) for the details.
