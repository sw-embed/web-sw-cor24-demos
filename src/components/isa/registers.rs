use yew::prelude::*;

use crate::data::isa;

fn constraint_mark(ok: bool) -> Html {
    if ok {
        html! { <span class="constraint-yes">{"\u{2713}"}</span> }
    } else {
        html! { <span class="constraint-no">{"\u{2014}"}</span> }
    }
}

fn register_row(r: &isa::RegisterInfo) -> Html {
    let name_html = if r.is_gp {
        html! { <code>{r.name}</code> }
    } else {
        html! { <code class="reg-special">{r.name}</code> }
    };
    html! {
        <div class="reg-grid-row">
            <span class="reg-grid-name">{name_html}</span>
            <span class="reg-grid-purpose">{r.purpose}</span>
            <span class="reg-grid-check">{constraint_mark(r.can_load_dest)}</span>
            <span class="reg-grid-check">{constraint_mark(r.can_alu_dest)}</span>
            <span class="reg-grid-check">{constraint_mark(r.can_push_pop)}</span>
            <span class="reg-grid-check">{constraint_mark(r.can_base_reg)}</span>
            <span class="reg-grid-notes">{r.notes}</span>
        </div>
    }
}

fn register_table() -> Html {
    let regs = isa::all_registers();
    html! {
        <div class="reg-grid-wrap">
            <div class="reg-grid-header">
                <span class="reg-grid-name">{"Register"}</span>
                <span class="reg-grid-purpose">{"Purpose"}</span>
                <span class="reg-grid-check">{"Load"}</span>
                <span class="reg-grid-check">{"ALU"}</span>
                <span class="reg-grid-check">{"Push"}</span>
                <span class="reg-grid-check">{"Base"}</span>
                <span class="reg-grid-notes">{"Notes"}</span>
            </div>
            {regs.iter().map(register_row).collect::<Html>()}
        </div>
    }
}

fn constraint_legend() -> Html {
    html! {
        <div class="isa-legend">
            <span class="legend-item">
                <span class="constraint-yes">{"\u{2713}"}</span>
                <span>{"= allowed"}</span>
            </span>
            <span class="legend-item">
                <span class="constraint-no">{"\u{2014}"}</span>
                <span>{"= not available"}</span>
            </span>
        </div>
    }
}

fn constraints_details() -> Html {
    html! {
        <div class="isa-details">
            <h3>{"Key Constraints"}</h3>
            <ul class="isa-detail-list">
                <li><strong>{"Load dest (lb/lbu/lw/la/lc):"}</strong>{" r0, r1, r2 only."}</li>
                <li><strong>{"ALU dest (add/sub/mul/and/or/xor/shifts):"}</strong>{" r0, r1, r2 only."}</li>
                <li><strong>{"add immediate:"}</strong>{" r0, r1, r2, sp (sp is the only special case)."}</li>
                <li><strong>{"Push/pop:"}</strong>{" r0, r1, r2, fp. sp, iv, ir, z cannot be pushed."}</li>
                <li><strong>{"Base register (load/store):"}</strong>{" r0, r1, r2, fp. sp NOT valid."}</li>
                <li>
                    <strong>{"Jump targets (jmp):"}</strong>
                    {" r0, r1, r2, ir. "}
                    <code>{"jmp (ir)"}</code>
                    {" returns from interrupt."}
                </li>
                <li><strong>{"jal:"}</strong>{" Always saves to r1. Target: r0, r1, or r2."}</li>
                <li>
                    <strong>{"mov:"}</strong>
                    {" Cannot read fp. "}
                    <code>{"mov fp, sp"}</code>
                    {" allowed. "}
                    <code>{"mov iv, r0"}</code>
                    {" sets interrupt vector."}
                </li>
            </ul>
        </div>
    }
}

fn zero_and_flag_section() -> Html {
    html! {
        <div class="isa-details">
            <h3>{"Zero Constant (z)"}</h3>
            <p class="isa-detail-text">
                {"The symbol "}
                <code>{"z"}</code>
                {" represents a hardwired constant zero. When used as a source operand \
                 (e.g., in compare instructions like "}
                <code>{"ceq r0, z"}</code>
                {"), it provides the value 0. It is not a register \u{2014} there is no \
                 physical register that can be read or written."}
            </p>
            <h3>{"Condition Flag (c)"}</h3>
            <p class="isa-detail-text">
                {"The symbol "}
                <code>{"c"}</code>
                {" represents a single condition flag. It is set by compare instructions \
                 "}
                <code>{"ceq"}</code>
                {", "}
                <code>{"cls"}</code>
                {", and "}
                <code>{"clu"}</code>
                {". The condition is tested by branch instructions "}
                <code>{"brt"}</code>
                {" (branch if true) and "}
                <code>{"brf"}</code>
                {" (branch if false). The condition can also be read as a 0/1 value \
                 with "}
                <code>{"mov r0, c"}</code>
                {". The flag is not a register and is unrelated to "}
                <code>{"z"}</code>
                {"."}
            </p>
        </div>
    }
}

#[function_component(RegistersSection)]
pub fn registers_section() -> Html {
    html! {
        <section id="registers" class="isa-section">
            <h2 class="section-heading">{"Registers"}</h2>
            <p class="isa-intro">
                {"COR24 has 7 registers and 2 special symbols. Only r0, r1, and r2 are \
                 general-purpose (register numbers 0, 1, 2 in opcodes). All other registers \
                 are referenced only by name: fp (frame pointer), sp (stack pointer), \
                 iv (interrupt vector), ir (interrupt return). The symbol z provides a \
                 constant zero and c provides a condition flag \u{2014} neither is a register."}
            </p>
            {register_table()}
            {constraint_legend()}
            {constraints_details()}
            {zero_and_flag_section()}
        </section>
    }
}
