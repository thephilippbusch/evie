# 10 — Deployment & operations

**Kind:** horizontal · **Status:** pending

## Requirements
- Stage 00 — release tags, the release workflow outputs and subagents that run headless must exist.
- Stage 01 — Docker images and CI must exist.

This stage can run in parallel with the feature stages. It should be finished before the first real users are invited.

## Desired outcome
evie runs in a production environment that is deployed automatically, monitored and backed up.

## Scope
### In scope
- Choosing a hosting target (VPS, container platform or Kubernetes) and a managed vs. self-hosted PostgreSQL.
- Environments: staging and production, with configuration and secret management.
- CD pipeline: build, publish images, run migrations, deploy on merge or tag. When release-please creates a release (stage 00), every app image is published tagged with the release version.
- BetterAuth secrets and trusted origins, email provider and web push keys.
- TLS, domain and reverse proxy.
- Observability: structured logs, metrics, tracing across BFF and backend, uptime alerts.
- Database backups with tested restore.
- Basic security hardening and dependency updates.
- Automatically triggered agentic workflows, using the stage 00 skills and subagents headless in GitHub Actions:
  - refine new GitHub issues;
  - review every PR when it is opened or updated;
  - nightly improvement PRs;
  - vulnerability screening that raises fix PRs.
- Guardrails for unattended agent runs: they only open PRs or comments and never push to `main`, use scoped tokens, are gated by labels where needed, and stay within a budget.

### Out of scope
- Mobile app distribution.
- Multi-region or high-availability setups beyond what the chosen host offers.

## Milestones
1. **Hosting decision**: done when the hosting target, database option and estimated costs are documented and agreed.
2. **Staging environment**: done when every merge to main deploys to staging automatically, including migrations.
3. **Production environment**: done when production runs with TLS, secrets (including BetterAuth), separate database roles for the BFF and backend, and an email provider, and deploys from tagged releases (`vX.Y.Z`, see stage 00).
4. **Observability & backups**: done when logs, metrics and alerts are in place and a backup restore has been tested.
5. **Agentic automation**: done when issue refinement, PR review, nightly improvement PRs and vulnerability screening run on their triggers and only ever open PRs or comments.

## Open questions
- Hosting target and budget.
