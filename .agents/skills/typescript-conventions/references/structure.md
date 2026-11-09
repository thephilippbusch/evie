# Structure

## Feature folders

Group by feature, not by kind of file. Routes stay thin and import from the features.

```text
src/
├── routes/                     # SolidStart file routes; thin, default exports
│   └── events/
│       └── [eventId].tsx
├── events/
│   ├── event-card.tsx
│   ├── event-card.test.tsx
│   └── event-dates.ts
├── bookings/
│   ├── booking-form.tsx
│   ├── free-places.ts
│   └── free-places.test.ts
└── lib/                        # helpers that belong to no feature
    └── dates.ts
```

Not this:

```text
src/
├── components/
├── hooks/
├── types/
└── utils/
```

UI primitives from shadcn/ui follow their own skill, `shadcn-component` (once it exists).

## One concept per module

A file exports one main thing, plus the types that belong to it. When a second concept shows up, move it to its own file.

```ts
// src/bookings/free-places.ts
export type Capacity = {
  readonly places: number;
  readonly booked: number;
};

export function freePlaces({ places, booked }: Capacity): number {
  return Math.max(places - booked, 0);
}
```

## Exports

Named exports only. Default exports only where a framework requires one, such as a SolidStart route:

```tsx
// src/routes/events/[eventId].tsx
import { useParams } from "@solidjs/router";
import { EventCard } from "~/events/event-card";

export default function EventPage() {
  const params = useParams<{ eventId: string }>();
  return <EventCard eventId={params.eventId} />;
}
```

Export only what another module uses. Helpers stay unexported, so they can change freely.

## Imports

- Across folders: the `~/` alias for `src/`, e.g. `import { freePlaces } from "~/bookings/free-places";`.
- Within a folder: `./`, e.g. `import { freePlaces } from "./free-places";`.
- Never `../`: the alias keeps imports stable when files move.
- No barrel `index.ts` files that re-export a folder. Import from the defining file, so the import shows where a thing lives and bundles stay small.

```ts
// wrong
import { freePlaces } from "~/bookings";
import { formatDay } from "../lib/dates";

// right
import { freePlaces } from "~/bookings/free-places";
import { formatDay } from "~/lib/dates";
```
