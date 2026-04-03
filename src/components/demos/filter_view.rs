use yew::prelude::*;

use super::filter_bar::{
    filter_categories, filter_statuses, CategoryFilter, FilterState, StatusFilter,
};

pub fn render_filter_bar(
    filter: &FilterState,
    on_search: Callback<web_sys::InputEvent>,
    on_category: Callback<web_sys::Event>,
    on_status: Callback<web_sys::Event>,
) -> Html {
    html! {
        <div class="demos-filter-bar">
            <input
                type="text"
                class="demos-search-input"
                placeholder="Search demos..."
                value={filter.search.clone()}
                oninput={on_search}
            />
            <select class="demos-select" onchange={on_category}>
                {filter_categories().iter().map(|(label, val)| {
                    let selected = match &filter.category {
                        CategoryFilter::All => *val == "all",
                        CategoryFilter::Specific(c) => c == val,
                    };
                    html! { <option value={val.clone()} selected={selected}>{*label}</option> }
                }).collect::<Html>()}
            </select>
            <select class="demos-select" onchange={on_status}>
                {filter_statuses().iter().map(|(label, sf)| {
                    let val = status_to_val(sf);
                    let selected = match (&filter.status, sf) {
                        (StatusFilter::All, StatusFilter::All) => true,
                        (StatusFilter::Specific(a), StatusFilter::Specific(b)) => a == b,
                        _ => false,
                    };
                    html! { <option value={val} selected={selected}>{*label}</option> }
                }).collect::<Html>()}
            </select>
        </div>
    }
}

pub fn render_count(count: usize, total: usize) -> Html {
    html! {
        <div class="demos-count">
            <span class="demos-count-num">{count}</span>
            <span class="demos-count-of">{" of "}</span>
            <span class="demos-count-total">{total}</span>
            <span class="demos-count-label">{" demos"}</span>
        </div>
    }
}

fn status_to_val(sf: &StatusFilter) -> &'static str {
    match sf {
        StatusFilter::All => "all",
        StatusFilter::Specific(crate::data::demos::DemoStatus::Active) => "active",
        StatusFilter::Specific(crate::data::demos::DemoStatus::Wip) => "wip",
        StatusFilter::Specific(crate::data::demos::DemoStatus::Testing) => "testing",
        StatusFilter::Specific(crate::data::demos::DemoStatus::Design) => "design",
        StatusFilter::Specific(crate::data::demos::DemoStatus::LongTerm) => "longterm",
    }
}
