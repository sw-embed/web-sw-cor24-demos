use super::types::*;

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
}
