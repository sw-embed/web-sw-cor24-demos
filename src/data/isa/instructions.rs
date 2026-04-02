use super::category::InstrCategory;

#[derive(Clone, PartialEq, Debug)]
pub struct InstructionInfo {
    pub mnemonic: &'static str,
    pub operands: &'static str,
    pub size: u8,
    pub category: InstrCategory,
    pub description: &'static str,
}

pub fn all_instructions() -> &'static [InstructionInfo] {
    &INSTRUCTIONS
}

pub fn instructions_by_category(cat: &InstrCategory) -> Vec<&'static InstructionInfo> {
    all_instructions()
        .iter()
        .filter(|i| &i.category == cat)
        .collect()
}

static INSTRUCTIONS: [InstructionInfo; 34] = [
    InstructionInfo {
        mnemonic: "add",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Arithmetic,
        description: "Add registers: Ra = Ra + Rb (24-bit wrapping)",
    },
    InstructionInfo {
        mnemonic: "add",
        operands: "Ra, imm8",
        size: 2,
        category: InstrCategory::Arithmetic,
        description: "Add immediate: Ra = Ra + sign_ext(imm8). Dest: r0, r1, r2, sp",
    },
    InstructionInfo {
        mnemonic: "sub",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Arithmetic,
        description: "Subtract registers: Ra = Ra - Rb (24-bit wrapping)",
    },
    InstructionInfo {
        mnemonic: "sub",
        operands: "sp, imm24",
        size: 4,
        category: InstrCategory::Arithmetic,
        description: "Subtract from SP: sp = sp - imm24 (stack frame allocation)",
    },
    InstructionInfo {
        mnemonic: "mul",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Arithmetic,
        description: "Multiply registers: Ra = Ra * Rb (24-bit wrapping, no overflow)",
    },
    InstructionInfo {
        mnemonic: "and",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Logical,
        description: "Bitwise AND: Ra = Ra & Rb",
    },
    InstructionInfo {
        mnemonic: "or",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Logical,
        description: "Bitwise OR: Ra = Ra | Rb",
    },
    InstructionInfo {
        mnemonic: "xor",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Logical,
        description: "Bitwise XOR: Ra = Ra ^ Rb",
    },
    InstructionInfo {
        mnemonic: "shl",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Shift,
        description: "Shift left logical: Ra = Ra << (Rb & 0x1F), masked to 24 bits",
    },
    InstructionInfo {
        mnemonic: "sra",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Shift,
        description: "Shift right arithmetic: Ra = (signed)Ra >> (Rb & 0x1F), sign preserved",
    },
    InstructionInfo {
        mnemonic: "srl",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Shift,
        description: "Shift right logical: Ra = Ra >> (Rb & 0x1F), zeros from MSB",
    },
    InstructionInfo {
        mnemonic: "ceq",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Comparison,
        description: "Compare equal: c = (Ra == Rb). Use rb=5 to compare with flag",
    },
    InstructionInfo {
        mnemonic: "cls",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Comparison,
        description: "Compare less signed: c = (signed)Ra < (signed)Rb. Use rb=5 for flag",
    },
    InstructionInfo {
        mnemonic: "clu",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Comparison,
        description: "Compare less unsigned: c = Ra < Rb. Use ra=5 to compare flag with reg",
    },
    InstructionInfo {
        mnemonic: "la",
        operands: "Ra, imm24",
        size: 4,
        category: InstrCategory::Load,
        description: "Load address: Ra = imm24 (24-bit LE). Ra=ir encodes jmp imm24",
    },
    InstructionInfo {
        mnemonic: "lc",
        operands: "Ra, imm8",
        size: 2,
        category: InstrCategory::Load,
        description: "Load constant signed: Ra = sign_extend_8(imm8). Dest: r0, r1, r2",
    },
    InstructionInfo {
        mnemonic: "lcu",
        operands: "Ra, imm8",
        size: 2,
        category: InstrCategory::Load,
        description: "Load constant unsigned: Ra = zero_extend_8(imm8). Dest: r0, r1, r2",
    },
    InstructionInfo {
        mnemonic: "lb",
        operands: "Ra, disp(Rb)",
        size: 2,
        category: InstrCategory::Load,
        description: "Load byte signed: Ra = sign_extend_8(Mem[Rb + disp])",
    },
    InstructionInfo {
        mnemonic: "lbu",
        operands: "Ra, disp(Rb)",
        size: 2,
        category: InstrCategory::Load,
        description: "Load byte unsigned: Ra = zero_extend_8(Mem[Rb + disp])",
    },
    InstructionInfo {
        mnemonic: "lw",
        operands: "Ra, disp(Rb)",
        size: 2,
        category: InstrCategory::Load,
        description: "Load word (3 bytes): Ra = Mem[Rb + disp]",
    },
    InstructionInfo {
        mnemonic: "sxt",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Load,
        description: "Sign-extend byte: Ra = sign_extend_8(Rb & 0xFF)",
    },
    InstructionInfo {
        mnemonic: "zxt",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Load,
        description: "Zero-extend byte: Ra = Rb & 0xFF",
    },
    InstructionInfo {
        mnemonic: "sb",
        operands: "Ra, disp(Rb)",
        size: 2,
        category: InstrCategory::Store,
        description: "Store byte: Mem[Rb + disp] = Ra & 0xFF",
    },
    InstructionInfo {
        mnemonic: "sw",
        operands: "Ra, disp(Rb)",
        size: 2,
        category: InstrCategory::Store,
        description: "Store word (3 bytes): Mem[Rb + disp] = Ra",
    },
    InstructionInfo {
        mnemonic: "bra",
        operands: "disp",
        size: 2,
        category: InstrCategory::Branch,
        description: "Branch always: PC = PC+4 + sign_ext(disp). Self-branch halts CPU",
    },
    InstructionInfo {
        mnemonic: "brt",
        operands: "disp",
        size: 2,
        category: InstrCategory::Branch,
        description: "Branch if true: if c then PC = PC+4 + disp, else fall through",
    },
    InstructionInfo {
        mnemonic: "brf",
        operands: "disp",
        size: 2,
        category: InstrCategory::Branch,
        description: "Branch if false: if !c then PC = PC+4 + disp, else fall through",
    },
    InstructionInfo {
        mnemonic: "jal",
        operands: "r1, (Rb)",
        size: 1,
        category: InstrCategory::CallReturn,
        description: "Jump and link: r1 = PC+1, PC = Rb. Target: r0, r1, or r2",
    },
    InstructionInfo {
        mnemonic: "jmp",
        operands: "(Ra)",
        size: 1,
        category: InstrCategory::CallReturn,
        description: "Jump register: PC = Ra. jmp (ir) also clears interrupt-in-service",
    },
    InstructionInfo {
        mnemonic: "push",
        operands: "Ra",
        size: 1,
        category: InstrCategory::Stack,
        description: "Push word: sp -= 3; Mem[sp] = Ra. Pushable: r0, r1, r2, fp",
    },
    InstructionInfo {
        mnemonic: "pop",
        operands: "Ra",
        size: 1,
        category: InstrCategory::Stack,
        description: "Pop word: Ra = Mem[sp]; sp += 3. Poppable: r0, r1, r2, fp",
    },
    InstructionInfo {
        mnemonic: "mov",
        operands: "Ra, Rb",
        size: 1,
        category: InstrCategory::Move,
        description: "Register copy: Ra = Rb. rb=5: Ra = c ? 1 : 0 (condition flag)",
    },
    InstructionInfo {
        mnemonic: "nop",
        operands: "-",
        size: 1,
        category: InstrCategory::Misc,
        description: "No operation. Byte 0xFF, advances PC by 1",
    },
    InstructionInfo {
        mnemonic: "halt",
        operands: "-",
        size: 1,
        category: InstrCategory::Misc,
        description: "Halt CPU. Only byte 0x00 at address 0; otherwise decodes as add r0, r0",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_instruction_count() {
        assert_eq!(all_instructions().len(), 34);
    }

    #[test]
    fn all_instructions_have_valid_size() {
        for instr in all_instructions() {
            assert!(
                matches!(instr.size, 1 | 2 | 4),
                "invalid size {} for {}",
                instr.size,
                instr.mnemonic
            );
        }
    }
}
