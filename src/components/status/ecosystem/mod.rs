use yew::prelude::*;

pub mod dep_blocks;
pub mod eco_about;
pub mod eco_deps;
pub mod eco_diagram;
pub mod eco_info;

pub fn render_ecosystem() -> Html {
    html! {
        <>
            {eco_diagram::render_dependency_diagram()}
            {eco_info::render_getting_started()}
            {eco_about::render_about()}
        </>
    }
}
