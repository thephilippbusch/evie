# 06 — Notifications foundation

**Kind:** horizontal · **Status:** pending

## Requirements
- Stage 02 — users, locales and contact emails must exist.

## Desired outcome
Any backend domain can reliably send localized email and web push notifications that respect user preferences.

## Scope
### In scope
- Notification outbox in PostgreSQL (written in the same transaction as the domain change) and a background worker that delivers it.
- Email transport (SMTP or a provider API) with German and English templates, plain-text fallback and an accessible layout.
- Web push (VAPID) for the responsive web app: subscription management and delivery.
- In-app notification list with read/unread state.
- Per-user preferences per notification type and channel.
- Retries, failure handling and a dev mail catcher in docker-compose.
- Retrofitting contact requests (stage 04) and household invites (stage 03) to send notifications, if those stages are done.

### Out of scope
- Domain-specific notification content for invitations, accommodations and rides (added by stages 07–09).
- SMS or native mobile push.
- Reminders scheduled relative to event dates (added by stage 07).

## Milestones
1. **Outbox & worker**: done when a domain service can enqueue a notification transactionally and a worker delivers it with retries.
2. **Email channel**: done when localized templated emails arrive in the dev mail catcher and in a staging provider.
3. **Web push channel**: done when users can enable browser notifications and receive a test push.
4. **In-app inbox & preferences**: done when users see their notifications in the app and can turn types or channels on and off.
