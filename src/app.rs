use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use crate::components::chrome::{Footer, Header};
use crate::pages;

#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Home,
    About,
    Status,
    Isa,
    Toolchain,
    Languages,
    Hardware,
}

impl Route {
    pub fn from_hash(hash: &str) -> Self {
        match hash.trim_start_matches('#').trim_start_matches('/') {
            "about" => Self::About,
            "status" => Self::Status,
            "isa" => Self::Isa,
            "toolchain" => Self::Toolchain,
            "languages" => Self::Languages,
            "hardware" => Self::Hardware,
            _ => Self::Home,
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "",
            Self::About => "about",
            Self::Status => "status",
            Self::Isa => "isa",
            Self::Toolchain => "toolchain",
            Self::Languages => "languages",
            Self::Hardware => "hardware",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::About => "About",
            Self::Status => "Status",
            Self::Isa => "ISA",
            Self::Toolchain => "Toolchain",
            Self::Languages => "Languages",
            Self::Hardware => "Hardware",
        }
    }

    pub fn nav_items() -> &'static [Route] {
        &[
            Self::Status,
            Self::Isa,
            Self::Toolchain,
            Self::Languages,
            Self::Hardware,
            Self::About,
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
        Route::About => html! { <pages::AboutPage /> },
        Route::Status => html! { <pages::StatusPage /> },
        Route::Isa => html! { <pages::IsaPage /> },
        Route::Toolchain => html! { <pages::ToolchainPage /> },
        Route::Languages => html! { <pages::LanguagesPage /> },
        Route::Hardware => html! { <pages::HardwarePage /> },
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
