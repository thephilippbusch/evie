# ui

## Purpose

evie's UI library, `@evie/ui`: presentational components, the design tokens and the Tailwind CSS setup. The [frontend](../../apps/frontend/AGENTS.md) uses it; the library is never deployed on its own.

## Structure

| Path             | Contents                                                                                                          |
| ---------------- | ----------------------------------------------------------------------------------------------------------------- |
| `src/styles.css` | Tailwind CSS, the design tokens (shadcn names, light and dark) and base styles; imported as `@evie/ui/styles.css` |
| `src/cn.ts`      | `cn`, which joins class names and resolves conflicting Tailwind classes; imported as `@evie/ui/cn`                |

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

## Local rules

- Follow the `typescript-conventions` skill.
- Components stay presentational: texts and data come in through props and children. No data loading, no Effect, no built-in texts.
- Import within the library with `./`. Unlike the frontend, the library has no `~/` alias, so its files resolve the same way inside any app.
- Dark mode follows `prefers-color-scheme`. Change colours through the tokens in `src/styles.css`, not in components.
