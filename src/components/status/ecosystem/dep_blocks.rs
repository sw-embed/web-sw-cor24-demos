use yew::prelude::*;

use super::eco_deps::EDGES;

pub(crate) fn render_web_demo_deps() -> Html {
    let edges: Vec<_> = EDGES
        .iter()
        .filter(|e| e.from.starts_with("web-"))
        .collect();
    dep_block("Web demos build on", &edges)
}

pub(crate) fn render_cross_compiler_deps() -> Html {
    let edges: Vec<_> = EDGES
        .iter()
        .filter(|e| {
            matches!(
                e.from,
                "sw-cor24-x-tinyc"
                    | "sw-cor24-x-assembler"
                    | "sw-cor24-rust"
                    | "sw-cor24-x-pc-aotc"
            )
        })
        .collect();
    dep_block("Cross-compilers depend on", &edges)
}

pub(crate) fn render_native_lang_deps() -> Html {
    let tc24r_edges: Vec<_> = EDGES
        .iter()
        .filter(|e| {
            matches!(
                e.from,
                "sw-cor24-macrolisp"
                    | "sw-cor24-apl"
                    | "sw-cor24-plsw"
                    | "sw-cor24-script"
                    | "sw-cor24-assembler"
                    | "sw-cor24-yocto-ed"
            )
        })
        .collect();
    let pascal_edges: Vec<_> = EDGES
        .iter()
        .filter(|e| matches!(e.from, "sw-cor24-basic"))
        .collect();
    let plsw_edges: Vec<_> = EDGES
        .iter()
        .filter(|e| matches!(e.from, "sw-cor24-snobol4" | "sw-cor24-fortran"))
        .collect();
    html! {
        <>
            {dep_block("Native tools compiled by tc24r", &tc24r_edges)}
            {dep_block("Native tools compiled by p24p (Pascal)", &pascal_edges)}
            {dep_block("Native tools compiled by PL/SW", &plsw_edges)}
        </>
    }
}

pub(crate) fn render_system_deps() -> Html {
    let edges: Vec<_> = EDGES
        .iter()
        .filter(|e| matches!(e.from, "sw-cor24-monitor"))
        .collect();
    dep_block("Monitor depends on", &edges)
}

pub(crate) fn render_pcode_deps() -> Html {
    let edges: Vec<_> = EDGES
        .iter()
        .filter(|e| {
            matches!(
                e.from,
                "sw-cor24-pcode" | "sw-cor24-pascal" | "sw-cor24-x-pc-aotc"
            ) && !e.to.starts_with("web-")
        })
        .collect();
    dep_block("P-code system interconnect", &edges)
}

fn dep_block(title: &str, edges: &[&super::eco_deps::DepEdge]) -> Html {
    let rows = edges
        .iter()
        .map(|e| {
            html! {
                <div class="dep-row">
                    {dep_node(e.from)}
                    {dep_arrow(e.label)}
                    {dep_node(e.to)}
                </div>
            }
        })
        .collect::<Vec<_>>();
    html! {
        <div class="dep-block">
            <h3 class="dep-block-title">{title}</h3>
            <div class="dep-list">{rows}</div>
        </div>
    }
}

fn dep_node(name: &str) -> Html {
    let cls = super::eco_deps::repo_group_class(name);
    html! {
        <span class={format!("dep-node {}", cls)}>
            <code>{name}</code>
        </span>
    }
}

fn dep_arrow(label: &str) -> Html {
    html! {
        <span class="dep-arrow">
            <span class="dep-arrow-line"></span>
            <span class="dep-arrow-label">{label}</span>
        </span>
    }
}
