# syntax=docker/dockerfile:1
# Builds the backend and the domain migrations from a moon scaffold, so this
# file lists no libraries and no tool versions. Run from the repository root:
#   docker build -f apps/backend/Dockerfile .                   # the backend
#   docker build -f apps/backend/Dockerfile --target migrate .  # the migrations

# proto and moon. The Rust image brings rustup, which proto's Rust plugin
# needs; the Rust version comes from .prototools and rust-toolchain.toml.
FROM rust:slim-trixie AS base
ARG PROTO_VERSION=0.63.1
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl git xz-utils \
    && rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.proto/shims:/root/.proto/bin:$PATH"
RUN curl -fsSL https://moonrepo.dev/install/proto.sh | bash -s -- "$PROTO_VERSION" --yes
WORKDIR /app
COPY .prototools ./
RUN proto install moon

# The skeleton: moon copies the manifests and lockfiles of the whole workspace
# to .moon/docker/configs, with empty stub sources for every Cargo workspace
# member, and the sources of these projects and their `dependsOn` to
# .moon/docker/sources.
FROM base AS skeleton
COPY . .
RUN moon docker scaffold backend domain-migrations

# Installs the toolchain while only the manifests are here, so a source change
# reuses that layer. No `moon docker prune`: the runtime stages copy only the
# two binaries.
FROM base AS builder
COPY --from=skeleton /app/.moon/docker/configs .
RUN moon docker setup
COPY --from=skeleton /app/.moon/docker/sources .
# target/ is a cache mount and gone after this step, so the binaries are
# copied out in the same step.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked -p backend -p domain-migrations \
    && mkdir -p /out \
    && cp target/release/backend target/release/domain-migrations /out/

FROM debian:trixie-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home evie
USER evie

# One-shot: applies the pending migrations to the database in DATABASE_URL.
# The backend never migrates on start, so this runs before it.
FROM runtime AS migrate
COPY --from=builder /out/domain-migrations /usr/local/bin/domain-migrations
ENTRYPOINT ["domain-migrations"]
CMD ["up"]

# The default target, so it comes last.
FROM runtime AS backend
COPY --from=builder /out/backend /usr/local/bin/backend
ENV GRPC_ADDR=0.0.0.0:50051
EXPOSE 50051
ENTRYPOINT ["backend"]
