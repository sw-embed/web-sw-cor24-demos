# PRD -- COR24 Ecosystem Landing Page

## Product Vision

A single, authoritative landing page for the COR24 ecosystem that serves as the entry point for all live demos, ISA documentation, and tooling references. The page should feel like a polished product portal, not a wiki -- informative, navigable, and visually consistent with the Catppuccin Mocha dark theme used across all COR24 web demos.

## Goals

### Primary Goals

1. **Central Navigation Hub** -- Provide a single URL that links to all 9 live web demos and all ~20 GitHub repositories
2. **ISA Documentation** -- Comprehensive, browsable reference for the COR24 instruction set architecture
3. **Toolchain Documentation** -- Description of every tool in the ecosystem, organized by function
4. **Ecosystem Map** -- Show how all the pieces fit together (compilers, interpreters, VM, monitor, etc.)

### Secondary Goals

5. **Onboarding** -- Help newcomers understand what COR24 is and how to get started
6. **Search/Filter** -- Allow users to find specific demos, tools, or ISA references quickly
7. **Consistency** -- Match the visual language of all other COR24 web demos

## Target Users

| User | Need |
|------|------|
| New visitors | Understand what COR24 is, see live demos |
| Students | ISA reference, try assembly in the IDE |
| Developers | Find tools, understand toolchain pipeline |
| Contributors | Repository map, contribution guide |
| FPGA engineers | ISA specs, licensing info (makerlisp.com link) |

## Functional Requirements

### FR-1: Home Page
- Hero section with COR24 logo/title and brief description
- "Try it live" call-to-action linking to a featured demo
- Quick-link grid to all live web demos
- Brief ecosystem overview

### FR-2: Live Demos Directory
- Grid of demo cards, each showing:
  - Demo name and one-line description
  - Live URL (opens in new tab)
  - GitHub repo URL
  - Status badge (active/beta/planned)
  - Category tags (compiler, interpreter, IDE, etc.)
- Filter by category, status, or search by name

### FR-3: ISA Reference
- Browsable sections for:
  - Registers (r0-r7 with roles and constraints)
  - Instruction set (all 49 mnemonics with encoding)
  - Memory map (SRAM, EBR, I/O addresses)
  - Addressing modes
  - Calling conventions (stack frame layout)
  - Interrupt handling
- Table of contents / sidebar navigation within ISA section
- Link to makerlisp.com for hardware/fpga details

### FR-4: Toolchain Documentation
- Organized by category:
  - Foundation (emulator, assemblers)
  - Cross-compilers (tc24r, rust-to-cor24)
  - P-code system (VM, assembler, linker, AOT compiler, Pascal)
  - Native languages (lisp, forth, apl, basic, fortran, plsw, script)
  - System software (monitor, debugger)
- Each tool entry shows:
  - Name and one-line description
  - Language it's written in
  - What it targets/produces
  - GitHub repo link
  - Whether it has a live web demo (link to it)
  - Build/run instructions summary

### FR-5: Ecosystem Map
- Visual representation of how tools relate to each other
- Pipeline diagrams showing compilation flows:
  - C source -> tc24r -> COR24 asm -> as24 -> binary
  - Pascal source -> p24p -> p-code -> pvm.s OR pc-aotc -> native
  - Rust source -> rustc -> MSP430 asm -> msp430-to-cor24 -> COR24 asm
  - Lisp/APL/Forth/BASIC source -> tc24r -> COR24 binary
- Dependency relationships between repos

### FR-6: Navigation
- Top navigation bar with section links
- Responsive hamburger menu on mobile
- Breadcrumb or section indicator
- Smooth scroll or SPA-style routing

## Non-Functional Requirements

### NFR-1: Performance
- Initial WASM load under 100 KB (gzipped)
- First contentful paint under 2 seconds
- No external dependencies loaded at runtime

### NFR-2: Visual Consistency
- Catppuccin Mocha dark theme (same as all other web demos)
- Monospace font stack: JetBrains Mono, Fira Code, Cascadia Code
- Consistent spacing, typography, and color tokens

### NFR-3: Responsiveness
- Usable on desktop (1024px+) and mobile (320px+)
- Grid layouts collapse to single column on mobile

### NFR-4: Accessibility
- Semantic HTML elements
- Proper heading hierarchy (h1 -> h2 -> h3)
- Sufficient color contrast
- Keyboard navigable

### NFR-5: Maintainability
- File size limit: 500 lines per file
- Function size limit: 50 lines
- Content data separated from rendering components
- Easy to add new demo entries or tool descriptions

## Out of Scope

- Interactive ISA simulator (that's the assembler IDE's job)
- Running any COR24 code (this is documentation-only)
- User accounts or authentication
- Comments or community features
- Downloading/building tools from the page (link to repos instead)
