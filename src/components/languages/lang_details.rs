use yew::prelude::*;

use crate::data::lang_descriptions;

#[function_component(LangOverview)]
pub fn lang_overview() -> Html {
    let summaries = lang_descriptions::summaries();

    html! {
        <div class="lang-overview">
            <h2>{"Language Overview"}</h2>
            <table class="lang-overview-table">
                <thead>
                    <tr>
                        <th>{"Language"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {summaries.iter().map(|s| {
                        html! {
                            <tr>
                                <td class="lang-overview-name">
                                    <a href={format!("#{}", s.section_id)} class="lang-anchor">
                                        {s.label}
                                    </a>
                                </td>
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
                html! {
                    <section class="lang-detail-section" id={d.section_id}>
                        <h3 class="lang-detail-title">
                            <a href={format!("#{}", d.section_id)} class="lang-anchor">{d.label}</a>
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
                        </div>
                    </section>
                }
            }).collect::<Html>()}
        </div>
    }
}
