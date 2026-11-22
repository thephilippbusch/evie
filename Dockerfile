# syntax=docker/dockerfile:1
# Every app image, one target each, built from a moon scaffold of that app.
# Run from the repository root:
#   docker build --target frontend .
#   docker build --target backend .
#   docker build --target migrate .   # the one-shot domain migrations

#### BASE: proto and moon, for scaffolding and building. The Rust image has
#### rustup and every tool proto needs. moon comes at the version in
#### .prototools, and `moon docker setup` installs bun and Rust from there too.
#### proto goes where moon looks for its own copy, so moon doesn't download it
#### again: keep PROTO_VERSION at the version moon installs.
FROM rust:1.99.1-trixie AS base
ARG PROTO_VERSION=0.62.2
WORKDIR /app
RUN mkdir -p "/root/.proto/tools/proto/${PROTO_VERSION}" \
    && curl -fsSL "https://github.com/moonrepo/proto/releases/download/v${PROTO_VERSION}/proto_cli-$(uname -m)-unknown-linux-gnu.tar.xz" \
       | tar -xJ --strip-components=1 -C "/root/.proto/tools/proto/${PROTO_VERSION}"
ENV PATH="/root/.proto/shims:/root/.proto/bin:/root/.proto/tools/proto/${PROTO_VERSION}:$PATH"
COPY .prototools ./
RUN proto install moon

#### SKELETONS: one moon scaffold per app: the workspace manifests, and the
#### sources of the app and its `dependsOn`. A change to one app doesn't
#### rebuild the others.
FROM base AS sources
COPY . .

FROM sources AS frontend-skeleton
RUN moon docker scaffold frontend

FROM sources AS backend-skeleton
RUN moon docker scaffold backend

#### BUILDS
# Package bins such as vite start with `#!/usr/bin/env node`, so `node` links
# to bun.
FROM base AS frontend-build
COPY --from=frontend-skeleton /app/.moon/docker/configs .
RUN moon docker setup \
    && ln -s "$(proto bin bun)" /usr/local/bin/node
COPY --from=frontend-skeleton /app/.moon/docker/sources .
RUN moon run frontend:build \
    && mkdir -p /out \
    && cp "$(proto bin bun)" /out/bun

# target/ is a cache mount, so the binaries are copied out in the same step.
FROM base AS backend-build
COPY --from=backend-skeleton /app/.moon/docker/configs .
RUN moon docker setup
COPY --from=backend-skeleton /app/.moon/docker/sources .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked -p backend -p domain-migrations \
    && mkdir -p /out \
    && cp target/release/backend target/release/domain-migrations /out/

#### RUNTIME: one base for every app image, on the same Debian release as the
#### build image, as a non-root user.
FROM debian:13.7-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home evie
USER evie

# One-shot: applies the pending migrations to the database in DATABASE_URL.
# The backend never migrates on start, so this runs before it.
FROM runtime AS migrate
COPY --from=backend-build /out/domain-migrations /usr/local/bin/domain-migrations
ENTRYPOINT ["domain-migrations"]
CMD ["up"]

FROM runtime AS backend
COPY --from=backend-build /out/backend /usr/local/bin/backend
ENV GRPC_ADDR=0.0.0.0:50051
EXPOSE 50051
ENTRYPOINT ["backend"]

# The build bundles the server and its dependencies into .output, so the
# image needs only bun and that folder.
FROM runtime AS frontend
COPY --from=frontend-build /out/bun /usr/local/bin/bun
WORKDIR /app
COPY --from=frontend-build /app/apps/frontend/.output .output
ENV NODE_ENV=production \
    HOST=0.0.0.0 \
    PORT=3000
EXPOSE 3000
CMD ["bun", ".output/server/index.mjs"]
