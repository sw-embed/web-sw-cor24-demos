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
            <td><code>{r.alias}</code></td>
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
                        <th>{"Alias"}</th>
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
                <li><strong>{"Push/pop:"}</strong>{" r0, r1, r2, fp. sp, z, iv, ir cannot be pushed."}</li>
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
                <li>
                    <strong>{"z / c:"}</strong>
                    {" Hardwired zero. "}
                    <code>{"mov r0, c"}</code>
                    {" reads condition flag (set by ceq/cls/clu)."}
                </li>
            </ul>
        </div>
    }
}

#[function_component(RegistersSection)]
pub fn registers_section() -> Html {
    html! {
        <section id="registers" class="isa-section">
            <h2 class="section-heading">{"Registers"}</h2>
            <p class="isa-intro">
                {"COR24 has 8 registers (r0\u{2013}r7). Only r0, r1, and r2 are general-purpose: \
                they are the only registers that can be load or ALU destinations. The frame pointer \
                (fp) serves as the base register for stack-relative load/store. The stack pointer \
                (sp) can only be adjusted via add immediate. Registers z, iv, and ir have special \
                hardware functions."}
            </p>
            {register_table()}
            {constraint_legend()}
            {constraints_details()}
        </section>
    }
}
