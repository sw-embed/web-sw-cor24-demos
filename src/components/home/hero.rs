use yew::prelude::*;

#[function_component(HeroSection)]
pub fn hero_section() -> Html {
    html! {
        <div class="hero">
            <div class="hero-layout">
                <div class="hero-main">
                    <h1 class="hero-title">{"Software Wrighter COR24 Tools Project"}</h1>
                    <p class="hero-tagline">{"24-bit RISC for FPGA embedded systems"}</p>
                </div>
                <aside class="hero-arch-box">
                    <p>
                        {"COR24 is a 24-bit RISC processor designed for FPGA-based embedded systems. \
                        It provides a clean, orthogonal instruction set optimized for code density and simplicity, \
                        supporting a full software ecosystem of cross-compilers, interpreters, and live browser-based tools. "}
                        <a href="https://makerlisp.com" target="_blank" rel="noopener noreferrer">
                            {"makerlisp.com"}
                        </a>
                    </p>
                </aside>
            </div>
        </div>
    }
}
