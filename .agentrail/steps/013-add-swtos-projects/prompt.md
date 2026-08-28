Add the SWTOS projects (sw-embed/sw-tos and sw-embed/web-sw-tos) to the COR24 landing page.

SWTOS is a MINIX-1-inspired tiny microkernel operating system for COR24 --
NOT a programming language. It is written in PL/SW and runs on the COR24-TB
FPGA board and in the emulator. web-sw-tos is its browser demo (Rust/WASM,
live at https://sw-embed.github.io/web-sw-tos/).

Tasks:
1. src/data/demos.rs -- add a new 'operating-system' Category (label
   'Operating System') between 'scripting' and 'tools', holding SWTOS
   (sw-tos) and the SWTOS Web Terminal (web-sw-tos) demo entries.
2. src/data/tools/data.rs -- add a new 'operating-systems' ToolGroup with the
   SWTOS entry; update the group/tool count tests.
3. src/data/status/mod.rs -- add ProjectRow entries for sw-tos (group
   'Operating System') and web-sw-tos (group 'Web UIs'); bump the PROJECTS
   array length.
4. src/components/status/ecosystem/eco_deps.rs -- add dependency edges
   (web-sw-tos -> sw-tos WASM build; sw-tos -> sw-cor24-plsw compiled by;
   sw-tos -> sw-cor24-emulator ISA defs) and a DepGroup for the OS; update
   repo_group() and the edge_count test.
5. src/components/status/ecosystem/dep_blocks.rs -- render the new OS deps.
6. src/components/toolchain/pipelines.rs -- add a SWTOS pipeline card.
7. tools/gen-status/src/main.rs and scripts/gen-status.sh -- add sw-tos and
   web-sw-tos to the REPOS lists.
8. README.md and AGENTS.md -- list both repos in the ecosystem sections.
9. Regenerate status data: cargo run --manifest-path tools/gen-status/Cargo.toml
   and cargo run --manifest-path tools/gen-issue-chart/Cargo.toml.
   (reports/commits.html is generated in the sibling sw-cor24-project repo and
   is explicitly OUT OF SCOPE for this step, per user decision.)
10. Run the pre-commit gate (fmt, clippy, tests, wasm check), then
    ./scripts/build-pages.sh and commit pages/.

Verify: cargo test passes, clippy clean, site builds, SWTOS appears on Home,
Toolchain, and Status pages.