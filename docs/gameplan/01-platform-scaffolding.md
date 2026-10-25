# 01 — Platform scaffolding

**Kind:** horizontal · **Status:** pending

## Requirements
- Stage 00 — commit conventions, the release setup and the agent setup (root `AGENTS.md`, core skills) must exist.

## Desired outcome
A working monorepo where a SolidStart page renders data fetched from a Rust gRPC service backed by PostgreSQL, a user can sign up, sign in and sign out through BetterAuth, and everything is built and checked in CI.

## Scope
### In scope
- moonrepo workspace with tasks for build, lint, format, test, codegen and database migrations across all projects.
- Apps and libraries are separate moon project types. Crates inherit the workspace version (`version.workspace = true`), and both apps are registered for releases using the stage 00 "Adding an app" checklist.
- Rust workspace: Tokio + Tonic gRPC server, SeaORM entities, migration crate, configuration and structured logging.
- Shared protobuf definitions with code generation for Rust (tonic-build) and TypeScript (BFF client).
- SolidStart app with shadcn/ui (Solid port), base layout, theming and a design-token foundation.
- BFF pattern: SolidStart server functions call the backend over native gRPC; the browser never talks gRPC directly.
- Effect as the frontend foundation: all SolidStart server and shared code is written in idiomatic Effect from the start. Services are `Context.Tag`s provided by `Layer`s, configuration goes through `Config`, input and output are validated and decoded with `Schema`, errors are typed (`Data.TaggedError`), the gRPC client and the database are wrapped as Effect services, and server functions run through one shared `ManagedRuntime`. Tests use `@effect/vitest`.
- BetterAuth for authentication: BetterAuth server and Solid client integrated into SolidStart, using the standard stateful mode with email/password. Sessions are stored in the database and referenced by an HTTP-only cookie. BetterAuth is exposed to the rest of the app as an Effect service.
- Drizzle for BFF-owned data: BetterAuth persists through the Drizzle adapter into a dedicated `auth` Postgres schema. The Drizzle schema is generated with the BetterAuth CLI and migrated with drizzle-kit through moon tasks. SeaORM keeps ownership of the domain schema.
- Schema ownership guardrails, set before any tables exist:
  - drizzle-kit is configured with `schemaFilter: ["auth"]`. `drizzle-kit push` is allowed only in local dev; everywhere else, migrations are generated and reviewed.
  - No foreign keys between the domain schema and `auth`. The domain stores the BetterAuth user ID as an opaque value.
  - One moon `migrate` task runs the auth migrations, then the domain migrations. Dev, CI and deploy all use it.
  - Separate Postgres roles: the BFF role owns `auth` only; the backend role owns the domain schema and has no access to `auth`.
  - Cleanup across the boundary (e.g. account deletion) goes through explicit BetterAuth hooks that call gRPC, designed in stage 02.
- i18n framework with German and English locales, locale detection and switching.
- Local dev environment via docker-compose: PostgreSQL, a dev mail sink (e.g. Mailpit) for auth emails, the backend and the frontend.
- CI pipeline: lint, format check, tests, build and Docker image build (no publishing).
- Basic test setup for Rust (unit + DB integration) and frontend (unit + component).
- Agent support for the stack:
  - A per-project `AGENTS.md` (with a `CLAUDE.md` symlink) for every moon project, covering its purpose, structure, commands and local rules. The `moon-new-*` skills create it.
  - moon templates that the `moon-new-library` and `moon-new-app` skills use.
  - Stack-specific skills: Effect service, gRPC RPC end to end (proto → Rust → BFF), SeaORM and Drizzle migrations, i18n text, shadcn/ui component.

### Out of scope
- Profiles, identity propagation to the backend, route guards and authorization (stage 02).
- External identity providers (OIDC/social login). These may be added later as BetterAuth plugins.
- Domain features.
- Production hosting and deployment (stage 10).
- Mobile clients.

## Milestones
1. **Monorepo & toolchain**: done when `moon` can run build, lint and test for an empty Rust workspace and an empty SolidStart app.
2. **Backend skeleton**: done when a Tonic service with a health/ping RPC runs, connects to PostgreSQL via SeaORM and runs migrations on start or via a task.
3. **Protobuf pipeline**: done when changing a `.proto` file regenerates the Rust and TS code through a moon task and CI fails on stale generated code.
4. **Frontend shell & BFF**: done when a SolidStart page renders the result of a server function that calls the ping RPC through an Effect service and the app runtime, with typed errors and Schema-decoded responses, using shadcn/ui components and the app layout.
5. **Auth foundation**: done when a user can sign up, sign in and sign out with email/password locally, the auth tables are created in the `auth` schema by the moon `migrate` task, and a server function reads the current session through the auth Effect service.
6. **i18n**: done when every UI string goes through the i18n layer and the shell is fully translated into German and English, with a language switcher.
7. **Dev environment & CI**: done when `docker compose up` starts the whole stack locally and CI runs lint, test and build on every PR, and fails on stale Drizzle migrations.
8. **Agent support for the stack**: done when every project has an `AGENTS.md`, the `moon-new-*` skills generate projects from the moon templates, and the stack-specific skills were used to build the ping RPC end to end.
