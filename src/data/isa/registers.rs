#[derive(Clone, PartialEq, Debug)]
pub struct RegisterInfo {
    pub name: &'static str,
    pub number: u8,
    pub purpose: &'static str,
    pub can_load_dest: bool,
    pub can_alu_dest: bool,
    pub can_push_pop: bool,
    pub can_base_reg: bool,
    pub notes: &'static str,
}

pub fn all_registers() -> &'static [RegisterInfo; 7] {
    &REGISTERS
}

static REGISTERS: [RegisterInfo; 7] = [
    RegisterInfo {
        name: "r0",
        number: 0,
        purpose: "General purpose, return value",
        can_load_dest: true,
        can_alu_dest: true,
        can_push_pop: true,
        can_base_reg: true,
        notes: "Work register (W) in Forth calling convention",
    },
    RegisterInfo {
        name: "r1",
        number: 1,
        purpose: "General purpose, link register (jal target)",
        can_load_dest: true,
        can_alu_dest: true,
        can_push_pop: true,
        can_base_reg: true,
        notes: "RSP in Forth calling convention; jal always saves return address here",
    },
    RegisterInfo {
        name: "r2",
        number: 2,
        purpose: "General purpose",
        can_load_dest: true,
        can_alu_dest: true,
        can_push_pop: true,
        can_base_reg: true,
        notes: "IP (instruction pointer) in Forth calling convention",
    },
    RegisterInfo {
        name: "fp",
        number: 3,
        purpose: "Frame pointer for stack-frame locals",
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: true,
        can_base_reg: true,
        notes: "Only base register for EBR stack indexing; cannot be ALU dest",
    },
    RegisterInfo {
        name: "sp",
        number: 4,
        purpose: "Stack pointer (grows downward)",
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: false,
        can_base_reg: false,
        notes: "Init 0xFEEC00; add imm8 target only; not load/store base",
    },
    RegisterInfo {
        name: "iv",
        number: 6,
        purpose: "Interrupt vector (ISR address)",
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: false,
        can_base_reg: false,
        notes: "CPU jumps to this address on interrupt; set via mov iv, r0",
    },
    RegisterInfo {
        name: "ir",
        number: 7,
        purpose: "Interrupt return (saved PC)",
        can_load_dest: false,
        can_alu_dest: false,
        can_push_pop: false,
        can_base_reg: false,
        notes: "CPU saves PC here on interrupt; jmp (ir) returns and clears intis",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_count() {
        assert_eq!(all_registers().len(), 7);
    }

    #[test]
    fn register_numbers_unique() {
        let mut nums: Vec<u8> = all_registers().iter().map(|r| r.number).collect();
        nums.sort();
        let unique: std::collections::HashSet<_> = nums.iter().copied().collect();
        assert_eq!(nums.len(), unique.len(), "duplicate register numbers");
    }

    #[test]
    fn only_gp_registers_are_load_dests() {
        for reg in all_registers().iter() {
            let is_gp = matches!(reg.number, 0..=2);
            assert_eq!(
                reg.can_load_dest, is_gp,
                "load dest mismatch for {} (number {})",
                reg.name, reg.number
            );
        }
    }

    #[test]
    fn only_gp_registers_are_alu_dests() {
        for reg in all_registers().iter() {
            let is_gp = matches!(reg.number, 0..=2);
            assert_eq!(
                reg.can_alu_dest, is_gp,
                "ALU dest mismatch for {} (number {})",
                reg.name, reg.number
            );
        }
    }
}
