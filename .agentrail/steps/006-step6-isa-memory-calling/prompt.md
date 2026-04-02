Phase 3, Step 6: ISA Memory Map + Calling Conventions

Complete the ISA documentation with memory map, calling conventions, addressing modes, and interrupts.

Tasks:
1. Read sw-cor24-emulator source to extract memory layout:
   - src/cpu/state.rs (memory regions, I/O addresses)
   - Also check CLAUDE.md files for UART/LED addresses
2. Add to src/data/isa.rs:
   - MemoryRegion struct (name, start_addr, end_addr, size, description, type)
   - Static array of regions: SRAM (1MB), EBR stack (8KB at 0xFEEC00 growing down), I/O (UART at 0xFF0100-01, LED/switch at 0xFF0000)
   - CallingConvention data (stack frame layout, argument passing, return value)
   - AddressingMode data (immediate, register, absolute, indexed, stack-relative)
3. Create src/components/isa/memory_map.rs with MemoryMapSection:
   - Table: Region, Start Address, End Address, Size, Type, Description
   - Visual memory map diagram (text-based bar chart showing address ranges)
4. Create src/components/isa/calling_conv.rs with CallingConventionsSection:
   - Stack frame layout diagram (text/CSS)
   - Argument passing rules (args on stack)
   - Return value (in r0)
   - Link register (r1 for jal)
   - Callee-saved vs caller-saved register conventions
5. Create src/components/isa/addressing_modes.rs with AddressingModesSection:
   - Table of all addressing modes with examples
6. Create interrupt handling section in ISA page (IV/IR registers r6/r7)
7. Update ISA sidebar navigation with all new sections
8. Verify: all ISA sections render, sidebar links scroll to correct sections, memory map diagram is accurate