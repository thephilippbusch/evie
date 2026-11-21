# ui

## Purpose

evie's own component library, `@evie/ui`: presentational components built on [Kobalte](https://kobalte.dev), the design tokens and the Tailwind CSS setup. The [frontend](../../apps/frontend/AGENTS.md) uses it; the library is never deployed on its own.

## Structure

| Path                    | Contents                                                                                            |
| ----------------------- | --------------------------------------------------------------------------------------------------- |
| `src/styles.css`        | Tailwind CSS, the design tokens (light and dark) and base styles; imported as `@evie/ui/styles.css` |
| `src/<name>/<name>.tsx` | One component and its parts, such as `src/button/button.tsx`; imported as `@evie/ui/<name>`         |
| `src/<name>/`           | Everything else for that component: its variants (`<name>-variants.ts`), tests and helpers          |
| `src/utils/`            | Library-wide helpers, such as `cn`; imported as `#utils/<name>` inside the library                  |
| `tsconfig.json`         | Compiler options for Solid and bun; extends the root `tsconfig.base.json`                           |

The package ships its sources; the app's Vite build compiles them. It has no barrel file: the `./*` export of [`package.json`](package.json) maps `@evie/ui/<name>` to `src/<name>/<name>.tsx`, and `@evie/ui/cn` to `src/utils/cn.ts`.

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

Use the `ui-component` skill.

## Local rules

- Follow the `typescript-conventions` skill.
- Components stay presentational: texts and data come in through props and children. No data loading, no Effect, no built-in texts.
- Build interactive components on Kobalte primitives, and import each through its own subpath.
- Import within a folder with `./` and from `src/utils/` with `#utils/<name>`; never `../`. `#utils/*` is a package `imports` alias, so it resolves the same way inside any app, in `tsc` and in `bun test`.
- Dark mode follows `prefers-color-scheme`. Change colours through the tokens in `src/styles.css`, not in components.
