use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <section class="page-section">
            <div class="hero">
                <h1 class="hero-title">{"COR24 Ecosystem"}</h1>
                <p class="hero-subtitle">
                    {"A complete 24-bit computing platform: ISA, toolchain, languages, and live browser demos."}
                </p>
            </div>
            <div class="placeholder">
                <p>{"Home page content coming in the next step."}</p>
            </div>
        </section>
    }
}

#[function_component(IsaPage)]
pub fn isa_page() -> Html {
    html! {
        <section class="page-section">
            <h1>{"COR24 ISA Documentation"}</h1>
            <div class="placeholder">
                <p>{"ISA documentation coming soon."}</p>
            </div>
        </section>
    }
}

#[function_component(DemosPage)]
pub fn demos_page() -> Html {
    html! {
        <section class="page-section">
            <h1>{"Live Web Demos"}</h1>
            <div class="placeholder">
                <p>{"Demos directory coming soon."}</p>
            </div>
        </section>
    }
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
