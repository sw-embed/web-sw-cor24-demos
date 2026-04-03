use yew::prelude::*;

use crate::app::{navigate_to, Route};

fn nav_callback(route: Route) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| navigate_to(route.clone()))
}

#[function_component(OverviewSection)]
pub fn overview_section() -> Html {
    html! {
        <section class="overview-section">
            <h2 class="section-title">{"What is COR24?"}</h2>
            <div class="overview-content">
                <p>
                    {"COR24 is a 24-bit RISC processor designed for FPGA-based embedded systems. "}
                    {"It provides a clean, orthogonal instruction set optimized for code density and simplicity, "}
                    {"while supporting a full software ecosystem of compilers, interpreters, and tools."}
                </p>
                <div class="overview-facts">
                    <div class="fact-item">
                        <span class="fact-value">{"8"}</span>
                        <span class="fact-label">{"General-purpose registers"}</span>
                    </div>
                    <div class="fact-item">
                        <span class="fact-value">{"1 MB"}</span>
                        <span class="fact-label">{"SRAM address space"}</span>
                    </div>
                    <div class="fact-item">
                        <span class="fact-value">{"3 KB"}</span>
                        <span class="fact-label">{"EBR stack (3K/8K emulator)"}</span>
                    </div>
                    <div class="fact-item">
                        <span class="fact-value">{"1-4 B"}</span>
                        <span class="fact-label">{"Variable-length instructions"}</span>
                    </div>
                </div>
                <div class="overview-links">
                    <button class="btn btn-secondary" onclick={nav_callback(Route::Isa)}>{"ISA Documentation"}</button>
                    <button class="btn btn-secondary" onclick={nav_callback(Route::Demos)}>{"Browse Demos"}</button>
                    <button class="btn btn-secondary" onclick={nav_callback(Route::Toolchain)}>{"Toolchain"}</button>
                </div>
            </div>
        </section>
    }
}
