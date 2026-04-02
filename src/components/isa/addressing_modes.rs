use yew::prelude::*;

use crate::data::isa::all_addressing_modes;

fn mode_row(m: &crate::data::isa::AddressingModeInfo) -> Html {
    html! {
        <tr>
            <td class="am-name"><strong>{m.name}</strong></td>
            <td class="am-syntax"><code>{m.syntax}</code></td>
            <td class="am-example"><code>{m.example}</code></td>
            <td class="am-desc">{m.description}</td>
        </tr>
    }
}

fn addressing_table() -> Html {
    let modes = all_addressing_modes();
    html! {
        <div class="isa-table-wrap">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>{"Mode"}</th>
                        <th>{"Syntax"}</th>
                        <th>{"Example"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {modes.iter().map(mode_row).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}

#[function_component(AddressingModesSection)]
pub fn addressing_modes_section() -> Html {
    html! {
        <section id="addressing-modes" class="isa-section">
            <h2 class="section-heading">{"Addressing Modes"}</h2>
            <p class="isa-intro">
                {"COR24 supports seven addressing modes, determined by the instruction encoding. \
                Load/store instructions use base + displacement with signed 8-bit offsets. Branch \
                instructions use PC-relative addressing with signed 8-bit displacements."}
            </p>
            {addressing_table()}
            <div class="isa-details">
                <h3>{"Notes"}</h3>
                <ul class="isa-detail-list">
                    <li>
                        <strong>{"Base register constraint:"}</strong>
                        {"Load/store displacement addressing only allows r0, r1, r2, or fp as the base register. \
                         sp cannot be used as a base register."}
                    </li>
                    <li>
                        <strong>{"Displacement range:"}</strong>
                        {"Signed 8-bit displacement, sign-extended to 24 bits. Range: -128 to +127 bytes."}
                    </li>
                    <li>
                        <strong>{"Branch target:"}</strong>
                        {"Relative to PC+4 (not PC), due to the hardware pipeline. \
                         A self-branch (branch to itself) is detected as a halt."}
                    </li>
                    <li>
                        <strong>{"Absolute addressing:"}</strong>
                        {"The 4-byte la instruction loads a full 24-bit address. \
                         When used with r7 (ir), it encodes an absolute jump."}
                    </li>
                </ul>
            </div>
        </section>
    }
}
