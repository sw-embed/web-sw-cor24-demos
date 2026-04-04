use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::shared::demo_card::DemoCard;
use crate::data::demos;
use crate::data::demos::DemoStatus;

#[derive(Clone, PartialEq)]
struct FilterState {
    search: String,
    language: LanguageFilter,
    status: StatusFilter,
}

#[derive(Clone, PartialEq, Debug)]
enum LanguageFilter {
    All,
    Specific(String),
}

#[derive(Clone, PartialEq, Debug)]
enum StatusFilter {
    All,
    Specific(DemoStatus),
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            search: String::new(),
            language: LanguageFilter::All,
            status: StatusFilter::All,
        }
    }
}

impl FilterState {
    fn matches(&self, entry: &demos::DemoEntry) -> bool {
        if !self.search.is_empty() {
            let q = self.search.to_lowercase();
            let name_match = entry.name.to_lowercase().contains(&q);
            let desc_match = entry.description.to_lowercase().contains(&q);
            let tag_match = entry.tags.iter().any(|t| t.to_lowercase().contains(&q));
            if !name_match && !desc_match && !tag_match {
                return false;
            }
        }
        if let LanguageFilter::Specific(lang) = &self.language
            && entry.group_id != lang.as_str()
        {
            return false;
        }
        if let StatusFilter::Specific(s) = &self.status
            && &entry.status != s
        {
            return false;
        }
        true
    }
}

fn filter_languages() -> Vec<(&'static str, String)> {
    demos::filter_languages()
}

fn filter_statuses() -> Vec<(&'static str, StatusFilter)> {
    vec![
        ("All", StatusFilter::All),
        ("Active", StatusFilter::Specific(DemoStatus::Active)),
        ("WIP", StatusFilter::Specific(DemoStatus::Wip)),
        ("Testing", StatusFilter::Specific(DemoStatus::Testing)),
        ("Design", StatusFilter::Specific(DemoStatus::Design)),
        ("Long-term", StatusFilter::Specific(DemoStatus::LongTerm)),
    ]
}

#[function_component(DemoGrid)]
pub fn demo_grid() -> Html {
    let filter = use_state(FilterState::default);
    let categories = demos::all_categories();
    let total = demos::all_demos().len();

    let count: usize = categories
        .iter()
        .flat_map(|c| c.items.iter())
        .filter(|d| filter.matches(d))
        .count();

    let search_cb = {
        let filter = filter.clone();
        Callback::from(move |e: web_sys::InputEvent| {
            let mut f = (*filter).clone();
            let val = e
                .target()
                .and_then(|t| {
                    let el: &web_sys::HtmlInputElement = t.dyn_ref()?;
                    Some(el.value())
                })
                .unwrap_or_default();
            f.search = val;
            filter.set(f);
        })
    };

    let language_cb = {
        let filter = filter.clone();
        Callback::from(move |e: web_sys::Event| {
            let mut f = (*filter).clone();
            let val = e
                .target()
                .and_then(|t| {
                    let el: &web_sys::HtmlSelectElement = t.dyn_ref()?;
                    Some(el.value())
                })
                .unwrap_or_default();
            f.language = if val == "all" {
                LanguageFilter::All
            } else {
                LanguageFilter::Specific(val)
            };
            filter.set(f);
        })
    };

    let status_cb = {
        let filter = filter.clone();
        Callback::from(move |e: web_sys::Event| {
            let mut f = (*filter).clone();
            let val = e
                .target()
                .and_then(|t| {
                    let el: &web_sys::HtmlSelectElement = t.dyn_ref()?;
                    Some(el.value())
                })
                .unwrap_or_default();
            f.status = match val.as_str() {
                "active" => StatusFilter::Specific(DemoStatus::Active),
                "wip" => StatusFilter::Specific(DemoStatus::Wip),
                "testing" => StatusFilter::Specific(DemoStatus::Testing),
                "design" => StatusFilter::Specific(DemoStatus::Design),
                "longterm" => StatusFilter::Specific(DemoStatus::LongTerm),
                _ => StatusFilter::All,
            };
            filter.set(f);
        })
    };

    let langs = filter_languages();
    let statuses = filter_statuses();

    let sections_html = categories
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
        .collect::<Html>();

    html! {
        <section class="demo-grid-section">
            <h2 class="section-heading">{"COR24 Software Stack"}</h2>
            <div class="demos-filter-bar">
                <input
                    type="text"
                    class="demos-search-input"
                    placeholder="Search demos..."
                    value={filter.search.clone()}
                    oninput={search_cb}
                />
                <select class="demos-select" onchange={language_cb}>
                    {langs.iter().map(|(label, val)| {
                        let selected = match &filter.language {
                            LanguageFilter::All => *val == "all",
                            LanguageFilter::Specific(c) => c == val,
                        };
                        html! { <option value={val.clone()} selected={selected}>{*label}</option> }
                    }).collect::<Html>()}
                </select>
                <select class="demos-select" onchange={status_cb}>
                    {statuses.iter().map(|(label, sf)| {
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
            {sections_html}
        </section>
    }
}

fn status_to_val(sf: &StatusFilter) -> &'static str {
    match sf {
        StatusFilter::All => "all",
        StatusFilter::Specific(DemoStatus::Active) => "active",
        StatusFilter::Specific(DemoStatus::Wip) => "wip",
        StatusFilter::Specific(DemoStatus::Testing) => "testing",
        StatusFilter::Specific(DemoStatus::Design) => "design",
        StatusFilter::Specific(DemoStatus::LongTerm) => "longterm",
    }
}
