use yew::prelude::*;

pub fn render_all_pipelines() -> Html {
    html! {
        <>
            <div class="pipeline-intro">
                <p>
                    {"The COR24 toolchain has two layers. "}
                    <strong>{"Rust-built host tools"}</strong>
                    {" run on the developer's machine and produce COR24 binaries. "}
                    <strong>{"C-built native tools"}</strong>
                    {" are cross-compiled with tc24r and run on COR24 hardware (or the emulator)."}
                </p>
            </div>
            {render_foundation()}
            {render_languages()}
        </>
    }
}

fn step(label: &'static str, note: &str) -> Html {
    html! {
        <span class="pipe-step">
            <span class="pipe-step-label">{label}</span>
            <span class="pipe-step-note">{note}</span>
        </span>
    }
}

fn arrow() -> Html {
    html! { <span class="pipe-arrow">{"\u{2192}"}</span> }
}

fn file(f: &str) -> Html {
    html! { <span class="pipe-file">{f}</span> }
}

fn render_foundation() -> Html {
    html! {
        <div class="pipeline-group">
            <h3 class="pipeline-group-title">{"Rust Toolchain (host-side)"}</h3>
            <p class="pipeline-group-desc">
                {"These three tools form the backbone. All are written in Rust."}
            </p>
            <div class="pipeline-cards">
                <div class="pipeline-card">
                    <h4>{"COR24 Emulator (cor24-run)"}</h4>
                    <div class="pipe-flow">
                        {file(".bin")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                        {arrow()}
                        {file("execution")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Takes one or more .bin files as input. Loads and executes COR24 binaries \
                         in a virtual machine with full register and memory inspection. \
                         Also compiles to WebAssembly for browser-based web UIs."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Cross-Assembler"}</h4>
                    <div class="pipe-flow">
                        {file(".s")}
                        {arrow()}
                        {step("cross-assembler", "Rust")}
                        {arrow()}
                        {file(".bin")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Takes COR24 assembly (.s) as input, produces .bin machine code. \
                         Also available as a Rust library crate used by cross-compilers and web UIs."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Tiny C Cross-Compiler (tc24r)"}</h4>
                    <div class="pipe-flow">
                        {file(".c .h")}
                        {arrow()}
                        {step("tc24r", "Rust")}
                        {arrow()}
                        {file(".s")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Takes C source and headers as input, produces COR24 assembly (.s). \
                         Restricted C subset: no structs, no heap, 24-bit int. Output feeds into \
                         the cross-assembler to produce .bin."}
                    </p>
                </div>
            </div>
        </div>
    }
}

fn render_languages() -> Html {
    html! {
        <div class="pipeline-group">
            <h3 class="pipeline-group-title">{"Native Languages & Tools (COR24-side)"}</h3>
            <p class="pipeline-group-desc">
                {"Written in C, cross-compiled with tc24r, assembled to .bin, loaded by cor24-run."}
            </p>
            <div class="pipeline-cards">
                <div class="pipeline-card">
                    <h4>{"APL (apl-sw)"}</h4>
                    <div class="pipe-flow">
                        {file(".apl")}
                        {arrow()}
                        {step("interpreter", "C")}
                        {arrow()}
                        {step("tc24r", "Rust")}
                        {arrow()}
                        {file(".s")}
                        {arrow()}
                        {step("assembler", "Rust")}
                        {arrow()}
                        {file(".bin")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Tree-walking evaluator with lazy iota generator, rank <= 2 arrays, \
                         ASCII keyword syntax."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Forth"}</h4>
                    <div class="pipe-flow">
                        {file(".forth")}
                        {arrow()}
                        {step("DTC Forth IDE", "C")}
                        {arrow()}
                        {step("tc24r", "Rust")}
                        {arrow()}
                        {file(".s")}
                        {arrow()}
                        {step("assembler", "Rust")}
                        {arrow()}
                        {file(".bin")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Direct-threaded code Forth with interactive IDE, dictionary browsing, \
                         and stack inspection."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Fortran"}</h4>
                    <div class="pipe-flow">
                        {step("emulator", "Rust")}
                        {arrow()}
                        {step("PL/SW", "C")}
                        {arrow()}
                        {step("SNOBOL4", "PL/SW")}
                        {arrow()}
                        {step("Fortran compiler", "SNOBOL4")}
                        {arrow()}
                        {file(".bin")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"In development. Each layer enables the next: emulator → PL/SW → SNOBOL4 → Fortran."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Macro Lisp"}</h4>
                    <div class="pipe-flow">
                        {file(".lisp")}
                        {arrow()}
                        {step("interpreter + GC", "C")}
                        {arrow()}
                        {step("tc24r", "Rust")}
                        {arrow()}
                        {file(".s")}
                        {arrow()}
                        {step("assembler", "Rust")}
                        {arrow()}
                        {file(".bin")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Lisp-1 interpreter with mark-sweep garbage collector, lexical scoping, \
                         defmacro, and closures."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"OCaml (via P-code VM)"}</h4>
                    <div class="pipe-flow">
                        {file(".ml")}
                        {arrow()}
                        {step("compiler", "C")}
                        {arrow()}
                        {file(".spc")}
                        {arrow()}
                        {step("pvm", "COR24 asm")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"In development. Compiles a subset of OCaml to P-code bytecode, reusing \
                         the Pascal P-code VM infrastructure on COR24."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Pascal + BASIC (via P-code VM)"}</h4>
                    <div class="pipe-flow">
                        {file(".pas / .bas")}
                        {arrow()}
                        {step("compiler", "C")}
                        {arrow()}
                        {file(".spc")}
                        {arrow()}
                        {step("pvm", "COR24 asm")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Pascal compiler (p24p) and BASIC interpreter produce P-code bytecode. \
                         The P-code VM (pvm) is a stack-based virtual machine written in COR24 \
                         assembly. An AOT compiler (pc-aotc) can convert .p24 to native .s."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"PL/SW"}</h4>
                    <div class="pipe-flow">
                        {file(".plsw .msw")}
                        {arrow()}
                        {step("compiler", "C")}
                        {arrow()}
                        {file(".s")}
                        {arrow()}
                        {step("assembler", "Rust")}
                        {arrow()}
                        {file(".bin")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"PL/I-inspired systems language. Produces .s directly (not via tc24r). \
                         Rich types (BIT, BYTE, WORD, INT, CHAR, PTR), inline ASM, macro system."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"Prolog (via WAM-like VM)"}</h4>
                    <div class="pipe-flow">
                        {step("emulator", "Rust")}
                        {arrow()}
                        {step("PL/SW", "C")}
                        {arrow()}
                        {step("WAM 8+8 VM", "PL/SW")}
                        {arrow()}
                        {file(".bin")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"In development. WAM-like 8+8 register virtual machine (8 argument + 8 temporary) \
                         implemented in PL/SW. Provides unification and backtracking for logic programming \
                         on COR24."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"SNOBOL4"}</h4>
                    <div class="pipe-flow">
                        {step("emulator", "Rust")}
                        {arrow()}
                        {step("PL/SW", "C")}
                        {arrow()}
                        {step("SNOBOL4 interpreter", "PL/SW")}
                        {arrow()}
                        {file(".bin")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Pattern-matching language interpreter implemented in PL/SW. \
                         Provides powerful string and pattern manipulation capabilities on COR24."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"SWS"}</h4>
                    <div class="pipe-flow">
                        {file(".sws")}
                        {arrow()}
                        {step("interpreter", "C")}
                        {arrow()}
                        {step("tc24r", "Rust")}
                        {arrow()}
                        {file(".s")}
                        {arrow()}
                        {step("assembler", "Rust")}
                        {arrow()}
                        {file(".bin")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Tcl-like scripting language for shell and editor automation."}
                    </p>
                </div>
                <div class="pipeline-card">
                    <h4>{"yocto-ed"}</h4>
                    <div class="pipe-flow">
                        {file("ASCII text")}
                        {arrow()}
                        {step("editor", "C")}
                        {arrow()}
                        {step("tc24r", "Rust")}
                        {arrow()}
                        {file(".s")}
                        {arrow()}
                        {step("assembler", "Rust")}
                        {arrow()}
                        {file(".bin")}
                        {arrow()}
                        {step("cor24-run", "Rust")}
                    </div>
                    <p class="pipeline-card-detail">
                        {"Gap-buffer modal text editor. Reads/writes via UART. \
                         Dogfooding project for the full tc24r toolchain."}
                    </p>
                </div>
            </div>
        </div>
    }
}
