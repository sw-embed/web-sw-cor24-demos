use yew::prelude::*;

use crate::data::demos::{DemoEntry, all_demos};

fn tag_class(tag: &str) -> &'static str {
    match tag {
        "IDE" => "tag-ide",
        "Compiler" => "tag-compiler",
        "Interpreter" => "tag-interpreter",
        "Debugger" => "tag-debugger",
        "VM" => "tag-vm",
        "Docs" => "tag-docs",
        _ => "tag-default",
    }
}

#[derive(Properties, PartialEq)]
pub struct DemoCardProps {
    pub entry: DemoEntry,
}

#[function_component(DemoCard)]
pub fn demo_card(props: &DemoCardProps) -> Html {
    let entry = &props.entry;
    let live_url = entry.live_url();
    let repo_url = entry.repo_url();
    let status_class = entry.status.css_class();

    html! {
        <div class={classes!("demo-card", "card")}>
            <div class="demo-card-header">
                <h3 class="card-title">{entry.name}</h3>
                <span class={classes!("badge", status_class)}>{entry.status.label()}</span>
            </div>
            <p class="card-desc">{entry.description}</p>
            <div class="demo-card-tags">
                {entry.tags.iter().map(|tag| {
                    html! { <span class={classes!("tag", tag_class(tag))}>{*tag}</span> }
                }).collect::<Html>()}
            </div>
            <div class="card-links">
                if entry.is_this_site {
                    <span class="card-link card-link-current">{"You are here"}</span>
                } else {
                    <a href={live_url} target="_blank" rel="noopener noreferrer" class="card-link">
                        {"Try Live"}
                    </a>
                }
                <a href={repo_url} target="_blank" rel="noopener noreferrer" class="card-link">
                    {"Source"}
                </a>
            </div>
        </div>
    }
}

#[function_component(DemoGrid)]
pub fn demo_grid() -> Html {
    let demos = all_demos();
    html! {
        <section class="demo-grid-section">
            <h2 class="section-heading">{"Live Browser Demos"}</h2>
            <div class="demo-grid">
                {demos.iter().map(|d| html! { <DemoCard entry={d.clone()} /> }).collect::<Html>()}
            </div>
        </section>
    }
}
