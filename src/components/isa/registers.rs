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
    html! {
        <tr>
            <td><code>{r.name}</code></td>
            <td class="reg-number">{r.number}</td>
            <td>{r.purpose}</td>
            <td class="constraint-cell">{constraint_mark(r.can_load_dest)}</td>
            <td class="constraint-cell">{constraint_mark(r.can_alu_dest)}</td>
            <td class="constraint-cell">{constraint_mark(r.can_push_pop)}</td>
            <td class="constraint-cell">{constraint_mark(r.can_base_reg)}</td>
            <td class="notes-cell">{r.notes}</td>
        </tr>
    }
}

fn register_table() -> Html {
    let regs = isa::all_registers();
    html! {
        <div class="isa-table-wrap">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>{"Register"}</th>
                        <th>{"#"}</th>
                        <th>{"Purpose"}</th>
                        <th class="constraint-col">{"Load"}</th>
                        <th class="constraint-col">{"ALU"}</th>
                        <th class="constraint-col">{"Push"}</th>
                        <th class="constraint-col">{"Base"}</th>
                        <th>{"Notes"}</th>
                    </tr>
                </thead>
                <tbody>
                    {regs.iter().map(register_row).collect::<Html>()}
                </tbody>
            </table>
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
                <li><strong>{"Push/pop:"}</strong>{" r0, r1, r2, fp. sp, iv, ir cannot be pushed."}</li>
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
                physical register r5 that can be read or written."}
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
                {"COR24 has 7 registers. Only r0, r1, and r2 are general-purpose: \
                they are the only registers that can be load or ALU destinations. The frame pointer \
                (fp) serves as the base register for stack-relative load/store. The stack pointer \
                (sp) can only be adjusted via add immediate. Registers iv and ir handle interrupts. \
                Additionally, the symbols z and c provide a constant zero and a condition flag, \
                respectively \u{2014} neither is a register."}
            </p>
            {register_table()}
            {constraint_legend()}
            {constraints_details()}
            {zero_and_flag_section()}
        </section>
    }
}
