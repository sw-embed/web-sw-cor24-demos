use yew::prelude::*;

use super::dep_blocks;
use super::eco_deps::DepGroup;

pub fn render_dependency_diagram() -> Html {
    html! {
        <section class="eco-section">
            <h2 class="section-heading">{"Ecosystem Dependencies"}</h2>
            <p class="eco-intro">
                {"How repositories depend on each other. Arrows point from dependent to dependency."}
            </p>
            {render_legend()}
            <div class="dep-grid">
                {dep_blocks::render_web_demo_deps()}
                {dep_blocks::render_cross_compiler_deps()}
                {dep_blocks::render_native_lang_deps()}
                {dep_blocks::render_system_deps()}
                {dep_blocks::render_pcode_deps()}
            </div>
        </section>
    }
}

fn render_legend() -> Html {
    let groups = [
        (DepGroup::WebDemos, "Web Demos"),
        (DepGroup::Foundation, "Foundation"),
        (DepGroup::CrossCompiler, "Cross-compilers"),
        (DepGroup::PCode, "P-code"),
        (DepGroup::NativeLang, "Native Languages"),
        (DepGroup::System, "System"),
    ];
    let items = groups
        .iter()
        .map(|(g, label)| {
            html! {
                <span class="dep-legend-item">
                    <span class={format!("dep-legend-dot {}", g.css_class())}></span>
                    {label}
                </span>
            }
        })
        .collect::<Vec<_>>();
    html! { <div class="dep-legend">{items}</div> }
}
