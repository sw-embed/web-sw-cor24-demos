use yew::prelude::*;

use super::category::CategorySection;
use crate::data::tools;

#[function_component(ToolchainPage)]
pub fn toolchain_page() -> Html {
    let groups = tools::all_groups();

    html! {
        <div class="page-section">
            <h1>{"Toolchain Documentation"}</h1>
            <p class="toolchain-intro">
                {"Every tool in the COR24 ecosystem, organized by category. \
                  From the core assembler and emulator to cross-compilers, native languages, \
                  and system software."}
            </p>
            {groups.iter().map(|g| {
                html! { <CategorySection group={(*g).clone()} /> }
            }).collect::<Html>()}
        </div>
    }
}
