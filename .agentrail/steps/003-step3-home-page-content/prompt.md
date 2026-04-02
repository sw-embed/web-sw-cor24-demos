Phase 2, Step 3: Home Page Content

Build the complete home page with hero section, demo grid, and overview.

Tasks:
1. Create src/data/demos.rs with DemoEntry struct and static data for all 9 live web demos:
   - web-sw-cor24-assembler: COR24 Assembly IDE (active, IDE)
   - web-sw-cor24-pcode: P-code VM Debugger (active, debugger)
   - web-sw-cor24-tinyc: Tiny C Compiler (active, compiler)
   - web-sw-cor24-macrolisp: Lisp REPL (active, interpreter)
   - web-sw-cor24-pascal: Pascal Demos (active, compiler)
   - web-sw-cor24-apl: APL Environment (active, interpreter)
   - web-sw-cor24-forth: Forth Debugger (active, debugger)
   - web-sw-cor24-plsw: PL/SW Development Environment (active, IDE)
   - web-sw-cor24-demos: THIS landing page (active, docs)
   Each entry needs: name, repo slug, description, live_url, repo_url, status, tags
2. Create src/components/home/mod.rs with HomePage component
3. Create src/components/home/hero.rs with HeroSection:
   - Large 'COR24 Ecosystem' title
   - Tagline: '24-bit RISC for FPGA embedded systems'
   - Link to makerlisp.com
   - Brief description paragraph
4. Create src/components/home/demo_grid.rs with DemoCard and DemoGrid components:
   - Card shows: name, one-line description, status badge, category tags
   - Two buttons: 'Try Live' (opens live URL) and 'Source' (opens GitHub repo)
   - Responsive grid: 3 cols desktop, 2 tablet, 1 mobile
5. Add 'What is COR24?' overview section below the demo grid:
   - Brief description of the 24-bit RISC architecture
   - Key facts: 8 registers, 1MB SRAM, 8KB EBR stack, variable-length instructions
   - Link to ISA documentation section
6. Verify: all 9 demo cards render, links open in new tabs, responsive layout works