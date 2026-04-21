use yew::prelude::*;

static CHARTS: &[(&str, &str, &str)] = &[
    ("emulator", "sw-cor24-emulator", "Emulator"),
    ("forth", "sw-cor24-forth", "Forth"),
    ("hlasm", "sw-cor24-hlasm", "HLASM"),
    ("pascal", "sw-cor24-pascal", "Pascal"),
    ("pcode", "sw-cor24-pcode", "P-code VM"),
    ("plsw", "sw-cor24-plsw", "PL/SW"),
    ("rpg-ii", "sw-cor24-rpg-ii", "RPG-II"),
    ("tinyc", "sw-cor24-x-tinyc", "Tiny C (tc24r)"),
];

pub fn render_issue_charts() -> Html {
    let build_ts = option_env!("BUILD_TIMESTAMP").unwrap_or("0");
    let charts: Vec<Html> = CHARTS
        .iter()
        .map(|(slug, repo, label)| {
            let src = format!("images/{slug}-issue-chart.svg?ts={build_ts}");
            html! {
                <div class="issue-chart-wrap">
                    <object data={src} type="image/svg+xml" class="issue-chart-svg">
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
        })
        .collect();
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
