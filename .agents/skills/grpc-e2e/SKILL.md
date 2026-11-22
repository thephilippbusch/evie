---
name: grpc-e2e
description: Adds or changes one gRPC RPC end to end, from the protobuf definition over the Rust handler and the BFF client service to the server function and page that show its result. Use when a page needs backend data, or when asked to add a backend call, an RPC, or a gRPC service or method.
---

# gRPC end to end

A workflow: one RPC through every layer, bottom to top. The browser never talks gRPC; it calls a server function, which calls the backend through an Effect client service.

```text
browser ─> server function ─> client service ─gRPC─> tonic service ─> domain service ─> PostgreSQL
           apps/frontend      apps/frontend          apps/backend      apps/backend
           src/<feature>/     src/<feature>/         src/grpc/         src/<feature>/
```

| Read                                           | For step |
| ---------------------------------------------- | -------- |
| [references/proto.md](references/proto.md)     | 1        |
| [references/backend.md](references/backend.md) | 3        |
| [references/bff.md](references/bff.md)         | 4        |
| [references/ui.md](references/ui.md)           | 5        |

## Steps

Each step ends with its check passing. If a check fails, fix it before the next step. Commit each step with the `commit` skill, so every commit builds and passes its tests.

1. **Proto.** Add or change the RPC in `libs/proto/definitions/evie/<area>/v<n>/<area>.proto`, then run `moon run proto:codegen`. Check: `moon run proto:check`.
2. **Table.** Only if the RPC stores new data: use the `seaorm-migration` skill.
3. **Backend.** Implement the generated service trait, map the domain errors to `tonic::Status`, register the service in `apps/backend/src/grpc.rs` and add an integration test. Check: `moon run backend:check backend:test`.
4. **BFF client.** Add a client service with the `effect-service` skill: it calls the RPC over Connect, decodes the response with `Schema` and turns status codes into tagged errors. Check: `moon run frontend:check frontend:test`.
5. **UI.** Add a server function that runs the client service, and a route that renders its result. Use the `ui-component` skill for a component `@evie/ui` doesn't have yet. Check: `moon run frontend:build`, then the page in the running app, with the backend up and down.
6. Use the `verify` skill at the whole-branch scope.

## First RPC

The first RPC of the app sets up what every later one reuses. The First RPC section of each step's reference lists it:

- Step 3: [references/backend.md](references/backend.md#first-rpc)
- Step 4: [references/bff.md](references/bff.md#first-rpc)

## Rules

- Never hand-edit `libs/proto/gen/`; change the definitions and run `codegen`.
- Never reuse or renumber a field number; mark a removed field `reserved`.
- Domain code never builds a `tonic::Status`; one `From` impl per domain error does, at the gRPC edge.
- The browser never imports `@evie/proto`, `@connectrpc/*`, services or `appRuntime`; only server function bodies use them. The `effect-service` skill shows how to check the built bundle.
- An RPC that returns or changes someone's data needs the verified user and an access check on both sides: the server function checks the session, and the backend returns `permission_denied` or `not_found` to a user without access. #34 (_planned_) defines how the BFF passes the user; until then, add only RPCs for public data.
- Every text the page shows exists in German and English.

## Output

One commit per step, with the checks of each step and the `verify` skill passing, and the page showing the RPC's result.
