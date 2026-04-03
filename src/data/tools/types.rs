#[derive(Clone, PartialEq, Debug)]
pub struct ToolEntry {
    pub name: &'static str,
    pub repo: &'static str,
    pub description: &'static str,
    pub language: ToolLanguage,
    pub target: ToolTarget,
    pub has_web_ui: bool,
    pub live_url_override: Option<&'static str>,
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
