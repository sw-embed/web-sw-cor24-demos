use yew::prelude::*;

fn ext_link(url: &'static str, text: &'static str) -> Html {
    html! {
        <a href={url} target="_blank" rel="noopener noreferrer">{text}</a>
    }
}

fn intro_closing() -> Html {
    html! {
        <p>
            {"I\u{2019}m driven by curiosity about how to build programming language implementations \
            in a constrained environment, building on my own tools \u{2014} dogfooding at every step. \
            I\u{2019}m currently exploring different garbage collectors, an AOT compiler for p-code \
            to COR24 native code, and JIT compilation. I may add other languages if they\u{2019}re \
            interesting, or if building one language from another is a compelling puzzle."}
        </p>
    }
}

fn intro_swtos() -> Html {
    html! {
        <p>
            {"PL/SW also turned out to be a capable systems language, so I used it to write "}
            {ext_link("https://github.com/sw-embed/sw-tos", "SWTOS")}
            {", a MINIX-inspired microkernel with message-passing IPC and preemptive \
            scheduling, which now "}
            {ext_link("https://sw-embed.github.io/web-sw-tos/", "runs in the browser")}
            {" alongside its tiled terminal frontend."}
        </p>
    }
}

fn tech_doc_callout() -> Html {
    html! {
        <a
            href="https://github.com/sw-embed/web-sw-cor24-demos/blob/main/docs/language-building-tech.md"
            target="_blank"
            rel="noopener noreferrer"
            class="about-doc-callout"
        >
            <div class="about-doc-callout-body">
                <h2 class="about-doc-callout-title">
                    {"The Approach, Goals, and Rationale"}
                </h2>
                <p class="about-doc-callout-desc">
                    {"A deep-dive on how the COR24 language stack is designed, layered, and \
                    bootstrapped \u{2014} from ISA up through the language groups, tooling, and \
                    porting strategy."}
                </p>
            </div>
            <span class="about-doc-callout-cta">{"Read on GitHub \u{2192}"}</span>
        </a>
    }
}

pub fn intro_text() -> Html {
    html! {
        <div class="about-intro-text">
            <p>
                {"I started by creating emulators for the IBM 1130, IBM 370, RCA 1802, and RISC-V ISAs \
                to study computer architecture from the ground up. Through that work I discovered "}
                {ext_link("https://makerlisp.com", "MakerLisp")}
                {"\u{2014} a Lisp-based system built on a custom FPGA-based \u{201c}C-Oriented RISC, 24-bit\u{201d} ISA. \
                I found a reference assembler for it, and from there used Rust to build an emulator, \
                a clone of the assembler, and then C compilers."}
            </p>
            <p>
                {"From those building blocks I explored how to cross-compile a subset of "}
                {ext_link("https://github.com/sw-embed/sw-cor24-rust", "Rust")}
                {" to run on the emulator. Then I started implementing languages on COR24 itself: "}
                {ext_link("https://github.com/sw-embed/sw-cor24-macrolisp", "APL")}
                {" and "}
                {ext_link("https://github.com/sw-embed/sw-cor24-apl", "Lisp")}
                {" in C, and "}
                {ext_link("https://github.com/sw-embed/sw-cor24-forth", "Forth")}
                {" in COR24 assembly."}
            </p>
            <p>
                {"I built a "}
                {ext_link("https://github.com/sw-embed/sw-cor24-pcode", "p-code stack VM")}
                {" to serve as a Pascal runtime, creating a Pascal compiler and linker in C. \
                Then I developed "}
                {ext_link("https://github.com/sw-embed/sw-cor24-plsw", "PL/SW")}
                {", a PL/I-inspired system programming language with macros. I used PL/SW as a \
                SIL (SNOBOL Implementation Language) to build "}
                {ext_link("https://github.com/sw-embed/sw-cor24-script", "SNOBOL4")}
                {"."}
            </p>
            <p>
                {"After hearing a podcast on SNOBOL4, I learned that Bell Labs once implemented a \
                Fortran compiler in SNOBOL4 \u{2014} so I started that too. Once Pascal was working, \
                I built a "}
                {ext_link("https://github.com/sw-embed/sw-cor24-basic", "BASIC interpreter")}
                {" in it, and then started an "}
                {ext_link("https://github.com/sw-embed/sw-cor24-ocaml", "OCaml REPL")}
                {". My latest work is a WAM-like register VM for "}
                {ext_link("https://github.com/sw-embed/sw-cor24-prolog", "Prolog")}
                {", with the VM implemented in PL/SW and the parser in SNOBOL4."}
            </p>
            {intro_swtos()}
            {intro_closing()}
            {tech_doc_callout()}
        </div>
    }
}
