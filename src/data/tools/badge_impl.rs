use super::types::{ToolLanguage, ToolTarget};

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

#[cfg(test)]
mod tests {
    use super::*;

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
