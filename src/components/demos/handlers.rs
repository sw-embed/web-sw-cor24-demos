use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, InputEvent};
use yew::prelude::*;

use super::filter_bar::{FilterState, LanguageFilter, StatusFilter};

pub fn on_search(filter: &yew::UseStateHandle<FilterState>) -> Callback<InputEvent> {
    let filter = filter.clone();
    Callback::from(move |e: InputEvent| {
        let mut f = (*filter).clone();
        f.search = input_value(&e);
        filter.set(f);
    })
}

pub fn on_language(filter: &yew::UseStateHandle<FilterState>) -> Callback<Event> {
    let filter = filter.clone();
    Callback::from(move |e: Event| {
        let mut f = (*filter).clone();
        let val = select_value(&e);
        f.language = if val == "all" {
            LanguageFilter::All
        } else {
            LanguageFilter::Specific(val)
        };
        filter.set(f);
    })
}

pub fn on_status(filter: &yew::UseStateHandle<FilterState>) -> Callback<Event> {
    let filter = filter.clone();
    Callback::from(move |e: Event| {
        let mut f = (*filter).clone();
        let val = select_value(&e);
        f.status = match val.as_str() {
            "active" => StatusFilter::Specific(crate::data::demos::DemoStatus::Active),
            "wip" => StatusFilter::Specific(crate::data::demos::DemoStatus::Wip),
            "testing" => StatusFilter::Specific(crate::data::demos::DemoStatus::Testing),
            "design" => StatusFilter::Specific(crate::data::demos::DemoStatus::Design),
            "longterm" => StatusFilter::Specific(crate::data::demos::DemoStatus::LongTerm),
            _ => StatusFilter::All,
        };
        filter.set(f);
    })
}

fn input_value(e: &InputEvent) -> String {
    e.target()
        .and_then(|t| {
            let el: &HtmlInputElement = t.dyn_ref()?;
            Some(el.value())
        })
        .unwrap_or_default()
}

fn select_value(e: &Event) -> String {
    e.target()
        .and_then(|t| {
            let el: &HtmlSelectElement = t.dyn_ref()?;
            Some(el.value())
        })
        .unwrap_or_default()
}
