#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

export JSII_RUNTIME_PACKAGE_CACHE_ROOT="${JSII_RUNTIME_PACKAGE_CACHE_ROOT:-$ROOT_DIR/infra/.jsii-cache}"
export JSII_SILENCE_WARNING_UNTESTED_NODE_VERSION="${JSII_SILENCE_WARNING_UNTESTED_NODE_VERSION:-1}"
export UV_CACHE_DIR="${UV_CACHE_DIR:-$ROOT_DIR/.uv-cache}"

uv sync --directory infra

rustup target add wasm32-unknown-unknown

if ! cargo leptos --version >/dev/null 2>&1; then
  cargo install cargo-leptos --locked
fi

cargo test --lib --test content_validation
cargo leptos build --release
PV_EXPORT_STATIC=1 cargo leptos serve --release

if command -v cdk >/dev/null 2>&1; then
  cdk deploy --require-approval never "$@"
else
  npx cdk deploy --require-approval never "$@"
fi
