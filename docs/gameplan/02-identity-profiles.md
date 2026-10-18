# 02 — Identity & profiles

**Kind:** vertical · **Status:** pending

## Requirements
- Stage 01 — monorepo, gRPC pipeline, BFF and local OIDC IdP must exist.

## Desired outcome
Users can sign in through an external OIDC provider and manage their own profile, including an optional host location.

## Scope
### In scope
- OIDC login/logout via the SolidStart BFF (authorization code + PKCE), with a server-side session and an HTTP-only cookie.
- Passing the user's identity from the BFF to the backend (forwarded token or service token + user claims), validated by the backend.
- Creating the user record on first login, linked to the IdP subject.
- Profile: display name, avatar, preferred locale, contact email.
- Host location on the profile: address and practical notes (parking, access). Used later by activities and accommodations.
- Base authorization context in the backend (current user available to every RPC), and guards for authenticated routes.
- Account deletion / data export basics (GDPR-minded).

### Out of scope
- Households, managed profiles and acting on behalf of others (stage 03).
- Contacts (stage 04).
- Event-level roles such as organizer and host (stage 05).

## Milestones
1. **IdP integration**: done when a user can sign up / sign in and sign out through the chosen OIDC provider in the local environment.
2. **Session & identity propagation**: done when every backend RPC receives a verified user identity and rejects unauthenticated calls.
3. **User & profile**: done when users can view and edit their name, avatar and locale, and the UI switches to their locale.
4. **Host location**: done when users can add, edit and remove a location on their profile.
5. **Account lifecycle**: done when users can delete their account and export their data.

## Open questions
- Which OIDC provider: self-hosted (Keycloak, Zitadel, Authentik) or hosted? Decide at the start of this stage.
- Does the BFF forward the user's access token, or use its own service credentials plus asserted user claims?
- One location per host, or several (e.g. house + holiday home)? Starting assumption: one.
