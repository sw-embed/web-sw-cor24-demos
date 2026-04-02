module.exports = {
  testDir: ".",
  timeout: 15000,
  retries: 0,
  use: {
    headless: true,
    screenshot: "off",
    video: "off",
    trace: "off",
  },
  projects: [{ name: "chromium", use: { browserName: "chromium" } }],
};
