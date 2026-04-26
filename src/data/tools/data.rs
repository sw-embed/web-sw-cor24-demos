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

static GROUPS: [ToolGroup; 5] = [
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
            ToolEntry {
                name: "High-Level Assembler (HLASM)",
                repo: "sw-cor24-hlasm",
                description: "High-level assembler for COR24 providing structured macros, symbolic expressions, \
                 and higher-level constructs. Written in COR24 assembly, runs natively on the target platform.",
                language: ToolLanguage::Assembly,
                target: ToolTarget::Cor24,
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
        ],
    },
    ToolGroup {
        id: "cross-compilers",
        label: "Cross-Compilers",
        description: "Compilers that run on the host and generate COR24 code. Cross-compile C, Rust, \
         and other languages to COR24 assembly or binary.",
        items: &[
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
        ],
    },
    ToolGroup {
        id: "pcode-system",
        label: "P-code System",
        description: "The COR24 P-code virtual machine and its toolchain. Pascal compiles to P-code, \
         which runs on the P-code VM. An AOT compiler can convert P-code to native COR24 binary.",
        items: &[
            ToolEntry {
                name: "OCaml Compiler",
                repo: "sw-cor24-ocaml",
                description: "OCaml compiler targeting the COR24 P-code VM. Implemented in Pascal, compiles \
                 a subset of OCaml to P-code bytecode, reusing the Pascal P-code pipeline and VM infrastructure.",
                language: ToolLanguage::Mixed("Pascal"),
                target: ToolTarget::Cor24,
                has_web_ui: false,
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
    ToolGroup {
        id: "native-languages",
        label: "Native Languages",
        description: "Programming languages and interpreters that run directly on COR24 hardware. \
         Written in C or COR24 assembly, these are the software that the COR24 platform was built to host.",
        items: &[
            ToolEntry {
                name: "APL Interpreter (apl-sw)",
                repo: "sw-cor24-apl",
                description: "APL interpreter with integer-only arithmetic, rank <= 2 arrays, and \
                 ASCII surface syntax (Latin keywords). Supports live array operations.",
                language: ToolLanguage::C,
                target: ToolTarget::Cor24,
                has_web_ui: true,
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-apl/"),
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "BASIC Interpreter",
                repo: "sw-cor24-basic",
                description: "BASIC interpreter for COR24. Supports classic BASIC constructs: line \
                 numbers, GOTO/GOSUB, PRINT, INPUT, FOR/NEXT loops, and string variables.",
                language: ToolLanguage::C,
                target: ToolTarget::Cor24,
                has_web_ui: true,
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-basic/"),
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "Forth IDE",
                repo: "sw-cor24-forth",
                description: "Direct-threaded code (DTC) Forth interpreter with interactive IDE. \
                 Clean-room implementation written in COR24 assembly with dictionary browsing and stack inspection.",
                language: ToolLanguage::Assembly,
                target: ToolTarget::Cor24,
                has_web_ui: true,
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-forth/"),
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "Fortran Compiler",
                repo: "sw-cor24-fortran",
                description: "Fortran compiler targeting COR24. Compiler written in SNOBOL4, runtime in \
                 PL/SW. Translates Fortran source to COR24 assembly for scientific and numeric computation.",
                language: ToolLanguage::C,
                target: ToolTarget::Cor24,
                has_web_ui: false,
                live_url_override: None,
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "PL/SW Compiler",
                repo: "sw-cor24-plsw",
                description: "PL/I-inspired systems programming language compiler. Supports structured \
                 programming, record types, and low-level hardware access on COR24.",
                language: ToolLanguage::C,
                target: ToolTarget::Cor24,
                has_web_ui: true,
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-plsw/"),
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "Prolog Interpreter",
                repo: "sw-cor24-prolog",
                description: "Prolog interpreter with a WAM-like 8+8 register virtual machine implemented \
                 in PL/SW. Provides logic programming with unification and backtracking on COR24.",
                language: ToolLanguage::Mixed("PL/SW"),
                target: ToolTarget::Cor24,
                has_web_ui: false,
                live_url_override: None,
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "RPG-II",
                repo: "sw-cor24-rpg-ii",
                description: "Simplified RPG-II report generator compiled through HLASM. Targets business \
                 data processing and report generation on COR24.",
                language: ToolLanguage::Assembly,
                target: ToolTarget::Cor24,
                has_web_ui: false,
                live_url_override: None,
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "Smalltalk",
                repo: "sw-cor24-smalltalk",
                description: "Smalltalk environment implemented in COR24 BASIC. Object-oriented \
                 messaging on top of the BASIC + P-code stack on COR24.",
                language: ToolLanguage::Mixed("BASIC"),
                target: ToolTarget::Cor24,
                has_web_ui: true,
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-smalltalk/"),
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "SNOBOL4 Interpreter",
                repo: "sw-cor24-snobol4",
                description: "SNOBOL4 pattern-matching language interpreter implemented in PL/SW. \
                 Provides powerful string and pattern manipulation capabilities on COR24.",
                language: ToolLanguage::Mixed("PL/SW"),
                target: ToolTarget::Cor24,
                has_web_ui: true,
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-snobol4/"),
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "SWS Scripting Language",
                repo: "sw-cor24-script",
                description: "Tcl-like scripting language for shell and editor automation on COR24. \
                 Provides string processing, control flow, and integration with the monitor.",
                language: ToolLanguage::C,
                target: ToolTarget::Cor24,
                has_web_ui: false,
                live_url_override: None,
                category: ToolCategory::NativeLanguage,
            },
            ToolEntry {
                name: "Tiny Macro Lisp",
                repo: "sw-cor24-macrolisp",
                description: "Lisp-1 interpreter with lexical scoping, defmacro, closures, and garbage \
                 collection. A self-contained Lisp environment running on COR24 hardware.",
                language: ToolLanguage::C,
                target: ToolTarget::Cor24,
                has_web_ui: true,
                live_url_override: Some("https://sw-embed.github.io/web-sw-cor24-macrolisp/"),
                category: ToolCategory::NativeLanguage,
            },
        ],
    },
    ToolGroup {
        id: "system-software",
        label: "System Software",
        description: "Low-level system software for the COR24 platform: the resident monitor that boots \
         at address zero, and the planned source-level debugger.",
        items: &[
            ToolEntry {
                name: "Resident Monitor",
                repo: "sw-cor24-monitor",
                description: "Boots at address 0 and provides program invocation, I/O services, and the \
                 system ABI. Written in COR24 assembly with C extensions for complex services.",
                language: ToolLanguage::Mixed("Assembly + C"),
                target: ToolTarget::Cor24,
                has_web_ui: false,
                live_url_override: None,
                category: ToolCategory::SystemSoftware,
            },
            ToolEntry {
                name: "Source-Level Debugger",
                repo: "sw-cor24-debugger",
                description: "Planned source-level debugger for COR24. Will support stepping through \
                 assembly and high-level source, with register and memory inspection.",
                language: ToolLanguage::Mixed("Assembly + C"),
                target: ToolTarget::Cor24,
                has_web_ui: false,
                live_url_override: None,
                category: ToolCategory::SystemSoftware,
            },
            ToolEntry {
                name: "yocto-ed",
                repo: "sw-cor24-yocto-ed",
                description: "Minimal modal text editor with gap buffer, 3-line display, edit/command modes. \
                 Dogfooding project for the tc24r toolchain (C -> COR24 assembly -> emulator).",
                language: ToolLanguage::C,
                target: ToolTarget::Cor24,
                has_web_ui: false,
                live_url_override: None,
                category: ToolCategory::SystemSoftware,
            },
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts() {
        let g = all_groups();
        assert_eq!(g.len(), 5);
        assert_eq!(g[0].items.len(), 5);
        assert_eq!(g[1].items.len(), 2);
        assert_eq!(g[2].items.len(), 4);
        assert_eq!(g[3].items.len(), 11);
        assert_eq!(g[4].items.len(), 3);
        assert_eq!(all_tools().len(), 25);
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
    fn items_alphabetical_within_groups() {
        for g in all_groups() {
            let names: Vec<&str> = g.items.iter().map(|t| t.name).collect();
            let mut sorted = names.clone();
            sorted.sort_by_key(|a| a.to_lowercase());
            assert_eq!(
                names, sorted,
                "tools in group '{}' are not alphabetical",
                g.label
            );
        }
    }
}
