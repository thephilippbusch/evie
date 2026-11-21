# Accessible names

Every control, dialog and field needs a name a screen reader can announce. `@evie/ui` holds no texts, so every name comes in through props or children, in the reader's language.

| Element                                 | Where its name comes from                                                           |
| --------------------------------------- | ----------------------------------------------------------------------------------- |
| button or link with text                | its children                                                                        |
| icon-only button                        | `aria-label` from the app: `<Button size="icon" aria-label={texts.edit}>`           |
| control the component renders by itself | a required prop, such as `closeLabel` on `DialogContent`                            |
| dialog, alert dialog                    | its `…Title` part, which Kobalte links with `aria-labelledby`; always render one    |
| text field, select, checkbox            | the Kobalte `Label` part, with `Description` and `ErrorMessage` linked the same way |
| decorative icon or symbol               | none: `aria-hidden="true"`                                                          |

## Rules

- A name that the component needs is a required prop (`closeLabel: string`), not an optional one, so the type check catches a missing name.
- A title that shouldn't show stays in the DOM with `class="sr-only"`; never drop it.
- Pass `aria-*` props through `others`, as `button.tsx` does with `splitProps`, so apps can add them.
- Never set a name inside the library, not even a default such as "Close"; the app passes the German or English text.

## Check them

In the browser's developer tools, open the accessibility tree and check each control and dialog has the name you expect. Then go through the component with the keyboard only.
