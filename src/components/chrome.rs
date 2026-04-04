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
    Route::nav_items()
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
                    <img src="images/sw-lab-logo.jpg?ts=1775170317225" alt="Software Wrighter Lab" class="brand-logo" />
                    <span class="brand-text">{"Software Wrighter Lab"}</span>
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

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="site-footer">
            <div class="footer-inner">
                <span class="footer-item">{"\u{00a9} 2026 Michael A Wright"}</span>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <span class="footer-item">{"MIT License"}</span>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://makerlisp.com" target="_blank" class="footer-item footer-link">
                    {"makerlisp.com"}
                </a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed" target="_blank" class="footer-item footer-link">
                    {"GitHub"}
                </a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <span class="footer-item footer-dim">{env!("BUILD_SHA")}</span>
            </div>
        </footer>
    }
}
