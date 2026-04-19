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

fn intro_text() -> Html {
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
            {intro_closing()}
            {tech_doc_callout()}
        </div>
    }
}

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="about-page page-section">
            <h1 class="about-title">{"About"}</h1>
            <div class="about-intro-row">
                <div class="about-intro-links">
                    {links().iter().map(link_card).collect::<Html>()}
                </div>
                {intro_text()}
            </div>
        </div>
    }
}
