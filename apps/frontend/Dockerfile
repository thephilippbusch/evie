# syntax=docker/dockerfile:1
# Builds the frontend. Run from the repository root:
#   docker build -f apps/frontend/Dockerfile .

# The bun version matches .prototools.
FROM oven/bun:1.4.4 AS builder
WORKDIR /app

# The manifests first, so the install layer stays cached until a dependency
# changes. bun.lock covers every bun workspace; with each one's package.json
# here, the install matches `bun install` at the root.
COPY package.json bun.lock tsconfig.base.json ./
COPY apps/frontend/package.json apps/frontend/
COPY libs/ui/package.json libs/ui/
COPY libs/proto/package.json libs/proto/
RUN bun install --frozen-lockfile

COPY apps/frontend apps/frontend
COPY libs/ui libs/ui
COPY libs/proto libs/proto
WORKDIR /app/apps/frontend
RUN bun run build

# The build bundles the server and its dependencies into .output, so the
# runtime needs nothing else.
FROM oven/bun:1.4.4-slim
WORKDIR /app
COPY --from=builder /app/apps/frontend/.output .output
ENV NODE_ENV=production \
    HOST=0.0.0.0 \
    PORT=3000
USER bun
EXPOSE 3000
CMD ["bun", ".output/server/index.mjs"]
