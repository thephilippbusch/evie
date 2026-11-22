# syntax=docker/dockerfile:1
# Builds the frontend from a moon scaffold. Run from the repository root:
#   docker build -f apps/frontend/Dockerfile .

#### BASE: proto and moon. moon comes at the version in .prototools, and
#### `moon docker setup` installs bun from there too. The Rust image already
#### has every tool proto needs, and the backend shares this stage.
FROM rust:latest AS base
WORKDIR /app
RUN curl -fsSL https://moonrepo.dev/install/proto.sh | bash -s -- --yes
ENV PATH="/root/.proto/shims:/root/.proto/bin:$PATH"
COPY .prototools ./
RUN proto install moon

#### SKELETON: the workspace manifests, and the sources of the frontend and
#### its `dependsOn`.
FROM base AS skeleton
COPY . .
RUN moon docker scaffold frontend

#### BUILD: installs bun and the dependencies, then builds .output. Package
#### bins such as vite start with `#!/usr/bin/env node`, so `node` links to bun.
FROM base AS build
COPY --from=skeleton /app/.moon/docker/configs .
RUN moon docker setup \
    && ln -s "$(proto bin bun)" /usr/local/bin/node
COPY --from=skeleton /app/.moon/docker/sources .
RUN moon run frontend:build \
    && mkdir -p /out \
    && cp "$(proto bin bun)" /out/bun

#### START: the build bundles the server and its dependencies into .output,
#### so the image needs only bun and that folder.
FROM debian:stable-slim AS start
RUN useradd --system --uid 10001 --create-home evie
COPY --from=build /out/bun /usr/local/bin/bun
WORKDIR /app
COPY --from=build /app/apps/frontend/.output .output
ENV NODE_ENV=production \
    HOST=0.0.0.0 \
    PORT=3000
USER evie
EXPOSE 3000
CMD ["bun", ".output/server/index.mjs"]
