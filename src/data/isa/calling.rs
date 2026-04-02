#[derive(Clone, PartialEq, Debug)]
pub struct StackFrameEntry {
    pub offset: &'static str,
    pub contents: &'static str,
    pub description: &'static str,
}

pub fn stack_frame_layout() -> &'static [StackFrameEntry; 7] {
    &STACK_FRAME
}

static STACK_FRAME: [StackFrameEntry; 7] = [
    StackFrameEntry {
        offset: "fp+6",
        contents: "arg2",
        description: "Second argument (word above saved fp)",
    },
    StackFrameEntry {
        offset: "fp+3",
        contents: "arg1",
        description: "First argument (word above saved fp)",
    },
    StackFrameEntry {
        offset: "fp",
        contents: "saved fp",
        description: "Caller's frame pointer, pushed by callee",
    },
    StackFrameEntry {
        offset: "fp-3",
        contents: "return addr",
        description: "Saved in r1 by jal; callee pushes r1 to stack",
    },
    StackFrameEntry {
        offset: "fp-6",
        contents: "local 1",
        description: "First local variable",
    },
    StackFrameEntry {
        offset: "fp-9",
        contents: "local 2",
        description: "Second local variable",
    },
    StackFrameEntry {
        offset: "sp",
        contents: "...",
        description: "Stack pointer (grows downward)",
    },
];

#[derive(Clone, PartialEq, Debug)]
pub struct CallingConventionRule {
    pub topic: &'static str,
    pub detail: &'static str,
}

pub fn calling_convention_rules() -> &'static [CallingConventionRule; 7] {
    &RULES
}

static RULES: [CallingConventionRule; 7] = [
    CallingConventionRule {
        topic: "Argument passing",
        detail: "Arguments are pushed onto the stack right-to-left before the call. The callee accesses them at fp+3, fp+6, etc.",
    },
    CallingConventionRule {
        topic: "Return value",
        detail: "Return value is placed in r0 before returning.",
    },
    CallingConventionRule {
        topic: "Link register",
        detail: "jal always saves the return address (PC+1) into r1. The callee typically pushes r1 to the stack alongside fp.",
    },
    CallingConventionRule {
        topic: "Frame pointer",
        detail: "fp (r3) points to the saved frame pointer on the stack. Locals are at negative offsets from fp.",
    },
    CallingConventionRule {
        topic: "Stack frame setup",
        detail: "push fp / mov fp, sp / sub sp, N. Restore: mov sp, fp / pop fp.",
    },
    CallingConventionRule {
        topic: "Caller-saved",
        detail: "r0, r1, r2. The callee may freely use these. Caller must save if needed across calls.",
    },
    CallingConventionRule {
        topic: "Callee-saved",
        detail: "fp (r3). The callee must preserve fp by pushing it on entry and popping on return.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_frame_entry_count() {
        assert_eq!(stack_frame_layout().len(), 7);
    }

    #[test]
    fn calling_convention_rule_count() {
        assert_eq!(calling_convention_rules().len(), 7);
    }
}
