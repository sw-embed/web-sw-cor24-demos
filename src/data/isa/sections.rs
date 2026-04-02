#[derive(Clone, PartialEq, Debug)]
pub enum IsaSection {
    Registers,
    Instructions,
    MemoryMap,
    CallingConventions,
    AddressingModes,
    Interrupts,
}

impl IsaSection {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Registers => "registers",
            Self::Instructions => "instructions",
            Self::MemoryMap => "memory-map",
            Self::CallingConventions => "calling-conventions",
            Self::AddressingModes => "addressing-modes",
            Self::Interrupts => "interrupts",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Registers => "Registers",
            Self::Instructions => "Instructions",
            Self::MemoryMap => "Memory Map",
            Self::CallingConventions => "Calling Conventions",
            Self::AddressingModes => "Addressing Modes",
            Self::Interrupts => "Interrupts",
        }
    }

    pub fn all() -> &'static [IsaSection; 6] {
        &[
            Self::Registers,
            Self::Instructions,
            Self::MemoryMap,
            Self::CallingConventions,
            Self::AddressingModes,
            Self::Interrupts,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isa_section_ids_unique() {
        let ids: Vec<_> = IsaSection::all().iter().map(|s| s.id()).collect();
        let unique: std::collections::HashSet<_> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len(), "duplicate section IDs found");
    }

    #[test]
    fn isa_section_labels_unique() {
        let labels: Vec<_> = IsaSection::all().iter().map(|s| s.label()).collect();
        let unique: std::collections::HashSet<_> = labels.iter().copied().collect();
        assert_eq!(labels.len(), unique.len(), "duplicate section labels found");
    }
}
