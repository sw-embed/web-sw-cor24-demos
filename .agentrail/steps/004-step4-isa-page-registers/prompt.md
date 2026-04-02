Phase 3, Step 4: ISA Page Shell + Registers

Create the ISA documentation page structure and the registers section.

Tasks:
1. Read sw-cor24-emulator source to extract register definitions:
   - src/cpu/instruction.rs (register constraints)
   - src/cpu/state.rs (register usage)
   - src/assembler.rs (register names)
   - Also check sw-cor24-forth/CLAUDE.md for register allocation details
2. Create src/data/isa.rs with:
   - Register struct (name, alias, number, purpose, read_constraints, write_constraints)
   - Static array of all 8 registers: r0 (GP/scratch), r1 (GP/scratch/link), r2 (GP/scratch), r3/fp (frame pointer), r4/sp (stack pointer), r5/z (zero, hardwired), r6/iv (interrupt vector), r7/ir (interrupt return)
   - Document constraints: only r0-r2 can be load/ALU destinations, fp can be base register for load/store, sp can be push/pop target
3. Create src/components/isa/mod.rs with IsaPage component and sidebar navigation
4. Create src/components/isa/registers.rs with RegistersSection:
   - Table with columns: Register, Alias, Number, Purpose, Load Dest?, ALU Dest?, Notes
   - Highlight constraints (e.g., 'ILLEGAL' uses marked in red/accent color)
   - Include cell size note (3 bytes = 24-bit words)
5. Wire IsaPage into the app router
6. Verify: register table renders completely with all 8 registers and their constraints documented accurately