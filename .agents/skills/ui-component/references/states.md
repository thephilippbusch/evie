# States

Kobalte sets `data-*` attributes on its parts for each state, and native elements have pseudo-classes. Style both with Tailwind variants and colours from the tokens in `libs/ui/src/styles.css`, so dark mode follows on its own. `libs/ui/src/button/button-variants.ts` is the model.

| State                         | Classes                                                                                            |
| ----------------------------- | -------------------------------------------------------------------------------------------------- |
| keyboard focus                | `outline-hidden focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring` |
| disabled, native element      | `disabled:pointer-events-none disabled:opacity-50`                                                 |
| disabled, Kobalte part        | `data-disabled:pointer-events-none data-disabled:opacity-50`                                       |
| open (dialog, menu, popover)  | `data-expanded:…`, closed: `data-closed:…`                                                         |
| checked (checkbox, switch)    | `data-checked:bg-primary data-checked:text-primary-foreground`                                     |
| highlighted list or menu item | `data-highlighted:bg-accent data-highlighted:text-accent-foreground`                               |
| invalid field                 | `data-invalid:border-destructive`                                                                  |
| hover                         | `hover:…`, as an extra only: phones have no hover                                                  |

- Use `focus-visible`, not `focus`, so a tap doesn't leave a ring.
- Every interactive part shows the focus ring; never remove it without a visible replacement.
- Look up a part's attributes on its kobalte.dev page, under "Data attributes".
- A state never relies on colour alone: pair it with a border, an icon or text the app passes in.

## Test them

Put the state classes in the `cva` base or variants, and assert on them in `<name>-variants.test.ts`, as `button-variants.test.ts` asserts the touch target:

```ts
// libs/ui/src/thing/thing-variants.test.ts
import { describe, expect, test } from "bun:test";
import { thingVariants } from "./thing-variants";

const VARIANTS = ["default", "brand"] as const;

/** Splits a class string, so `toContain` matches whole classes, not substrings. */
const classesOf = (value: string): ReadonlyArray<string> => value.split(/\s+/);

describe("thingVariants", () => {
  for (const variant of VARIANTS) {
    test(`shows the focus ring in the ${variant} variant`, () => {
      expect(classesOf(thingVariants({ variant }))).toContain("focus-visible:outline-ring");
    });
  }
});
```
