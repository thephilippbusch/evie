# frontend

## Purpose

evie's SolidStart web app, used by everyone in the browser. Its server is the BFF: server functions call the backend over gRPC.
For now it renders one empty page; layout, theming and the backend call come with the [v0.2 milestone](https://github.com/thephilippbusch/evie/milestone/2) issues.

## Structure

| Path                   | Contents                                                             |
| ---------------------- | -------------------------------------------------------------------- |
| `src/app.tsx`          | The root component                                                   |
| `src/entry-server.tsx` | The HTML document rendered on the server                             |
| `src/entry-client.tsx` | Mounts the app in the browser                                        |
| `public/`              | Static files served as they are                                      |
| `vite.config.ts`       | Vite with the SolidStart and Nitro plugins                           |
| `.output/`             | The production build (`bun .output/server/index.mjs`), not committed |

Biome and TypeScript are dev dependencies of the root `package.json`, and `biome.json` at the root configures Biome. Dependencies install into the root `bun.lock`.

## Commands

`build` is defined in [`moon.yml`](moon.yml); the other tasks come from [`.moon/tasks/typescript.yml`](../../.moon/tasks/typescript.yml), shared by every TypeScript project.

| Command                       | What it does                                     |
| ----------------------------- | ------------------------------------------------ |
| `moon run frontend:build`     | Build the app for production with Vite           |
| `moon run frontend:lint`      | Lint and check formatting and imports with Biome |
| `moon run frontend:typecheck` | Check the types with `tsc`                       |
| `moon run frontend:test`      | Run the tests with `bun test`                    |
| `moon run frontend:format`    | Format the code and organize imports with Biome  |

For a dev server, run `bun run dev` in `apps/frontend/`.

## Local rules

- Follow the `typescript-conventions` skill.
- Add packages with `bun add <pkg>` inside `apps/frontend/`; shared dev tools go in the root `package.json`.
