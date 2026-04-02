# Implementation Plan -- COR24 Ecosystem Landing Page

## Phase 1: Project Scaffold

### Step 1: Rust/Yew/Trunk Project Init
- Set up Cargo.toml with Yew 0.21, wasm-bindgen, web-sys
- Create index.html with Trunk directives and Catppuccin Mocha theme
- Set up scripts/serve.sh and scripts/build-pages.sh
- Create build.rs with BUILD_SHA, BUILD_HOST, BUILD_TIMESTAMP
- Configure Cargo.toml release profile (opt-level "z", lto true)
- Set up .gitignore (target/, dist/)
- Create GitHub Actions workflow for pages/ deployment
- Verify: `./scripts/serve.sh` shows blank page, `cargo check --target wasm32-unknown-unknown` passes

### Step 2: App Shell + Routing + Theme
- Create App component with enum-based routing (Home, Isa, Demos, Toolchain, Ecosystem)
- Create Header component with nav links and mobile hamburger menu
- Create Footer component
- Set up CSS custom properties for Catppuccin Mocha colors
- Create layout.css with responsive grid breakpoints
- Create components.css with card, table, badge base styles
- Add monospace font stack (JetBrains Mono, Fira Code, Cascadia Code)
- Verify: navigation between sections works, theme renders correctly

## Phase 2: Home Page

### Step 3: Home Page Content
- Create HeroSection component (title, tagline, makerlisp.com link)
- Create demo data module (data/demos.rs) with all 9 web demo entries
- Create DemoCard component (name, description, status badge, live link, repo link)
- Create demo grid layout on home page (all 9 demos in responsive grid)
- Add "What is COR24?" overview section
- Verify: all demo cards render, links open correctly

## Phase 3: ISA Documentation

### Step 4: ISA Page Shell + Registers
- Create ISA page component with sidebar navigation
- Create isa data module (data/isa.rs) with register definitions
- Create RegistersSection component with table of r0-r7
- Include register constraints (which regs can be load/ALU destinations)
- Verify: register table renders with all 8 registers documented

### Step 5: ISA Instruction Set
- Add instruction data to isa module (all 49 mnemonics)
- Create InstructionsSection component with category-grouped table
- Categories: arithmetic, logical, comparison, load/store, branch, call/return, stack, misc
- Include encoding format notes (1/2/4 byte variable-length)
- Verify: instruction table renders completely

### Step 6: ISA Memory Map + Calling Conventions
- Add memory region data to isa module (SRAM, EBR, I/O addresses)
- Create MemoryMapSection with address range table
- Create CallingConventionsSection with stack frame layout description
- Create AddressingModes section
- Add interrupt handling section (IV/IR registers)
- Verify: all ISA sections render and sidebar nav links work

## Phase 4: Demos Directory

### Step 7: Full Demos Page
- Create DemosPage component with filter bar
- Implement search-by-name filtering
- Implement category filter (compiler, interpreter, IDE, debugger)
- Implement status filter (active, beta, planned)
- Reuse DemoCard component from home page
- Add planned/future demo entries (debugger, etc.)
- Verify: filtering works, all cards display correctly on mobile

## Phase 5: Toolchain Documentation

### Step 8: Toolchain Data + Foundation Tools
- Create tool data module (data/tools.rs)
- Add foundation tools: emulator, cross-assembler, native assembler
- Create ToolCard component (name, description, language, target, links)
- Create CategorySection component for grouping tools
- Verify: foundation tool cards render with correct repo links

### Step 9: Cross-compilers + P-code System
- Add cross-compiler tools: tc24r, rust-to-cor24
- Add p-code system tools: pcode, pc-aotc, pascal
- Create pipeline diagram components (text/CSS based)
  - C pipeline: .c -> tc24r -> .s -> as24 -> .bin
  - Pascal pipeline: .pas -> p24p -> .spc -> pa24r -> .p24 -> pvm.s
  - AOT pipeline: .p24 -> pc-aotc -> .s -> as24 -> .bin
  - Rust pipeline: .rs -> rustc -> MSP430 asm -> msp430-to-cor24 -> .s
- Verify: all tool cards render, pipeline diagrams are readable

### Step 10: Native Languages + System Software
- Add native language tools: macrolisp, forth, apl, basic, fortran, plsw, script
- Add system software tools: monitor, debugger (planned)
- Add tc24r C subset constraints documentation
- Verify: all tools documented, category grouping is correct

## Phase 6: Ecosystem Map

### Step 11: Ecosystem Page + Repository Map
- Create EcosystemPage component
- Create RepoGrid component showing all ~20 repos with links
- Create dependency diagram (text-based, showing which repos depend on which)
- Add "Getting Started" section with links to first steps
- Add contribution guide link
- Verify: all repos listed, dependency graph is accurate

## Phase 7: Polish & Deploy

### Step 12: Responsive Design + Accessibility
- Test all pages at mobile (320px), tablet (768px), desktop (1024px+)
- Ensure hamburger menu works on mobile
- Verify heading hierarchy (h1 -> h2 -> h3)
- Check color contrast ratios
- Add proper `alt` text and aria labels
- Verify: pages look good and are usable on all sizes

### Step 13: Build + Deploy
- Run full pre-commit quality gate
- Run `./scripts/build-pages.sh` to generate pages/
- Create pages/.nojekyll
- Commit pages/ directory
- Verify: GitHub Actions deploys successfully
- Verify: https://sw-embed.github.io/web-sw-cor24-demos/ loads correctly

## Effort Estimates

| Phase | Steps | Est. Complexity |
|-------|-------|----------------|
| Phase 1: Scaffold | 2 | Medium (Trunk/Yew setup) |
| Phase 2: Home | 1 | Low (data + cards) |
| Phase 3: ISA Docs | 3 | Medium (lots of tabular content) |
| Phase 4: Demos | 1 | Low (filter UI) |
| Phase 5: Toolchain | 3 | Medium (pipeline diagrams) |
| Phase 6: Ecosystem | 1 | Low (repo grid) |
| Phase 7: Polish | 2 | Low (CSS tuning) |

**Total: 13 steps, estimated 8-12 agent sessions**

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| ISA content accuracy | Cross-reference with sw-cor24-emulator CLAUDE.md files |
| Broken demo links | All URLs follow predictable pattern; verify at deploy time |
| WASM bundle size | No heavy dependencies; content is static Rust data |
| Theme drift | Use same CSS custom properties as other demos |
