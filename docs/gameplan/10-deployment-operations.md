# 10 — Deployment & operations

**Kind:** horizontal · **Status:** pending

## Requirements
- Stage 01 — Docker images and CI must exist.

This stage can run in parallel with the feature stages. It should be finished before the first real users are invited.

## Desired outcome
evie runs in a production environment that is deployed automatically, monitored and backed up.

## Scope
### In scope
- Choosing a hosting target (VPS, container platform or Kubernetes) and a managed vs. self-hosted PostgreSQL.
- Environments: staging and production, with configuration and secret management.
- CD pipeline: build, publish images, run migrations, deploy on merge or tag.
- Production OIDC IdP setup (if self-hosted), email provider and web push keys.
- TLS, domain and reverse proxy.
- Observability: structured logs, metrics, tracing across BFF and backend, uptime alerts.
- Database backups with tested restore.
- Basic security hardening and dependency updates.

### Out of scope
- Mobile app distribution.
- Multi-region or high-availability setups beyond what the chosen host offers.

## Milestones
1. **Hosting decision**: done when the hosting target, database option and estimated costs are documented and agreed.
2. **Staging environment**: done when every merge to main deploys to staging automatically, including migrations.
3. **Production environment**: done when production runs with TLS, secrets, a production IdP and an email provider, and deploys from tagged releases.
4. **Observability & backups**: done when logs, metrics and alerts are in place and a backup restore has been tested.

## Open questions
- Hosting target and budget.
- Self-hosted vs. managed IdP (depends on the decision in stage 02).
