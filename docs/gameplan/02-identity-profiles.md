# 02 — Identity & profiles

**Kind:** vertical · **Status:** pending

## Requirements
- Stage 01 — monorepo, gRPC pipeline, BFF and the BetterAuth foundation must exist.

## Desired outcome
Users can create an account with BetterAuth, verify their email, recover their password and manage their own profile, including an optional host location.

## Scope
### In scope
- Complete BetterAuth account flows in the SolidStart BFF: email verification on sign-up and password reset, sent through the dev mail sink locally (production email comes with stages 06/10).
- Passing the user's identity from the BFF to the backend, validated by the backend.
- A backend user record linked to the BetterAuth user ID, stored as an opaque value without a cross-schema foreign key.
- Profile: display name, avatar, preferred locale, contact email.
- Host location on the profile: address and practical notes (parking, access). Used later by activities and accommodations.
- Base authorization context in the backend (current user available to every RPC), and guards for authenticated routes.
- Account deletion / data export basics (GDPR-minded). Deleting a user runs a BetterAuth `user.delete` hook that calls a backend RPC to remove domain data; there are no cross-schema cascades.

### Out of scope
- Households, managed profiles and acting on behalf of others (stage 03).
- Contacts (stage 04).
- Event-level roles such as organizer and host (stage 05).
- External identity providers (OIDC/social login).

## Milestones
1. **Account flows**: done when sign-up includes email verification and users can reset their password.
2. **Session & identity propagation**: done when every backend RPC receives a verified user identity and rejects unauthenticated calls.
3. **User & profile**: done when users can view and edit their name, avatar and locale, and the UI switches to their locale.
4. **Host location**: done when users can add, edit and remove a location on their profile.
5. **Account lifecycle**: done when users can delete their account and export their data.

## Open questions
- How does the backend verify identity? Either the BFF forwards a BetterAuth-issued JWT (JWT plugin with a JWKS endpoint the Rust backend verifies), or it uses its own service credentials plus asserted user claims.
- Which profile fields live in BetterAuth's `user` table (name, image, email) and which in the domain schema (locale, host location)?
- One location per host, or several (e.g. house + holiday home)? Starting assumption: one.
