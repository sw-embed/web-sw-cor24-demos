use yew::prelude::*;

use super::tool_card::ToolCard;
use crate::data::tools::ToolGroup;

#[derive(Properties, PartialEq)]
pub struct CategorySectionProps {
    pub group: ToolGroup,
}

#[function_component(CategorySection)]
pub fn category_section(props: &CategorySectionProps) -> Html {
    let g = &props.group;

    html! {
        <section class="toolchain-section">
            <h2 class="toolchain-section-title">{g.label}</h2>
            <p class="toolchain-section-desc">{g.description}</p>
            <div class="tool-grid">
                {g.items.iter().map(|t| {
                    html! { <ToolCard info={(*t).clone()} /> }
                }).collect::<Html>()}
            </div>
        </section>
    }
}
