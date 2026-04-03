use yew::prelude::*;

use super::filter_bar::FilterState;
use super::filter_view;
use super::handlers;
use crate::components::shared::demo_card::DemoCard;
use crate::data::demos;

#[function_component(DemosPage)]
pub fn demos_page() -> Html {
    let filter = use_state(FilterState::default);
    let categories = demos::all_categories();
    let total = demos::all_demos().len();

    let count: usize = categories
        .iter()
        .flat_map(|c| c.items.iter())
        .filter(|d| filter.matches(d))
        .count();

    let search_cb = handlers::on_search(&filter);
    let language_cb = handlers::on_language(&filter);
    let status_cb = handlers::on_status(&filter);

    let sections_html = render_sections(categories, &filter);

    html! {
        <div class="page-section">
            <h1>{"Live Web Demos"}</h1>
            <p class="demos-intro">
                {"All COR24 web demos and tools. Filter by name, language, or status."}
            </p>
            {filter_view::render_filter_bar(&filter, search_cb, language_cb, status_cb)}
            {filter_view::render_count(count, total)}
            {sections_html}
        </div>
    }
}

fn render_sections(categories: &'static [demos::Category], filter: &FilterState) -> Html {
    categories
        .iter()
        .map(|cat| {
            let matching: Vec<_> = cat.items.iter().filter(|d| filter.matches(d)).collect();
            if matching.is_empty() {
                return html! {};
            }
            html! {
                <div class="demo-category">
                    <h2 class="demo-category-title">{cat.label}</h2>
                    <div class="demo-grid">
                        {matching.iter().map(|d| {
                            html! { <DemoCard info={(*d).clone()} /> }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        })
        .collect::<Html>()
}
