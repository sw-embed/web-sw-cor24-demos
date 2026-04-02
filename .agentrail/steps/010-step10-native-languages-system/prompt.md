Phase 5, Step 10: Native Languages + System Software

Complete the toolchain documentation with native languages and system software.

Tasks:
1. Add to src/data/tools.rs:
   - Native languages:
     * sw-cor24-macrolisp: Tiny Macro Lisp - Lisp-1 with lexical scope, defmacro, closures, GC (C, COR24, has web demo)
     * sw-cor24-forth: DTC Forth interpreter - clean-room implementation (Assembly, COR24, has web demo)
     * sw-cor24-apl: APL interpreter - integer-only, rank<=2, ASCII surface syntax (C, COR24, has web demo)
     * sw-cor24-basic: BASIC interpreter (C, COR24, planned web demo)
     * sw-cor24-fortran: Fortran compiler (C, COR24, planned web demo)
     * sw-cor24-plsw: PL/SW compiler - PL/I-inspired systems language (C, COR24, has web demo)
     * sw-cor24-script: SWS scripting language - Tcl-like shell/editor scripting (C, COR24, planned web demo)
   - System software:
     * sw-cor24-monitor: Resident monitor/service processor - boots at address 0, program invocation ABI (Assembly+C, COR24)
     * sw-cor24-debugger: Source-level debugger (planned)
2. Add NativeLanguages section and SystemSoftware section to ToolchainPage
3. Add a note about tc24r C subset constraints (no structs, no malloc, no string library, 24-bit int, single translation unit)
4. Show which tools have live web demos (link to them) vs planned
5. Verify: all ~20 tools are documented, category grouping is correct, live demo links work