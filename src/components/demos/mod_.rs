use yew::prelude::*;

use super::filter_bar::FilterState;
use super::filter_view;
use super::handlers;
use crate::components::shared::demo_card::DemoCard;
use crate::data::demos;

#[function_component(DemosPage)]
pub fn demos_page() -> Html {
    let filter = use_state(FilterState::default);
    let all = demos::all_demos();
    let total = all.len();

    let matched: Vec<_> = all.iter().filter(|d| filter.matches(d)).collect();
    let count = matched.len();

    let search_cb = handlers::on_search(&filter);
    let category_cb = handlers::on_category(&filter);
    let status_cb = handlers::on_status(&filter);

    let grid_html = if matched.is_empty() {
        let on_clear = {
            let filter = filter.clone();
            Callback::from(move |_: MouseEvent| filter.set(FilterState::default()))
        };
        html! {
            <div class="demos-empty">
                <p class="demos-empty-text">{"No demos match your filter."}</p>
                <button class="demos-clear-btn" onclick={on_clear}>{"Clear filters"}</button>
            </div>
        }
    } else {
        html! {
            <div class="demo-grid">
                {matched.iter().map(|d| {
                    html! { <DemoCard info={(*d).clone()} /> }
                }).collect::<Html>()}
            </div>
        }
    };

    html! {
        <div class="page-section">
            <h1>{"Live Web Demos"}</h1>
            <p class="demos-intro">
                {"All COR24 web demos and tools. Filter by name, category, or status."}
            </p>
            {filter_view::render_filter_bar(&filter, search_cb, category_cb, status_cb)}
            {filter_view::render_count(count, total)}
            {grid_html}
        </div>
    }
}
