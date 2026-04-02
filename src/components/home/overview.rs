use yew::prelude::*;

use crate::app::Route;

fn navigate_to(route: Route) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        let path = route.path();
        let hash = if path.is_empty() {
            "#".to_string()
        } else {
            format!("#/{}", path)
        };
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_hash(&hash);
        }
    })
}

#[function_component(OverviewSection)]
pub fn overview_section() -> Html {
    let isa_nav = navigate_to(Route::Isa);
    let demos_nav = navigate_to(Route::Demos);
    let toolchain_nav = navigate_to(Route::Toolchain);
    let ecosystem_nav = navigate_to(Route::Ecosystem);

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
                        <span class="fact-value">{"8 KB"}</span>
                        <span class="fact-label">{"EBR stack memory"}</span>
                    </div>
                    <div class="fact-item">
                        <span class="fact-value">{"1-4 B"}</span>
                        <span class="fact-label">{"Variable-length instructions"}</span>
                    </div>
                </div>
                <div class="overview-links">
                    <button class="btn btn-secondary" onclick={isa_nav}>{"ISA Documentation"}</button>
                    <button class="btn btn-secondary" onclick={demos_nav}>{"Browse Demos"}</button>
                    <button class="btn btn-secondary" onclick={toolchain_nav}>{"Toolchain"}</button>
                    <button class="btn btn-secondary" onclick={ecosystem_nav}>{"Ecosystem Map"}</button>
                </div>
            </div>
        </section>
    }
}
