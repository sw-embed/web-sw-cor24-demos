use yew::prelude::*;

pub fn render_getting_started() -> Html {
    html! {
        <section class="eco-section">
            <h2 class="section-heading">{"Getting Started"}</h2>
            <div class="eco-grid">
                {getting_browser()}
                {getting_languages()}
                {getting_cli()}
            </div>
        </section>
    }
}

fn getting_browser() -> Html {
    html! {
        <div class="eco-card">
            <h3 class="eco-card-title">{"Try it in the browser"}</h3>
            <p class="eco-card-desc">
                {"The "}
                <a href="https://sw-embed.github.io/web-sw-cor24-assembler/"
                   target="_blank" rel="noopener noreferrer">{"Assembly IDE"}</a>
                {" is the easiest entry point. Write COR24 assembly, assemble it, and run it step-by-step in your browser. No installation required."}
            </p>
        </div>
    }
}

fn getting_languages() -> Html {
    html! {
        <div class="eco-card">
            <h3 class="eco-card-title">{"Try a high-level language"}</h3>
            <p class="eco-card-desc">
                {"Explore the "}
                <a href="https://sw-embed.github.io/web-sw-cor24-pascal/"
                   target="_blank" rel="noopener noreferrer">{"Pascal demos"}</a>
                {", "}
                <a href="https://sw-embed.github.io/web-sw-cor24-macrolisp/"
                   target="_blank" rel="noopener noreferrer">{"Lisp REPL"}</a>
                {", or "}
                <a href="https://sw-embed.github.io/web-sw-cor24-apl/"
                   target="_blank" rel="noopener noreferrer">{"APL environment"}</a>
                {" -- all running on COR24 via the browser-based emulator."}
            </p>
        </div>
    }
}

fn getting_cli() -> Html {
    html! {
        <div class="eco-card">
            <h3 class="eco-card-title">{"Command-line tools"}</h3>
            <p class="eco-card-desc">
                {"The "}
                <a href="https://github.com/sw-embed/sw-cor24-emulator"
                   target="_blank" rel="noopener noreferrer">{"sw-cor24-emulator"}</a>
                {" repo provides the assembler, emulator, and ISA definitions used by all other tools. Install it with "}
                <code>{"cargo install --path ."}</code>
                {"."}
            </p>
        </div>
    }
}
