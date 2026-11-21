---
name: ui-component
description: Builds a presentational component for evie's `@evie/ui` library on a Kobalte primitive, with its own folder, variants, tests and export. Use when adding or changing a UI component, button, dialog, form control or other primitive in libs/ui.
---

# UI component

`libs/ui` (`@evie/ui`) is evie's own component library on [Kobalte](https://kobalte.dev). Apps import its components; they never build their own primitives. Read `libs/ui/AGENTS.md` first.

## Steps

1. Check `libs/ui/src/` for a component that already does the job; extend it instead of adding a second one.
2. Pick the base:
   - Interactive or accessible behaviour (buttons, dialogs, menus, form controls): the Kobalte primitive from `@kobalte/core/<primitive>`.
   - Static layout (like `card`): plain elements.
3. Create the folder `libs/ui/src/<name>/` and keep everything for the component in it:

   | File                      | Holds                                                                         |
   | ------------------------- | ----------------------------------------------------------------------------- |
   | `<name>.tsx`              | the component and its parts, such as `Card` and `CardTitle`                   |
   | `<name>-variants.ts`      | the `cva` variants; bun can't load Solid JSX, so they live outside the `.tsx` |
   | `<name>-variants.test.ts` | tests of the variants, e.g. that every size keeps the 44 px touch target      |
   | `<helper>.ts`             | a helper only this component uses, with its `<helper>.test.ts`                |

   Library-wide helpers go in `src/utils/` and are imported as `#utils/<name>`. Import within the folder with `./`; never `../`.
4. Write the component as a `const` arrow function with the shape of `libs/ui/src/button/button.tsx`:

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

5. Use it as `@evie/ui/<name>`: the `./*` export in `libs/ui/package.json` maps it to `src/<name>/<name>.tsx`. Nothing to register.
6. Check with `moon run ui:check ui:test`, then `moon run frontend:build` if the frontend uses it. Use the `moon-run` skill.

## Rules

- Presentational only: no data loading, no Effect, no built-in texts. Labels and content come in through props and children, so apps can translate them.
- Colours only from the tokens in `libs/ui/src/styles.css`; dark mode follows from them.
- Touch targets at least 44 px (`min-h-11 min-w-11`); mobile-first classes (base for phones, `sm:` and up for larger screens).
- One component family per folder; no barrel files.
- Follow the `typescript-conventions` skill.

## Output

The new or changed files under `libs/ui/src/<name>/`, with `moon run ui:check ui:test` passing.
