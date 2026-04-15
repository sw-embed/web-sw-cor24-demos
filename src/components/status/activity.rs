use yew::prelude::*;

pub fn render_activity_reports() -> Html {
    html! {
        <section class="activity-section">
            <h2 class="section-heading">{"Activity Reports"}</h2>
            <p class="eco-intro">
                {"Commit heatmaps and closed-issue timelines across all sw-embed repositories. \
                Regenerate with "}
                <code>{"scripts/gen-commits.sh"}</code>
                {" and "}
                <code>{"scripts/gen-closed-issues.sh"}</code>
                {" in "}
                <code>{"sw-cor24-project"}</code>
                {"."}
            </p>
            {render_commits_report()}
            {render_issues_report()}
        </section>
    }
}

fn iframe_wrap(title: &str, src: &str) -> Html {
    html! {
        <div class="activity-report-wrap">
            <h3 class="activity-report-title">{title}</h3>
            <iframe
                src={src.to_string()}
                class="activity-report-iframe"
                title={title.to_string()}
                loading="lazy"
                sandbox="allow-scripts"
            ></iframe>
        </div>
    }
}

fn render_commits_report() -> Html {
    iframe_wrap("Commits by Repo & Hour", "reports/commits.html")
}

fn render_issues_report() -> Html {
    iframe_wrap("Closed Issues by Repo & Date", "reports/closed-issues.html")
}
