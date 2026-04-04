use yew::prelude::*;

use crate::data::isa::{all_regions, MemoryType};

use super::io_table::{hex, io_table};

fn type_badge(t: &MemoryType) -> Html {
    let label = t.label();
    let class = match t {
        MemoryType::Ram => "mem-type-ram",
        MemoryType::Stack => "mem-type-stack",
        MemoryType::Unmapped => "mem-type-unmapped",
        MemoryType::Io => "mem-type-io",
    };
    html! { <span class={format!("mem-type-badge {}", class)}>{label}</span> }
}

fn region_row(r: &crate::data::isa::MemoryRegion) -> Html {
    html! {
        <tr>
            <td><code>{r.name}</code></td>
            <td class="mem-addr">{hex(r.start)}</td>
            <td class="mem-addr">{hex(r.end)}</td>
            <td class="mem-size">{r.size}</td>
            <td>{type_badge(&r.region_type)}</td>
            <td class="mem-desc">{r.description}</td>
        </tr>
    }
}

fn memory_table() -> Html {
    let regions = all_regions();
    html! {
        <div class="isa-table-wrap">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>{"Region"}</th>
                        <th>{"Start"}</th>
                        <th>{"End"}</th>
                        <th>{"Size"}</th>
                        <th>{"Type"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {regions.iter().map(region_row).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}

fn visual_map() -> Html {
    html! {
        <div class="mem-visual">
            <div class="mem-bar mem-bar-ram">
                <span class="mem-bar-label">{"SRAM"}</span>
                <span class="mem-bar-range">{"0x000000 \u{2013} 0x0FFFFF (1 MB)"}</span>
            </div>
            <div class="mem-bar mem-bar-unmapped">
                <span class="mem-bar-label">{"Unmapped"}</span>
                <span class="mem-bar-range">{"0x100000 \u{2013} 0xFEDFFF"}</span>
            </div>
            <div class="mem-bar mem-bar-stack">
                <span class="mem-bar-label">{"EBR Stack"}</span>
                <span class="mem-bar-range">{"0xFEE000 \u{2013} 0xFEFFFF (8 KB)"}</span>
            </div>
            <div class="mem-bar mem-bar-io">
                <span class="mem-bar-label">{"I/O"}</span>
                <span class="mem-bar-range">{"0xFF0000 \u{2013} 0xFFFFFF"}</span>
            </div>
            <div class="mem-bar-legend">
                <span class="mem-legend-item mem-bar-ram"><span>{"RAM"}</span></span>
                <span class="mem-legend-item mem-bar-stack"><span>{"Stack"}</span></span>
                <span class="mem-legend-item mem-bar-io"><span>{"I/O"}</span></span>
                <span class="mem-legend-item mem-bar-unmapped"><span>{"Unmapped"}</span></span>
            </div>
        </div>
    }
}

fn key_details() -> Html {
    html! {
        <div class="isa-details">
            <h3>{"Key Details"}</h3>
            <ul class="isa-detail-list">
                <li>
                    <strong>{"Reset vector:"}</strong>
                    {"CPU starts execution at address 0x000000. A 0x00 byte at address 0 halts."}
                </li>
                <li>
                    <strong>{"Stack pointer:"}</strong>
                    {"Initialized to 0xFEEC00 (top of EBR populated area). Grows downward, 3 bytes per push."}
                </li>
                <li>
                    <strong>{"Word size:"}</strong>
                    {"3 bytes (24-bit). All lw/sw operations read/write 3-byte words."}
                </li>
                <li>
                    <strong>{"UART TX timing:"}</strong>
                    {"Transmit takes 10 instruction cycles per character. Writing while TX busy drops the character."}
                </li>
            </ul>
        </div>
    }
}

#[function_component(MemoryMapSection)]
pub fn memory_map_section() -> Html {
    html! {
        <section id="memory-map" class="isa-section">
            <h2 class="section-heading">{"Memory Map"}</h2>
            <p class="isa-intro">
                {"COR24 has a 24-bit address space (16 MB). The lower 1 MB is SRAM for code and data. \
                The EBR region near the top of the address space provides the hardware stack (3 KB on "}
                <a href="https://www.latticesemi.com/en/Products/FPGAandCPLD/MachXO" target="_blank" rel="noopener noreferrer">{"MachXO"}</a>
                {", 8 KB window). I/O registers are memory-mapped in the last 64 KB."}
            </p>
            {visual_map()}
            {memory_table()}
            <h3 class="isa-subheading">{"I/O Registers"}</h3>
            <p class="isa-intro">
                {"All I/O is memory-mapped. The UART provides serial communication; the LED/switch \
                register controls the on-board LED D2 and reads button S2."}
            </p>
            {io_table()}
            {key_details()}
        </section>
    }
}
