use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct LinkEntry {
    label: &'static str,
    description: &'static str,
    url: &'static str,
    image: &'static str,
}

fn link_card(entry: &LinkEntry) -> Html {
    html! {
        <a href={entry.url} target="_blank" rel="noopener noreferrer" class="about-card">
            <img src={entry.image} alt={entry.label} class="about-card-icon" />
            <div class="about-card-body">
                <h2 class="about-card-title">{entry.label}</h2>
                <p class="about-card-desc">{entry.description}</p>
                <span class="about-card-cta">{"Visit \u{2192}"}</span>
            </div>
        </a>
    }
}

fn links() -> &'static [LinkEntry; 4] {
    &LINKS
}

static LINKS: [LinkEntry; 4] = [
    LinkEntry {
        label: "Software Wrighter Lab Blog",
        description: "Articles on FPGA design, computer architecture, COR24 development, and embedded systems education.",
        url: "https://software-wrighter-lab.github.io/",
        image: "images/sw-lab-logo.jpg",
    },
    LinkEntry {
        label: "Discord Community",
        description: "Join the Software Wrighter Lab Discord for discussions, questions, and project updates.",
        url: "https://discord.com/invite/Ctzk5uHggZ",
        image: "images/sw-lab-discord.png",
    },
    LinkEntry {
        label: "sw-embed on GitHub",
        description: "Browse the full COR24 ecosystem: emulators, cross-compilers, interpreters, and web demos.",
        url: "https://github.com/sw-embed",
        image: "images/sw-embed-git-org-logo.jpeg",
    },
    LinkEntry {
        label: "SoftwareWrighter on YouTube",
        description: "Video content covering FPGA builds, COR24 walkthroughs, and embedded systems tutorials.",
        url: "https://www.youtube.com/@SoftwareWrighter",
        image: "images/sw-yt-logo.jpg",
    },
];

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="about-page page-section">
            <h1 class="about-title">{"About"}</h1>

            <div class="about-intro-row">
                <div class="about-intro-links">
                    {links().iter().map(link_card).collect::<Html>()}
                </div>
                <div class="about-intro-text">
                    <p>
                        {"I started by creating emulators for the IBM 1130, IBM 370, RCA 1802, and RISC-V ISAs \
                        to study computer architecture from the ground up. Through that work I discovered "}
                        <a href="https://makerlisp.com" target="_blank" rel="noopener noreferrer">{"MakerLisp"}</a>
                        {"\u{2014} a Lisp-based system built on a custom FPGA-based \u{201c}C-Oriented RISC, 24-bit\u{201d} ISA. I found a reference assembler for it, and from there used Rust to build an emulator, a clone of the assembler, and then C compilers."}
                    </p>
                    <p>
                        {"From those building blocks I explored how to cross-compile a subset of "}
                        <a href="https://github.com/sw-embed/sw-cor24-rust" target="_blank" rel="noopener noreferrer">{"Rust"}</a>
                        {" to run on the emulator. Then I started implementing languages on COR24 itself: "}
                        <a href="https://github.com/sw-embed/sw-cor24-macrolisp" target="_blank" rel="noopener noreferrer">{"APL"}</a>
                        {" and "}
                        <a href="https://github.com/sw-embed/sw-cor24-apl" target="_blank" rel="noopener noreferrer">{"Lisp"}</a>
                        {" in C, and "}
                        <a href="https://github.com/sw-embed/sw-cor24-forth" target="_blank" rel="noopener noreferrer">{"Forth"}</a>
                        {" in COR24 assembly."}
                    </p>
                    <p>
                        {"I built a "}
                        <a href="https://github.com/sw-embed/sw-cor24-pcode" target="_blank" rel="noopener noreferrer">{"p-code stack VM"}</a>
                        {" to serve as a Pascal runtime, creating a Pascal compiler and linker in C. \
                        Then I developed "}
                        <a href="https://github.com/sw-embed/sw-cor24-plsw" target="_blank" rel="noopener noreferrer">{"PL/SW"}</a>
                        {", a PL/I-inspired system programming language with macros. I used PL/SW as a \
                        SIL (SNOBOL Implementation Language) to build "}
                        <a href="https://github.com/sw-embed/sw-cor24-script" target="_blank" rel="noopener noreferrer">{"SNOBOL4"}</a>
                        {"."}
                    </p>
                    <p>
                        {"After hearing a podcast on SNOBOL4, I learned that Bell Labs once implemented a Fortran compiler in SNOBOL4 \u{2014} so I started that too. Once Pascal was working, I built a "}
                        <a href="https://github.com/sw-embed/sw-cor24-basic" target="_blank" rel="noopener noreferrer">{"BASIC interpreter"}</a>
                        {" in it, and then started an "}
                        <a href="https://github.com/sw-embed/sw-cor24-ocaml" target="_blank" rel="noopener noreferrer">{"OCaml REPL"}</a>
                        {". My latest work is a WAM-like register VM for "}
                        <a href="https://github.com/sw-embed/sw-cor24-prolog" target="_blank" rel="noopener noreferrer">{"Prolog"}</a>
                        {", with the VM implemented in PL/SW and the parser in SNOBOL4."}
                    </p>
                    <p>
                        {"I\u{2019}m driven by curiosity about how to build programming language implementations in a constrained environment, building on my own tools \u{2014} dogfooding at every step. I\u{2019}m currently exploring different garbage collectors, an AOT compiler for p-code to COR24 native code, and JIT compilation. I may add other languages if they\u{2019}re interesting, or if building one language from another is a compelling puzzle."}
                    </p>
                </div>
            </div>
        </div>
    }
}
