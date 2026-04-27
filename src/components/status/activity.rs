use yew::prelude::*;

const COMMITS_HTML: &str = include_str!("../../../reports/commits.html");
const CLOSED_ISSUES_HTML: &str = include_str!("../../../reports/closed-issues.html");

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
            <ReportTable title="Commits by Repo &amp; Hour" inner_html={COMMITS_HTML} />
            <ReportTable title="Closed Issues by Repo &amp; Date" inner_html={CLOSED_ISSUES_HTML} />
        </section>
    }
}

#[function_component(ReportTable)]
fn report_table(props: &ReportTableProps) -> Html {
    let scroll_ref = use_node_ref();

    use_effect_with(scroll_ref.clone(), move |scroll_ref| {
        if let Some(el) = scroll_ref.cast::<web_sys::HtmlElement>() {
            el.set_scroll_left(el.scroll_width() - el.client_width());
        }
        || ()
    });

    let on_first = {
        let scroll_ref = scroll_ref.clone();
        Callback::from(move |_| {
            let Some(el) = scroll_ref.cast::<web_sys::HtmlElement>() else {
                return;
            };
            el.set_scroll_left(0);
        })
    };

    let on_last = {
        let scroll_ref = scroll_ref.clone();
        Callback::from(move |_| {
            let Some(el) = scroll_ref.cast::<web_sys::HtmlElement>() else {
                return;
            };
            el.set_scroll_left(el.scroll_width() - el.client_width());
        })
    };

    let body = extract_body(&props.inner_html);

    html! {
        <div class="activity-report-wrap">
            <div class="activity-report-header">
                <h3 class="activity-report-title">{&props.title}</h3>
                <div class="activity-report-nav">
                    <button class="activity-nav-btn" onclick={on_first}>{"\u{2190} First"}</button>
                    <button class="activity-nav-btn" onclick={on_last}>{"Last \u{2192}"}</button>
                </div>
            </div>
            <div ref={scroll_ref} class="activity-report-scroll">
                <div class="activity-report-body">
                    <table class="activity-report-table">{ Html::from_html_unchecked(body.into()) }</table>
                </div>
            </div>
        </div>
    }
}

fn extract_body(html: &str) -> String {
    let start = html.find("<body>").map(|i| i + 6).unwrap_or(0);
    let end = html.rfind("</body>").unwrap_or(html.len());
    html[start..end].trim().to_string()
}

#[derive(Clone, PartialEq, Properties)]
struct ReportTableProps {
    title: String,
    inner_html: String,
}
