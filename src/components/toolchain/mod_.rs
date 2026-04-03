use yew::prelude::*;

use super::category::CategorySection;
use super::pipelines;
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
            <section class="toolchain-section">
                <h2 class="toolchain-section-title">{"Compilation Pipelines"}</h2>
                <p class="toolchain-section-desc">
                    {"How source code flows through the COR24 toolchain to produce executable binaries."}
                </p>
                <div class="pipeline-list">
                    {pipelines::render_all_pipelines()}
                </div>
            </section>
            <section class="toolchain-section">
                <h2 class="toolchain-section-title">{"BASIC on COR24"}</h2>
                <p class="toolchain-section-desc">
                    {"The COR24 software stack, from a simple \"Hello, World\" in BASIC \
                     down through the layers to hardware."}
                </p>
                <div class="basic-stack-container">
                    <img
                        src="images/cargo-container-stack.jpg"
                        alt="COR24 software stack: Hello World, BASIC, Pascal, P-code VM, Assembler, Emulator, COR24 ISA"
                        class="basic-stack-img"
                    />
                </div>
            </section>
        </div>
    }
}
