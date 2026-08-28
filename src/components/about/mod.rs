pub mod intro;

use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct LinkEntry {
    label: &'static str,
    description: &'static str,
    url: &'static str,
    image: &'static str,
}

fn link_card(entry: &LinkEntry) -> Html {
    html! {
        <a href={entry.url} target="_blank" rel="noopener noreferrer" class="about-card">
            <img src={entry.image} alt={entry.label} class="about-card-icon" />
            <div class="about-card-body">
                <h2 class="about-card-title">{entry.label}</h2>
                <p class="about-card-desc">{entry.description}</p>
                <span class="about-card-cta">{"Visit \u{2192}"}</span>
            </div>
        </a>
    }
}

fn links() -> &'static [LinkEntry; 4] {
    &LINKS
}

static LINKS: [LinkEntry; 4] = [
    LinkEntry {
        label: "Software Wrighter Lab Blog",
        description: "Articles on FPGA design, computer architecture, COR24 development, and embedded systems education.",
        url: "https://software-wrighter-lab.github.io/",
        image: "images/sw-lab-logo.jpg",
    },
    LinkEntry {
        label: "Discord Community",
        description: "Join the Software Wrighter Lab Discord for discussions, questions, and project updates.",
        url: "https://discord.com/invite/Ctzk5uHggZ",
        image: "images/sw-lab-discord.png",
    },
    LinkEntry {
        label: "sw-embed on GitHub",
        description: "Browse the full COR24 ecosystem: emulators, cross-compilers, interpreters, an operating system, and web demos.",
        url: "https://github.com/sw-embed",
        image: "images/sw-embed-git-org-logo.jpeg",
    },
    LinkEntry {
        label: "SoftwareWrighter on YouTube",
        description: "Video content covering FPGA builds, COR24 walkthroughs, and embedded systems tutorials.",
        url: "https://www.youtube.com/@SoftwareWrighter",
        image: "images/sw-yt-logo.jpg",
    },
];

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="about-page page-section">
            <h1 class="about-title">{"About"}</h1>
            <div class="about-intro-row">
                <div class="about-intro-links">
                    {links().iter().map(link_card).collect::<Html>()}
                </div>
                {intro::intro_text()}
            </div>
        </div>
    }
}
