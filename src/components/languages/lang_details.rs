use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::data::lang_descriptions;

fn scroll_to(section_id: &str) -> Callback<MouseEvent> {
    let id = section_id.to_string();
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if let Some(window) = web_sys::window()
            && let Some(doc) = window.document()
            && let Some(el) = doc.get_element_by_id(&id)
        {
            let el: &HtmlElement = el.dyn_ref().unwrap();
            el.scroll_into_view();
        }
    })
}

#[function_component(LangOverview)]
pub fn lang_overview() -> Html {
    let summaries = lang_descriptions::summaries();

    html! {
        <div class="lang-overview">
            <h2>{"Language Overview"}</h2>
            <table class="lang-overview-table">
                <thead>
                    <tr>
                        <th>{"COR24 Language"}</th>
                        <th>{"Inspired by"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {summaries.iter().map(|s| {
                        let onclick = scroll_to(s.section_id);
                        html! {
                            <tr>
                                <td class="lang-overview-name">
                                    <button class="lang-link-btn" onclick={onclick}>
                                        {s.label}
                                    </button>
                                </td>
                                <td class="lang-overview-inspired">{s.inspired_by}</td>
                                <td class="lang-overview-desc">{s.one_liner}</td>
                            </tr>
                        }
                    }).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}

#[function_component(LangDetails)]
pub fn lang_details() -> Html {
    let details = lang_descriptions::all_details();

    html! {
        <div class="lang-details">
            <h2>{"Language Details"}</h2>
            {details.iter().map(|d| {
                let onclick = scroll_to(d.section_id);
                html! {
                    <section class="lang-detail-section" id={d.section_id}>
                        <h3 class="lang-detail-title">
                            <button class="lang-link-btn" onclick={onclick}>
                                {d.label}
                            </button>
                            <span class="lang-detail-inspired-by">
                                {format!(" ({})", d.inspired_by)}
                            </span>
                        </h3>
                        <div class="lang-detail-body">
                            <p><strong>{"History: "}</strong>{d.history}</p>
                            <p><strong>{"Purpose: "}</strong>{d.purpose}</p>
                            <p><strong>{"Usage: "}</strong>{d.usage}</p>
                            <div class="lang-detail-pros-cons">
                                <div class="lang-detail-pros">
                                    <strong>{"Pros"}</strong>
                                    <ul>
                                        {d.pros.iter().map(|p| html! { <li>{*p}</li> }).collect::<Html>()}
                                    </ul>
                                </div>
                                <div class="lang-detail-cons">
                                    <strong>{"Cons"}</strong>
                                    <ul>
                                        {d.cons.iter().map(|c| html! { <li>{*c}</li> }).collect::<Html>()}
                                    </ul>
                                </div>
                            </div>
                            {render_glyph_table(d.glyph_table)}
                        </div>
                    </section>
                }
            }).collect::<Html>()}
        </div>
    }
}

fn render_glyph_table(table: Option<&[crate::data::lang_descriptions::GlyphRow]>) -> Html {
    let entries = match table {
        Some(t) => t,
        None => return html! {},
    };
    html! {
        <div class="lang-glyph-section">
            <h4>{"Latin \u{2192} APL Glyph \u{2192} Keyword Mapping"}</h4>
            <p class="lang-glyph-note">
                {"APL operators have different meanings depending on valence (number of arguments). \
                   0 args = niladic (read system state), 1 arg = monadic (prefix), 2 args = dyadic (infix)."}
            </p>
            <div class="lang-glyph-scroll">
                <table class="lang-glyph-table">
                    <thead>
                        <tr>
                            <th>{"Latin"}</th>
                            <th>{"Glyph"}</th>
                            <th>{"Niladic"}</th>
                            <th>{"Monadic"}</th>
                            <th>{"Dyadic"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {entries.iter().map(|row| {
                            html! {
                                <tr>
                                    <td><code>{row.latin}</code></td>
                                    <td class="lang-glyph-char">{row.glyph}</td>
                                    <td>{row.niladic}</td>
                                    <td>{row.monadic}</td>
                                    <td>{row.dyadic}</td>
                                </tr>
                            }
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
