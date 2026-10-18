# 07 — Invitations & attendance

**Kind:** vertical · **Status:** pending

## Requirements
- Stage 03 — households, managed profiles and the act-on-behalf model are needed for family RSVPs.
- Stage 04 — the contact list is a source for invites.
- Stage 05 — events, activities and attendance defaults must exist.
- Stage 06 — invites and RSVP changes are delivered as notifications.

## Desired outcome
Organizers can invite people to published events, and attendees (or someone acting for them) can RSVP to the event and pick their activities.

## Scope
### In scope
- Invite channels: email invite, shareable link (with optional expiry and approval by an organizer), pick from contacts, invite a whole household.
- Invites to unregistered emails: the recipient signs up (account required) and then lands on the event.
- Event RSVP: yes / no / maybe, per person (registered user or managed profile) rather than as an anonymous headcount.
- Activity selection: after RSVPing yes, attendees choose activities; the event's attendance default pre-fills the choice.
- On-behalf RSVP: household members and managers RSVP for others with the "for whom?" picker from stage 03; the audit shows who did it.
- Participant views: organizers see invitees, responses and per-activity headcounts; attendees see who else is coming (configurable).
- Notifications: invite received, RSVP changes (to organizers), event changes and cancellation (to attendees), reminders before the event.
- Handling of changes to activities after people responded.

### Out of scope
- Accommodations (stage 08) and rides (stage 09).
- Waitlists and capacity limits per activity (possible later stage).
- Guests without accounts.

## Milestones
1. **Invitations**: done when organizers can invite by email, link, contact and household, and invitees can accept and sign up if needed.
2. **Event RSVP**: done when invitees can respond yes/no/maybe and change their answer until the event starts.
3. **Activity selection**: done when attendees can choose activities, prefilled from the event's default.
4. **On-behalf participation**: done when users can RSVP and pick activities for household members and managed profiles, with the actor recorded.
5. **Participant overview**: done when organizers and hosts see invitees, responses and headcounts per activity.
6. **Notifications & reminders**: done when invites, RSVP changes, event updates, cancellations and reminders are sent through stage 06.

## Open questions
- What happens to activity choices when an activity is removed or rescheduled? (Shared with stage 05.)
- Can attendees invite others (plus-ones), or only organizers?
