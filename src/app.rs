use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use crate::components::chrome::{Footer, Header};
use crate::pages;

#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Home,
    Isa,
    Demos,
    Toolchain,
    Ecosystem,
}

impl Route {
    pub fn from_hash(hash: &str) -> Self {
        match hash.trim_start_matches('#').trim_start_matches('/') {
            "isa" => Self::Isa,
            "demos" => Self::Demos,
            "toolchain" => Self::Toolchain,
            "ecosystem" => Self::Ecosystem,
            _ => Self::Home,
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "",
            Self::Isa => "isa",
            Self::Demos => "demos",
            Self::Toolchain => "toolchain",
            Self::Ecosystem => "ecosystem",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Isa => "ISA",
            Self::Demos => "Demos",
            Self::Toolchain => "Toolchain",
            Self::Ecosystem => "Ecosystem",
        }
    }

    pub fn all() -> &'static [Route] {
        &[
            Self::Home,
            Self::Isa,
            Self::Demos,
            Self::Toolchain,
            Self::Ecosystem,
        ]
    }
}

pub fn navigate_to(target: Route) {
    let path = target.path();
    let hash = if path.is_empty() {
        "#".to_string()
    } else {
        format!("#/{}", path)
    };
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_hash(&hash);
    }
}

fn route_page(route: &Route) -> Html {
    match route {
        Route::Home => html! { <pages::HomePage /> },
        Route::Isa => html! { <pages::IsaPage /> },
        Route::Demos => html! { <pages::DemosPage /> },
        Route::Toolchain => html! { <pages::ToolchainPage /> },
        Route::Ecosystem => html! { <pages::EcosystemPage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let route = use_state(|| {
        let hash = web_sys::window()
            .and_then(|w| w.location().hash().ok())
            .unwrap_or_default();
        Route::from_hash(&hash)
    });
    let menu_open = use_state(|| false);

    {
        let route = route.clone();
        use_effect_with((), move |_| {
            let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let hash = web_sys::window()
                    .and_then(|w| w.location().hash().ok())
                    .unwrap_or_default();
                route.set(Route::from_hash(&hash));
            }) as Box<dyn Fn(web_sys::Event)>);
            let _ = web_sys::window().and_then(|w| {
                w.add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref())
                    .ok()
            });
            move || drop(cb)
        });
    }

    let menu = menu_open.clone();
    let nav = Callback::from(move |target: Route| {
        menu.set(false);
        navigate_to(target);
    });
    let menu = menu_open.clone();
    let toggle = Callback::from(move |_: MouseEvent| menu.set(!*menu));
    let menu = menu_open.clone();
    let close = Callback::from(move |_: MouseEvent| menu.set(false));
    let current = (*route).clone();

    html! {
        <div id="app-root">
            <Header current_route={current.clone()} menu_open={*menu_open}
                navigate={nav} toggle_menu={toggle} close_menu={close} />
            <main class="main-content">{route_page(&current)}</main>
            <Footer />
        </div>
    }
}
