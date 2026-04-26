use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use chrono::Datelike;

const ORG: &str = "sw-embed";

static REPOS: &[&str] = &[
    "sw-cor24-basic",
    "sw-cor24-emulator",
    "sw-cor24-forth",
    "sw-cor24-hlasm",
    "sw-cor24-ocaml",
    "sw-cor24-pascal",
    "sw-cor24-pcode",
    "sw-cor24-plsw",
    "sw-cor24-rpg-ii",
    "sw-cor24-smalltalk",
    "sw-cor24-x-tinyc",
];

fn repo_slug(repo: &str) -> &'static str {
    match repo {
        "sw-cor24-basic" => "basic",
        "sw-cor24-emulator" => "emulator",
        "sw-cor24-forth" => "forth",
        "sw-cor24-hlasm" => "hlasm",
        "sw-cor24-ocaml" => "ocaml",
        "sw-cor24-x-tinyc" => "tinyc",
        "sw-cor24-pascal" => "pascal",
        "sw-cor24-pcode" => "pcode",
        "sw-cor24-plsw" => "plsw",
        "sw-cor24-rpg-ii" => "rpg-ii",
        "sw-cor24-smalltalk" => "smalltalk",
        _ => "repo",
    }
}

struct Issue {
    created_at: chrono::NaiveDate,
    closed_at: Option<chrono::NaiveDate>,
}

struct DayCounts {
    opened: u32,
    closed: u32,
}

fn main() {
    let project_root = find_project_root();
    for &repo in REPOS {
        eprintln!("Processing {repo}...");
        let repo_created = fetch_repo_created(repo);
        eprintln!("  Repo created: {repo_created}");

        let issues = fetch_issues(repo);
        eprintln!("  Fetched {} issues", issues.len());

        let timeline = build_timeline(&repo_created, &issues);
        let svg = render_svg(&timeline, repo);

        let slug = repo_slug(repo);
        let dest = project_root.join(format!("images/{slug}-issue-chart.svg"));
        std::fs::write(&dest, &svg).expect("failed to write SVG");
        println!("Generated {}", dest.display());
    }
}

fn gh(args: &[&str]) -> String {
    let mut cmd = Command::new("gh");
    cmd.args(args);
    cmd.env("GH_PAGER", "cat");
    cmd.env("NO_COLOR", "1");
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::null());
    let child = cmd.spawn().expect("failed to spawn gh");
    let output = child.wait_with_output().expect("failed to wait for gh");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn fetch_repo_created(repo: &str) -> chrono::NaiveDate {
    let out = gh(&["api", &format!("repos/{ORG}/{repo}"), "--jq", ".created_at"]);
    parse_date(&out)
}

fn fetch_issues(repo: &str) -> Vec<Issue> {
    let mut all = Vec::new();
    let mut page = 1u32;
    while page <= 10 {
        let out = gh(&[
            "api",
            &format!(
                "repos/{ORG}/{repo}/issues?state=all&per_page=100&sort=created&direction=asc&page={page}"
            ),
            "--jq",
            "[.[] | {created_at, closed_at}]",
        ]);
        if out == "[]" || out.is_empty() {
            break;
        }
        let items: Vec<serde_json::Value> =
            serde_json::from_str(&out).expect("failed to parse issues JSON");
        if items.is_empty() {
            break;
        }
        for item in &items {
            let created = parse_date(item["created_at"].as_str().unwrap_or(""));
            let closed = item["closed_at"]
                .as_str()
                .filter(|s| !s.is_empty())
                .map(parse_date);
            all.push(Issue {
                created_at: created,
                closed_at: closed,
            });
        }
        page += 1;
    }
    all
}

fn parse_date(s: &str) -> chrono::NaiveDate {
    let dt = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%SZ")
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%:z"))
        .unwrap_or_else(|_| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                .unwrap()
        });
    dt.date()
}

fn format_date(d: chrono::NaiveDate) -> String {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    format!("{} {:02}", months[d.month0() as usize], d.day())
}

fn build_timeline(
    repo_created: &chrono::NaiveDate,
    issues: &[Issue],
) -> BTreeMap<chrono::NaiveDate, DayCounts> {
    let today = chrono::Utc::now().date_naive();
    let mut days: BTreeMap<chrono::NaiveDate, DayCounts> = BTreeMap::new();
    let mut d = *repo_created;
    while d <= today {
        days.insert(
            d,
            DayCounts {
                opened: 0,
                closed: 0,
            },
        );
        d += chrono::Duration::days(1);
    }
    for issue in issues {
        if let Some(entry) = days.get_mut(&issue.created_at) {
            entry.opened += 1;
        }
        if let Some(closed_date) = issue.closed_at
            && let Some(entry) = days.get_mut(&closed_date)
        {
            entry.closed += 1;
        }
    }
    days
}

fn cumulative_counts(
    timeline: &BTreeMap<chrono::NaiveDate, DayCounts>,
    field: impl Fn(&DayCounts) -> u32,
) -> Vec<u32> {
    let mut cum = Vec::new();
    let mut running = 0u32;
    for dc in timeline.values() {
        running += field(dc);
        cum.push(running);
    }
    cum
}

fn nice_ticks(max: u32, target: usize) -> Vec<u32> {
    if max == 0 {
        return vec![0];
    }
    let raw = max as f64 / target as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let residual = raw / mag;
    let nice = if residual <= 1.5 {
        1.0
    } else if residual <= 3.5 {
        2.0
    } else if residual <= 7.5 {
        5.0
    } else {
        10.0
    };
    let step = (nice * mag).round().max(1.0) as u32;
    let mut ticks = Vec::new();
    let mut v = 0u32;
    while v <= max {
        ticks.push(v);
        v += step;
    }
    if ticks.last() != Some(&max) {
        ticks.push(max);
    }
    ticks
}

fn render_svg(timeline: &BTreeMap<chrono::NaiveDate, DayCounts>, repo: &str) -> String {
    let dates: Vec<chrono::NaiveDate> = timeline.keys().copied().collect();
    let n = dates.len();
    if n == 0 {
        return "<svg xmlns='http://www.w3.org/2000/svg'></svg>".to_string();
    }

    let opened_cum = cumulative_counts(timeline, |dc| dc.opened);
    let closed_cum = cumulative_counts(timeline, |dc| dc.closed);
    let open_net: Vec<u32> = opened_cum
        .iter()
        .zip(closed_cum.iter())
        .map(|(&o, &c)| o - c)
        .collect();

    let max_open = *open_net.iter().max().unwrap_or(&1).max(&1);
    let max_closed = *closed_cum.iter().max().unwrap_or(&1).max(&1);
    let max_val = max_open.max(max_closed).max(1);

    let width = 720u32;
    let height = 320u32;
    let pad_left = 45u32;
    let pad_right = 20u32;
    let pad_top = 35u32;
    let pad_bottom = 55u32;
    let chart_w = width - pad_left - pad_right;
    let chart_h = height - pad_top - pad_bottom;
    let right_edge = width - pad_right;

    let x_scale = if n > 1 {
        chart_w as f64 / (n - 1) as f64
    } else {
        0.0
    };
    let y_scale = chart_h as f64 / max_val as f64;

    let to_x = |i: usize| pad_left as f64 + i as f64 * x_scale;
    let to_y = |v: u32| pad_top as f64 + chart_h as f64 - v as f64 * y_scale;

    let mut svg = String::new();
    writeln!(
        svg,
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {width} {height}'"
    )
    .unwrap();
    writeln!(
        svg,
        "     style='font-family: system-ui, sans-serif; font-size: 11px; color: #cdd6f4;'>"
    )
    .unwrap();
    writeln!(
        svg,
        "  <rect width='{width}' height='{height}' fill='#1e1e2e' rx='8'/>"
    )
    .unwrap();

    let y_ticks = nice_ticks(max_val, 5);
    let text_x = pad_left - 5;
    for &v in &y_ticks {
        let y = format!("{:.1}", to_y(v));
        writeln!(svg, "  <line x1='{pad_left}' y1='{y}' x2='{right_edge}' y2='{y}' stroke='#313244' stroke-dasharray='3,3'/>").unwrap();
        writeln!(svg, "  <text x='{text_x}' y='{y}' fill='#a6adc8' text-anchor='end' dominant-baseline='middle'>{v}</text>").unwrap();
    }

    let step = if n <= 8 { 1usize } else { n / 7 };
    let mut x_labels = Vec::new();
    for i in (0..n).step_by(step.max(1)) {
        x_labels.push((i, format_date(dates[i])));
    }
    if x_labels.last().map(|(i, _)| *i) != Some(n - 1) {
        x_labels.push((n - 1, format_date(dates[n - 1])));
    }
    let label_y = height - pad_bottom + 18;
    let bottom = height - pad_bottom;
    for &(i, ref label) in &x_labels {
        let x = format!("{:.1}", to_x(i));
        writeln!(
            svg,
            "  <text x='{x}' y='{label_y}' fill='#a6adc8' text-anchor='middle'>{label}</text>"
        )
        .unwrap();
        writeln!(
            svg,
            "  <line x1='{x}' y1='{pad_top}' x2='{x}' y2='{bottom}' stroke='#313244'/>"
        )
        .unwrap();
    }

    let zero_y = format!("{:.1}", to_y(0));
    let first_x = format!("{:.1}", to_x(0));
    let last_x = format!("{:.1}", to_x(n - 1));

    let open_points: Vec<String> = open_net
        .iter()
        .enumerate()
        .map(|(i, &v)| format!("{:.1},{:.1}", to_x(i), to_y(v)))
        .collect();
    if open_points.len() > 1 {
        let area: Vec<String> = open_points
            .iter()
            .cloned()
            .chain([
                last_x.clone() + "," + &zero_y,
                first_x.clone() + "," + &zero_y,
            ])
            .collect();
        writeln!(
            svg,
            "  <polygon fill='rgba(243,139,168,0.08)' points='{}'/>",
            area.join(" ")
        )
        .unwrap();
        writeln!(svg, "  <polyline fill='none' stroke='#f38ba8' stroke-width='2' stroke-linejoin='round' points='{}'/>",
            open_points.join(" ")).unwrap();
    }

    let closed_points: Vec<String> = closed_cum
        .iter()
        .enumerate()
        .map(|(i, &v)| format!("{:.1},{:.1}", to_x(i), to_y(v)))
        .collect();
    if closed_points.len() > 1 {
        let area: Vec<String> = closed_points
            .iter()
            .cloned()
            .chain([
                last_x.clone() + "," + &zero_y,
                first_x.clone() + "," + &zero_y,
            ])
            .collect();
        writeln!(
            svg,
            "  <polygon fill='rgba(166,227,161,0.08)' points='{}'/>",
            area.join(" ")
        )
        .unwrap();
        writeln!(svg, "  <polyline fill='none' stroke='#a6e3a1' stroke-width='2' stroke-linejoin='round' points='{}'/>",
            closed_points.join(" ")).unwrap();
    }

    let last_open = open_net.last().unwrap_or(&0);
    let last_closed = closed_cum.last().unwrap_or(&0);
    let lx = to_x(n - 1);
    let lx_fmt = format!("{lx:.1}");
    writeln!(
        svg,
        "  <circle cx='{lx_fmt}' cy='{lo:.1}' r='4' fill='#f38ba8'/>",
        lo = to_y(*last_open)
    )
    .unwrap();
    writeln!(
        svg,
        "  <circle cx='{lx_fmt}' cy='{lc:.1}' r='4' fill='#a6e3a1'/>",
        lc = to_y(*last_closed)
    )
    .unwrap();
    writeln!(svg, "  <text x='{ox:.1}' y='{oy:.1}' fill='#f38ba8' text-anchor='start' dominant-baseline='middle' font-size='11px' font-weight='600'>{last_open} open</text>",
        ox = lx + 8.0, oy = to_y(*last_open)).unwrap();
    writeln!(svg, "  <text x='{cx:.1}' y='{cy:.1}' fill='#a6e3a1' text-anchor='start' dominant-baseline='middle' font-size='11px' font-weight='600'>{last_closed} closed</text>",
        cx = lx + 8.0, cy = to_y(*last_closed)).unwrap();

    writeln!(svg, "  <text x='{cx}' y='18' fill='#cdd6f4' text-anchor='middle' font-size='13px' font-weight='600'>Issue Progress: {repo}</text>",
        cx = width / 2).unwrap();

    let legend_x = pad_left as f64 + 10.0;
    let legend_y = height as f64 - 14.0;
    writeln!(svg, "  <line x1='{legend_x}' y1='{legend_y}' x2='{lx2}' y2='{legend_y}' stroke='#f38ba8' stroke-width='2'/>",
        lx2 = legend_x + 16.0).unwrap();
    writeln!(
        svg,
        "  <text x='{tx}' y='{ty}' fill='#a6adc8' dominant-baseline='middle'>Open</text>",
        tx = legend_x + 20.0,
        ty = legend_y
    )
    .unwrap();
    let lx3 = legend_x + 65.0;
    writeln!(svg, "  <line x1='{lx3}' y1='{legend_y}' x2='{lx4}' y2='{legend_y}' stroke='#a6e3a1' stroke-width='2'/>",
        lx4 = legend_x + 81.0).unwrap();
    writeln!(
        svg,
        "  <text x='{tx2}' y='{ty2}' fill='#a6adc8' dominant-baseline='middle'>Closed</text>",
        tx2 = legend_x + 85.0,
        ty2 = legend_y
    )
    .unwrap();

    let start = format_date(dates[0]);
    let end = format_date(dates[n - 1]);
    let axis_cx = pad_left as f64 + chart_w as f64 / 2.0;
    writeln!(svg, "  <text x='{axis_cx}' y='{bot}' fill='#585b70' text-anchor='middle' font-size='9px'>{start} -- {end}</text>",
        bot = height as f64 - 2.0).unwrap();

    svg.push_str("</svg>\n");
    svg
}

fn find_project_root() -> PathBuf {
    let mut dir = std::env::current_dir().expect("no cwd");
    loop {
        if dir.join("src/data/isa").exists() {
            return dir;
        }
        if !dir.pop() {
            return dir;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_ticks_zero() {
        assert_eq!(nice_ticks(0, 5), vec![0]);
    }

    #[test]
    fn nice_ticks_small() {
        let t = nice_ticks(5, 5);
        assert!(t.contains(&0));
        assert!(t.contains(&5));
    }

    #[test]
    fn nice_ticks_large() {
        let t = nice_ticks(20, 5);
        assert!(t.contains(&0));
        assert!(t.contains(&20));
        assert!(t.len() <= 6);
    }

    #[test]
    fn parse_known_date() {
        let d = parse_date("2026-03-30T16:55:36Z");
        assert_eq!(d.year(), 2026);
        assert_eq!(d.month(), 3);
        assert_eq!(d.day(), 30);
    }

    #[test]
    fn format_known_date() {
        let d = chrono::NaiveDate::from_ymd_opt(2026, 4, 2).unwrap();
        assert_eq!(format_date(d), "Apr 02");
    }
}
