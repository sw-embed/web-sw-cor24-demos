use yew::prelude::*;

pub fn render_about() -> Html {
    html! {
        <section class="eco-section">
            <h2 class="section-heading">{"About COR24"}</h2>
            <div class="eco-about">
                {about_arch()}
                {about_ecosystem()}
                {about_hardware()}
            </div>
        </section>
    }
}

fn about_arch() -> Html {
    html! {
        <p class="eco-about-p">
            {"COR24 (C-Oriented RISC) is a real 24-bit RISC architecture designed for embedded systems education. It features 3 general-purpose registers (plus fp, sp, z, and interrupt registers iv/ir), variable-length instructions (1/2/4 bytes), and a C-friendly calling convention that maps cleanly to the C abstract machine."}
        </p>
    }
}

fn about_ecosystem() -> Html {
    html! {
        <p class="eco-about-p">
            {"The architecture runs on FPGA via the "}
            <a href="https://makerlisp.com" target="_blank" rel="noopener noreferrer">
                {"MakerLisp COR24"}
            </a>
            {" implementation. The software ecosystem includes cross-compilers (C, Pascal, Rust), native-language interpreters (APL, Lisp, Forth, BASIC), a p-code VM system, a resident monitor, and browser-based live demos for most tools."}
        </p>
    }
}

fn about_hardware() -> Html {
    html! {
        <p class="eco-about-p">
            {"Visit "}
            <a href="https://makerlisp.com" target="_blank" rel="noopener noreferrer">
                {"makerlisp.com"}
            </a>
            {" for hardware details, FPGA bitstreams, and licensing information."}
        </p>
    }
}
