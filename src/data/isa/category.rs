#[derive(Clone, PartialEq, Debug)]
pub enum InstrCategory {
    Arithmetic,
    Logical,
    Shift,
    Comparison,
    Load,
    Store,
    Branch,
    CallReturn,
    Stack,
    Move,
    Misc,
}

impl InstrCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Arithmetic => "Arithmetic",
            Self::Logical => "Logical",
            Self::Shift => "Shift",
            Self::Comparison => "Comparison",
            Self::Load => "Load",
            Self::Store => "Store",
            Self::Branch => "Branch",
            Self::CallReturn => "Call / Return",
            Self::Stack => "Stack",
            Self::Move => "Move",
            Self::Misc => "Miscellaneous",
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::Arithmetic => "cat-arithmetic",
            Self::Logical => "cat-logical",
            Self::Shift => "cat-shift",
            Self::Comparison => "cat-comparison",
            Self::Load => "cat-load",
            Self::Store => "cat-store",
            Self::Branch => "cat-branch",
            Self::CallReturn => "cat-call-return",
            Self::Stack => "cat-stack",
            Self::Move => "cat-move",
            Self::Misc => "cat-misc",
        }
    }

    pub fn all() -> &'static [InstrCategory; 11] {
        &[
            Self::Arithmetic,
            Self::Logical,
            Self::Shift,
            Self::Comparison,
            Self::Load,
            Self::Store,
            Self::Branch,
            Self::CallReturn,
            Self::Stack,
            Self::Move,
            Self::Misc,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories_have_unique_ids() {
        let ids: Vec<_> = InstrCategory::all().iter().map(|c| c.id()).collect();
        let unique: std::collections::HashSet<_> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len());
    }

    #[test]
    fn categories_have_unique_labels() {
        let labels: Vec<_> = InstrCategory::all().iter().map(|c| c.label()).collect();
        let unique: std::collections::HashSet<_> = labels.iter().copied().collect();
        assert_eq!(labels.len(), unique.len());
    }
}
