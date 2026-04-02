use yew::prelude::*;

use crate::data::isa::{InstrCategory, instructions_by_category};

fn size_badge(size: u8) -> Html {
    let label = match size {
        1 => "1B",
        2 => "2B",
        4 => "4B",
        _ => "?",
    };
    html! { <span class={format!("instr-size instr-size-{}", label)}>{label}</span> }
}

fn instruction_row(instr: &crate::data::isa::InstructionInfo) -> Html {
    html! {
        <tr>
            <td class="instr-mnemonic"><code>{instr.mnemonic}</code></td>
            <td class="instr-operands"><code>{instr.operands}</code></td>
            <td class="instr-size-cell">{size_badge(instr.size)}</td>
            <td class="instr-desc">{instr.description}</td>
        </tr>
    }
}

fn category_table(cat: &InstrCategory) -> Html {
    let instrs = instructions_by_category(cat);
    html! {
        <div class="isa-table-wrap">
            <table class="data-table instr-table">
                <thead>
                    <tr>
                        <th>{"Mnemonic"}</th>
                        <th>{"Operands"}</th>
                        <th class="instr-size-col">{"Size"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {instrs.iter().map(|i| instruction_row(i)).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}

fn category_block(cat: &InstrCategory) -> Html {
    let instrs = instructions_by_category(cat);
    let count = instrs.len();
    html! {
        <details class="instr-category" open=true>
            <summary class="instr-category-summary">
                <span class="instr-category-name">{cat.label()}</span>
                <span class="instr-category-count">{count}{" instruction"}{if count != 1 { "s" } else { "" }}</span>
            </summary>
            {category_table(cat)}
        </details>
    }
}

fn encoding_format_section() -> Html {
    html! {
        <div class="isa-details encoding-notes">
            <h3>{"Encoding Formats"}</h3>
            <div class="encoding-grid">
                <div class="encoding-card">
                    <h4>{"1-byte instruction"}</h4>
                    <p>{"opcode + ra + rb packed into a single byte via hardware decode ROM (PLA). \
                       211 of 256 byte values map to valid instructions."}</p>
                </div>
                <div class="encoding-card">
                    <h4>{"2-byte instruction"}</h4>
                    <p>{"Byte 0: opcode + ra + rb (decoded). \
                       Byte 1: signed 8-bit immediate/displacement, sign-extended to 24 bits."}</p>
                </div>
                <div class="encoding-card">
                    <h4>{"4-byte instruction"}</h4>
                    <p>{"Byte 0: opcode + ra + rb (decoded). \
                       Bytes 1\u{2013}3: 24-bit little-endian immediate/address."}</p>
                </div>
            </div>
            <ul class="isa-detail-list">
                <li>
                    <strong>{"Branch offset:"}</strong>
                    {"target = (instruction_address + 4) + sign_extend_8(byte1). \
                     Range: \u{2212}128 to +127."}
                </li>
                <li>
                    <strong>{"Word size:"}</strong>
                    {"3 bytes (24-bit architecture). push/pop adjust sp by 3; lw/sw read/write 3 bytes."}
                </li>
                <li>
                    <strong>{"Not all register combos encodable:"}</strong>
                    {"the decode ROM only maps valid (opcode, ra, rb) combinations."}
                </li>
                <li>
                    <strong>{"Special bytes:"}</strong>
                    {"0xFF = nop; 0x00 at address 0 = halt."}
                </li>
            </ul>
        </div>
    }
}

#[function_component(InstructionsSection)]
pub fn instructions_section() -> Html {
    html! {
        <section id="instructions" class="isa-section">
            <h2 class="section-heading">{"Instruction Set"}</h2>
            <p class="isa-intro">
                {"COR24 has 34 instruction forms across 11 categories, using variable-length encoding \
                (1, 2, or 4 bytes). Only r0, r1, and r2 can be ALU or load destinations. \
                The hardware decode ROM maps 211 valid byte values out of 256 possible."}
            </p>
            {encoding_format_section()}
            <div class="instr-categories">
                {InstrCategory::all().iter().map(category_block).collect::<Html>()}
            </div>
        </section>
    }
}
