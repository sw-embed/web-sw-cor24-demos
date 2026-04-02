use yew::prelude::*;

use crate::data::isa::all_io_registers;

pub(super) fn hex(addr: u32) -> String {
    format!("0x{:06X}", addr)
}

pub(super) fn io_table() -> Html {
    let regs = all_io_registers();
    html! {
        <div class="isa-table-wrap">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>{"Register"}</th>
                        <th>{"Address"}</th>
                        <th>{"Size"}</th>
                        <th>{"R/W"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {regs.iter().map(io_row).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}

fn io_row(r: &crate::data::isa::IoRegister) -> Html {
    html! {
        <tr>
            <td><code>{r.name}</code></td>
            <td class="mem-addr">{hex(r.address)}</td>
            <td class="mem-size">{r.size}{"B"}</td>
            <td class="mem-rw">{r.read_write}</td>
            <td class="mem-desc">{r.description}</td>
        </tr>
    }
}
