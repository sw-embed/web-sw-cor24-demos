use wasm_bindgen::JsCast;
use yew::prelude::*;

use super::filter_bar::{
    filter_categories, filter_statuses, CategoryFilter, FilterState, StatusFilter,
};
use crate::components::shared::demo_card::DemoCard;
use crate::data::demos;

#[function_component(DemosPage)]
pub fn demos_page() -> Html {
    let filter = use_state(FilterState::default);
    let all = demos::all_demos();
    let total = all.len();

    let matched: Vec<_> = all.iter().filter(|d| filter.matches(d)).collect();
    let count = matched.len();

    let on_search = {
        let filter = filter.clone();
        Callback::from(move |e: InputEvent| {
            let mut f = (*filter).clone();
            f.search = input_value(&e);
            filter.set(f);
        })
    };

    let on_category = {
        let filter = filter.clone();
        Callback::from(move |e: Event| {
            let mut f = (*filter).clone();
            let val = select_value(&e);
            f.category = if val == "all" {
                CategoryFilter::All
            } else {
                CategoryFilter::Specific(val)
            };
            filter.set(f);
        })
    };

    let on_status = {
        let filter = filter.clone();
        Callback::from(move |e: Event| {
            let mut f = (*filter).clone();
            let val = select_value(&e);
            f.status = match val.as_str() {
                "active" => StatusFilter::Specific(demos::DemoStatus::Active),
                "wip" => StatusFilter::Specific(demos::DemoStatus::Wip),
                "testing" => StatusFilter::Specific(demos::DemoStatus::Testing),
                "design" => StatusFilter::Specific(demos::DemoStatus::Design),
                "longterm" => StatusFilter::Specific(demos::DemoStatus::LongTerm),
                _ => StatusFilter::All,
            };
            filter.set(f);
        })
    };

    let on_clear = {
        let filter = filter.clone();
        Callback::from(move |_: MouseEvent| {
            filter.set(FilterState::default());
        })
    };

    let grid_html = if matched.is_empty() {
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

            <div class="demos-count">
                <span class="demos-count-num">{count}</span>
                <span class="demos-count-of">{" of "}</span>
                <span class="demos-count-total">{total}</span>
                <span class="demos-count-label">{" demos"}</span>
            </div>

            {grid_html}
        </div>
    }
}

fn input_value(e: &InputEvent) -> String {
    e.target()
        .and_then(|t| {
            let el: &web_sys::HtmlInputElement = t.dyn_ref()?;
            Some(el.value())
        })
        .unwrap_or_default()
}

fn select_value(e: &Event) -> String {
    e.target()
        .and_then(|t| {
            let el: &web_sys::HtmlSelectElement = t.dyn_ref()?;
            Some(el.value())
        })
        .unwrap_or_default()
}

fn status_to_val(sf: &StatusFilter) -> &'static str {
    match sf {
        StatusFilter::All => "all",
        StatusFilter::Specific(demos::DemoStatus::Active) => "active",
        StatusFilter::Specific(demos::DemoStatus::Wip) => "wip",
        StatusFilter::Specific(demos::DemoStatus::Testing) => "testing",
        StatusFilter::Specific(demos::DemoStatus::Design) => "design",
        StatusFilter::Specific(demos::DemoStatus::LongTerm) => "longterm",
    }
}
