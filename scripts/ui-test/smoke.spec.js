const { test, expect } = require("@playwright/test");

const BASE = process.env.UI_TEST_BASE || "http://localhost:1300";
const PUBLIC_URL = process.env.UI_TEST_PUBLIC_URL || "/web-sw-cor24-demos";

test.describe("COR24 landing page", () => {
  test("loads successfully", async ({ page }) => {
    const resp = await page.goto(`${BASE}${PUBLIC_URL}/`, {
      waitUntil: "networkidle",
    });
    expect(resp).not.toBeNull();
    expect(resp.status()).toBe(200);
  });

  test("has header with navigation", async ({ page }) => {
    await page.goto(`${BASE}${PUBLIC_URL}/`, { waitUntil: "networkidle" });
    await expect(page.locator("header")).toBeVisible();
    await expect(page.locator("header a[href]").first()).toBeVisible();
  });

  test("has footer", async ({ page }) => {
    await page.goto(`${BASE}${PUBLIC_URL}/`, { waitUntil: "networkidle" });
    await expect(page.locator("footer")).toBeVisible();
  });

  test("has hero heading", async ({ page }) => {
    await page.goto(`${BASE}${PUBLIC_URL}/`, { waitUntil: "networkidle" });
    await expect(page.locator("h1").first()).toBeVisible();
  });
});

test.describe("ISA page", () => {
  test("renders ISA reference content", async ({ page }) => {
    await page.goto(`${BASE}${PUBLIC_URL}/`, { waitUntil: "networkidle" });
    await page.locator('a[href*="isa"], a[href="#isa"]').first().click();
    await page.waitForTimeout(500);
    await expect(page.getByText("ISA", { exact: false }).first()).toBeVisible();
  });

  test("instruction tables render", async ({ page }) => {
    await page.goto(`${BASE}${PUBLIC_URL}/`, { waitUntil: "networkidle" });
    await page.locator('a[href*="isa"], a[href="#isa"]').first().click();
    await page.waitForTimeout(500);
    await expect(page.locator("details.instr-category").first()).toBeVisible();
    await expect(page.locator("table.instr-table").first()).toBeVisible();
  });

  test("register table renders", async ({ page }) => {
    await page.goto(`${BASE}${PUBLIC_URL}/`, { waitUntil: "networkidle" });
    await page.locator('a[href*="isa"], a[href="#isa"]').first().click();
    await page.waitForTimeout(500);
    await expect(page.locator("table").first()).toBeVisible();
  });
});

test.describe("responsive", () => {
  test("mobile viewport (375x667)", async ({ browser }) => {
    const context = await browser.newContext({
      viewport: { width: 375, height: 667 },
    });
    const page = await context.newPage();
    await page.goto(`${BASE}${PUBLIC_URL}/`, { waitUntil: "networkidle" });
    await expect(page.locator("h1").first()).toBeVisible();
    await expect(page.locator("footer")).toBeVisible();
    await context.close();
  });
});
