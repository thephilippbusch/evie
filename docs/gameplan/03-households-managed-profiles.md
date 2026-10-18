# 03 — Households & managed profiles

**Kind:** vertical · **Status:** pending

## Requirements
- Stage 02 — users, profiles and the authorization context must exist.

## Desired outcome
Users can form households and create managed profiles for non-digital people, and the backend has an "act on behalf of" model that later stages use.

## Scope
### In scope
- Households: create, rename, invite users, accept/decline, leave, remove members. A user belongs to at most one household.
- Managed profiles: a registered user creates a profile (name, optional avatar, notes) for a person without an account. A managed profile can belong to a household.
- Managers: who may act for a managed profile (its creator, and optionally the creator's household members).
- Act-on-behalf model: every write action records an **actor** (who did it) and a **subject** (for whom). Authorization rules: users act for themselves, for members of their household, and for managed profiles they manage.
- Claiming: the manager sends a claim invite to an email address; after sign-up the managed profile and its history move to the new account.
- UI for switching or selecting "acting as" in a clear, family-friendly way.

### Out of scope
- Using the on-behalf model in event flows (stages 07–09 apply it).
- Contacts (stage 04).

## Milestones
1. **Households**: done when users can create a household, invite others by email or existing account, and members can accept, leave or be removed, with the one-household rule enforced.
2. **Managed profiles**: done when a user can create, edit and delete managed profiles and attach them to their household.
3. **Act-on-behalf authorization**: done when the backend has a reusable actor/subject check, tested for self, household members and managed profiles, and actions are audited with both.
4. **Acting-as UI**: done when the frontend offers a reusable "for whom?" picker that later stages can drop into forms.
5. **Claiming**: done when a managed profile can be claimed by a new account and keeps its data and history.

## Open questions
- Can household members act on behalf of each other for everything, or only for event participation (RSVP, bookings)?
- What happens to managed profiles when their manager leaves the household or deletes their account?
