# AGENTS.md

This file provides guidance to AI coding agents (opencode, Claude Code, Gemini CLI, etc.) when working with code in this repository. It is the agentrail equivalent of a CLAUDE.md file.

## Project: web-sw-cor24-demos -- COR24 Ecosystem Landing Page & Documentation Hub

Browser-based landing page and documentation hub for the entire COR24 ecosystem. Built with Rust, Yew 0.21, and WebAssembly via Trunk. Deployed to GitHub Pages.

This is the single entry point for all COR24 live demos, ISA documentation, and tooling references. It provides:
- Directory of all live web demos with links and descriptions
- Comprehensive COR24 ISA documentation (registers, instruction set, memory map, calling conventions)
- Documentation for every tool in the sw-embed ecosystem (cross-compilers, assemblers, interpreters, VM, monitor, etc.)
- Links to all GitHub repositories

## CRITICAL: AgentRail Session Protocol (MUST follow exactly)

Every agent session follows this 6-step loop. Do NOT skip or reorder steps.

### 1. START (do this FIRST, before anything else)
```bash
agentrail next
```
Read the output carefully. It contains your current step, prompt, plan context, and any relevant skills/trajectories.

### 2. BEGIN (immediately after reading the next output)
```bash
agentrail begin
```

### 3. WORK (do what the step prompt says)
Do NOT ask "want me to proceed?" or "shall I start?". The step prompt IS your instruction. Execute it directly.

### 4. PRE-COMMIT QUALITY GATE (MANDATORY -- every step, no exceptions)
Every completed saga step must be high quality, documented, and pushed to GitHub.
If anything fails, fix the underlying problem -- NEVER suppress, allow, or work around a check.

#### Phase A: Rust quality + documentation
1. `cargo fmt --all` -- Format all Rust code
2. `cargo clippy --all-targets --all-features -- -D warnings` -- Zero warnings.
   If clippy reports a warning, fix the code. Do NOT add `#[allow(...)]` or
   change the clippy invocation. The underlying problem must be resolved.
3. `cargo test` -- All tests pass. Fix any failures before proceeding.
4. `cargo check --target wasm32-unknown-unknown` -- WASM compilation check
5. Review staged files with `git status` -- No build artifacts, no unintended files
6. Verify documentation is up-to-date -- if code changed affected public APIs,
   components, or behavior, update relevant doc comments and any docs/ files
7. **Commit** the formatted, clippy-clean, test-passing, documented code now.

#### Phase B: sw-checklist conformance
8. `sw-checklist` -- Fix all FAIL and WARN items to avoid tech-debt accumulation.
   You may commit the Phase A code before fixing sw-checklist issues.
   If you have questions about how to fix a specific FAIL/WARN, ask for help.
   Re-run `sw-checklist` after fixes to confirm clean.
9. **Commit** the sw-checklist fixes.

#### Phase B+: Re-verify if code changed
10. If Phase B made code changes, re-run Phase A steps 1-4 (fmt, clippy, test, wasm check).
    Fix any regressions. Commit if needed.

#### Phase B++: UI build verification (MANDATORY)
11. `./scripts/build.sh` -- Full Trunk release build. This catches template errors,
    broken html! macro usage, and other issues that `cargo check --target wasm32-unknown-unknown`
    does NOT catch. Fix any build failures.

#### Phase C: Push
12. **Commit** the UI build fixes (if any).
13. `git push` -- Every completed step must be pushed to GitHub

### 5. COMPLETE (LAST thing, after committing and pushing)
```bash
agentrail complete --summary "what you accomplished" \
  --reward 1 \
  --actions "tools and approach used"
```
- If the step failed: `--reward -1 --failure-mode "what went wrong"`
- If the saga is finished: add `--done`

### 6. STOP (after complete, DO NOT continue working)
Do NOT make further code changes after running `agentrail complete`.
Any changes after complete are untracked and invisible to the next session.
Future work belongs in the NEXT step, not this one.

## Key Rules

- **Do NOT skip steps** -- the next session depends on accurate tracking
- **Do NOT ask for permission** -- the step prompt is the instruction
- **Do NOT continue working** after `agentrail complete`
- **Commit before complete** -- always commit first, then record completion
- **NO Python** -- do not use Python for anything in this project. No venvs,
  no pip, no python3 scripts. This is a Rust/WASM project. Period.
- **NO npm / NO node_modules / NO package.json** -- do not use npm, node packages,
  or any JavaScript package manager. No package.json, no package-lock.json,
  no node_modules/. This is a Rust/WASM project. Use only Rust, HTML, CSS,
  and minimal hand-written JS where Rust absolutely cannot be used.
- **Only use scripts/*.sh to build and serve** -- never run `trunk` commands
  directly. Never use ad-hoc servers. All build and serve operations go
  through the shell scripts in scripts/.
- **ISA register naming** -- COR24 registers are named, not numbered. Use
  fp, sp, z, ir, iv (not r3, r4, r5, r6, r7). The ISA documentation must
  never expose raw register numbers above r2.
- **Status tab updates must be live** -- When asked to update the status
  page, the task is NOT done until the user can see the changes on the
  live site (https://sw-embed.github.io/web-sw-cor24-demos/#/status).
  After regenerating data/charts, you MUST run `./scripts/build-pages.sh`,
  commit the `pages/` directory, and `git push`. Do NOT stop after
  regenerating source files or SVGs in `images/` -- those are invisible
  to the user until built and deployed.

## Useful Commands

```bash
agentrail status          # Current saga state
agentrail history         # All completed steps
agentrail plan            # View the plan
agentrail next            # Current step + context
```

## Related Projects

All COR24 repos live under `~/github/sw-embed/` as siblings. This landing page documents ALL of them.

### Foundation
- `sw-cor24-emulator` -- COR24 assembler and emulator (Rust)
- `sw-cor24-x-assembler` -- Cross-assembler library (Rust)
- `sw-cor24-assembler` -- Native assembler (C, runs on COR24)
- `sw-cor24-project` -- Ecosystem hub/portal (docs only)

### Cross-compilers (run on host)
- `sw-cor24-x-tinyc` -- Tiny C cross-compiler (Rust)
- `sw-cor24-rust` -- Rust-to-COR24 pipeline

### P-code system
- `sw-cor24-pcode` -- P-code VM (COR24 asm), assembler (Rust), linker (Rust)
- `sw-cor24-x-pc-aotc` -- P-code AOT compiler (p-code to COR24 native)
- `sw-cor24-pascal` -- Pascal compiler (C) + runtime

### Languages (native, run on COR24)
- `sw-cor24-macrolisp` -- Tiny Macro Lisp (C)
- `sw-cor24-ocaml` -- OCaml compiler (C, P-code target)
- `sw-cor24-forth` -- Forth (COR24 assembly)
- `sw-cor24-apl` -- APL interpreter (C)
- `sw-cor24-basic` -- BASIC interpreter (C)
- `sw-cor24-fortran` -- Fortran compiler (C)
- `sw-cor24-plsw` -- PL/SW compiler (C, PL/I-inspired)
- `sw-cor24-prolog` -- Prolog interpreter (PL/SW, WAM-like VM)
- `sw-cor24-script` -- SWS scripting language (C, Tcl-like)

### System software
- `sw-cor24-monitor` -- Resident monitor/service processor (COR24 asm + C)
- `sw-cor24-debugger` -- Source-level debugger (planned)

### Web UIs (live browser demos)
- `web-sw-cor24-assembler` -- COR24 assembly IDE
- `web-sw-cor24-pcode` -- P-code VM debugger
- `web-sw-cor24-tinyc` -- Tiny C compiler
- `web-sw-cor24-macrolisp` -- Lisp REPL
- `web-sw-cor24-pascal` -- Pascal demos
- `web-sw-cor24-apl` -- APL environment
- `web-sw-cor24-forth` -- Forth debugger
- `web-sw-cor24-plsw` -- PL/SW development environment
- `web-sw-cor24-demos` -- THIS REPO (landing page)

### GitHub Pages URLs
All live demos are at: `https://sw-embed.github.io/<repo-name>/`

## Build

Edition 2024 for all Rust code. Never suppress warnings.

```bash
./scripts/serve.sh             # Dev server with hot reload
./scripts/build-pages.sh       # Release build to pages/ for GitHub Pages
./scripts/build.sh             # Release build to dist/
./scripts/ui-test.sh           # playwright-cli smoke tests + screenshots
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo fmt --all                # Format
cargo check --target wasm32-unknown-unknown  # Full WASM check
```

**CRITICAL: NEVER run `trunk` commands directly.** Always use the shell scripts.
Running bare `trunk serve` or `trunk build` with wrong flags breaks the build.

## Deployment

- `trunk build --release` outputs to `dist/` (gitignored)
- `./scripts/build-pages.sh` builds to `dist/` then rsyncs to `pages/` (tracked)
- `pages/.nojekyll` is committed once and never regenerated
- GitHub Actions (`.github/workflows/pages.yml`) deploys `pages/` on push to main
- Live URL: https://sw-embed.github.io/web-sw-cor24-demos/

## Architecture

- **Trunk** builds the WASM binary and serves it
- **Yew 0.21** CSR framework for the UI (Component trait, Msg enum, html! macro)
- **wasm-bindgen** + **web-sys** for browser APIs
- **Catppuccin Mocha** dark theme (consistent with all other web-sw-cor24-* demos)
- **pages/** directory committed to git, deployed via GitHub Pages
- No path dependencies on other sw-embed repos (this is a documentation-only site)

## CRITICAL: Scaling Rules -- Always Split, Never Consolidate

When a module, crate, or component grows too large, the answer is ALWAYS to split
and distribute, never to consolidate or merge. The goal is a loosely-coupled,
modular, testable architecture with small, focused, easy-to-maintain units.

### Scaling Hierarchy (bottom-up)

| Signal | Action |
|--------|--------|
| Too many functions in a module | Add a new module and distribute functions across it |
| Too many modules in a crate | Add a new workspace crate and move modules into it |
| Too many crates in a workspace | Add a new top-level component directory |

### Component Architecture

- Top-level repo root contains `./scripts/` (build-all, serve) and `./components/`
- `./components/` exists when there are 2 or more components
- Each component is its own directory under `./components/`
- Each component is a Cargo workspace with up to 5 crates
- Crates within a component share a focused domain (e.g., data, rendering, routing)

### Code Style (enforced by sw-checklist)

- **File size limit**: 500 lines per file (prefer 200-300)
- **Function size limit**: 50 lines (prefer 10-30)
- **Module function count**: max 7 functions per module (warn at 4)
- **Crate module count**: max 7 modules per crate (warn at 5)
- **Pure functions**: prefer stateless pure functions that take composed structs
- **Separate test modules**: every module with logic gets a `#[cfg(test)] mod tests`
- **Composed structs**: pass data as typed structs, not loose tuples or many params

### Anti-patterns (NEVER do these)

- Do NOT merge modules to reduce module count -- split into crates instead
- Do NOT inline helper functions to reduce function count -- extract to new modules
- Do NOT add `#[allow(...)]` to suppress warnings -- fix the underlying issue
- Do NOT flatten namespaces to "simplify" -- prefer deep, focused module trees

## Key Conventions

- Monospace font stack: JetBrains Mono, Fira Code, Cascadia Code
- CSS custom properties for Catppuccin Mocha colors
- Responsive design for desktop and mobile
- No JavaScript outside of WASM bootstrap
- Release profile: `opt-level = "z"`, `lto = true` for minimal WASM size

## Available Task Types

- `rust-project-init` -- Scaffold Cargo.toml, src/, Trunk config, scripts/
- `yew-component` -- Build a Yew Component (html!, Msg enum, update/view)
- `wasm-build` -- Configure wasm-bindgen, web-sys features, Trunk pipeline
- `css-styling` -- Catppuccin Mocha theme, responsive layout, typography
- `content` -- Add documentation content (ISA docs, tool descriptions, demos)
- `integration` -- Wire up navigation, routing, links to live demos
- `feature` -- Add UI feature (search, filtering, interactive diagrams)
- `bug-fix` -- Fix a defect
- `docs` -- Documentation updates
- `pre-commit` -- Run full pre-commit quality gate

## Pre-Commit Quality Gate (MANDATORY)

Every commit must pass this sequence. Split into two commits if sw-checklist
issues are found: commit the clean Rust code first, then fix and commit
sw-checklist issues separately.

1. `cargo fmt --all` -- Format all Rust code
2. `cargo clippy --all-targets --all-features -- -D warnings` -- Zero warnings.
   If clippy reports a warning, fix the code. Do NOT add `#[allow(...)]` or
   change the clippy invocation. The underlying problem must be resolved.
3. `cargo test` -- All tests pass. Fix any failures before proceeding.
4. `cargo check --target wasm32-unknown-unknown` -- WASM compilation check
5. Review staged files with `git status` -- No build artifacts, no unintended files
6. Verify documentation is up-to-date -- if code changed affected public APIs,
   components, or behavior, update relevant doc comments and any docs/ files
7. Commit with a clear descriptive message summarizing what was done and why
8. `sw-checklist` -- Fix all FAIL and WARN items to avoid tech-debt accumulation.
   You may commit the Phase A code before fixing sw-checklist issues.
   If you have questions about how to fix a specific FAIL/WARN, ask for help.
   Re-run `sw-checklist` after fixes to confirm clean.
9. Commit the sw-checklist fixes (separate commit if Phase A was already committed)
10. If step 9 made code changes, re-run steps 1-4 (fmt, clippy, test, wasm check).
    Fix any regressions. Commit if needed.
11. `./scripts/build.sh` -- Full Trunk release build. Fix any build failures.
12. `git push` -- Every completed step must be pushed to GitHub
