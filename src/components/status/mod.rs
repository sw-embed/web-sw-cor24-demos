use yew::prelude::*;

use crate::data::status::{all_projects, r#generated_status, github_issues_url, github_repo_url};

fn status_badge(level: &crate::data::status::StatusLevel, label: &str) -> Html {
    html! {
        <span class={format!("status-badge {}", level.class())}>
            {label}
        </span>
    }
}

fn saga_cell(p: &crate::data::status::ProjectRow) -> Html {
    let found = r#generated_status()
        .iter()
        .find(|(r, _, _, _)| *r == p.repo);
    let (has_plan, has_steps) = match found {
        Some((_, _, plan, steps)) => (*plan, *steps),
        None => (false, false),
    };
    if has_plan || has_steps {
        let plan = if has_plan {
            html! {
                <a href={format!("https://github.com/sw-embed/{}/blob/main/.agentrail/plan.md", p.repo)} target="_blank" rel="noopener noreferrer" class="status-link">
                    {"plan"}
                </a>
            }
        } else {
            html! { <span class="status-dash">{"plan"}</span> }
        };
        let steps = if has_steps {
            html! {
                <a href={format!("https://github.com/sw-embed/{}/tree/main/.agentrail/steps", p.repo)} target="_blank" rel="noopener noreferrer" class="status-link">
                    {"steps"}
                </a>
            }
        } else {
            html! { <span class="status-dash">{"steps"}</span> }
        };
        html! {
            <td class="status-cell">
                {plan}
                {" / "}
                {steps}
            </td>
        }
    } else {
        html! { <td class="status-cell"><span class="status-dash">{"n/a"}</span></td> }
    }
}

fn project_row(p: &crate::data::status::ProjectRow) -> Html {
    let repo_url = github_repo_url(p.repo);
    let issues_url = github_issues_url(p.repo);
    let found = r#generated_status()
        .iter()
        .find(|(r, _, _, _)| *r == p.repo);
    let issue_count = found.map(|(_, c, _, _)| *c).unwrap_or(0);
    html! {
        <tr class="status-row">
            <td class="status-repo">
                <a href={repo_url} target="_blank" rel="noopener noreferrer">
                    <code>{p.repo}</code>
                </a>
            </td>
            <td class="status-desc">{p.description}</td>
            <td class="status-cell">{status_badge(&p.repo_status.level, p.repo_status.label)}</td>
            <td class="status-cell">{status_badge(&p.has_web_ui.level, p.has_web_ui.label)}</td>
            {saga_cell(p)}
            <td class="status-cell status-issues">
                <a href={issues_url} target="_blank" rel="noopener noreferrer" class="status-issues-link">
                    {issue_count}
                </a>
            </td>
        </tr>
    }
}

fn status_table() -> Html {
    let projects = all_projects();
    let mut current_group = "";
    let mut rows = Vec::new();
    for p in projects.iter() {
        if p.group != current_group {
            current_group = p.group;
            rows.push(html! {
                <tr class="status-group-row">
                    <td colspan="6" class="status-group-cell">{current_group}</td>
                </tr>
            });
        }
        rows.push(project_row(p));
    }
    html! {
        <div class="isa-table-wrap">
            <table class="data-table status-table">
                <thead>
                    <tr>
                        <th>{"Repo"}</th>
                        <th>{"Description"}</th>
                        <th class="status-col">{"Repo"}</th>
                        <th class="status-col">{"Web UI"}</th>
                        <th class="status-col">{"AgentRail"}</th>
                        <th class="status-col">{"Issues"}</th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>
    }
}

fn legend() -> Html {
    html! {
        <div class="status-legend">
            <span class="status-badge status-green">{"Try it"}</span>
            <span class="status-badge status-yellow">{"In dev"}</span>
            <span class="status-badge status-orange">{"In plan / Planned"}</span>
            <span class="status-badge status-red">{"Future"}</span>
            <span class="status-badge status-neutral">{"n/a"}</span>
        </div>
    }
}

#[function_component(StatusPage)]
pub fn status_page() -> Html {
    html! {
        <div class="status-page page-section">
            <h1 class="about-title">{"Project Status"}</h1>
            <p class="about-intro">
                {"Status of all repositories in the sw-embed organization. \
                Run "}
                <code>{"cargo run -p gen-status"}</code>
                {" to refresh issue counts and agentrail status."}
            </p>
            {legend()}
            {status_table()}

            <section class="gaps-section">
                <h2 class="section-heading">{"Gaps"}</h2>
                <div class="isa-table-wrap">
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th class="gaps-priority-col">{"Priority"}</th>
                                <th>{"Gap"}</th>
                                <th>{"Details"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="gaps-priority-cell">{status_badge(&crate::data::status::StatusLevel::Orange, "High")}</td>
                                <td>{"Software floating-point library"}</td>
                                <td>{"No hardware FPU. A software FP library is needed to support APL numeric features, the FORTRAN compiler, and scientific computing workloads."}</td>
                            </tr>
                            <tr>
                                <td class="gaps-priority-cell">{status_badge(&crate::data::status::StatusLevel::Yellow, "Medium")}</td>
                                <td>{"Missing web UI demos"}</td>
                                <td>{"Many native-language and system components lack browser-based demos (BASIC, Fortran, SWS, Debugger, Monitor, etc.)."}</td>
                            </tr>
                            <tr>
                                <td class="gaps-priority-cell">{status_badge(&crate::data::status::StatusLevel::Red, "Wishlist")}</td>
                                <td>{"Native COR24 C compiler"}</td>
                                <td>{"tc24r runs on the host. A native C compiler running on COR24 itself would enable self-hosted C development (far future)."}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>
        </div>
    }
}
