Phase 3, Step 5: ISA Instruction Set

Add the complete COR24 instruction set documentation.

Tasks:
1. Read sw-cor24-emulator source to extract ALL instructions:
   - src/cpu/instruction.rs (opcode definitions, categories)
   - src/cpu/encode.rs (encoding tables, instruction formats)
   - src/cpu/executor.rs (execution semantics)
2. Add to src/data/isa.rs:
   - Instruction struct (mnemonic, format, encoding_bytes, description, category)
   - InstrCategory enum (Arithmetic, Logical, Comparison, LoadStore, Branch, CallReturn, Stack, Move, Misc)
   - Static array of all ~49 mnemonics with encoding details
   - Variable-length instruction format notes (1/2/4 bytes)
3. Create src/components/isa/instructions.rs with InstructionsSection:
   - Category-grouped tables (arithmetic, logical, comparison, load/store, branch, call/return, stack, move, misc)
   - Each table row: Mnemonic, Format/Operands, Encoding, Description
   - Collapsible sections per category
   - Encoding format summary section at top (1-byte, 2-byte, 4-byte formats)
4. Update ISA sidebar navigation to include instruction categories
5. Verify: instruction table renders with all 49 mnemonics, encoding formats are documented, categories are collapsible