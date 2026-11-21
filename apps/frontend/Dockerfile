# syntax=docker/dockerfile:1
# Builds the frontend from a moon scaffold, so this file lists no libraries
# and no tool versions. Run from the repository root:
#   docker build -f apps/frontend/Dockerfile .

# moon, at the version pinned in .prototools. `moon docker setup` installs
# proto and bun itself, at their pinned versions.
FROM debian:trixie-slim AS base
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl git xz-utils \
    && rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.moon/bin:/root/.proto/shims:/root/.proto/bin:$PATH"
WORKDIR /app
COPY .prototools ./
RUN curl -fsSL https://moonrepo.dev/install/moon.sh \
    | bash -s -- "$(sed -n 's/^moon = "\(.*\)"$/\1/p' .prototools)"

# The skeleton: moon copies the manifests and lockfiles of the whole workspace
# to .moon/docker/configs, and the sources of the frontend and its `dependsOn`
# to .moon/docker/sources.
FROM base AS skeleton
COPY . .
RUN moon docker scaffold frontend

# Installs bun and the dependencies while only the manifests are here, so a
# source change reuses that layer. No `moon docker prune`: the build bundles
# the server and its dependencies into .output, and the runtime stage copies
# only that and the bun binary.
FROM base AS builder
COPY --from=skeleton /app/.moon/docker/configs .
# Package bins such as vite start with `#!/usr/bin/env node`; bun runs them,
# as the official bun images do with the same link. The link exists only in
# this build stage; the runtime stage gets bun alone.
RUN moon docker setup \
    && ln -s "$(command -v bun)" /usr/local/bin/node
COPY --from=skeleton /app/.moon/docker/sources .
RUN moon run frontend:build \
    && mkdir -p /out \
    && cp -L "$(command -v bun)" /out/bun

FROM debian:trixie-slim
RUN useradd --system --uid 10001 --create-home evie
COPY --from=builder /out/bun /usr/local/bin/bun
WORKDIR /app
COPY --from=builder /app/apps/frontend/.output .output
ENV NODE_ENV=production \
    HOST=0.0.0.0 \
    PORT=3000
USER evie
EXPOSE 3000
CMD ["bun", ".output/server/index.mjs"]
