#!/usr/bin/env sh

set -eux

DOCKER_EXEC=${DOCKER_EXEC:=docker}
DOCKER_REPO=${DOCKER_REPO:=docker.io}
DOCKER_INSECURE=${DOCKER_INSECURE:=}

SCRIPT_DIR=$(dirname "$(realpath """$0""")")
REPO_ROOT=$(git -C "${SCRIPT_DIR}" rev-parse --show-toplevel)
GIT_REF=$(git -C "${REPO_ROOT}" describe --long)
GIT_URL=$(git -C "${REPO_ROOT}" remote get-url origin)
GIT_BRANCH=$(git -C "${REPO_ROOT}" name-rev --name-only HEAD)
BUILD_USER=$(whoami)
BUILD_TIME=$(date --iso-8601=sec)
VERSION_FULL=$(sed '/^version = */!d; s///;q' "${REPO_ROOT}/binary/Cargo.toml" | sed s/\"//g)
VERSION_MAJOR=$(echo "${VERSION_FULL}" | cut -d "." -f 1 -)
VERSION_MINOR=$(echo "${VERSION_FULL}" | cut -d "." -f 2 -)
VERSION_PATCH=$(echo "${VERSION_FULL}" | cut -d "." -f 3 -)
RUST_VERSION=1.67

[ -n "${DOCKER_INSECURE}" ] && DOCKER_INSECURE="--insecure";

echo DOCKER_INSECURE = "${DOCKER_INSECURE}"
echo DOCKER_REPO = "${DOCKER_REPO}"
echo SCRIPT_DIR = "${SCRIPT_DIR}"
echo REPO_ROOT = "${REPO_ROOT}"
echo GIT_REF = "${GIT_REF}"
echo GIT_URL = "${GIT_URL}"
echo GIT_BRANCH = "${GIT_BRANCH}"
echo BUILD_USER = "${BUILD_USER}"
echo BUILD_TIME = "${BUILD_TIME}"
echo VERSION_FULL = "${VERSION_FULL}"
echo VERSION_MAJOR = "${VERSION_MAJOR}"
echo VERSION_MINOR = "${VERSION_MINOR}"
echo VERSION_PATCH = "${VERSION_PATCH}"
echo DOCKER_EXEC = "${DOCKER_EXEC}"
echo RUST_VERSION = "${RUST_VERSION}"

"${DOCKER_EXEC}" login "${DOCKER_REPO}"
"${DOCKER_EXEC}" buildx build \
    --platform linux/arm64,linux/amd64,linux/arm/v7 \
    --tag "${DOCKER_REPO}"/benjaminsattler/renvy:latest \
    --tag "${DOCKER_REPO}"/benjaminsattler/renvy:"${VERSION_MAJOR}" \
    --tag "${DOCKER_REPO}"/benjaminsattler/renvy:"${VERSION_MAJOR}"."${VERSION_MINOR}" \
    --tag "${DOCKER_REPO}"/benjaminsattler/renvy:"${VERSION_MAJOR}"."${VERSION_MINOR}"."${VERSION_PATCH}" \
    --build-arg BUILD_TIME="${BUILD_TIME}" \
    --build-arg BUILD_USER="${BUILD_USER}" \
    --build-arg GIT_BRANCH="${GIT_BRANCH}" \
    --build-arg GIT_REF="${GIT_REF}" \
    --build-arg GIT_URL="${GIT_URL}" \
    --build-arg VERSION="${VERSION_FULL}" \
    --build-arg RUST_VERSION="${RUST_VERSION}" \
    --file "${REPO_ROOT}/docker/Dockerfile" \
    --push \
    "${REPO_ROOT}"