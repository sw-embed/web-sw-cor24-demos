pub mod demo_grid;
pub mod hero;
pub mod overview;

use yew::prelude::*;

use demo_grid::DemoGrid;
use hero::HeroSection;
use overview::OverviewSection;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="page-section">
            <HeroSection />
            <DemoGrid />
            <OverviewSection />
        </div>
    }
}
