use std::borrow::Cow;

#[derive(Clone, PartialEq, Debug)]
pub struct DemoEntry {
    pub name: &'static str,
    pub slug: &'static str,
    pub description: &'static str,
    pub status: DemoStatus,
    pub tags: &'static [&'static str],
    pub is_this_site: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DemoStatus {
    Active,
    Beta,
    Planned,
}

impl DemoStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Beta => "Beta",
            Self::Planned => "Planned",
        }
    }

    pub fn css_class(self) -> &'static str {
        match self {
            Self::Active => "badge-active",
            Self::Beta => "badge-beta",
            Self::Planned => "badge-planned",
        }
    }
}

impl DemoEntry {
    pub fn live_url(&self) -> Cow<'static, str> {
        if self.is_this_site {
            "https://sw-embed.github.io/web-sw-cor24-demos/".into()
        } else {
            format!("https://sw-embed.github.io/{}/", self.slug).into()
        }
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/sw-embed/{}", self.slug)
    }
}

pub fn all_demos() -> &'static [DemoEntry] {
    &[
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
    ]
}
