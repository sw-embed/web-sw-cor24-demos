use yew::prelude::*;

use super::pipelines;
use super::sections;

#[function_component(ToolchainPage)]
pub fn toolchain_page() -> Html {
    html! {
        <div class="page-section">
            <h1>{"Toolchain Documentation"}</h1>
            <p class="toolchain-intro">
                {"How COR24 tools are built and used. Implementation details for the \
                 emulator, P-code VM, garbage collector, Forth runtime, and web UIs."}
            </p>
            {pipelines::render_all_pipelines()}
            <div class="toolchain-diagram-wrap">
                <img
                    src="images/mermaid-diagram.png"
                    alt="COR24 ecosystem architecture diagram"
                    class="toolchain-diagram-img"
                />
            </div>
            {sections::render_pcode_vm()}
            {sections::render_lisp_gc()}
            {sections::render_forth_dtc()}
            {sections::render_web_ui()}
            {sections::render_tc24r_constraints()}
            <section class="toolchain-section">
                <h2 class="toolchain-section-title">{"BASIC on COR24"}</h2>
                <p class="toolchain-section-desc">
                    {"The COR24 software stack, from a simple \"Hello, World\" in BASIC \
                     down through the layers to hardware."}
                </p>
                <div class="basic-stack-container">
                    <img
                        src="images/cargo-container-stack-with-cranes.jpg"
                        alt="COR24 software stack: Hello World, BASIC, Pascal, P-code VM, Assembler, Emulator, COR24 ISA, with C and Rust cranes building the stack"
                        class="basic-stack-img"
                    />
                </div>
            </section>
        </div>
    }
}
