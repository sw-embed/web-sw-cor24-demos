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
            format!(
                "https://{}/{}/",
                super::repo_pages_host(self.repo),
                self.slug
            )
        }
    }

    pub fn repo_url(&self) -> String {
        format!(
            "https://github.com/{}/{}",
            super::repo_org(self.repo),
            self.repo
        )
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

static CATEGORIES: [Category; 6] = [
    Category {
        id: "emulator",
        label: "Emulator",
        items: &[DemoEntry {
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
            group_id: "emulator",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "assemblers",
        label: "Assemblers",
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
                group_id: "assemblers",
                live_url_override: Some("https://sw-embed.github.io/cor24-rs/"),
                secondary_live_url: Some("https://sw-embed.github.io/web-sw-cor24-assembler/"),
                secondary_live_label: "Assembler-Only Port",
            },
            DemoEntry {
                name: "High-Level Assembler (HLASM)",
                slug: "sw-cor24-hlasm",
                description: "High-level assembler for COR24 with structured macros and symbolic expressions. Written in COR24 assembly.",
                status: DemoStatus::Wip,
                tags: &["Assembler", "HLASM"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "COR24 Asm Source",
                badge_image: "hlasm-badge.png",
                repo: "sw-cor24-hlasm",
                group_id: "assemblers",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
        ],
    },
    Category {
        id: "system-pl",
        label: "System Programming Languages",
        items: &[
            DemoEntry {
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
                group_id: "system-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "PL/SW IDE",
                slug: "web-sw-cor24-plsw",
                description: "PL/SW development environment for the PL/I-inspired PL/SW language running on COR24.",
                status: DemoStatus::Active,
                tags: &["Compiler", "PL/SW"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "C Source",
                badge_image: "sw-pl-badge2.png",
                repo: "sw-cor24-plsw",
                group_id: "system-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Rust MSP430 Translator",
                slug: "web-sw-cor24-rust",
                description: "Experimental Rust-to-COR24 pipeline. Compile a subset of Rust via MSP430 translation to COR24 assembly.",
                status: DemoStatus::Testing,
                tags: &["Compiler", "Rust"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "Rust Source",
                badge_image: "rust-gear-logo-red.png",
                repo: "sw-cor24-rust",
                group_id: "system-pl",
                live_url_override: Some("https://sw-embed.github.io/cor24-rs/"),
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Tiny C Compiler",
                slug: "web-sw-cor24-tinyc",
                description: "Tiny C cross-compiler for COR24. Compile C code to COR24 assembly in the browser.",
                status: DemoStatus::Active,
                tags: &["Compiler", "C"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "Rust Source",
                badge_image: "c-badge.png",
                repo: "sw-cor24-x-tinyc",
                group_id: "system-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
        ],
    },
    Category {
        id: "application-pl",
        label: "Application Programming Languages",
        items: &[
            DemoEntry {
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
                group_id: "application-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "BASIC Interpreter",
                slug: "web-sw-cor24-basic",
                description: "BASIC interpreter for COR24. Write and run BASIC programs on the COR24 platform.",
                status: DemoStatus::Active,
                tags: &["Interpreter", "BASIC"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "C Source",
                badge_image: "basic-badge.png",
                repo: "sw-cor24-basic",
                group_id: "application-pl",
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-basic/"),
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Forth Debugger",
                slug: "web-sw-cor24-forth",
                description: "Forth language debugger with dictionary browsing, stack inspection, and word definitions.",
                status: DemoStatus::Active,
                tags: &["Interpreter", "Forth"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "C & Forth Source",
                badge_image: "forth-badge.png",
                repo: "sw-cor24-forth",
                group_id: "application-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Fortran Compiler",
                slug: "web-sw-cor24-fortran",
                description: "Fortran compiler for COR24. Compiler written in SNOBOL4, runtime in PL/SW.",
                status: DemoStatus::Wip,
                tags: &["Compiler", "Fortran"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "C Source",
                badge_image: "fortran-badge.png",
                repo: "sw-cor24-fortran",
                group_id: "application-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Macro Lisp",
                slug: "web-sw-cor24-macrolisp",
                description: "Tiny Macro Lisp REPL. Write and evaluate Lisp expressions that run on COR24 hardware.",
                status: DemoStatus::Active,
                tags: &["Interpreter", "Lisp"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "C & Lisp Source",
                badge_image: "lisp-badge.png",
                repo: "sw-cor24-macrolisp",
                group_id: "application-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "OCaml Compiler",
                slug: "web-sw-cor24-ocaml",
                description: "OCaml compiler targeting the COR24 P-code VM. Compiles a subset of OCaml to P-code bytecode \
                         via the Pascal P-code pipeline.",
                status: DemoStatus::Testing,
                tags: &["Compiler", "OCaml"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "Pascal Source",
                badge_image: "ocaml-badge.png",
                repo: "sw-cor24-ocaml",
                group_id: "application-pl",
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-ocaml/"),
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Pascal Compiler",
                slug: "web-sw-cor24-pascal",
                description: "Pascal compiler. Compile Pascal source to P-code and COR24 assembly.",
                status: DemoStatus::Active,
                tags: &["Compiler", "Pascal"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "C Source",
                badge_image: "pascal-badge.png",
                repo: "sw-cor24-pascal",
                group_id: "application-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Prolog Interpreter",
                slug: "sw-cor24-prolog",
                description: "Prolog interpreter with a WAM-like 8+8 register virtual machine implemented in PL/SW, \
                         running on COR24.",
                status: DemoStatus::Design,
                tags: &["Interpreter", "Prolog", "Logic Programming"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "PL/SW Source",
                badge_image: "prolog-badge.png",
                repo: "sw-cor24-prolog",
                group_id: "application-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "RPG-II",
                slug: "sw-cor24-rpg-ii",
                description: "Simplified RPG-II report generator compiled through HLASM. Business data processing on COR24.",
                status: DemoStatus::Wip,
                tags: &["Compiler", "RPG-II"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "HLASM Source",
                badge_image: "rpg-ii-badge.png",
                repo: "sw-cor24-rpg-ii",
                group_id: "application-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Smalltalk",
                slug: "sw-cor24-smalltalk",
                description: "Smalltalk environment implemented in COR24 BASIC. Object-oriented messaging on top of the BASIC/P-code stack.",
                status: DemoStatus::Wip,
                tags: &["Interpreter", "Smalltalk"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "BASIC Source",
                badge_image: "smalltalk-badge.png",
                repo: "sw-cor24-smalltalk",
                group_id: "application-pl",
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-smalltalk/"),
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "SNOBOL4 Interpreter",
                slug: "sw-cor24-snobol4",
                description: "SNOBOL4 pattern-matching language interpreter implemented in PL/SW, running on COR24.",
                status: DemoStatus::Active,
                tags: &["Interpreter", "SNOBOL4", "Pattern Matching"],
                has_live_demo: true,
                is_this_site: false,
                source_label: "PL/SW Source",
                badge_image: "snobol4-badge.png",
                repo: "sw-cor24-snobol4",
                group_id: "application-pl",
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-snobol4/"),
                secondary_live_url: None,
                secondary_live_label: "",
            },
            DemoEntry {
                name: "Tuplet",
                slug: "tuplet",
                description: "Experimental named-tuple infix language with user-grown constructs (Lisp-like macros over a small set of essential special forms). Implemented in OCaml with a Forth runtime.",
                status: DemoStatus::Wip,
                tags: &["Compiler", "Tuplet"],
                has_live_demo: false,
                is_this_site: false,
                source_label: "OCaml & Forth Source",
                badge_image: "tuplet-badge.png",
                repo: "tuplet",
                group_id: "application-pl",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
        ],
    },
    Category {
        id: "scripting",
        label: "Scripting / Shell",
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
            group_id: "scripting",
            live_url_override: None,
            secondary_live_url: None,
            secondary_live_label: "",
        }],
    },
    Category {
        id: "tools",
        label: "Tools",
        items: &[
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
                group_id: "tools",
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
                group_id: "tools",
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
                group_id: "tools",
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
                badge_image: "yocto-ed-badge.png",
                repo: "sw-cor24-yocto-ed",
                group_id: "tools",
                live_url_override: None,
                secondary_live_url: None,
                secondary_live_label: "",
            },
        ],
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
        "System" | "Monitor" | "Scripting" | "Editor" => "tag-default",
        "C" | "Lisp" | "Pascal" | "APL" | "Forth" | "PL/SW" | "BASIC" | "Rust" | "OCaml"
        | "Prolog" | "SNOBOL4" | "Smalltalk" | "Tuplet" | "RPG-II" | "HLASM" => "tag-lang",
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
        ("Application PLs", "application-pl".to_string()),
        ("Assemblers", "assemblers".to_string()),
        ("Emulator", "emulator".to_string()),
        ("Scripting / Shell", "scripting".to_string()),
        ("System PLs", "system-pl".to_string()),
        ("Tools", "tools".to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_count() {
        assert_eq!(all_categories().len(), 6);
    }

    #[test]
    fn filter_languages_count() {
        let filters = filter_languages();
        assert_eq!(filters.len(), 7);
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
                    url.starts_with("https://github.com/"),
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
                        url.starts_with("https://") && url.contains(".github.io/"),
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
        assert_eq!(tag_class("Editor"), "tag-default");
        assert_eq!(tag_class("RPG-II"), "tag-lang");
        assert_eq!(tag_class("Smalltalk"), "tag-lang");
        assert_eq!(tag_class("Tuplet"), "tag-lang");
        assert_eq!(tag_class("HLASM"), "tag-lang");
        assert_eq!(tag_class("Unknown"), "tag-default");
    }
}
