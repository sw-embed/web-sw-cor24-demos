use yew::prelude::*;

static CHARTS: &[(&str, &str, &str)] = &[
    ("tinyc", "sw-cor24-x-tinyc", "Tiny C (tc24r)"),
    ("pascal", "sw-cor24-pascal", "Pascal"),
    ("pcode", "sw-cor24-pcode", "P-code VM"),
    ("plsw", "sw-cor24-plsw", "PL/SW"),
];

pub fn render_issue_charts() -> Html {
    let charts: Vec<Html> = CHARTS.iter().map(|(slug, repo, label)| {
        html! {
            <div class="issue-chart-wrap">
                <object data={format!("images/{slug}-issue-chart.svg")} type="image/svg+xml" class="issue-chart-svg">
                    {"Issue progress chart"}
                </object>
                <span class="issue-chart-label">
                    <a href={format!("https://github.com/sw-embed/{repo}")}
                       target="_blank" rel="noopener noreferrer">
                        {label}
                    </a>
                </span>
            </div>
        }
    }).collect();
    html! {
        <section class="eco-section issue-chart-section">
            <h2 class="section-heading">{"Issue Progress"}</h2>
            <p class="eco-intro">
                {"Cumulative open and closed issue counts. Regenerate with "}
                <code>{"cargo run -p gen-issue-chart"}</code>
                {"."}
            </p>
            {charts}
        </section>
    }
}
