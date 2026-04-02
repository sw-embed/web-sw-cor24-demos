use yew::prelude::*;

use crate::data::demos::{self, DemoEntry};

#[derive(Properties, PartialEq)]
pub struct DemoCardProps {
    pub info: DemoEntry,
}

#[function_component(DemoCard)]
pub fn demo_card(props: &DemoCardProps) -> Html {
    let d = &props.info;
    let status_class = demos::status_badge_class(&d.status);
    let status_label = demos::status_label(&d.status);

    html! {
        <div class={classes!("demo-card", "card")}>
            <div class="demo-card-header">
                <h3 class="card-title">{d.name}</h3>
                <span class={classes!("badge", status_class)}>{status_label}</span>
            </div>
            <p class="card-desc">{d.description}</p>
            <div class="demo-card-tags">
                {d.tags.iter().map(|tag| {
                    html! { <span class={classes!("tag", demos::tag_class(tag))}>{*tag}</span> }
                }).collect::<Html>()}
            </div>
            <div class="card-links">
                if d.is_this_site {
                    <span class="card-link card-link-current">{"You are here"}</span>
                } else {
                    <a href={d.live_url()} target="_blank" rel="noopener noreferrer" class="card-link">
                        {"Try Live"}
                    </a>
                }
                <a href={d.repo_url()} target="_blank" rel="noopener noreferrer" class="card-link">
                    {"Source"}
                </a>
            </div>
        </div>
    }
}

#[function_component(DemoGrid)]
pub fn demo_grid() -> Html {
    let entries = demos::all_demos();
    html! {
        <section class="demo-grid-section">
            <h2 class="section-heading">{"Live Browser Demos"}</h2>
            <div class="demo-grid">
                {entries.iter().map(|d| html! { <DemoCard info={(*d).clone()} /> }).collect::<Html>()}
            </div>
        </section>
    }
}
