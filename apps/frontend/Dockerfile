# syntax=docker/dockerfile:1
# Builds the frontend from a moon scaffold. Run from the repository root:
#   docker build -f apps/frontend/Dockerfile .

#### BASE: moon on the bun image. moon uses the image's bun instead of
#### installing its own, so keep the tag at the bun version in .prototools.
FROM oven/bun:1.4.4-alpine AS base
WORKDIR /app
RUN apk --no-cache add bash curl
COPY .prototools ./
RUN curl -fsSL https://moonrepo.dev/install/moon.sh \
    | bash -s -- "$(sed -n 's/^moon = "\(.*\)"$/\1/p' .prototools)"
ENV PATH="/root/.moon/bin:$PATH" \
    MOON_TOOLCHAIN_FORCE_GLOBALS=1

#### SKELETON: the workspace manifests, and the sources of the frontend and
#### its `dependsOn`.
FROM base AS skeleton
COPY . .
RUN moon docker scaffold frontend

#### BUILD: installs the dependencies, then builds .output.
FROM base AS build
COPY --from=skeleton /app/.moon/docker/configs .
RUN moon docker setup
COPY --from=skeleton /app/.moon/docker/sources .
RUN moon run frontend:build

#### START: the build bundles the server and its dependencies into .output,
#### so the image needs only bun and that folder.
FROM oven/bun:1.4.4-alpine AS start
WORKDIR /app
COPY --from=build /app/apps/frontend/.output .output
ENV NODE_ENV=production \
    HOST=0.0.0.0 \
    PORT=3000
USER bun
EXPOSE 3000
CMD ["bun", ".output/server/index.mjs"]
