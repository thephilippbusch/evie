---
name: typescript-conventions
description: Sets evie's TypeScript conventions that the linter and formatter don't enforce, for structure, naming, types and tests with bun, each with an example. Use when writing or reviewing TypeScript code, or adding a module, component, type or test.
---

# TypeScript conventions

Rules the TypeScript compiler, linter and formatter don't check. For everything else, follow their output.
Stack-specific patterns belong to their own skills (once they exist): Effect services, layers, `Config`, `Schema` and errors in `effect-service`, components in `shadcn-component`, backend calls in `grpc-rpc`.

| Read                                               | When you                                             |
| -------------------------------------------------- | ---------------------------------------------------- |
| [references/structure.md](references/structure.md) | add a file or folder, or decide what to export       |
| [references/naming.md](references/naming.md)       | name a file, type, function, variable or constant    |
| [references/types.md](references/types.md)         | model data, handle a union or take in external input |
| [references/testing.md](references/testing.md)     | add or run a test                                    |

## Structure

- Group code by feature in `src/<feature>/` (`events/`, `bookings/`); helpers that belong to no feature go in `src/lib/`.
- Route files in `src/routes/` stay thin: they load data and render components from feature folders.
- One concept per module: a file exports one main thing plus its own types. A second concept gets its own file.
- Named exports only. Default exports only where a framework requires them, such as SolidStart route files.
- No barrel `index.ts` files; import from the file that defines the thing.
- Import across folders with the `~/` alias and within a folder with `./`; never `../`.
- Export only what another module uses.

## Naming

- Files and folders are kebab-case: `booking-form.tsx`, `free-places.ts`.
- Types and components are PascalCase; functions, variables and parameters camelCase.
- Module-level primitive constants are UPPER_SNAKE_CASE; objects, arrays and functions stay camelCase.
- Acronyms are words: `eventId`, `IcsExport`, not `eventID`, `ICSExport`.
- Functions start with a verb (`createBooking`); booleans read as a question (`isFull`, `canBook`).
- No type prefixes or suffixes: `Booking`, not `IBooking`, `TBooking` or `BookingType`.

## Types

- Use `type` for object shapes; `interface` only when it must be extended or merged.
- Use string literal unions, not `enum`.
- Model variants as a discriminated union on `_tag`, as Effect does, not as optional fields.
- Close every `switch` over a union with `default: return value satisfies never;`.
- Never cast external data with `as`; decode it with `Schema` where it enters. `as const` is fine.
- Treat data as immutable: `readonly` fields and `ReadonlyArray`; return new values instead of mutating arguments.

## Testing

- Use `bun test` with imports from `bun:test`.
- A test file sits next to its source: `free-places.ts` and `free-places.test.ts`.
- `describe` names the unit; `test` states the behaviour as a sentence. Use `test`, not `it`.
- Test through a module's exports; don't export internals for a test.
- Don't `mock.module` your own code; pass the dependency in instead.
- Run tests with `moon run <project>:test`; add `-- -t "<name>"` for one test. Use the `moon-run` skill.
