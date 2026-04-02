#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

PORT="${UI_TEST_PORT:-1301}"
PUBLIC_URL="${UI_TEST_PUBLIC_URL:-/web-sw-cor24-demos}"
BASE="http://localhost:${PORT}"
SCREENSHOT_DIR="${PROJECT_DIR}/images/screenshots"

if [ ! -d "dist" ] || [ -z "$(ls -A dist 2>/dev/null)" ]; then
    echo "ERROR: dist/ not found or empty. Run ./scripts/build.sh first."
    exit 1
fi

mkdir -p "$SCREENSHOT_DIR"

cleanup() {
    playwright-cli close 2>/dev/null || true
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
}
trap cleanup EXIT

ln -sfn . dist/web-sw-cor24-demos

npx serve dist/ -l "$PORT" -s &
SERVER_PID=$!
sleep 2

PASSED=0
FAILED=0
failures=""

assert_page_loads() {
    local label="$1" url="$2"
    playwright-cli goto "$url" 2>/dev/null
    if playwright-cli eval "document.title.length > 0" 2>/dev/null | grep -q "true"; then
        echo "  ok:   $label loads"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL: $label did not load"
        FAILED=$((FAILED + 1))
        failures="$failures\n  - $label"
    fi
}

assert_selector_exists() {
    local label="$1" selector="$2"
    local result
    result=$(playwright-cli eval "document.querySelector('$selector') !== null" 2>/dev/null)
    if echo "$result" | grep -q "true"; then
        echo "  ok:   $label"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL: $label"
        FAILED=$((FAILED + 1))
        failures="$failures\n  - $label"
    fi
}

assert_text_exists() {
    local label="$1" text="$2"
    local result
    result=$(playwright-cli eval "document.body.textContent.includes('$text')" 2>/dev/null)
    if echo "$result" | grep -q "true"; then
        echo "  ok:   $label"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL: $label"
        FAILED=$((FAILED + 1))
        failures="$failures\n  - $label"
    fi
}

capture_screenshot() {
    local name="$1" url="$2"
    local path="$SCREENSHOT_DIR/$name"
    playwright-cli goto "$url" 2>/dev/null
    sleep 0.5
    playwright-cli screenshot 2>/dev/null > "$path"
    echo "  snap: $name"
}

echo ""
echo "UI smoke tests against $BASE"
echo ""

echo "Page load:"
assert_page_loads "Home page" "${BASE}${PUBLIC_URL}/"

echo ""
echo "Header / navigation:"
assert_selector_exists "header exists" "header"
assert_selector_exists "nav links" "header a[href]"

echo ""
echo "Content:"
assert_selector_exists "h1 heading" "h1"
assert_selector_exists "footer" "footer"

echo ""
echo "Screenshots:"
capture_screenshot "home.png" "${BASE}${PUBLIC_URL}/"
capture_screenshot "isa.png" "${BASE}${PUBLIC_URL}/#/isa"
capture_screenshot "hardware.png" "${BASE}${PUBLIC_URL}/#/hardware"
capture_screenshot "demos.png" "${BASE}${PUBLIC_URL}/#/demos"
capture_screenshot "toolchain.png" "${BASE}${PUBLIC_URL}/#/toolchain"
capture_screenshot "ecosystem.png" "${BASE}${PUBLIC_URL}/#/ecosystem"

echo ""
echo "Mobile (375x667):"
playwright-cli resize 375 667 2>/dev/null
capture_screenshot "home-mobile.png" "${BASE}${PUBLIC_URL}/"
assert_selector_exists "mobile h1" "h1"

echo ""
echo "=========================================="
echo "Results: $PASSED passed, $FAILED failed"
if [ "$FAILED" -gt 0 ]; then
    echo ""
    echo "Failed:"
    echo -e "$failures"
fi
echo ""

exit $FAILED
