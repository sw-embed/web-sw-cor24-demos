#[derive(Clone, PartialEq, Debug)]
pub struct RegisterInfo {
    pub name: &'static str,
    pub purpose: &'static str,
    pub is_gp: bool,
    pub can_load_dest: bool,
    pub can_alu_dest: bool,
    pub can_push_pop: bool,
    pub can_base_reg: bool,
    pub notes: &'static str,
}

pub fn all_registers() -> &'static [RegisterInfo; 9] {
    &REGISTERS
}

static REGISTERS: [RegisterInfo; 9] = [
    RegisterInfo {
        name: "r0a",
        purpose: "DEBUG TEST ROW A",
        is_gp: true,
        can_load_dest: true,
        can_alu_dest: true,
        can_push_pop: true,
        can_base_reg: true,
        notes: "This row proves the first row renders",
    },
    RegisterInfo {
        name: "r0",
        purpose: "General purpose, return value",
        is_gp: true,
        can_load_dest: true,
        can_alu_dest: true,
        can_push_pop: true,
        can_base_reg: true,
        notes: "Register number 0 in opcodes; work register (W) in Forth",
    },
    RegisterInfo {
        name: "r1",
        purpose: "General purpose, link register (jal target)",
        is_gp: true,
        can_load_dest: true,
        can_alu_dest: true,
        can_push_pop: true,
        can_base_reg: true,
        notes: "Register number 1 in opcodes; jal always saves return address here",
    },
    RegisterInfo {
        name: "r2",
        purpose: "General purpose",
        is_gp: true,
        can_load_dest: true,
        can_alu_dest: true,
        can_push_pop: true,
        can_base_reg: true,
        notes: "Register number 2 in opcodes; IP (instruction pointer) in Forth",
    },
    RegisterInfo {
        name: "fp",
        purpose: "Frame pointer for stack-frame locals",
        is_gp: false,
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: true,
        can_base_reg: true,
        notes: "Only base register for EBR stack indexing; cannot be ALU dest or read by mov",
    },
    RegisterInfo {
        name: "sp",
        purpose: "Stack pointer (grows downward)",
        is_gp: false,
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: false,
        can_base_reg: false,
        notes: "Init 0xFEEC00; add imm8 target only; not load/store base; cannot be pushed",
    },
    RegisterInfo {
        name: "z",
        purpose: "Constant zero (not a register)",
        is_gp: false,
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: false,
        can_base_reg: false,
        notes: "Hardwired zero; used as source operand (e.g. ceq r0, z); cannot be written",
    },
    RegisterInfo {
        name: "iv",
        purpose: "Interrupt vector (ISR address)",
        is_gp: false,
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: false,
        can_base_reg: false,
        notes: "CPU jumps to this address on interrupt; set via mov iv, r0",
    },
    RegisterInfo {
        name: "ir",
        purpose: "Interrupt return (saved PC)",
        is_gp: false,
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: false,
        can_base_reg: false,
        notes: "CPU saves PC here on interrupt; jmp (ir) returns and clears interrupt",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_count() {
        assert_eq!(all_registers().len(), 9);
    }

    #[test]
    fn only_gp_registers_are_load_dests() {
        for reg in all_registers().iter() {
            assert_eq!(
                reg.can_load_dest, reg.is_gp,
                "load dest mismatch for {}",
                reg.name
            );
        }
    }

    #[test]
    fn only_gp_registers_are_alu_dests() {
        for reg in all_registers().iter() {
            assert_eq!(
                reg.can_alu_dest, reg.is_gp,
                "ALU dest mismatch for {}",
                reg.name
            );
        }
    }
}
