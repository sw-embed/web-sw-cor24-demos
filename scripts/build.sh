#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Building UI ==="
cd "$PROJECT_DIR"
trunk build --release --public-url /web-sw-cor24-demos/
cp favicon.ico dist/favicon.ico

echo "=== Build succeeded ==="
echo "Output in: $PROJECT_DIR/dist/"
