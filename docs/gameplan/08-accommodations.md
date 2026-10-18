# 08 — Accommodations

**Kind:** vertical · **Status:** pending

## Requirements
- Stage 07 — attendees, on-behalf participation and event notifications must exist.

## Desired outcome
Hosts can offer overnight accommodations for an event, and attendees get a place to sleep through requests or host assignment.

## Scope
### In scope
- Accommodation units, defined by hosts at their location: flexible types (room, sofa, air mattress, tent spot…), each with a capacity, description and amenities.
- Availability per night, on any dates, including nights before or after the event.
- External tips (hotel, B&B, campsite): informational only, not bookable; attendees can mark "staying there" so organizers have an overview.
- Booking modes per event, set by the organizer: attendee request → host confirms/declines, host assigns, or both.
- Per-night bookings for attendees, including on-behalf bookings for household members and managed profiles.
- Capacity checks per unit and night, and conflict handling.
- Occupancy overview for hosts and organizers (who sleeps where, which night); "my stay" view for attendees.
- Notifications: request received, confirmed, declined, assigned, changed.

### Out of scope
- Payments or cost sharing for accommodations.
- Integration with external booking platforms.
- Pick-up and drop-off (stage 09).

## Milestones
1. **Units & availability**: done when hosts can create units with capacity and per-night availability for an event.
2. **External tips**: done when organizers and hosts can add non-bookable tips and attendees can mark that they are staying there.
3. **Request flow**: done when attendees (or someone acting for them) can request nights in a unit and hosts can confirm or decline within capacity.
4. **Assignment flow**: done when hosts/organizers can assign people to units and nights, and the event's booking mode controls which flows are available.
5. **Overviews & notifications**: done when hosts and organizers see a per-night occupancy plan, attendees see their stay, and every booking change notifies the people involved.

## Open questions
- Can attendees share or swap units among themselves, or does every change go through the host?
- Should hosts be able to block nights for themselves without creating a booking?
