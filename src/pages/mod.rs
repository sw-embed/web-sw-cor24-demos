use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! { <crate::components::home::HomePage /> }
}

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! { <crate::components::about::AboutPage /> }
}

#[function_component(StatusPage)]
pub fn status_page() -> Html {
    html! { <crate::components::status::StatusPage /> }
}

#[function_component(IsaPage)]
pub fn isa_page() -> Html {
    html! { <crate::components::isa::IsaPage /> }
}

#[function_component(ToolchainPage)]
pub fn toolchain_page() -> Html {
    html! { <crate::components::toolchain::ToolchainPage /> }
}

#[function_component(LanguagesPage)]
pub fn languages_page() -> Html {
    html! { <crate::components::languages::LanguagesPage /> }
}

#[function_component(HardwarePage)]
pub fn hardware_page() -> Html {
    html! { <crate::components::hardware::HardwarePage /> }
}
