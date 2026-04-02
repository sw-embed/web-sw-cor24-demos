# Architecture -- COR24 Ecosystem Landing Page

## Overview

The landing page is a documentation-only Yew 0.21 SPA deployed to GitHub Pages. It has NO runtime dependencies on other sw-embed repos -- all content is static text and links. This keeps the WASM bundle minimal and eliminates build-time coupling.

## Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Language | Rust (edition 2024) | Application code |
| Framework | Yew 0.21 | CSR UI components |
| Build | Trunk | WASM packaging + dev server |
| WASM bridge | wasm-bindgen + web-sys | Browser APIs |
| Styling | CSS custom properties | Catppuccin Mocha theme |
| Fonts | JetBrains Mono, Fira Code | Monospace font stack |
| Deploy | GitHub Pages | Static hosting via pages/ directory |

## System Architecture

```
                    GitHub Pages
                         |
                    index.html
                         |
                   WASM bootstrap
                         |
                    Yew App
                         |
            +-------+----+-------+
            |       |            |
         Home    ISA Docs    Demos
            |       |            |
        Hero     Sections    Grid
        Links    Nav         Cards
        Overview              Links
```

### Page Sections (Top-Level Navigation)

1. **Home** -- Hero section, project overview, quick links to demos
2. **ISA Reference** -- COR24 ISA documentation (registers, instructions, memory map)
3. **Demos** -- Grid of live web demos with links
4. **Toolchain** -- Documentation for all tools organized by category
5. **Ecosystem** -- Repository map, dependency graph, contribution guide

### Component Hierarchy

```
App
 +-- Header (nav bar, logo, section links)
 +-- Router (Yew conditional rendering by section)
 |    +-- HomePage
 |    |    +-- HeroSection
 |    |    +-- QuickLinks
 |    |    +-- FeaturedDemo
 |    +-- IsaPage
 |    |    +-- IsaNav (sidebar or top tabs)
 |    |    +-- RegistersSection
 |    |    +-- InstructionsSection
 |    |    +-- MemoryMapSection
 |    |    +-- CallingConventionsSection
 |    +-- DemosPage
 |    |    +-- DemoCard (repeated)
 |    +-- ToolchainPage
 |    |    +-- ToolCategory
 |    |    |    +-- ToolCard (repeated)
 |    +-- EcosystemPage
 |         +-- RepoGrid
 |         +-- DependencyDiagram
 +-- Footer
```

## Content Architecture

### ISA Documentation Sections

The ISA docs are organized as Rust structs holding static data, rendered by Yew components. No markdown parsing at runtime -- all content is embedded at compile time.

| Section | Content |
|---------|---------|
| Overview | COR24 history, design philosophy, makerlisp.com link |
| Registers | r0-r2 (GP), fp, sp, z, iv, ir -- roles, constraints, calling convention |
| Instruction Set | All 49 mnemonics, encoding formats, variable-length instructions |
| Memory Map | SRAM (1 MB), EBR stack (8 KB), I/O addresses (UART, LED/switch) |
| Addressing Modes | Immediate, register, absolute, indexed, stack |
| Calling Convention | Stack frame layout, argument passing, return value |
| Interrupts | IV/IR registers, interrupt handling |

### Toolchain Documentation Categories

| Category | Tools |
|----------|-------|
| Foundation | sw-cor24-emulator, sw-cor24-x-assembler, sw-cor24-assembler |
| Cross-compilers | sw-cor24-x-tinyc, sw-cor24-rust |
| P-code system | sw-cor24-pcode, sw-cor24-x-pc-aotc, sw-cor24-pascal |
| Native languages | macrolisp, forth, apl, basic, fortran, plsw, script |
| System software | sw-cor24-monitor, sw-cor24-debugger |
| Web demos | All 9 web-sw-cor24-* repos |

### Demo Card Data Model

```rust
struct DemoEntry {
    name: &'static str,
    slug: &'static str,        // repo name (web-sw-cor24-*)
    description: &'static str,
    url: &'static str,         // https://sw-embed.github.io/...
    repo_url: &'static str,    // https://github.com/sw-embed/...
    has_web_ui: bool,
    status: DemoStatus,        // Active, Beta, Planned
    tags: &'static [&'static str],
}
```

## Deployment Architecture

```
source (src/)  --Trunk-->  dist/ (gitignored)  --rsync-->  pages/ (committed)
                                                                |
                                                          GitHub Actions
                                                          (push to main)
                                                                |
                                                          GitHub Pages
                                                          (sw-embed.github.io/...)
```

## Constraints

- **No path dependencies** on other sw-embed repos (this is docs-only)
- **No runtime content fetching** -- all content is compile-time embedded
- **Minimal WASM size** -- opt-level "z", LTO enabled
- **Consistent theme** -- Catppuccin Mocha matching all other web-sw-cor24-* demos
- **Responsive** -- works on desktop and mobile
- **Accessible** -- semantic HTML, proper heading hierarchy
