#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

trunk serve --port 1300 --public-url / &
TRUNK_PID=$!

sleep 3
cp favicon.ico dist/favicon.ico 2>/dev/null || true

wait $TRUNK_PID
