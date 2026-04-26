pub mod demos;
pub mod isa;
pub mod lang_descriptions;
pub mod languages;
pub mod status;
pub mod tools;

pub fn repo_org(repo: &str) -> &'static str {
    match repo {
        "tuplet" => "sw-vibe-coding",
        _ => "sw-embed",
    }
}

pub fn repo_pages_host(repo: &str) -> &'static str {
    match repo {
        "tuplet" => "sw-vibe-coding.github.io",
        _ => "sw-embed.github.io",
    }
}
