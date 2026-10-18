# 01 — Platform scaffolding

**Kind:** horizontal · **Status:** pending

## Requirements
- None. This is the root stage.

## Desired outcome
A working monorepo where a SolidStart page renders data fetched from a Rust gRPC service backed by PostgreSQL, built and checked in CI.

## Scope
### In scope
- moonrepo workspace with tasks for build, lint, format, test and codegen across all projects.
- Rust workspace: Tokio + Tonic gRPC server, SeaORM entities, migration crate, configuration and structured logging.
- Shared protobuf definitions with code generation for Rust (tonic-build) and TypeScript (BFF client).
- SolidStart app with shadcn/ui (Solid port), base layout, theming and a design-token foundation.
- BFF pattern: SolidStart server functions call the backend over native gRPC; the browser never talks gRPC directly.
- i18n framework with German and English locales, locale detection and switching.
- Local dev environment via docker-compose: PostgreSQL, a local OIDC IdP for development, the backend and the frontend.
- CI pipeline: lint, format check, tests, build and Docker image build (no publishing).
- Basic test setup for Rust (unit + DB integration) and frontend (unit + component).

### Out of scope
- Real authentication flows (stage 02).
- Domain features.
- Production hosting and deployment (stage 10).
- Mobile clients.

## Milestones
1. **Monorepo & toolchain**: done when `moon` can run build, lint and test for an empty Rust workspace and an empty SolidStart app.
2. **Backend skeleton**: done when a Tonic service with a health/ping RPC runs, connects to PostgreSQL via SeaORM and runs migrations on start or via a task.
3. **Protobuf pipeline**: done when changing a `.proto` file regenerates the Rust and TS code through a moon task and CI fails on stale generated code.
4. **Frontend shell & BFF**: done when a SolidStart page renders the result of a server function that calls the ping RPC, using shadcn/ui components and the app layout.
5. **i18n**: done when every UI string goes through the i18n layer and the shell is fully translated into German and English, with a language switcher.
6. **Dev environment & CI**: done when `docker compose up` starts the whole stack locally and CI runs lint, test and build on every PR.
