use yew::prelude::*;

use crate::data::tools;

pub fn render_tc24r_constraints() -> Html {
    html! {
        <section class="toolchain-section">
            <h2 class="toolchain-section-title">{"tc24r C Subset Constraints"}</h2>
            <p class="toolchain-section-desc">
                {"The Tiny C cross-compiler (tc24r) targets a restricted subset of C due to the \
                 COR24 architecture's 24-bit word size and simplified memory model."}
            </p>
            <div class="tc24r-constraints">
                <ul>
                    <li>{"No structs or unions (flat scalar types only)"}</li>
                    <li>{"No malloc/free or heap allocation"}</li>
                    <li>{"No standard C library (string.h, stdio.h, etc.)"}</li>
                    <li>{"24-bit int (no 32-bit or 64-bit types)"}</li>
                    <li>{"Single translation unit per compilation"}</li>
                    <li>{"Function pointers supported, no function recursion limit"}</li>
                </ul>
            </div>
        </section>
    }
}

pub fn render_demo_links() -> Html {
    let all = tools::all_tools();
    let live: Vec<_> = all.iter().filter(|t| t.has_web_ui).collect();
    let planned: Vec<_> = all.iter().filter(|t| !t.has_web_ui).collect();

    html! {
        <section class="toolchain-section">
            <h2 class="toolchain-section-title">{"Live Web Demos"}</h2>
            <p class="toolchain-section-desc">
                {"Several tools have interactive web-based frontends that run entirely in the browser \
                 via WebAssembly. These let you try COR24 development without installing anything."}
            </p>
            <div class="toolchain-demo-list">
                <h4 class="toolchain-demo-heading">{"Available Now"}</h4>
                <ul class="toolchain-demo-ul">
                    {live.iter().map(|t| {
                        let url = t.demo_url().unwrap_or_default();
                        html! {
                            <li>
                                <a href={url} target="_blank" rel="noopener noreferrer" class="card-link">
                                    {t.name}
                                </a>
                            </li>
                        }
                    }).collect::<Html>()}
                </ul>
                <h4 class="toolchain-demo-heading">{"Planned"}</h4>
                <ul class="toolchain-demo-ul toolchain-demo-planned">
                    {planned.iter().map(|t| {
                        html! {
                            <li>{t.name}</li>
                        }
                    }).collect::<Html>()}
                </ul>
            </div>
        </section>
    }
}
