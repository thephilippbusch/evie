# ui

## Purpose

evie's UI library, `@evie/ui`: presentational components, the design tokens and the Tailwind CSS setup. The [frontend](../../apps/frontend/AGENTS.md) uses it; the library is never deployed on its own.

## Structure

| Path             | Contents                                                                                                          |
| ---------------- | ----------------------------------------------------------------------------------------------------------------- |
| `src/styles.css` | Tailwind CSS, the design tokens (shadcn names, light and dark) and base styles; imported as `@evie/ui/styles.css` |
| `src/cn.ts`      | `cn`, which joins class names and resolves conflicting Tailwind classes; imported as `@evie/ui/cn`                |
| `src/<name>.tsx` | One solid-ui component per file, such as `button.tsx`; imported as `@evie/ui/<name>`                              |
| `ui.config.json` | The solid-ui CLI config: components go to `src/`, `cn` lives in `src/cn.ts`                                       |

The package ships its sources; the app's Vite build compiles them. It has no barrel file: every module has its own subpath in the `exports` of [`package.json`](package.json).

## Commands

The tasks come from [`.moon/tasks/typescript.yml`](../../.moon/tasks/typescript.yml), shared by every TypeScript project.

| Command                 | What it does                                                             |
| ----------------------- | ------------------------------------------------------------------------ |
| `moon run ui:lint`      | Lint with Biome                                                          |
| `moon run ui:check`     | Run `lint` and `typecheck`, then check formatting and imports with Biome |
| `moon run ui:typecheck` | Check the types with `tsc`                                               |
| `moon run ui:test`      | Run the tests with `bun test`                                            |
| `moon run ui:format`    | Format the code and organize imports with Biome                          |

## Adding a component

Components come from [solid-ui](https://www.solid-ui.com), the Solid port of shadcn/ui, through its CLI. Run each step in `libs/ui/`:

1. Add the component: `bunx solidui-cli@0.7.2 add <name>`. With bun, the CLI writes the file but then fails to install the component's dependencies, so add one component per run.
2. Install the dependencies the component imports, if they're missing: `bun add <pkg>`.
3. Rewrite the CLI's `src/…` imports to `./`: `perl -pi -e 's#from "src/#from "./#' src/<name>.tsx`.
4. Format: `moon run ui:format`.
5. Check that interactive parts stay at least 44 px tall and wide, as `button-variants.ts` does with `min-h-11 min-w-11`. Keep `cva` variants in their own `<name>-variants.ts`, so `bun test` can test them: it can't load Solid JSX.
6. Check: `moon run ui:check`.

The CLI writes Tailwind CSS v3 classes. Most work in v4 unchanged; fix the ones that don't.

## Local rules

- Follow the `typescript-conventions` skill.
- Components stay presentational: texts and data come in through props and children. No data loading, no Effect, no built-in texts.
- Add components only with the solid-ui CLI, as above, and import each through its own subpath.
- Import within the library with `./`. Unlike the frontend, the library has no `~/` alias, so its files resolve the same way inside any app.
- Dark mode follows `prefers-color-scheme`. Change colours through the tokens in `src/styles.css`, not in components.
