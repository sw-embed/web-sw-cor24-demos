use yew::prelude::*;

use crate::data::isa::{InstrCategory, IsaSection};

use instructions::InstructionsSection;
use registers::RegistersSection;

mod instructions;
mod registers;

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
                    <section id="memory-map" class="isa-section">
                        <h2 class="section-heading">{"Memory Map"}</h2>
                        <div class="placeholder">
                            <p>{"Memory map documentation coming soon."}</p>
                        </div>
                    </section>
                    <section id="calling-conventions" class="isa-section">
                        <h2 class="section-heading">{"Calling Conventions"}</h2>
                        <div class="placeholder">
                            <p>{"Calling conventions documentation coming soon."}</p>
                        </div>
                    </section>
                    <section id="addressing-modes" class="isa-section">
                        <h2 class="section-heading">{"Addressing Modes"}</h2>
                        <div class="placeholder">
                            <p>{"Addressing modes documentation coming soon."}</p>
                        </div>
                    </section>
                    <section id="interrupts" class="isa-section">
                        <h2 class="section-heading">{"Interrupts"}</h2>
                        <div class="placeholder">
                            <p>{"Interrupt handling documentation coming soon."}</p>
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}
