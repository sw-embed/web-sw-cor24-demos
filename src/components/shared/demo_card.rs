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
    let show_live = d.has_live_demo && !d.is_this_site;

    let badge_html = if !d.badge_image.is_empty() {
        html! {
            <img
                src={format!("images/{}?ts=1775170317225", d.badge_image)}
                alt={d.name}
                class="demo-card-badge"
            />
        }
    } else {
        html! {}
    };

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
            <div class="card-footer">
                <div class="card-links">
                    if d.is_this_site {
                        <span class="card-link card-link-current">{"You are here"}</span>
                    } else if show_live {
                        <a href={d.live_url()} target="_blank" rel="noopener noreferrer" class="card-link">
                            {"Try Live"}
                        </a>
                    }
                    {
                        if let Some(url) = d.secondary_live_url {
                            html! {
                                <a href={url} target="_blank" rel="noopener noreferrer" class="card-link card-link-secondary">
                                    {d.secondary_live_label}
                                </a>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <a href={d.repo_url()} target="_blank" rel="noopener noreferrer" class="card-link">
                        {d.source_label}
                    </a>
                </div>
                {badge_html}
            </div>
        </div>
    }
}
