use yew::prelude::*;

use crate::components::shared::demo_card::DemoCard;
use crate::data::demos;

#[function_component(DemoGrid)]
pub fn demo_grid() -> Html {
    let categories = demos::all_categories();
    html! {
        <section class="demo-grid-section">
            <h2 class="section-heading">{"COR24 Software Stack"}</h2>
            <div class="category-list">
                {categories.iter().map(|cat| {
                    let label = cat.label;
                    let items = cat.items;
                    html! {
                        <div class="category-section">
                            <h3 class="category-heading">{label}</h3>
                            <div class="demo-grid">
                                {items.iter().map(|d| {
                                    html! { <DemoCard info={(*d).clone()} /> }
                                }).collect::<Html>()}
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </section>
    }
}
