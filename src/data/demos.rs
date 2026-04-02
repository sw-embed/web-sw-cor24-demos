#[derive(Clone, PartialEq, Debug)]
pub struct DemoEntry {
    pub name: &'static str,
    pub slug: &'static str,
    pub description: &'static str,
    pub status: DemoStatus,
    pub tags: &'static [&'static str],
    pub is_this_site: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DemoStatus {
    Active,
    Beta,
    Planned,
}

impl DemoEntry {
    pub fn live_url(&self) -> String {
        format!("https://sw-embed.github.io/{}/", self.slug)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/sw-embed/{}", self.slug)
    }
}

pub fn all_demos() -> &'static [DemoEntry] {
    &DEMO_ENTRIES
}

static DEMO_ENTRIES: [DemoEntry; 9] = [
    DemoEntry {
        name: "Assembly IDE",
        slug: "web-sw-cor24-assembler",
        description: "Full COR24 assembly IDE with syntax highlighting, real-time assembly, and register view.",
        status: DemoStatus::Active,
        tags: &["IDE", "Assembler"],
        is_this_site: false,
    },
    DemoEntry {
        name: "P-code VM",
        slug: "web-sw-cor24-pcode",
        description: "Interactive P-code virtual machine debugger with instruction stepping and stack visualization.",
        status: DemoStatus::Active,
        tags: &["Debugger", "VM"],
        is_this_site: false,
    },
    DemoEntry {
        name: "Tiny C Compiler",
        slug: "web-sw-cor24-tinyc",
        description: "Tiny C cross-compiler for COR24. Compile C code to COR24 assembly in the browser.",
        status: DemoStatus::Active,
        tags: &["Compiler", "C"],
        is_this_site: false,
    },
    DemoEntry {
        name: "Macro Lisp",
        slug: "web-sw-cor24-macrolisp",
        description: "Tiny Macro Lisp REPL. Write and evaluate Lisp expressions that run on COR24 hardware.",
        status: DemoStatus::Active,
        tags: &["Interpreter", "Lisp"],
        is_this_site: false,
    },
    DemoEntry {
        name: "Pascal Demos",
        slug: "web-sw-cor24-pascal",
        description: "Pascal compiler demos. Compile Pascal source to P-code and COR24 assembly.",
        status: DemoStatus::Active,
        tags: &["Compiler", "Pascal"],
        is_this_site: false,
    },
    DemoEntry {
        name: "APL Environment",
        slug: "web-sw-cor24-apl",
        description: "APL interpreter with live array operations. Explore APL's concise array notation.",
        status: DemoStatus::Active,
        tags: &["Interpreter", "APL"],
        is_this_site: false,
    },
    DemoEntry {
        name: "Forth Debugger",
        slug: "web-sw-cor24-forth",
        description: "Forth language debugger with dictionary browsing, stack inspection, and word definitions.",
        status: DemoStatus::Active,
        tags: &["Debugger", "Forth"],
        is_this_site: false,
    },
    DemoEntry {
        name: "PL/SW IDE",
        slug: "web-sw-cor24-plsw",
        description: "PL/SW development environment for the PL/I-inspired PL/SW language running on COR24.",
        status: DemoStatus::Active,
        tags: &["IDE", "Compiler"],
        is_this_site: false,
    },
    DemoEntry {
        name: "Ecosystem Hub",
        slug: "web-sw-cor24-demos",
        description: "This landing page. Documentation hub and directory for the entire COR24 ecosystem.",
        status: DemoStatus::Active,
        tags: &["Docs"],
        is_this_site: true,
    },
];

pub fn tag_class(tag: &str) -> &'static str {
    match tag {
        "IDE" => "tag-ide",
        "Compiler" | "Assembler" => "tag-compiler",
        "Interpreter" => "tag-interpreter",
        "Debugger" => "tag-debugger",
        "VM" => "tag-vm",
        "Docs" => "tag-docs",
        "C" | "Lisp" | "Pascal" | "APL" | "Forth" => "tag-lang",
        _ => "tag-default",
    }
}

pub fn status_badge_class(status: &DemoStatus) -> &'static str {
    match status {
        DemoStatus::Active => "badge-active",
        DemoStatus::Beta => "badge-beta",
        DemoStatus::Planned => "badge-planned",
    }
}

pub fn status_label(status: &DemoStatus) -> &'static str {
    match status {
        DemoStatus::Active => "Active",
        DemoStatus::Beta => "Beta",
        DemoStatus::Planned => "Planned",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_count_is_nine() {
        assert_eq!(all_demos().len(), 9);
    }

    #[test]
    fn all_demos_have_required_fields() {
        for d in all_demos() {
            assert!(!d.name.is_empty(), "demo name is empty");
            assert!(d.slug.starts_with("web-sw-cor24-"), "bad slug: {}", d.slug);
            assert!(
                !d.description.is_empty(),
                "description empty for {}",
                d.name
            );
            assert!(!d.tags.is_empty(), "no tags for {}", d.name);
        }
    }

    #[test]
    fn live_urls_valid() {
        for d in all_demos() {
            let url = d.live_url();
            assert!(url.starts_with("https://sw-embed.github.io/"));
            assert!(url.ends_with('/'));
        }
    }

    #[test]
    fn repo_urls_valid() {
        for d in all_demos() {
            let url = d.repo_url();
            assert!(url.starts_with("https://github.com/sw-embed/"));
        }
    }

    #[test]
    fn this_site_flag() {
        let hub = all_demos().iter().find(|d| d.slug == "web-sw-cor24-demos");
        assert!(hub.is_some());
        assert!(hub.unwrap().is_this_site);
        assert_eq!(all_demos().iter().filter(|d| !d.is_this_site).count(), 8);
    }

    #[test]
    fn tag_class_known() {
        assert_eq!(tag_class("IDE"), "tag-ide");
        assert_eq!(tag_class("Compiler"), "tag-compiler");
        assert_eq!(tag_class("Interpreter"), "tag-interpreter");
        assert_eq!(tag_class("Debugger"), "tag-debugger");
        assert_eq!(tag_class("VM"), "tag-vm");
        assert_eq!(tag_class("Docs"), "tag-docs");
        assert_eq!(tag_class("C"), "tag-lang");
        assert_eq!(tag_class("Assembler"), "tag-compiler");
        assert_eq!(tag_class("Unknown"), "tag-default");
    }

    #[test]
    fn status_helpers() {
        assert_eq!(DemoStatus::Active.as_str(), "Active");
        assert_eq!(status_badge_class(&DemoStatus::Beta), "badge-beta");
        assert_eq!(status_label(&DemoStatus::Planned), "Planned");
    }
}
