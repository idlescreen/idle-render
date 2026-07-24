# Multi-stage Alpine build for render (headless export tooling).
FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev pkgconfig freetype-dev fontconfig-dev \
    dbus-dev wayland-dev libxkbcommon-dev openssl-dev
WORKDIR /src
# Expect build context with idle-core sibling via compose or pre-copied tree.
COPY . /src/render
# Placeholder: full image builds require idle-core path dep; use CI matrix instead for release.
WORKDIR /src/render
RUN echo "Use GitHub Actions for linked idle-core builds" > /build-note

FROM alpine:3.20
RUN apk add --no-cache ffmpeg ca-certificates
COPY --from=build /build-note /build-note
ENTRYPOINT ["/bin/sh"]
