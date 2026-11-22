# syntax=docker/dockerfile:1
# Builds the backend and the domain migrations from a moon scaffold. Run from
# the repository root:
#   docker build -f apps/backend/Dockerfile .                   # the backend
#   docker build -f apps/backend/Dockerfile --target migrate .  # the migrations

#### BASE: proto and moon. moon comes at the version in .prototools, and
#### `moon docker setup` installs Rust from there too. The Rust image already
#### has rustup and every tool proto needs, and the frontend shares this stage.
FROM rust:latest AS base
WORKDIR /app
RUN curl -fsSL https://moonrepo.dev/install/proto.sh | bash -s -- --yes
ENV PATH="/root/.proto/shims:/root/.proto/bin:$PATH"
COPY .prototools ./
RUN proto install moon

#### SKELETON: the workspace manifests, with stub sources for the other Cargo
#### workspace members, and the sources of the backend and its `dependsOn`.
FROM base AS skeleton
COPY . .
RUN moon docker scaffold backend

#### BUILD: release binaries of the backend and the migrations. target/ is a
#### cache mount, so the binaries are copied out in the same step.
FROM base AS build
COPY --from=skeleton /app/.moon/docker/configs .
RUN moon docker setup
COPY --from=skeleton /app/.moon/docker/sources .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked -p backend -p domain-migrations \
    && mkdir -p /out \
    && cp target/release/backend target/release/domain-migrations /out/

#### START: one runtime for both binaries, as a non-root user.
FROM debian:stable-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home evie
USER evie

# One-shot: applies the pending migrations to the database in DATABASE_URL.
# The backend never migrates on start, so this runs before it.
FROM runtime AS migrate
COPY --from=build /out/domain-migrations /usr/local/bin/domain-migrations
ENTRYPOINT ["domain-migrations"]
CMD ["up"]

# The default target, so it comes last.
FROM runtime AS backend
COPY --from=build /out/backend /usr/local/bin/backend
ENV GRPC_ADDR=0.0.0.0:50051
EXPOSE 50051
ENTRYPOINT ["backend"]
