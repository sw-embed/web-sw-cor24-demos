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

#[function_component(DemosPage)]
pub fn demos_page() -> Html {
    html! { <crate::components::demos::DemosPage /> }
}

#[function_component(ToolchainPage)]
pub fn toolchain_page() -> Html {
    html! {
        <section class="page-section">
            <h1>{"Toolchain Documentation"}</h1>
            <div class="placeholder">
                <p>{"Toolchain docs coming soon."}</p>
            </div>
        </section>
    }
}

#[function_component(EcosystemPage)]
pub fn ecosystem_page() -> Html {
    html! {
        <section class="page-section">
            <h1>{"Ecosystem Overview"}</h1>
            <div class="placeholder">
                <p>{"Ecosystem map coming soon."}</p>
            </div>
        </section>
    }
}

#[function_component(HardwarePage)]
pub fn hardware_page() -> Html {
    html! { <crate::components::hardware::HardwarePage /> }
}
