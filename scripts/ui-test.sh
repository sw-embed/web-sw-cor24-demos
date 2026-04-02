#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

PORT="${UI_TEST_PORT:-1301}"
PUBLIC_URL="${UI_TEST_PUBLIC_URL:-/web-sw-cor24-demos}"
TEST_DIR="$SCRIPT_DIR/ui-test"

if [ ! -d "dist" ] || [ -z "$(ls -A dist 2>/dev/null)" ]; then
    echo "ERROR: dist/ not found or empty. Run ./scripts/build.sh first."
    exit 1
fi

if [ ! -d "$TEST_DIR/node_modules" ]; then
    echo "Installing playwright dependencies..."
    (cd "$TEST_DIR" && npm install && npx playwright install chromium)
fi

cleanup() {
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
}
trap cleanup EXIT

ln -sfn . dist/web-sw-cor24-demos

python3 -m http.server "$PORT" --directory dist &
SERVER_PID=$!
sleep 1

export UI_TEST_BASE="http://localhost:${PORT}"
export UI_TEST_PUBLIC_URL="${PUBLIC_URL}"

(cd "$TEST_DIR" && npx playwright test . --reporter line 2>&1)
