# Issues

The `issue` skill sets the structure: title, sections, labels and links. This file covers the sentences in them.

## Title: the intent, starting with a verb

Before:

> Problems with the event page when rescheduling stuff

After:

> Keep booked nights when an event is rescheduled

## Description: who gets what, in two sentences at most

Before:

> As discussed, it would be really nice if we could maybe think about some kind of way for organizers to work on events before everybody can see them, since right now that's not possible and it's a bit annoying.

After:

> Organizers can create an event as a draft and edit it before publishing. Drafts are visible only to organizers.

## Acceptance Criteria: testable by a person

Before:

> - The `EventStatus` enum has a `Draft` variant and `publish_event` is implemented properly.

After:

> - An organizer can save an event as a draft and publish it later.
> - An invited person can't see a draft.

## Technical Details: short bullets

Before:

> I think we'll probably need to touch the events table, and also the RPC, and there might be something in the BFF too, not sure yet.

After:

> - `events.status` column with `draft`, `published`, `cancelled`; migration in the backend.
> - `PublishEvent` RPC; the BFF server function calls it.
> - Depends on #20.
