ARG BUILD_TIME
ARG BUILD_USER
ARG GIT_BRANCH
ARG GIT_REF
ARG GIT_URL
ARG RUST_VERSION
ARG VERSION

FROM --platform=${BUILDPLATFORM} docker.io/rust:${RUST_VERSION} AS builder
ARG TARGETPLATFORM

WORKDIR /usr/src/myapp
COPY .. .
RUN rustup -v target add $(./docker/platform.sh) \
    && apt-get update \
    && apt-get install -y unzip $(./docker/compiler.sh) \
    && cargo build -v --release --target=$(./docker/platform.sh) \
    && cp /usr/src/myapp/target/$(./docker/platform.sh)/release/renvy /usr/src/myapp/target/release/renvy

FROM docker.io/debian:stable-slim
ARG BUILD_TIME
ARG BUILD_USER
ARG GIT_BRANCH
ARG GIT_REF
ARG GIT_URL
ARG VERSION

LABEL net.benjaminsattler.renvy.build_time="${BUILD_TIME}" \
    net.benjaminsattler.renvy.build_user="${BUILD_USER}" \
    net.benjaminsattler.renvy.git_branch="${GIT_BRANCH}" \
    net.benjaminsattler.renvy.git_ref="${GIT_REF}" \
    net.benjaminsattler.renvy.git_url="${GIT_URL}" \
    net.benjaminsattler.renvy.version="${VERSION}"

COPY --from=builder /usr/src/myapp/target/release/renvy /renvy

ENTRYPOINT ["/renvy"]
WORKDIR /opt

