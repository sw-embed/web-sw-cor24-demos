# CLAUDE.md

This file mirrors the critical rules from AGENTS.md for Claude Code.

## Key Rules

- **Status tab updates must be live** -- When asked to update the status
  page, the task is NOT done until the user can see the changes on the
  live site (https://sw-embed.github.io/web-sw-cor24-demos/#/status).
  After regenerating data/charts, you MUST run `./scripts/build-pages.sh`,
  commit the `pages/` directory, and `git push`. Do NOT stop after
  regenerating source files or SVGs in `images/` -- those are invisible
  to the user until built and deployed.

See AGENTS.md for the full project documentation and rules.
