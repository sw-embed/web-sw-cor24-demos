#[derive(Clone, PartialEq, Debug)]
pub struct ToolEntry {
    pub name: &'static str,
    pub repo: &'static str,
    pub description: &'static str,
    pub language: ToolLanguage,
    pub target: ToolTarget,
    pub has_web_ui: bool,
    pub category: ToolCategory,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ToolCategory {
    Foundation,
    CrossCompiler,
    PCode,
    NativeLanguage,
    SystemSoftware,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ToolLanguage {
    Rust,
    C,
    Assembly,
    Mixed(&'static str),
    Docs,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ToolTarget {
    Host,
    Cor24,
    Cor24Emulated,
    Docs,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ToolGroup {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub items: &'static [ToolEntry],
}

impl ToolEntry {
    pub fn repo_url(&self) -> String {
        format!("https://github.com/sw-embed/{}", self.repo)
    }

    pub fn demo_url(&self) -> Option<String> {
        if self.has_web_ui {
            Some(format!("https://sw-embed.github.io/{}/", self.repo))
        } else {
            None
        }
    }
}

impl ToolLanguage {
    pub fn label(&self) -> &'static str {
        match self {
            ToolLanguage::Rust => "Rust",
            ToolLanguage::C => "C",
            ToolLanguage::Assembly => "Assembly",
            ToolLanguage::Mixed(s) => s,
            ToolLanguage::Docs => "Docs",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            ToolLanguage::Rust => "tool-badge-rust",
            ToolLanguage::C => "tool-badge-c",
            ToolLanguage::Assembly => "tool-badge-asm",
            ToolLanguage::Mixed(_) => "tool-badge-mixed",
            ToolLanguage::Docs => "tool-badge-docs",
        }
    }
}

impl ToolTarget {
    pub fn label(&self) -> &'static str {
        match self {
            ToolTarget::Host => "Host",
            ToolTarget::Cor24 => "COR24",
            ToolTarget::Cor24Emulated => "COR24 (emulated)",
            ToolTarget::Docs => "Docs",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            ToolTarget::Host => "tool-badge-host",
            ToolTarget::Cor24 => "tool-badge-cor24",
            ToolTarget::Cor24Emulated => "tool-badge-emu",
            ToolTarget::Docs => "tool-badge-docs",
        }
    }
}

pub fn all_groups() -> &'static [ToolGroup] {
    &GROUPS
}

pub fn all_tools() -> Vec<ToolEntry> {
    all_groups()
        .iter()
        .flat_map(|g| g.items.iter().cloned())
        .collect()
}

static GROUPS: [ToolGroup; 1] = [ToolGroup {
    id: "foundation",
    label: "Foundation",
    description: "Core tools for the COR24 ecosystem: the assembler/emulator, cross-assembler library, \
         native assembler, and project documentation hub.",
    items: &[
        ToolEntry {
            name: "COR24 Assembler & Emulator",
            repo: "sw-cor24-emulator",
            description: "Assembler and emulator for the COR24 processor. Parses COR24 assembly, \
                 assembles it to binary, and executes binaries in a virtual machine with full \
                 register and memory inspection.",
            language: ToolLanguage::Rust,
            target: ToolTarget::Host,
            has_web_ui: false,
            category: ToolCategory::Foundation,
        },
        ToolEntry {
            name: "Cross-Assembler Library",
            repo: "sw-cor24-x-assembler",
            description: "Cross-assembler library for COR24. Provides a Rust API for assembling COR24 \
                 machine code from assembly text, used by cross-compilers and web-based tools.",
            language: ToolLanguage::Rust,
            target: ToolTarget::Host,
            has_web_ui: false,
            category: ToolCategory::Foundation,
        },
        ToolEntry {
            name: "Native Assembler (as24)",
            repo: "sw-cor24-assembler",
            description: "Native assembler written in C that runs directly on COR24 hardware. Part of the \
                 self-hosting toolchain, assembling COR24 assembly to binary on the target platform.",
            language: ToolLanguage::C,
            target: ToolTarget::Cor24,
            has_web_ui: false,
            category: ToolCategory::Foundation,
        },
        ToolEntry {
            name: "Ecosystem Hub",
            repo: "sw-cor24-project",
            description: "Central documentation portal for the entire COR24 ecosystem. Contains ISA specs, \
                 design documents, tooling guides, and project planning resources.",
            language: ToolLanguage::Docs,
            target: ToolTarget::Docs,
            has_web_ui: false,
            category: ToolCategory::Foundation,
        },
    ],
}];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_count() {
        assert_eq!(all_groups().len(), 1);
    }

    #[test]
    fn foundation_tools_count() {
        let foundation = &all_groups()[0];
        assert_eq!(foundation.items.len(), 4);
    }

    #[test]
    fn all_entries_have_required_fields() {
        for g in all_groups() {
            assert!(!g.id.is_empty());
            assert!(!g.label.is_empty());
            assert!(!g.description.is_empty());
            for t in g.items {
                assert!(!t.name.is_empty(), "name empty in {}", g.label);
                assert!(!t.repo.is_empty(), "repo empty for {}", t.name);
                assert!(
                    !t.description.is_empty(),
                    "description empty for {}",
                    t.name
                );
            }
        }
    }

    #[test]
    fn repo_urls_valid() {
        for g in all_groups() {
            for t in g.items {
                let url = t.repo_url();
                assert!(
                    url.starts_with("https://github.com/sw-embed/"),
                    "bad repo url for {}: {}",
                    t.name,
                    url
                );
            }
        }
    }

    #[test]
    fn demo_urls_only_when_has_web_ui() {
        for g in all_groups() {
            for t in g.items {
                if t.has_web_ui {
                    assert!(
                        t.demo_url().is_some(),
                        "has_web_ui but no demo_url for {}",
                        t.name
                    );
                } else {
                    assert!(
                        t.demo_url().is_none(),
                        "no web_ui but has demo_url for {}",
                        t.name
                    );
                }
            }
        }
    }

    #[test]
    fn language_labels() {
        assert_eq!(ToolLanguage::Rust.label(), "Rust");
        assert_eq!(ToolLanguage::C.label(), "C");
        assert_eq!(ToolLanguage::Assembly.label(), "Assembly");
        assert_eq!(ToolLanguage::Mixed("C & Rust").label(), "C & Rust");
        assert_eq!(ToolLanguage::Docs.label(), "Docs");
    }

    #[test]
    fn target_labels() {
        assert_eq!(ToolTarget::Host.label(), "Host");
        assert_eq!(ToolTarget::Cor24.label(), "COR24");
        assert_eq!(ToolTarget::Cor24Emulated.label(), "COR24 (emulated)");
        assert_eq!(ToolTarget::Docs.label(), "Docs");
    }
}
