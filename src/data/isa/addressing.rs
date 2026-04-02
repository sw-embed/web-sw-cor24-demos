#[derive(Clone, PartialEq, Debug)]
pub struct AddressingModeInfo {
    pub name: &'static str,
    pub syntax: &'static str,
    pub example: &'static str,
    pub description: &'static str,
}

pub fn all_addressing_modes() -> &'static [AddressingModeInfo; 7] {
    &ADDRESSING_MODES
}

static ADDRESSING_MODES: [AddressingModeInfo; 7] = [
    AddressingModeInfo {
        name: "Register",
        syntax: "Ra, Rb",
        example: "add r0, r1",
        description: "Both operands are registers. Used by most ALU and logical instructions.",
    },
    AddressingModeInfo {
        name: "Immediate 8-bit",
        syntax: "Ra, imm8",
        example: "add r0, 42",
        description: "Signed 8-bit immediate, sign-extended to 24 bits. 2-byte instruction.",
    },
    AddressingModeInfo {
        name: "Immediate 24-bit",
        syntax: "Ra, imm24",
        example: "la r0, 0x123456",
        description: "Full 24-bit immediate. 4-byte instruction. Used by la and sub sp.",
    },
    AddressingModeInfo {
        name: "Base + Displacement",
        syntax: "Ra, disp(Rb)",
        example: "lw r0, 3(fp)",
        description: "Signed 8-bit displacement from base register. Used by load/store. Rb: r0, r1, r2, fp.",
    },
    AddressingModeInfo {
        name: "Register Indirect",
        syntax: "(Ra)",
        example: "jmp (r1)",
        description: "Jump to address in register. Used by jmp and jal.",
    },
    AddressingModeInfo {
        name: "Relative",
        syntax: "disp",
        example: "bra loop",
        description: "Signed 8-bit displacement relative to PC+4. Range: -128 to +127.",
    },
    AddressingModeInfo {
        name: "Implicit Stack",
        syntax: "Ra",
        example: "push r0",
        description: "Implicit sp operand. push: sp -= 3, store. pop: load, sp += 3.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addressing_mode_count() {
        assert_eq!(all_addressing_modes().len(), 7);
    }

    #[test]
    fn all_modes_have_examples() {
        for mode in all_addressing_modes() {
            assert!(!mode.example.is_empty());
            assert!(!mode.syntax.is_empty());
        }
    }
}
