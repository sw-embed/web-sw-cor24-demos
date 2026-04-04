use yew::prelude::*;

pub fn render_issue_chart() -> Html {
    html! {
        <section class="eco-section issue-chart-section">
            <h2 class="section-heading">{"Issue Progress"}</h2>
            <p class="eco-intro">
                {"Cumulative open and closed issue counts for "}
                <a href="https://github.com/sw-embed/sw-cor24-x-tinyc"
                   target="_blank" rel="noopener noreferrer">
                    {"sw-cor24-x-tinyc"}
                </a>
                {". Regenerate with "}
                <code>{"cargo run -p gen-issue-chart"}</code>
                {"."}
            </p>
            <div class="issue-chart-wrap">
                <object data="images/tinyc-issue-chart.svg" type="image/svg+xml" class="issue-chart-svg">
                    {"Issue progress chart"}
                </object>
            </div>
        </section>
    }
}
