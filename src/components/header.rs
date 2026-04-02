use yew::prelude::*;

use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub current_route: Route,
    pub menu_open: bool,
    pub navigate: Callback<Route>,
    pub toggle_menu: Callback<MouseEvent>,
    pub close_menu: Callback<MouseEvent>,
}

fn nav_items(props: &HeaderProps) -> Html {
    Route::all()
        .iter()
        .map(|r| {
            let is_active = *r == props.current_route;
            let target = r.clone();
            let cb = props.navigate.reform(move |_: MouseEvent| target.clone());
            html! {
                <button
                    class={classes!("nav-link", is_active.then_some("nav-active"))}
                    onclick={cb}
                >
                    {r.label()}
                </button>
            }
        })
        .collect()
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header class="site-header">
            <div class="header-inner">
                <a href="#/" class="header-brand">
                    <span class="brand-icon">{"\u{2699}"}</span>
                    <span class="brand-text">{"COR24 Ecosystem"}</span>
                </a>
                <button class="hamburger" onclick={&props.toggle_menu} aria-label="Toggle menu">
                    <span class={classes!("hamburger-line", props.menu_open.then_some("hamburger-open"))}></span>
                    <span class={classes!("hamburger-line", props.menu_open.then_some("hamburger-open"))}></span>
                    <span class={classes!("hamburger-line", props.menu_open.then_some("hamburger-open"))}></span>
                </button>
                <nav class={classes!("nav-links", props.menu_open.then_some("nav-open"))}>
                    {nav_items(props)}
                </nav>
            </div>
        </header>
    }
}
