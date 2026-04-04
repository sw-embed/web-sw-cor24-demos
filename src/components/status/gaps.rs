use yew::prelude::*;

use crate::data::status::StatusLevel;

pub fn render_gaps() -> Html {
    html! {
        <section class="gaps-section">
            <h2 class="section-heading">{"Gaps"}</h2>
            <div class="gaps-grid">
                <div class="gaps-grid-header">
                    <span class="gaps-grid-priority">{"Priority"}</span>
                    <span class="gaps-grid-gap">{"Gap"}</span>
                    <span class="gaps-grid-details">{"Details"}</span>
                </div>
                {gap_row(StatusLevel::Orange, "High", "Software floating-point library",
                    "No hardware FPU. A software FP library is needed to support APL numeric features, the FORTRAN compiler, and scientific computing workloads.")}
                {gap_row(StatusLevel::Yellow, "Medium", "Missing web UI demos",
                    "Many native-language and system components lack browser-based demos (BASIC, Fortran, SWS, Debugger, Monitor, etc.).")}
                {gap_row(StatusLevel::Red, "Wishlist", "Native COR24 C compiler",
                    "tc24r runs on the host. A native C compiler running on COR24 itself would enable self-hosted C development (far future).")}
                {gap_row(StatusLevel::Yellow, "Medium", "APL operations backlog",
                    "Many APL operations in the backlog. Some are in-progress, some are gated by floating-point support.")}
                {gap_row(StatusLevel::Yellow, "Medium", "C cross-compiler gaps",
                    "Many C cross-compiler features in the backlog. Many beej, bgc, chibicc, and C samples do not compile yet.")}
            </div>
        </section>
    }
}

fn gap_row(level: StatusLevel, priority: &str, gap: &str, details: &str) -> Html {
    html! {
        <div class="gaps-grid-row">
            <span class="gaps-grid-priority">{status_badge(&level, priority)}</span>
            <span class="gaps-grid-gap">{gap}</span>
            <span class="gaps-grid-details">{details}</span>
        </div>
    }
}

fn status_badge(level: &StatusLevel, label: &str) -> Html {
    html! {
        <span class={format!("status-badge {}", level.class())}>
            {label}
        </span>
    }
}
