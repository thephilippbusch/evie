---
name: ui-component
description: Builds a presentational component for evie's `@evie/ui` library on a Kobalte primitive, with its own folder, variants, accessibility, states, tests and export. Use when adding or changing a UI component in libs/ui.
---

# UI component

`libs/ui` (`@evie/ui`) is evie's own component library on [Kobalte](https://kobalte.dev). Apps import its components; they never build their own primitives. Read `libs/ui/AGENTS.md` first.

| Read                                                             | When you build                                                    |
| ---------------------------------------------------------------- | ----------------------------------------------------------------- |
| [references/compound.md](references/compound.md)                 | a component with several parts, such as a dialog, menu or select  |
| [references/states.md](references/states.md)                     | focus, disabled, open, checked or invalid styles                  |
| [references/accessible-names.md](references/accessible-names.md) | anything a screen reader announces: icon buttons, dialogs, fields |

## Steps

1. Check `libs/ui/src/` for a component that already does the job; extend it instead of adding a second one.
2. Pick the base:
   - Interactive or accessible behaviour (buttons, dialogs, menus, form controls): the Kobalte primitive from `@kobalte/core/<primitive>`. Read its page on kobalte.dev for its parts, props and `data-*` attributes.
   - Static layout (like `card`): plain elements.
3. Create the folder `libs/ui/src/<name>/` and keep everything for the component in it:

   | File                      | Holds                                                                         |
   | ------------------------- | ----------------------------------------------------------------------------- |
   | `<name>.tsx`              | the component and its parts, such as `Card` and `CardTitle`                   |
   | `<name>-variants.ts`      | the `cva` variants; bun can't load Solid JSX, so they live outside the `.tsx` |
   | `<name>-variants.test.ts` | tests of the variants, e.g. that every size keeps the 44 px touch target      |
   | `<helper>.ts`             | a helper only this component uses, with its `<helper>.test.ts`                |

   Library-wide helpers go in `src/utils/` and are imported as `#utils/<name>`. Import within the folder with `./`; never `../`.
4. Write each part as a `const` arrow function.
   - On a Kobalte primitive, use the shape of `libs/ui/src/button/button.tsx`:

     ```tsx
     const Thing = <T extends ValidComponent = "div">(props: PolymorphicProps<T, ThingProps<T>>) => {
       const [local, others] = splitProps(props as ThingProps, ["variant", "class"]);
       return (
         <ThingPrimitive.Root
           class={cn(thingVariants({ variant: local.variant }), local.class)}
           {...others}
         />
       );
     };

     export type { ThingProps };
     export { Thing };
     ```

     Keep the cast and its comment from `button.tsx`: it narrows the generic props so `splitProps` and Kobalte's types fit.
   - On plain elements, use the shape of `libs/ui/src/card/card.tsx`: `Component<ComponentProps<"div">>`, with `splitProps` for `class`. A part whose element varies, like `CardTitle`'s heading level, takes an `as` prop and renders `Dynamic`.
   - With several parts, follow [references/compound.md](references/compound.md).
5. Style focus, disabled and other states from the tokens, as [references/states.md](references/states.md) shows.
6. Give every control and dialog an accessible name through props, as [references/accessible-names.md](references/accessible-names.md) shows.
7. Use it as `@evie/ui/<name>`: the `./*` export in `libs/ui/package.json` maps it to `src/<name>/<name>.tsx`. Nothing to register.
8. Check with `moon run ui:check ui:test`, then `moon run frontend:build` if the frontend uses it. Use the `moon-run` skill.
9. Check it in the running frontend, see [Check it in the browser](#check-it-in-the-browser).
10. If the change adds a kind of file, a library-wide helper, an export or a dependency, update `libs/ui/AGENTS.md`. Add a package with `bun add <pkg>` in `libs/ui/`.

## Check it in the browser

1. Render it where the frontend uses it. If nothing uses it yet, render it on a scratch route in `apps/frontend/src/routes/` and delete the route before you commit.
2. Start the frontend with `bun run dev` in `apps/frontend/` and open the URL it prints.
3. Check it at a phone width (375 px) and at a desktop width.
4. Check it in light and in dark mode: switch the system appearance, or emulate `prefers-color-scheme` in the browser's developer tools.
5. Use it with the keyboard only: Tab reaches it, the focus ring shows, Enter, Space and Escape do what the Kobalte primitive promises, and focus returns where it came from.

## Rules

- Presentational only: no data loading, no Effect, no built-in texts. Labels, accessible names and content come in through props and children, so apps can translate them.
- Colours only from the tokens in `libs/ui/src/styles.css`; dark mode follows from them.
- Touch targets at least 44 px (`min-h-11 min-w-11`); mobile-first classes (base for phones, `sm:` and up for larger screens).
- Never remove the focus ring without a visible replacement.
- One component family per folder; no barrel files.
- Follow the `typescript-conventions` skill.

## Output

The new or changed files under `libs/ui/src/<name>/`, and `libs/ui/AGENTS.md` if the structure changed, with `moon run ui:check ui:test` passing and the component checked in the browser.
