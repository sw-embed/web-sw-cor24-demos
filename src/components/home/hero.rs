use yew::prelude::*;

#[function_component(HeroSection)]
pub fn hero_section() -> Html {
    html! {
        <div class="hero">
            <h1 class="hero-title">{"Software Wrighter COR24 Tools Project"}</h1>
            <p class="hero-tagline">{"24-bit RISC for FPGA embedded systems"}</p>
            <p class="hero-desc">
                {"A complete computing platform — from ISA specification and silicon synthesis \
                through cross-compilers, interpreters, and live browser demos. "}
                <a href="https://makerlisp.com" target="_blank" rel="noopener noreferrer">
                    {"makerlisp.com"}
                </a>
            </p>
            <div class="hero-actions">
                <a href="#/isa" class="btn btn-primary">{"ISA Docs"}</a>
                <a href="#" class="btn btn-secondary">{"Browse Demos"}</a>
            </div>
        </div>
    }
}
