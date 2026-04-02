use yew::prelude::*;

use crate::data::isa::{InstrCategory, IsaSection};

mod addressing_modes;
mod calling_conv;
mod instructions;
mod interrupts;
mod io_table;
mod memory_map;
mod registers;

use addressing_modes::AddressingModesSection;
use calling_conv::CallingConventionsSection;
use instructions::InstructionsSection;
use interrupts::InterruptsSection;
use memory_map::MemoryMapSection;
use registers::RegistersSection;

fn instruction_category_links() -> Html {
    html! {
        <ul class="isa-sidebar-sublist">
            {InstrCategory::all().iter().map(|cat| {
                html! {
                    <li>
                        <a href={format!("#{}", cat.id())} class="isa-sidebar-sublink">
                            {cat.label()}
                        </a>
                    </li>
                }
            }).collect::<Html>()}
        </ul>
    }
}

fn section_link(section: &IsaSection) -> Html {
    let id = section.id();
    let label = section.label();
    let children = if *section == IsaSection::Instructions {
        html! { {instruction_category_links()} }
    } else {
        html! {}
    };
    html! {
        <li>
            <a href={format!("#{}", id)} class="isa-sidebar-link">
                {label}
            </a>
            {children}
        </li>
    }
}

fn sidebar_nav() -> Html {
    html! {
        <nav class="isa-sidebar">
            <h3 class="isa-sidebar-title">{"Sections"}</h3>
            <ul class="isa-sidebar-list">
                {IsaSection::all().iter().map(section_link).collect::<Html>()}
            </ul>
            <div class="isa-sidebar-note">
                <p class="isa-cell-size">{"24-bit word size (3 bytes)"}</p>
                <p class="isa-note">{"Variable-length encoding: 1, 2, or 4 bytes"}</p>
            </div>
        </nav>
    }
}

#[function_component(IsaPage)]
pub fn isa_page() -> Html {
    html! {
        <div class="isa-page page-section">
            <h1 class="isa-page-title">{"COR24 ISA Reference"}</h1>
            <p class="isa-page-subtitle">
                {"Instruction Set Architecture for the COR24 processor. 8 registers, 24-bit words, \
                variable-length instructions."}
            </p>
            <div class="isa-layout">
                {sidebar_nav()}
                <div class="isa-content">
                    <RegistersSection />
                    <InstructionsSection />
                    <MemoryMapSection />
                    <CallingConventionsSection />
                    <AddressingModesSection />
                    <InterruptsSection />
                </div>
            </div>
        </div>
    }
}
