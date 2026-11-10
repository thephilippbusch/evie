# Planning a feature

New or changed behaviour, for people using evie or for contributors and agents. Agent files and tooling count as features here, unless they are about CI.

## Find out first

1. List what a user can do once the feature lands, per role, from the issue's Acceptance Criteria. Include who must not see or do it.
2. Find the layers it touches, top to bottom. Read the code and the `AGENTS.md` of each layer's project.
3. Check what exists already: gRPC services, tables, server functions and components to extend instead of adding new ones.
4. For agent files and tooling, read the `authoring-skills` skill and the files that link to the ones you change.

## Plan sections

Add these under Approach, and leave out the layers the feature doesn't touch.

### Layers

Plan them in this order; each one builds on the one before.

| Layer       | What to plan                                                                                            |
| ----------- | ------------------------------------------------------------------------------------------------------- |
| proto       | new or changed messages and RPCs; run `codegen` after, never hand-edit the output                       |
| backend     | SeaORM entities, a reviewed migration for every schema change, the service logic and its errors         |
| BFF         | SolidStart server functions that call the backend over gRPC, with Effect services and `Schema` decoding |
| UI          | routes and components; the browser talks only to server functions, never gRPC                           |
| agent files | skills, subagents and `AGENTS.md` files, with the `authoring-skills` checklist and index updates        |

### Texts

Every user-facing text in German and English. List the new texts and where they live.

### Permissions

Per role: who can see and do what, and where the check runs. Missing permission edge cases get a test.

### Acting for others

From v0.5 on, participation features support acting for household members and managed profiles. Plan how the actor is chosen and how the backend records who acted. Before v0.5, leave this section out.

### Tests

One line per layer: the test file and what it covers. Test behaviour through each layer's public interface, with the `rust-conventions` and `typescript-conventions` skills.

Under Order, one commit per layer, bottom to top, so every commit builds and passes its tests.

## Verify

- Every acceptance criterion maps to a test or a manual check named under Verification.
- `moon run :codegen` leaves no diff, and `moon run :migrate` applies the new migrations on a fresh database.
- Agent files pass `moon run root:validate-agent-files`.
- For UI changes, check the feature in the running app in German and in English.
