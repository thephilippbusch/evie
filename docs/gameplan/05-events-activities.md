# 05 — Events & activities

**Kind:** vertical · **Status:** pending

## Requirements
- Stage 02 — users, profiles and host locations must exist.

## Desired outcome
Organizers can create events with one or more activities, assign co-organizers and hosts, and move events through their lifecycle.

## Scope
### In scope
- Event: title, description, date range, cover image, timezone.
- Lifecycle: draft → published → past (automatic after the end date) / cancelled. Drafts are only visible to organizers.
- Roles per event: multiple organizers (full admin), hosts (participants who provide a location; can manage the activities they host).
- Activities: title, description, start/end time, location as one of: a host's profile location, a free-text address, or a link (online).
- Host-scoped management: hosts can edit activities at their location, organizers can edit everything.
- Default attendance settings per event (e.g. attendees join all activities by default, or must opt in per activity).
- Event overview pages: list of my events (organizing / hosting / attending), event detail with activity timeline.

### Out of scope
- Inviting people and RSVPs (stage 07). In this stage, organizers and hosts are added directly from existing users.
- Accommodations (stage 08).
- Recurring events, templates, ticketing.

## Milestones
1. **Event CRUD & lifecycle**: done when an organizer can create a draft, edit it, publish, cancel, and see it marked as past after its end date.
2. **Activities**: done when events hold 1..n activities with all three location types, and an event cannot be published without at least one activity.
3. **Roles**: done when organizers can add and remove co-organizers and hosts, and permissions are enforced (organizers: everything; hosts: their activities).
4. **Attendance defaults**: done when organizers can configure the default attendance setting that stage 07 uses.
5. **Event views**: done when users see their events grouped by role and a readable, accessible event detail page in both languages.

## Open questions
- What happens when an activity is removed or rescheduled after people signed up or booked accommodations? (Shared with stage 07.)
- Can the last organizer leave an event? Starting assumption: no, they must hand over first.
