use yew::prelude::*;

use crate::app::{Route, navigate_to};

#[derive(Clone, PartialEq)]
struct DemoInfo {
    name: &'static str,
    slug: &'static str,
    description: &'static str,
    status: &'static str,
    status_class: &'static str,
    tags: &'static [&'static str],
    is_this_site: bool,
}

static DEMOS: [DemoInfo; 9] = [
    DemoInfo {
        name: "Assembly IDE",
        slug: "web-sw-cor24-assembler",
        description: "Full COR24 assembly IDE with syntax highlighting, real-time assembly, and register view.",
        status: "Active",
        status_class: "badge-active",
        tags: &["IDE", "Assembler"],
        is_this_site: false,
    },
    DemoInfo {
        name: "P-code VM",
        slug: "web-sw-cor24-pcode",
        description: "Interactive P-code virtual machine debugger with instruction stepping and stack visualization.",
        status: "Active",
        status_class: "badge-active",
        tags: &["Debugger", "VM"],
        is_this_site: false,
    },
    DemoInfo {
        name: "Tiny C Compiler",
        slug: "web-sw-cor24-tinyc",
        description: "Tiny C cross-compiler for COR24. Compile C code to COR24 assembly in the browser.",
        status: "Active",
        status_class: "badge-active",
        tags: &["Compiler", "C"],
        is_this_site: false,
    },
    DemoInfo {
        name: "Macro Lisp",
        slug: "web-sw-cor24-macrolisp",
        description: "Tiny Macro Lisp REPL. Write and evaluate Lisp expressions that run on COR24 hardware.",
        status: "Active",
        status_class: "badge-active",
        tags: &["Interpreter", "Lisp"],
        is_this_site: false,
    },
    DemoInfo {
        name: "Pascal Demos",
        slug: "web-sw-cor24-pascal",
        description: "Pascal compiler demos. Compile Pascal source to P-code and COR24 assembly.",
        status: "Active",
        status_class: "badge-active",
        tags: &["Compiler", "Pascal"],
        is_this_site: false,
    },
    DemoInfo {
        name: "APL Environment",
        slug: "web-sw-cor24-apl",
        description: "APL interpreter with live array operations. Explore APL's concise array notation.",
        status: "Active",
        status_class: "badge-active",
        tags: &["Interpreter", "APL"],
        is_this_site: false,
    },
    DemoInfo {
        name: "Forth Debugger",
        slug: "web-sw-cor24-forth",
        description: "Forth language debugger with dictionary browsing, stack inspection, and word definitions.",
        status: "Active",
        status_class: "badge-active",
        tags: &["Debugger", "Forth"],
        is_this_site: false,
    },
    DemoInfo {
        name: "PL/SW IDE",
        slug: "web-sw-cor24-plsw",
        description: "PL/SW development environment for the PL/I-inspired PL/SW language running on COR24.",
        status: "Active",
        status_class: "badge-active",
        tags: &["IDE", "Compiler"],
        is_this_site: false,
    },
    DemoInfo {
        name: "Ecosystem Hub",
        slug: "web-sw-cor24-demos",
        description: "This landing page. Documentation hub and directory for the entire COR24 ecosystem.",
        status: "Active",
        status_class: "badge-active",
        tags: &["Docs"],
        is_this_site: true,
    },
];

fn tag_class(tag: &str) -> &'static str {
    match tag {
        "IDE" => "tag-ide",
        "Compiler" => "tag-compiler",
        "Interpreter" => "tag-interpreter",
        "Debugger" => "tag-debugger",
        "VM" => "tag-vm",
        "Docs" => "tag-docs",
        _ => "tag-default",
    }
}

#[derive(Properties, PartialEq)]
struct DemoCardProps {
    info: DemoInfo,
}

#[function_component(DemoCard)]
fn demo_card(props: &DemoCardProps) -> Html {
    let d = &props.info;
    let live_url = if d.is_this_site {
        "https://sw-embed.github.io/web-sw-cor24-demos/".to_string()
    } else {
        format!("https://sw-embed.github.io/{}/", d.slug)
    };
    let repo_url = format!("https://github.com/sw-embed/{}", d.slug);
    html! {
        <div class={classes!("demo-card", "card")}>
            <div class="demo-card-header">
                <h3 class="card-title">{d.name}</h3>
                <span class={classes!("badge", d.status_class)}>{d.status}</span>
            </div>
            <p class="card-desc">{d.description}</p>
            <div class="demo-card-tags">
                {d.tags.iter().map(|tag| {
                    html! { <span class={classes!("tag", tag_class(tag))}>{*tag}</span> }
                }).collect::<Html>()}
            </div>
            <div class="card-links">
                if d.is_this_site {
                    <span class="card-link card-link-current">{"You are here"}</span>
                } else {
                    <a href={live_url} target="_blank" rel="noopener noreferrer" class="card-link">
                        {"Try Live"}
                    </a>
                }
                <a href={repo_url} target="_blank" rel="noopener noreferrer" class="card-link">
                    {"Source"}
                </a>
            </div>
        </div>
    }
}

#[function_component(DemoGrid)]
fn demo_grid() -> Html {
    html! {
        <section class="demo-grid-section">
            <h2 class="section-heading">{"Live Browser Demos"}</h2>
            <div class="demo-grid">
                {DEMOS.iter().map(|d| html! { <DemoCard info={(*d).clone()} /> }).collect::<Html>()}
            </div>
        </section>
    }
}

#[function_component(HeroSection)]
fn hero_section() -> Html {
    html! {
        <div class="hero">
            <h1 class="hero-title">{"COR24 Ecosystem"}</h1>
            <p class="hero-tagline">{"24-bit RISC for FPGA embedded systems"}</p>
            <p class="hero-desc">
                {"A complete computing platform — from ISA specification and silicon synthesis \
                through cross-compilers, interpreters, and live browser demos. "}
                <a href="https://makerlisp.com" target="_blank" rel="noopener noreferrer">
                    {"makerlisp.com"}
                </a>
            </p>
            <div class="hero-actions">
                <a href="#/isa" class="btn btn-primary">{"ISA Docs"}</a>
                <a href="#/demos" class="btn btn-secondary">{"Live Demos"}</a>
            </div>
        </div>
    }
}

fn nav_callback(route: Route) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| navigate_to(route.clone()))
}

#[function_component(OverviewSection)]
fn overview_section() -> Html {
    html! {
        <section class="overview-section">
            <h2 class="section-title">{"What is COR24?"}</h2>
            <div class="overview-content">
                <p>
                    {"COR24 is a 24-bit RISC processor designed for FPGA-based embedded systems. "}
                    {"It provides a clean, orthogonal instruction set optimized for code density and simplicity, "}
                    {"while supporting a full software ecosystem of compilers, interpreters, and tools."}
                </p>
                <div class="overview-facts">
                    <div class="fact-item">
                        <span class="fact-value">{"8"}</span>
                        <span class="fact-label">{"General-purpose registers"}</span>
                    </div>
                    <div class="fact-item">
                        <span class="fact-value">{"1 MB"}</span>
                        <span class="fact-label">{"SRAM address space"}</span>
                    </div>
                    <div class="fact-item">
                        <span class="fact-value">{"8 KB"}</span>
                        <span class="fact-label">{"EBR stack memory"}</span>
                    </div>
                    <div class="fact-item">
                        <span class="fact-value">{"1-4 B"}</span>
                        <span class="fact-label">{"Variable-length instructions"}</span>
                    </div>
                </div>
                <div class="overview-links">
                    <button class="btn btn-secondary" onclick={nav_callback(Route::Isa)}>{"ISA Documentation"}</button>
                    <button class="btn btn-secondary" onclick={nav_callback(Route::Demos)}>{"Browse Demos"}</button>
                    <button class="btn btn-secondary" onclick={nav_callback(Route::Toolchain)}>{"Toolchain"}</button>
                    <button class="btn btn-secondary" onclick={nav_callback(Route::Ecosystem)}>{"Ecosystem Map"}</button>
                </div>
            </div>
        </section>
    }
}

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
