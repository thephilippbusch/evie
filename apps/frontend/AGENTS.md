# frontend

## Purpose

evie's SolidStart web app, used by everyone in the browser. Its server is the BFF: server functions call the backend over gRPC.
For now it renders one page inside the app shell; the backend call comes with the [v0.2 milestone](https://github.com/thephilippbusch/evie/milestone/2) issues.

## Structure

| Path                   | Contents                                                                                        |
| ---------------------- | ----------------------------------------------------------------------------------------------- |
| `src/app.tsx`          | The root component: the router with the file routes, inside the app shell                       |
| `src/shell/`           | The app shell: the layout around every page, such as the header and the main column             |
| `src/routes/`          | File routes; `index.tsx` is `/`                                                                 |
| `src/entry-server.tsx` | The HTML document rendered on the server                                                        |
| `src/entry-client.tsx` | Mounts the app in the browser                                                                   |
| `src/global.d.ts`      | Loads the SolidStart types for the whole app                                                    |
| `public/`              | Static files served as they are                                                                 |
| `vite.config.ts`       | Vite with the SolidStart, Tailwind CSS and Nitro plugins                                        |
| `tsconfig.json`        | Compiler options for Solid and the `~/` alias for `src/`; extends the root `tsconfig.base.json` |
| `.output/`             | The production build (`bun .output/server/index.mjs`), not committed                            |
| `Dockerfile`           | The image: builds the app from a moon scaffold and runs `.output` with bun on port 3000         |

Biome and TypeScript are dev dependencies of the root `package.json`, and `biome.json` at the root configures Biome. Dependencies install into the root `bun.lock`.

## Commands

`build` is defined in [`moon.yml`](moon.yml); the other tasks come from [`.moon/tasks/typescript.yml`](../../.moon/tasks/typescript.yml), shared by every TypeScript project.

| Command                       | What it does                                                             |
| ----------------------------- | ------------------------------------------------------------------------ |
| `moon run frontend:build`     | Build the app for production with Vite                                   |
| `moon run frontend:lint`      | Lint with Biome                                                          |
| `moon run frontend:check`     | Run `lint` and `typecheck`, then check formatting and imports with Biome |
| `moon run frontend:typecheck` | Check the types with `tsc`                                               |
| `moon run frontend:test`      | Run the tests with `bun test`                                            |
| `moon run frontend:format`    | Format the code and organize imports with Biome                          |

For a dev server, run `bun run dev` in `apps/frontend/`.
To run the app with the rest of the stack, run `docker compose up` at the repository root and open `http://127.0.0.1:3000`. Build the image alone from the repository root, which is the build context: `docker build -f apps/frontend/Dockerfile .`.

## Local rules

- Follow the `typescript-conventions` skill.
- Add packages with `bun add <pkg>` inside `apps/frontend/`; shared dev tools go in the root `package.json`.
- When the app uses a new library from `libs/`, add it to `dependsOn` in `moon.yml`. `moon docker scaffold` copies only the sources of the app and its `dependsOn` into the image. The image tags carry the bun version, so change them together with `.prototools`.
- UI primitives, design tokens and `cn` come from [`@evie/ui`](../../libs/ui/AGENTS.md), each through its subpath, such as `@evie/ui/button`. Add a missing component there with the `ui-component` skill, not here.
- Layout that only this app uses, such as the app shell, stays here.
- Keep touch targets at least 44 px (`min-h-11`) and build mobile-first: base classes for phones, `sm:` and up for larger screens.
