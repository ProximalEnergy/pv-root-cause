#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

export JSII_RUNTIME_PACKAGE_CACHE_ROOT="${JSII_RUNTIME_PACKAGE_CACHE_ROOT:-$ROOT_DIR/infra/.jsii-cache}"
export JSII_SILENCE_WARNING_UNTESTED_NODE_VERSION="${JSII_SILENCE_WARNING_UNTESTED_NODE_VERSION:-1}"
export UV_CACHE_DIR="${UV_CACHE_DIR:-$ROOT_DIR/.uv-cache}"

exec uv run --project "$ROOT_DIR/infra" python "$ROOT_DIR/infra/app.py"
