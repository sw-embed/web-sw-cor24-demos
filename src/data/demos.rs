#[derive(Clone, PartialEq, Debug)]
pub struct DemoEntry {
    pub name: &'static str,
    pub slug: &'static str,
    pub description: &'static str,
    pub status: DemoStatus,
    pub tags: &'static [&'static str],
    pub has_live_demo: bool,
    pub is_this_site: bool,
    pub source_label: &'static str,
    pub badge_image: &'static str,
    pub repo: &'static str,
    pub group_id: &'static str,
    pub live_url_override: Option<&'static str>,
    pub secondary_live_url: Option<&'static str>,
    pub secondary_live_label: &'static str,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DemoStatus {
    Active,
    Wip,
    Testing,
    Design,
    LongTerm,
}

#[derive(Clone, PartialEq, Debug, yew::Properties)]
pub struct Category {
    pub id: &'static str,
    pub label: &'static str,
    pub items: &'static [DemoEntry],
}

impl DemoEntry {
    pub fn live_url(&self) -> String {
        if let Some(override_url) = self.live_url_override {
            override_url.to_string()
        } else {
            format!("https://sw-embed.github.io/{}/", self.slug)
        }
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/sw-embed/{}", self.repo)
    }
}

pub fn all_categories() -> &'static [Category] {
    &CATEGORIES
}

pub fn all_demos() -> Vec<DemoEntry> {
    all_categories()
        .iter()
        .flat_map(|c| c.items.iter().cloned())
        .collect()
}

pub fn all_tags() -> Vec<&'static str> {
    let mut tags: Vec<&'static str> = all_demos()
        .iter()
        .flat_map(|d| d.tags.iter().copied())
        .collect();
    tags.sort();
    tags.dedup();
    tags
}

static CATEGORIES: [Category; 11] = [
    Category {
        id: "cross-tools",
        label: "Cross-Assembler / Cross-Compiler",
        items: &[
            DemoEntry {
                name: "Assembly IDE",
                slug: "web-sw-cor24-assembler",
                description: "Full COR24 assembly IDE with syntax highlighting, real-time assembly, and register view.",
                status: DemoStatus::Active,
                tags: &["IDE", "Assembler"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "Rust Source",
                badge_image: "asm-badge.png",
                repo: "sw-cor24-emulator",
                group_id: "cross-tools",
                live_url_override: Some("https://sw-embed.github.io/cor24-rs/"),
                secondary_live_url: Some("https://sw-embed.github.io/web-sw-cor24-assembler/"),
                secondary_live_label: "Assembler-Only Port",
            },
            DemoEntry {
                name: "COR24 Emulator",
                slug: "sw-cor24-emulator",
                description: "Assembler and emulator for the COR24 processor. Runs COR24 binaries on the host.",
                status: DemoStatus::Active,
                tags: &["Emulator", "Assembler"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "Rust Source",
                badge_image: "",
                repo: "sw-cor24-emulator",
                group_id: "cross-tools",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "P-code VM Debugger",
                slug: "web-sw-cor24-pcode",
                description: "Interactive P-code virtual machine debugger with instruction stepping and stack visualization.",
                status: DemoStatus::Active,
                tags: &["VM", "Debugger"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "COR24 Asm & Rust Source",
                badge_image: "",
                repo: "sw-cor24-pcode",
                group_id: "cross-tools",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Source-Level Debugger",
                slug: "sw-cor24-debugger",
                description: "Source-level debugger for COR24. Step through code, inspect registers and memory.",
                status: DemoStatus::Design,
                tags: &["Debugger", "IDE"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "Planned",
                badge_image: "",
                repo: "sw-cor24-debugger",
                group_id: "cross-tools",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Resident Monitor",
                slug: "sw-cor24-monitor",
                description: "Resident monitor and service processor. Assembly + C, currently in testing.",
                status: DemoStatus::Testing,
                tags: &["System", "Monitor"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "C & Asm Source",
                badge_image: "",
                repo: "sw-cor24-monitor",
                group_id: "cross-tools",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "yocto-ed",
                slug: "sw-cor24-yocto-ed",
                description: "Minimal modal text editor with gap buffer, edit/command modes, and UART I/O. Dogfooding project for tc24r.",
                status: DemoStatus::Active,
                tags: &["Editor", "Application"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "C Source",
                badge_image: "",
                repo: "sw-cor24-yocto-ed",
                group_id: "cross-tools",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
        ],
    },
    Category {
        id: "apl",
        label: "APL",
        items: &[DemoEntry {
            name: "APL Environment",
            slug: "web-sw-cor24-apl",
            description: "APL interpreter with live array operations. Explore APL's concise array notation.",
            status: DemoStatus::Active,
            tags: &["Interpreter", "APL"],
            has_live_demo: true,
            is_this_site: false,
            source_label: "C Source",
            badge_image: "apl-sw-badge.png",
            repo: "sw-cor24-apl",
            group_id: "apl",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "basic",
        label: "BASIC",
        items: &[DemoEntry {
            name: "BASIC Interpreter",
            slug: "web-sw-cor24-basic",
            description: "BASIC interpreter for COR24. Write and run BASIC programs on the COR24 platform.",
            status: DemoStatus::Wip,
            tags: &["Interpreter", "BASIC"],
            has_live_demo: false,
            is_this_site: false,
            source_label: "Pascal Source",
            badge_image: "basic-badge.png",
            repo: "sw-cor24-basic",
            group_id: "basic",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "c",
        label: "C",
        items: &[DemoEntry {
            name: "Native C Compiler",
            slug: "sw-cor24-c-compiler",
            description: "Native C compiler that runs on COR24 hardware. Long-term goal for self-hosted C.",
            status: DemoStatus::LongTerm,
            tags: &["Compiler", "C"],
            has_live_demo: false,
            is_this_site: false,
            source_label: "C Source",
            badge_image: "c-badge.png",
            repo: "sw-cor24-c-compiler",
            group_id: "c",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "forth",
        label: "Forth",
        items: &[DemoEntry {
            name: "Forth Debugger",
            slug: "web-sw-cor24-forth",
            description: "Forth language debugger with dictionary browsing, stack inspection, and word definitions.",
            status: DemoStatus::Wip,
            tags: &["Interpreter", "Forth"],
            has_live_demo: true,
            is_this_site: false,
            source_label: "C & Forth Source",
            badge_image: "forth-badge.png",
            repo: "sw-cor24-forth",
            group_id: "forth",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "fortran",
        label: "Fortran",
        items: &[DemoEntry {
            name: "Fortran Compiler",
            slug: "web-sw-cor24-fortran",
            description: "Fortran compiler for COR24. Compile Fortran to COR24 assembly.",
            status: DemoStatus::Design,
            tags: &["Compiler", "Fortran"],
            has_live_demo: false,
            is_this_site: false,
            source_label: "Planned",
            badge_image: "",
            repo: "sw-cor24-fortran",
            group_id: "fortran",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "lisp",
        label: "Lisp",
        items: &[DemoEntry {
            name: "Macro Lisp",
            slug: "web-sw-cor24-macrolisp",
            description: "Tiny Macro Lisp REPL. Write and evaluate Lisp expressions that run on COR24 hardware.",
            status: DemoStatus::Wip,
            tags: &["Interpreter", "Lisp"],
            has_live_demo: true,
            is_this_site: false,
            source_label: "C & Lisp Source",
            badge_image: "lisp-badge.png",
            repo: "sw-cor24-macrolisp",
            group_id: "lisp",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "pascal",
        label: "Pascal",
        items: &[DemoEntry {
            name: "Pascal Compiler",
            slug: "web-sw-cor24-pascal",
            description: "Pascal compiler. Compile Pascal source to P-code and COR24 assembly.",
            status: DemoStatus::Wip,
            tags: &["Compiler", "Pascal"],
            has_live_demo: true,
            is_this_site: false,
            source_label: "C Source",
            badge_image: "pascal-badge.png",
            repo: "sw-cor24-pascal",
            group_id: "pascal",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "plsw",
        label: "PL/SW",
        items: &[DemoEntry {
            name: "PL/SW IDE",
            slug: "web-sw-cor24-plsw",
            description: "PL/SW development environment for the PL/I-inspired PL/SW language running on COR24.",
            status: DemoStatus::Wip,
            tags: &["Compiler", "PL/SW"],
            has_live_demo: true,
            is_this_site: false,
            source_label: "C Source",
            badge_image: "sw-pl-badge2.png",
            repo: "sw-cor24-plsw",
            group_id: "plsw",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "rust",
        label: "Rust",
        items: &[DemoEntry {
            name: "Rust MSP430 Translator",
            slug: "web-sw-cor24-rust",
            description: "Experimental Rust-to-COR24 pipeline. Compile a subset of Rust via MSP430 translation to COR24 assembly. (Rust tab)",
            status: DemoStatus::Wip,
            tags: &["Compiler", "Rust"],
            has_live_demo: true,
            is_this_site: false,
            source_label: "Rust Source",
            badge_image: "rust-gear-logo-red.png",
            repo: "sw-cor24-rust",
            group_id: "rust",
            live_url_override: Some("https://sw-embed.github.io/cor24-rs/"),
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "sws",
        label: "SWS",
        items: &[DemoEntry {
            name: "SWS Scripting",
            slug: "web-sw-cor24-script",
            description: "SWS scripting language (Tcl-like). Interactive scripting for COR24.",
            status: DemoStatus::Testing,
            tags: &["Interpreter", "Scripting"],
            has_live_demo: false,
            is_this_site: false,
            source_label: "C Source",
            badge_image: "sws-badge2.png",
            repo: "sw-cor24-script",
            group_id: "sws",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
];

pub fn tag_class(tag: &str) -> &'static str {
    match tag {
        "IDE" => "tag-ide",
        "Compiler" | "Assembler" => "tag-compiler",
        "Interpreter" => "tag-interpreter",
        "Debugger" => "tag-debugger",
        "VM" => "tag-vm",
        "Emulator" => "tag-vm",
        "Docs" => "tag-docs",
        "System" | "Monitor" | "Scripting" => "tag-default",
        "C" | "Lisp" | "Pascal" | "APL" | "Forth" | "PL/SW" | "BASIC" | "Rust" => "tag-lang",
        _ => "tag-default",
    }
}

pub fn status_badge_class(status: &DemoStatus) -> &'static str {
    match status {
        DemoStatus::Active => "badge-active",
        DemoStatus::Wip => "badge-wip",
        DemoStatus::Testing => "badge-testing",
        DemoStatus::Design => "badge-design",
        DemoStatus::LongTerm => "badge-longterm",
    }
}

pub fn status_label(status: &DemoStatus) -> &'static str {
    match status {
        DemoStatus::Active => "Active",
        DemoStatus::Wip => "WIP",
        DemoStatus::Testing => "Testing",
        DemoStatus::Design => "Design",
        DemoStatus::LongTerm => "Long-term",
    }
}

pub fn filter_languages() -> Vec<(&'static str, String)> {
    vec![
        ("All", "all".to_string()),
        ("Cross-Tools", "cross-tools".to_string()),
        ("APL", "apl".to_string()),
        ("BASIC", "basic".to_string()),
        ("C", "c".to_string()),
        ("Forth", "forth".to_string()),
        ("Fortran", "fortran".to_string()),
        ("Lisp", "lisp".to_string()),
        ("Pascal", "pascal".to_string()),
        ("PL/SW", "plsw".to_string()),
        ("Rust", "rust".to_string()),
        ("SWS", "sws".to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_count() {
        assert_eq!(all_categories().len(), 11);
    }

    #[test]
    fn category_labels_ordered() {
        let labels: Vec<&str> = all_categories().iter().map(|c| c.label).collect();
        assert_eq!(
            labels,
            vec![
                "Cross-Assembler / Cross-Compiler",
                "APL",
                "BASIC",
                "C",
                "Forth",
                "Fortran",
                "Lisp",
                "Pascal",
                "PL/SW",
                "Rust",
                "SWS",
            ]
        );
    }

    #[test]
    fn all_entries_have_required_fields() {
        for cat in all_categories() {
            for d in cat.items {
                assert!(!d.name.is_empty(), "name empty in {}", cat.label);
                assert!(!d.slug.is_empty(), "slug empty for {}", d.name);
                assert!(
                    !d.description.is_empty(),
                    "description empty for {}",
                    d.name
                );
                assert!(!d.tags.is_empty(), "no tags for {}", d.name);
                assert!(!d.group_id.is_empty(), "group_id empty for {}", d.name);
            }
        }
    }

    #[test]
    fn repo_urls_valid() {
        for cat in all_categories() {
            for d in cat.items {
                let url = d.repo_url();
                assert!(
                    url.starts_with("https://github.com/sw-embed/"),
                    "bad repo url for {}: {}",
                    d.name,
                    url
                );
            }
        }
    }

    #[test]
    fn live_urls_valid() {
        for cat in all_categories() {
            for d in cat.items {
                if d.has_live_demo && !d.is_this_site {
                    let url = d.live_url();
                    assert!(
                        url.starts_with("https://sw-embed.github.io/"),
                        "bad live url for {}: {}",
                        d.name,
                        url
                    );
                }
            }
        }
    }

    #[test]
    fn group_ids_match_category() {
        for cat in all_categories() {
            for d in cat.items {
                assert_eq!(
                    d.group_id, cat.id,
                    "group_id mismatch for {} in category {}",
                    d.name, cat.label
                );
            }
        }
    }

    #[test]
    fn status_helpers() {
        assert_eq!(status_label(&DemoStatus::Active), "Active");
        assert_eq!(status_label(&DemoStatus::Wip), "WIP");
        assert_eq!(status_label(&DemoStatus::Testing), "Testing");
        assert_eq!(status_label(&DemoStatus::Design), "Design");
        assert_eq!(status_label(&DemoStatus::LongTerm), "Long-term");
    }

    #[test]
    fn tag_class_coverage() {
        assert_eq!(tag_class("IDE"), "tag-ide");
        assert_eq!(tag_class("Compiler"), "tag-compiler");
        assert_eq!(tag_class("Interpreter"), "tag-interpreter");
        assert_eq!(tag_class("Debugger"), "tag-debugger");
        assert_eq!(tag_class("VM"), "tag-vm");
        assert_eq!(tag_class("Docs"), "tag-docs");
        assert_eq!(tag_class("BASIC"), "tag-lang");
        assert_eq!(tag_class("PL/SW"), "tag-lang");
        assert_eq!(tag_class("Emulator"), "tag-vm");
        assert_eq!(tag_class("Monitor"), "tag-default");
        assert_eq!(tag_class("Unknown"), "tag-default");
    }
}
