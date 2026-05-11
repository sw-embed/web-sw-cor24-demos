Refresh all status page data through today's date. Run all three data generation steps in order:

1. Issue counts + agentrail status: cargo run --manifest-path tools/gen-status/Cargo.toml
2. Issue progress SVG charts: cargo run --manifest-path tools/gen-issue-chart/Cargo.toml
3. Commits heatmap + closed-issues timeline from sw-cor24-project:
   cd ~/github/sw-embed/sw-cor24-project && ./scripts/gen-commits.sh && ./scripts/gen-closed-issues.sh
   cp commits/index.html ~/github/sw-embed/web-sw-cor24-demos/reports/commits.html
   cp closed-issues/index.html ~/github/sw-embed/web-sw-cor24-demos/reports/closed-issues.html

Then build and deploy: ./scripts/build-pages.sh, commit all generated files + pages/, git push.