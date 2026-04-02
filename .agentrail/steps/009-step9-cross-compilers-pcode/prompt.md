Phase 5, Step 9: Cross-compilers + P-code System

Add cross-compilers, p-code system tools, and pipeline diagrams.

Tasks:
1. Add to src/data/tools.rs:
   - Cross-compilers:
     * sw-cor24-x-tinyc: Tiny C cross-compiler (Rust, Host, has web demo)
     * sw-cor24-rust: Rust-to-COR24 pipeline (Rust, Host)
   - P-code system:
     * sw-cor24-pcode: P-code VM, assembler, linker (Rust + Assembly, Host/COR24, has web demo)
     * sw-cor24-x-pc-aotc: P-code AOT compiler (Rust, Host)
     * sw-cor24-pascal: Pascal compiler + runtime (C, COR24, has web demo)
2. Create src/components/toolchain/pipelines.rs with PipelineDiagram component:
   - C pipeline: .c source -> tc24r (cross-compile) -> .s assembly -> as24 (cross-assemble) -> .bin binary -> cor24-run
   - Pascal pipeline: .pas source -> p24p (compile on COR24) -> .spc p-code -> pl24r (link) -> pa24r (assemble) -> .p24 bytecode -> pvm.s (execute on COR24 emulator)
   - AOT pipeline: .p24 bytecode -> pc-aotc (AOT compile) -> .s native assembly -> as24 (assemble) -> .bin binary -> cor24-run
   - Rust pipeline: .rs source -> rustc --target msp430-none-elf -> MSP430 assembly -> msp430-to-cor24 -> COR24 assembly -> as24 -> .bin
   Use CSS flexbox/grid for pipeline arrows and boxes (no images)
3. Add CrossCompilers section and PCodeSystem section to ToolchainPage
4. Include pipeline diagrams within the P-code system section
5. Verify: all tool cards render, pipeline diagrams are readable and show correct flow, arrows connect properly