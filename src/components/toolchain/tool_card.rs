use yew::prelude::*;

use crate::data::tools::ToolEntry;

#[derive(Properties, PartialEq)]
pub struct ToolCardProps {
    pub info: ToolEntry,
}

#[function_component(ToolCard)]
pub fn tool_card(props: &ToolCardProps) -> Html {
    let t = &props.info;
    let lang = &t.language;
    let target = &t.target;
    let demo_link = t.demo_url();

    html! {
        <div class={classes!("tool-card", "card")}>
            <div class="tool-card-header">
                <h3 class="card-title">{t.name}</h3>
                <div class="tool-card-badges">
                    <span class={classes!("badge", lang.css_class())}>{lang.label()}</span>
                    <span class={classes!("badge", target.css_class())}>{target.label()}</span>
                </div>
            </div>
            <p class="card-desc">{t.description}</p>
            <div class="card-footer">
                <div class="card-links">
                    <a
                        href={t.repo_url()}
                        target="_blank"
                        rel="noopener noreferrer"
                        class="card-link"
                    >
                        {"GitHub"}
                    </a>
                    {
                        if let Some(url) = demo_link {
                            html! {
                                <a
                                    href={url}
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="card-link"
                                >
                                    {"Live Demo"}
                                </a>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>
        </div>
    }
}
