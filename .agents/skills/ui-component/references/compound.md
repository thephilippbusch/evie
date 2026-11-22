# Components with several parts

Dialogs, menus, popovers, selects and tabs are Kobalte primitives with several parts, such as `Root`, `Trigger`, `Portal`, `Content` and `Title`. The library wraps them as one family in one folder and one file, `libs/ui/src/<name>/<name>.tsx`.

## Rules

- Name every part with the family prefix: `Dialog`, `DialogTrigger`, `DialogContent`, `DialogTitle`.
- A part that needs no styling is the Kobalte part itself: `const Dialog = DialogPrimitive.Root;`.
- A styled part has the polymorphic shape of `button.tsx`, typed with Kobalte's `<Primitive><Part>Props<T>`, such as `DialogPrimitive.DialogContentProps<T>`.
- Put the parts that every use needs inside one part, so an app can't forget them: `DialogContent` renders `Portal`, `Overlay` and the close button.
- A part that renders its own control takes that control's accessible name as a required prop, such as `closeLabel` (see [accessible-names.md](accessible-names.md)).
- Export the parts and the props types of the styled parts; nothing else.

## Example: dialog

```tsx
// libs/ui/src/dialog/dialog.tsx
import * as DialogPrimitive from "@kobalte/core/dialog";
import type { PolymorphicProps } from "@kobalte/core/polymorphic";
import type { JSX, ValidComponent } from "solid-js";
import { splitProps } from "solid-js";
import { cn } from "#utils/cn";

const Dialog = DialogPrimitive.Root;
const DialogTrigger = DialogPrimitive.Trigger;

type DialogContentProps<T extends ValidComponent = "div"> = DialogPrimitive.DialogContentProps<T> & {
  class?: string | undefined;
  children?: JSX.Element;
  /** The accessible name of the close button, in the reader's language. */
  closeLabel: string;
};

// A bottom sheet on phones, a centred dialog from `sm:` up.
const DialogContent = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, DialogContentProps<T>>,
) => {
  const [local, others] = splitProps(props as DialogContentProps, ["class", "children", "closeLabel"]);
  return (
    <DialogPrimitive.Portal>
      <DialogPrimitive.Overlay class="fixed inset-0 z-50 bg-foreground/40" />
      <DialogPrimitive.Content
        class={cn(
          "fixed inset-x-0 bottom-0 z-50 grid max-h-[90dvh] gap-4 overflow-y-auto rounded-t-lg border bg-popover p-4 pb-[calc(1rem+env(safe-area-inset-bottom))] text-popover-foreground shadow-lg outline-hidden sm:inset-auto sm:top-1/2 sm:left-1/2 sm:w-full sm:max-w-lg sm:-translate-x-1/2 sm:-translate-y-1/2 sm:rounded-lg sm:p-6",
          local.class,
        )}
        {...others}
      >
        {local.children}
        <DialogPrimitive.CloseButton
          aria-label={local.closeLabel}
          class="absolute top-2 right-2 inline-flex size-11 items-center justify-center rounded-md outline-hidden hover:bg-accent hover:text-accent-foreground focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
        >
          <span aria-hidden="true">×</span>
        </DialogPrimitive.CloseButton>
      </DialogPrimitive.Content>
    </DialogPrimitive.Portal>
  );
};

type DialogTitleProps<T extends ValidComponent = "h2"> = DialogPrimitive.DialogTitleProps<T> & {
  class?: string | undefined;
};

const DialogTitle = <T extends ValidComponent = "h2">(props: PolymorphicProps<T, DialogTitleProps<T>>) => {
  const [local, others] = splitProps(props as DialogTitleProps, ["class"]);
  return (
    <DialogPrimitive.Title
      class={cn("pr-12 text-lg font-semibold leading-none tracking-tight", local.class)}
      {...others}
    />
  );
};

export type { DialogContentProps, DialogTitleProps };
export { Dialog, DialogContent, DialogTitle, DialogTrigger };
```

`DialogDescription` follows the shape of `DialogTitle` on `DialogPrimitive.Description`, with `text-sm text-muted-foreground`.

## Use in the app

The trigger renders an `@evie/ui` button through `as`, so it keeps the button's look and touch target:

```tsx
import { Button } from "@evie/ui/button";
import { Dialog, DialogContent, DialogTitle, DialogTrigger } from "@evie/ui/dialog";

<Dialog>
  <DialogTrigger as={Button} variant="brand">
    {texts.book}
  </DialogTrigger>
  <DialogContent closeLabel={texts.close}>
    <DialogTitle>{texts.bookTitle}</DialogTitle>
    {/* the booking form */}
  </DialogContent>
</Dialog>;
```

## Tests

bun can't render the `.tsx`, so test what is plain TypeScript: the variants, if the family has any, and helpers. Check the parts together in the browser, as the skill's last steps say.
