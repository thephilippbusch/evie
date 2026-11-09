# Naming

## Files and folders

kebab-case, components included. Tests add `.test` before the extension.

```text
booking-form.tsx        not BookingForm.tsx
free-places.ts          not freePlaces.ts
free-places.test.ts     not free-places.spec.ts
```

## Identifiers

| Kind                                 | Case             | Example                               |
| ------------------------------------ | ---------------- | ------------------------------------- |
| Type, component, class               | PascalCase       | `Booking`, `BookingForm`, `IcsExport` |
| Function, variable, parameter        | camelCase        | `createBooking`, `guestCount`         |
| Module-level primitive constant      | UPPER_SNAKE_CASE | `MAX_GUESTS_PER_PLACE`                |
| Module-level object, array, function | camelCase        | `defaultFilters`, `weekdays`          |

```ts
const MAX_GUESTS_PER_PLACE = 8;

const defaultFilters = {
  onlyFree: false,
  minPlaces: 1,
} as const;
```

## Acronyms are words

Only the first letter is capitalised, whatever the case style.

```ts
type IcsExport = { readonly eventId: string; readonly downloadUrl: string };
// not: ICSExport, eventID, downloadURL
```

## Functions and booleans

Functions start with a verb. Booleans read as a yes/no question: `is`, `has`, `can`, `should`.

```ts
type Place = { readonly capacity: number; readonly booked: number };

function isFull(place: Place): boolean {
  return place.booked >= place.capacity;
}

function canBook(place: Place, guests: number): boolean {
  return place.booked + guests <= place.capacity;
}
```

Not `full(place)`, `bookable`, `checkPlace(place)`.

## No type markers

A type's name is the noun it describes.

```ts
type Booking = { readonly id: string; readonly guests: number };
// not: IBooking, TBooking, BookingType, BookingInterface
```
