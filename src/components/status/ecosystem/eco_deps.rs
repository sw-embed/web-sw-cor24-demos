pub(crate) struct DepEdge {
    pub(crate) from: &'static str,
    pub(crate) to: &'static str,
    pub(crate) label: &'static str,
}

pub(crate) static EDGES: &[DepEdge] = &[
    DepEdge {
        from: "web-sw-cor24-assembler",
        to: "sw-cor24-emulator",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-pcode",
        to: "sw-cor24-pcode",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-tinyc",
        to: "sw-cor24-x-tinyc",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-macrolisp",
        to: "sw-cor24-macrolisp",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-apl",
        to: "sw-cor24-apl",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-pascal",
        to: "sw-cor24-pascal",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-forth",
        to: "sw-cor24-forth",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-plsw",
        to: "sw-cor24-plsw",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-snobol4",
        to: "sw-cor24-snobol4",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-rust",
        to: "sw-cor24-rust",
        label: "WASM build",
    },
    DepEdge {
        from: "web-sw-cor24-demos",
        to: "sw-cor24-emulator",
        label: "docs",
    },
    DepEdge {
        from: "sw-cor24-x-tinyc",
        to: "sw-cor24-emulator",
        label: "ISA defs",
    },
    DepEdge {
        from: "sw-cor24-x-assembler",
        to: "sw-cor24-emulator",
        label: "ISA defs",
    },
    DepEdge {
        from: "sw-cor24-x-pc-aotc",
        to: "sw-cor24-pcode",
        label: "p-code format",
    },
    DepEdge {
        from: "sw-cor24-x-pc-aotc",
        to: "sw-cor24-x-assembler",
        label: "emit COR24 asm",
    },
    DepEdge {
        from: "sw-cor24-pascal",
        to: "sw-cor24-pcode",
        label: "target VM",
    },
    DepEdge {
        from: "sw-cor24-monitor",
        to: "sw-cor24-emulator",
        label: "ISA defs",
    },
    DepEdge {
        from: "sw-cor24-monitor",
        to: "sw-cor24-x-assembler",
        label: "asm output",
    },
    DepEdge {
        from: "sw-cor24-rust",
        to: "sw-cor24-x-assembler",
        label: "asm output",
    },
    DepEdge {
        from: "sw-cor24-macrolisp",
        to: "sw-cor24-x-tinyc",
        label: "compiled by",
    },
    DepEdge {
        from: "sw-cor24-apl",
        to: "sw-cor24-x-tinyc",
        label: "compiled by",
    },
    DepEdge {
        from: "sw-cor24-basic",
        to: "sw-cor24-pascal",
        label: "compiled by",
    },
    DepEdge {
        from: "sw-cor24-fortran",
        to: "sw-cor24-snobol4",
        label: "compiled by",
    },
    DepEdge {
        from: "sw-cor24-plsw",
        to: "sw-cor24-x-tinyc",
        label: "compiled by",
    },
    DepEdge {
        from: "sw-cor24-script",
        to: "sw-cor24-x-tinyc",
        label: "compiled by",
    },
    DepEdge {
        from: "sw-cor24-snobol4",
        to: "sw-cor24-plsw",
        label: "compiled by",
    },
    DepEdge {
        from: "sw-cor24-assembler",
        to: "sw-cor24-x-tinyc",
        label: "compiled by",
    },
    DepEdge {
        from: "sw-cor24-yocto-ed",
        to: "sw-cor24-x-tinyc",
        label: "compiled by",
    },
];

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum DepGroup {
    WebDemos,
    Foundation,
    CrossCompiler,
    PCode,
    NativeLang,
    System,
}

impl DepGroup {
    pub(crate) fn css_class(&self) -> &'static str {
        match self {
            Self::WebDemos => "dep-blue",
            Self::Foundation => "dep-green",
            Self::CrossCompiler => "dep-mauve",
            Self::PCode => "dep-peach",
            Self::NativeLang => "dep-yellow",
            Self::System => "dep-red",
        }
    }
}

pub(crate) fn repo_group(name: &str) -> DepGroup {
    if name.starts_with("web-") {
        DepGroup::WebDemos
    } else {
        match name {
            "sw-cor24-emulator"
            | "sw-cor24-x-assembler"
            | "sw-cor24-assembler"
            | "sw-cor24-project" => DepGroup::Foundation,
            "sw-cor24-x-tinyc" | "sw-cor24-rust" => DepGroup::CrossCompiler,
            "sw-cor24-pcode" | "sw-cor24-x-pc-aotc" | "sw-cor24-pascal" => DepGroup::PCode,
            "sw-cor24-macrolisp" | "sw-cor24-apl" | "sw-cor24-basic" | "sw-cor24-forth"
            | "sw-cor24-fortran" | "sw-cor24-plsw" | "sw-cor24-script" | "sw-cor24-snobol4" => {
                DepGroup::NativeLang
            }
            "sw-cor24-monitor" | "sw-cor24-debugger" | "sw-cor24-yocto-ed" => DepGroup::System,
            _ => DepGroup::Foundation,
        }
    }
}

pub(crate) fn repo_group_class(name: &str) -> &'static str {
    repo_group(name).css_class()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_count() {
        assert_eq!(EDGES.len(), 28);
    }

    #[test]
    fn all_edges_have_valid_groups() {
        for e in EDGES {
            let _ = repo_group(e.from);
            let _ = repo_group(e.to);
        }
    }

    #[test]
    fn web_demos_target_correct_repos() {
        let web_edges: Vec<_> = EDGES
            .iter()
            .filter(|e| e.from.starts_with("web-"))
            .collect();
        for e in &web_edges {
            assert!(
                !e.to.starts_with("web-"),
                "{} -> {} should not target a web repo",
                e.from,
                e.to
            );
        }
    }
}
