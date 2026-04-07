#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RepoStatus {
    pub level: StatusLevel,
    pub label: &'static str,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StatusLevel {
    Green,
    Yellow,
    Orange,
    Red,
    Neutral,
}

impl StatusLevel {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Green => "status-green",
            Self::Yellow => "status-yellow",
            Self::Orange => "status-orange",
            Self::Red => "status-red",
            Self::Neutral => "status-neutral",
        }
    }
}

pub const fn green(label: &'static str) -> RepoStatus {
    RepoStatus {
        level: StatusLevel::Green,
        label,
    }
}

pub const fn yellow(label: &'static str) -> RepoStatus {
    RepoStatus {
        level: StatusLevel::Yellow,
        label,
    }
}

pub const fn orange(label: &'static str) -> RepoStatus {
    RepoStatus {
        level: StatusLevel::Orange,
        label,
    }
}

pub const fn red(label: &'static str) -> RepoStatus {
    RepoStatus {
        level: StatusLevel::Red,
        label,
    }
}

pub const fn neutral(label: &'static str) -> RepoStatus {
    RepoStatus {
        level: StatusLevel::Neutral,
        label,
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProjectRow {
    pub repo: &'static str,
    pub description: &'static str,
    pub repo_status: RepoStatus,
    pub has_web_ui: RepoStatus,
    pub has_saga: bool,
    pub is_web: bool,
    pub group: &'static str,
}

pub fn all_projects() -> &'static [ProjectRow] {
    &PROJECTS
}

static PROJECTS: [ProjectRow; 31] = [
    ProjectRow {
        repo: "sw-cor24-assembler",
        description: "Native assembler (C)",
        repo_status: green("Try it"),
        has_web_ui: orange("Planned"),
        has_saga: false,
        is_web: false,
        group: "Foundation",
    },
    ProjectRow {
        repo: "sw-cor24-emulator",
        description: "Assembler and emulator (Rust)",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: false,
        group: "Foundation",
    },
    ProjectRow {
        repo: "sw-cor24-project",
        description: "Ecosystem hub/docs",
        repo_status: green("Try it"),
        has_web_ui: orange("Planned"),
        has_saga: false,
        is_web: false,
        group: "Foundation",
    },
    ProjectRow {
        repo: "sw-cor24-x-assembler",
        description: "Cross-assembler library (Rust)",
        repo_status: green("Try it"),
        has_web_ui: orange("Planned"),
        has_saga: false,
        is_web: false,
        group: "Foundation",
    },
    ProjectRow {
        repo: "sw-cor24-rust",
        description: "Rust-to-COR24 pipeline",
        repo_status: yellow("In dev"),
        has_web_ui: green("Try it"),
        has_saga: false,
        is_web: false,
        group: "Cross-compilers",
    },
    ProjectRow {
        repo: "sw-cor24-x-tinyc",
        description: "Tiny C cross-compiler (Rust)",
        repo_status: yellow("In dev"),
        has_web_ui: green("Try it"),
        has_saga: true,
        is_web: false,
        group: "Cross-compilers",
    },
    ProjectRow {
        repo: "sw-cor24-pascal",
        description: "Pascal compiler (C) + runtime",
        repo_status: green("Try it"),
        has_web_ui: green("Try it"),
        has_saga: false,
        is_web: false,
        group: "P-code",
    },
    ProjectRow {
        repo: "sw-cor24-pcode",
        description: "P-code VM (asm) + assembler (Rust)",
        repo_status: green("Try it"),
        has_web_ui: green("Try it"),
        has_saga: true,
        is_web: false,
        group: "P-code",
    },
    ProjectRow {
        repo: "sw-cor24-x-pc-aotc",
        description: "P-code AOT compiler",
        repo_status: yellow("In dev"),
        has_web_ui: orange("Planned"),
        has_saga: true,
        is_web: false,
        group: "P-code",
    },
    ProjectRow {
        repo: "sw-cor24-apl",
        description: "APL interpreter (C)",
        repo_status: green("Try it"),
        has_web_ui: green("Try it"),
        has_saga: true,
        is_web: false,
        group: "Native langs",
    },
    ProjectRow {
        repo: "sw-cor24-basic",
        description: "BASIC interpreter (C)",
        repo_status: yellow("In dev"),
        has_web_ui: orange("Planned"),
        has_saga: false,
        is_web: false,
        group: "Native langs",
    },
    ProjectRow {
        repo: "sw-cor24-forth",
        description: "Forth (COR24 assembly)",
        repo_status: green("Try it"),
        has_web_ui: green("Try it"),
        has_saga: false,
        is_web: false,
        group: "Native langs",
    },
    ProjectRow {
        repo: "sw-cor24-fortran",
        description: "Fortran compiler (C)",
        repo_status: orange("In plan"),
        has_web_ui: orange("Planned"),
        has_saga: false,
        is_web: false,
        group: "Native langs",
    },
    ProjectRow {
        repo: "sw-cor24-macrolisp",
        description: "Tiny Macro Lisp (C)",
        repo_status: green("Try it"),
        has_web_ui: green("Try it"),
        has_saga: false,
        is_web: false,
        group: "Native langs",
    },
    ProjectRow {
        repo: "sw-cor24-plsw",
        description: "PL/SW compiler (C, PL/I-inspired)",
        repo_status: green("Try it"),
        has_web_ui: green("Try it"),
        has_saga: true,
        is_web: false,
        group: "Native langs",
    },
    ProjectRow {
        repo: "sw-cor24-snobol4",
        description: "SNOBOL4 interpreter (PL/SW)",
        repo_status: yellow("In dev"),
        has_web_ui: orange("Planned"),
        has_saga: false,
        is_web: false,
        group: "Native langs",
    },
    ProjectRow {
        repo: "sw-cor24-script",
        description: "SWS scripting language (C, Tcl-like)",
        repo_status: yellow("In dev"),
        has_web_ui: orange("Planned"),
        has_saga: true,
        is_web: false,
        group: "Native langs",
    },
    ProjectRow {
        repo: "sw-cor24-debugger",
        description: "Source-level debugger",
        repo_status: orange("In plan"),
        has_web_ui: orange("Planned"),
        has_saga: false,
        is_web: false,
        group: "System",
    },
    ProjectRow {
        repo: "sw-cor24-monitor",
        description: "Resident monitor (asm + C)",
        repo_status: yellow("In dev"),
        has_web_ui: orange("Planned"),
        has_saga: true,
        is_web: false,
        group: "System",
    },
    ProjectRow {
        repo: "sw-cor24-yocto-ed",
        description: "Modal text editor (C)",
        repo_status: green("Try it"),
        has_web_ui: orange("Planned"),
        has_saga: false,
        is_web: false,
        group: "System",
    },
    ProjectRow {
        repo: "web-sw-cor24-apl",
        description: "APL environment",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: true,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-assembler",
        description: "COR24 assembly IDE",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-demos",
        description: "Landing page (this site)",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: true,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-forth",
        description: "Forth debugger",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-macrolisp",
        description: "Lisp REPL",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-pascal",
        description: "Pascal demos",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-pcode",
        description: "P-code VM debugger",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: true,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-plsw",
        description: "PL/SW dev environment",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-snobol4",
        description: "SNOBOL4 interpreter",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-rust",
        description: "Rust MSP430 translator",
        repo_status: yellow("In dev"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: true,
        group: "Web UIs",
    },
    ProjectRow {
        repo: "web-sw-cor24-tinyc",
        description: "Tiny C compiler IDE",
        repo_status: green("Try it"),
        has_web_ui: neutral("n/a"),
        has_saga: false,
        is_web: true,
        group: "Web UIs",
    },
];

pub mod generated;

pub use generated::generated_status;

pub fn github_issues_url(repo: &str) -> String {
    format!("https://github.com/sw-embed/{}/issues", repo)
}

pub fn github_repo_url(repo: &str) -> String {
    format!("https://github.com/sw-embed/{}", repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_count() {
        assert_eq!(all_projects().len(), 31);
    }

    #[test]
    fn all_repos_have_groups() {
        for p in all_projects() {
            assert!(!p.group.is_empty(), "empty group for {}", p.repo);
        }
    }

    #[test]
    fn web_repos_have_na_web_ui() {
        for p in all_projects() {
            if p.is_web {
                assert_eq!(
                    p.has_web_ui.level,
                    StatusLevel::Neutral,
                    "web repo {} should have n/a web UI",
                    p.repo
                );
            }
        }
    }
}
