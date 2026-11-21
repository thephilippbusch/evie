# syntax=docker/dockerfile:1
# Builds the backend and the domain migrations. Run from the repository root:
#   docker build -f apps/backend/Dockerfile .                   # the backend
#   docker build -f apps/backend/Dockerfile --target migrate .  # the migrations

# The Rust version matches .prototools and rust-toolchain.toml. The image tag
# pins it, so rust-toolchain.toml stays out of the image: it would make rustup
# install clippy and rustfmt, which a release build doesn't need.
FROM rust:1.99.1-slim-trixie AS builder
WORKDIR /app

# cargo resolves the whole workspace, so it needs every member, not only the
# crates it builds.
COPY Cargo.toml Cargo.lock ./
COPY apps/backend apps/backend
COPY libs/domain-migrations libs/domain-migrations
COPY libs/proto libs/proto
COPY tools/proto-codegen tools/proto-codegen

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
