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

static GROUPS: [ToolGroup; 3] = [
    ToolGroup {
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
                live_url_override: None,
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
                live_url_override: None,
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
                live_url_override: None,
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
                live_url_override: None,
                category: ToolCategory::Foundation,
            },
        ],
    },
    ToolGroup {
        id: "cross-compilers",
        label: "Cross-Compilers",
        description: "Compilers that run on the host and generate COR24 code. Cross-compile C, Rust, \
         and other languages to COR24 assembly or binary.",
        items: &[
            ToolEntry {
                name: "Tiny C Cross-Compiler (tc24r)",
                repo: "sw-cor24-x-tinyc",
                description: "Tiny C cross-compiler targeting COR24. Compiles a subset of C to COR24 \
                 assembly. Runs on the host, outputs .s files for the cross-assembler.",
                language: ToolLanguage::Rust,
                target: ToolTarget::Host,
                has_web_ui: true,
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-tinyc/"),
                category: ToolCategory::CrossCompiler,
            },
            ToolEntry {
                name: "Rust-to-COR24 Pipeline",
                repo: "sw-cor24-rust",
                description: "Experimental Rust-to-COR24 pipeline. Compiles a subset of Rust via \
                 rustc --target msp430-none-elf, then translates MSP430 assembly to COR24 assembly.",
                language: ToolLanguage::Rust,
                target: ToolTarget::Host,
                has_web_ui: true,
                live_url_override: None,
                category: ToolCategory::CrossCompiler,
            },
        ],
    },
    ToolGroup {
        id: "pcode-system",
        label: "P-code System",
        description: "The COR24 P-code virtual machine and its toolchain. Pascal compiles to P-code, \
         which runs on the P-code VM. An AOT compiler can convert P-code to native COR24 binary.",
        items: &[
            ToolEntry {
                name: "P-code VM, Assembler & Linker",
                repo: "sw-cor24-pcode",
                description: "P-code virtual machine written in COR24 assembly, with an assembler and \
                 linker in Rust. Executes .p24 bytecode on COR24 hardware with full stack visualization.",
                language: ToolLanguage::Mixed("Assembly & Rust"),
                target: ToolTarget::Cor24Emulated,
                has_web_ui: true,
                live_url_override: None,
                category: ToolCategory::PCode,
            },
            ToolEntry {
                name: "P-code AOT Compiler (pc-aotc)",
                repo: "sw-cor24-x-pc-aotc",
                description: "Ahead-of-time compiler that converts .p24 P-code bytecode into native COR24 \
                 assembly. Enables running P-code programs at full native speed.",
                language: ToolLanguage::Rust,
                target: ToolTarget::Host,
                has_web_ui: false,
                live_url_override: None,
                category: ToolCategory::PCode,
            },
            ToolEntry {
                name: "Pascal Compiler (p24p)",
                repo: "sw-cor24-pascal",
                description: "Pascal compiler and runtime written in C. Compiles Pascal source to .spc \
                 P-code, which links and assembles for the P-code VM on COR24.",
                language: ToolLanguage::C,
                target: ToolTarget::Cor24,
                has_web_ui: true,
                live_url_override: None,
                category: ToolCategory::PCode,
            },
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_count() {
        assert_eq!(all_groups().len(), 3);
    }

    #[test]
    fn foundation_tools_count() {
        assert_eq!(all_groups()[0].items.len(), 4);
    }

    #[test]
    fn cross_compiler_count() {
        assert_eq!(all_groups()[1].items.len(), 2);
    }

    #[test]
    fn pcode_count() {
        assert_eq!(all_groups()[2].items.len(), 3);
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
