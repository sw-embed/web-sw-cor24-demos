Correct a factual error in the SWTOS descriptions added in step 013.

The COR24-TB board HAS hardware multiply. It does NOT have hardware
divide. The SWTOS copy added in step 013 wrongly claims 'no hardware
multiply' (it was taken from the sw-tos README's portability wording and
misapplied as a statement about the platform).

Fix all three occurrences to say 'no hardware divide' instead:
1. src/components/toolchain/pipelines.rs -- SWTOS pipeline card detail
2. src/data/demos.rs -- SWTOS demo card description
3. src/data/tools/data.rs -- SWTOS ToolEntry description

Note src/data/isa/instructions.rs already documents a 'mul' instruction
and no 'div', and docs/language-building-tech.md already says 'no hardware
divide' -- so the rest of the site is already correct and needs no change.

Then: run the pre-commit gate (fmt, clippy -D warnings, test, wasm check),
./scripts/build-pages.sh, commit, and merge to main with plain git
(checkout main, merge --no-ff, push) so the fix reaches the live site.
Do NOT use the gh CLI.