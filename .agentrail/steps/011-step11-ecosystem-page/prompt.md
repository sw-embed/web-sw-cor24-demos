Phase 6, Step 11: Ecosystem Page + Repository Map

Build the ecosystem overview page.

Tasks:
1. Create src/components/ecosystem/mod.rs with EcosystemPage component
2. Create src/components/ecosystem/repo_map.rs with RepoGrid showing ALL ~20 repos:
   - Grid of repo cards with: repo name, one-line description, GitHub link
   - Grouped by type: Web Demos, Foundation, Compilers, Languages, System, Other
   - Color-coded by type (web demos in blue, foundation in green, etc.)
3. Create a dependency diagram (text/CSS based) showing:
   - All web-sw-cor24-* demos depend on sw-cor24-emulator
   - Cross-compilers depend on sw-cor24-emulator/isa
   - Native tools (C) are compiled by sw-cor24-x-tinyc
   - P-code system tools interconnect (pcode <-> pascal <-> pc-aotc)
   - Monitor depends on emulator, x-assembler, x-tinyc
4. Add 'Getting Started' section:
   - Link to Assembly IDE as the easiest entry point
   - Link to emulator repo for CLI tools
   - Brief description of how to try COR24 in the browser
5. Add 'About' section:
   - COR24 is a real 24-bit RISC architecture (C-Oriented RISC)
   - Designed for embedded systems education
   - Runs on FPGA via MakerLisp COR24 implementation
   - Link to makerlisp.com for hardware details and licensing
6. Verify: all repos listed with correct links, dependency diagram is accurate, getting started links work