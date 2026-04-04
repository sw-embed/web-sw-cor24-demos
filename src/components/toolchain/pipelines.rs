use yew::prelude::*;

pub fn render_all_pipelines() -> Html {
    html! {
        <>
            <div class="pipeline-intro">
                <p>
                    {"All COR24 tools follow a consistent pattern: "}
                    <strong>{"Rust-built host tools"}</strong>
                    {" (emulator, cross-assembler, cross-compiler, web UIs) provide the \
                     development environment, while "}
                    <strong>{"C-built native tools"}</strong>
                    {" (languages, monitor, editor) run directly on COR24 hardware. \
                     The emulator ("}<code>{"cor24-run"}</code>{") is used for testing both."}
                </p>
            </div>
            {render_rust_tools()}
            {render_c_languages()}
            {render_pcode_vm()}
        </>
    }
}

fn render_rust_tools() -> Html {
    html! {
        <div class="pipeline-group">
            <h3 class="pipeline-group-title">{"Rust Host Tools"}</h3>
            <p class="pipeline-group-desc">
                {"Written in Rust, compiled to native binaries. These run on the \
                 developer's machine and produce COR24 code or provide browser-based \
                 frontends."}
            </p>
            <div class="pipeline-grid">
                <div class="pipeline-card">
                    <h4>{"COR24 Emulator"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-emulator"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"Rust"}</span>
                            <span class="pipeline-label">{"native binary"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"assemble + emulate COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Contains the assembler (as24) and emulator. Also compiles to \
                         WebAssembly for browser-based execution."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Cross-Assembler Library"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-x-assembler"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"Rust"}</span>
                            <span class="pipeline-label">{"library crate"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".s"}</span>
                            <span class="pipeline-label">{"COR24 machine code"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Rust API for assembling COR24 code. Used by cross-compilers and \
                         web UIs that need programmatic assembly."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Tiny C Cross-Compiler (tc24r)"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-x-tinyc"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".c"}</span>
                            <span class="pipeline-label">{"C source"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"tc24r"}</span>
                            <span class="pipeline-label">{"cross-compile"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".s"}</span>
                            <span class="pipeline-label">{"COR24 assembly"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Restricted C subset (no structs, no heap, 24-bit int). Output feeds \
                         into cor24-run or the P-code AOT compiler."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Web UIs"}</h4>
                    <p class="pipeline-card-note">{"web-sw-cor24-* repos"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"Rust + Yew"}</span>
                            <span class="pipeline-label">{"WASM binary"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"emulator"}</span>
                            <span class="pipeline-label">{"runs in browser"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"9 browser frontends (Assembler IDE, P-code debugger, Tiny C compiler, \
                         Lisp REPL, Pascal demos, APL environment, Forth IDE, PL/SW IDE, \
                         landing page). Each embeds the emulator as WebAssembly."}
                    </p>
                </div>
            </div>
        </div>
    }
}

fn render_c_languages() -> Html {
    html! {
        <div class="pipeline-group">
            <h3 class="pipeline-group-title">{"Native Languages (C on COR24)"}</h3>
            <p class="pipeline-group-desc">
                {"Written in C, cross-compiled with tc24r, assembled and tested on cor24-run. \
                 These run directly on COR24 hardware via the P-code VM or as native code."}
            </p>
            <div class="pipeline-grid">
                <div class="pipeline-card">
                    <h4>{"Macro Lisp"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-macrolisp"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".c"}</span>
                            <span class="pipeline-label">{"Lisp interpreter + GC"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"tc24r"}</span>
                            <span class="pipeline-label">{"cross-compile"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"native COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Lisp-1 interpreter with lexical scoping, defmacro, closures, and \
                         mark-sweep garbage collector."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Forth"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-forth"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".s"}</span>
                            <span class="pipeline-label">{"DTC Forth (native asm)"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"as24"}</span>
                            <span class="pipeline-label">{"assemble"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"native COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Direct-threaded code Forth written in COR24 assembly. Interactive IDE \
                         with dictionary browsing and stack inspection."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"APL (apl-sw)"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-apl"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".c"}</span>
                            <span class="pipeline-label">{"APL interpreter"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"tc24r"}</span>
                            <span class="pipeline-label">{"cross-compile"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"native COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Tree-walking evaluator with lazy iota generator, rank <= 2 arrays, \
                         and ASCII keyword syntax."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"PL/SW"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-plsw"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".c"}</span>
                            <span class="pipeline-label">{"PL/I compiler"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"tc24r"}</span>
                            <span class="pipeline-label">{"cross-compile"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"native COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"PL/I-inspired systems language. Rich types (BIT, BYTE, WORD, INT, CHAR, PTR), \
                         inline ASM, macro system."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"SWS"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-script"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".c"}</span>
                            <span class="pipeline-label">{"script interpreter"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"tc24r"}</span>
                            <span class="pipeline-label">{"cross-compile"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"native COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Tcl-like scripting language for shell and editor automation. \
                         Dynamic typing with integer and string values."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Fortran"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-fortran"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".c"}</span>
                            <span class="pipeline-label">{"Fortran compiler"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"tc24r"}</span>
                            <span class="pipeline-label">{"cross-compile"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"native COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Planned. Will compile Fortran source to COR24 assembly for scientific \
                         and numeric computation."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"yocto-ed"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-yocto-ed"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".c"}</span>
                            <span class="pipeline-label">{"modal text editor"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"tc24r"}</span>
                            <span class="pipeline-label">{"cross-compile"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"native COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Gap-buffer editor with 3-line display, edit/command modes. Dogfooding \
                         project for the full tc24r toolchain."}
                    </p>
                </div>
            </div>
        </div>
    }
}

fn render_pcode_vm() -> Html {
    html! {
        <div class="pipeline-group">
            <h3 class="pipeline-group-title">{"P-Code VM System"}</h3>
            <p class="pipeline-group-desc">
                {"The P-code VM (pvm) is a stack-based virtual machine written in COR24 assembly. \
                 Pascal and BASIC compile to P-code bytecode and run on this VM. An AOT compiler \
                 can convert P-code to native COR24 for performance."}
            </p>
            <div class="pipeline-grid">
                <div class="pipeline-card">
                    <h4>{"Pascal (p24p)"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-pascal"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".pas"}</span>
                            <span class="pipeline-label">{"Pascal source"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"p24p"}</span>
                            <span class="pipeline-label">{"compile to .spc"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"pvm"}</span>
                            <span class="pipeline-label">{"run on P-code VM"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"C compiler producing P-code bytecode. Runtime library (32 routines) \
                         provides I/O, string handling, and system calls."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"BASIC"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-basic"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".bas"}</span>
                            <span class="pipeline-label">{"BASIC source"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"interpreter"}</span>
                            <span class="pipeline-label">{"line-numbered BASIC"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"pvm"}</span>
                            <span class="pipeline-label">{"run on P-code VM"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"1970s-style integer BASIC interpreter. Line-numbered, GOTO/GOSUB, \
                         FOR/NEXT, PRINT/INPUT. Targets the P-code VM."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"P-code AOT Compiler"}</h4>
                    <p class="pipeline-card-note">{"sw-cor24-x-pc-aotc"}</p>
                    <div class="pipeline-flow">
                        <span class="pipeline-step">
                            <span class="pipeline-file">{".p24"}</span>
                            <span class="pipeline-label">{"P-code bytecode"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"pc-aotc"}</span>
                            <span class="pipeline-label">{"AOT compile"}</span>
                        </span>
                        <span class="pipeline-arrow">{"\u{2192}"}</span>
                        <span class="pipeline-step">
                            <span class="pipeline-file">{"cor24-run"}</span>
                            <span class="pipeline-label">{"native COR24"}</span>
                        </span>
                    </div>
                    <p class="pipeline-card-detail">
                        {"Converts P-code bytecode to native COR24 assembly for performance \
                         without the VM overhead."}
                    </p>
                </div>
            </div>
        </div>
    }
}
