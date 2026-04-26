use yew::prelude::*;

#[function_component(OverviewSection)]
pub fn overview_section() -> Html {
    html! {
        <section class="overview-section">
            <div class="overview-facts">
                <div class="fact-item">
                    <span class="fact-value">{"3"}</span>
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
        </section>
    }
}
