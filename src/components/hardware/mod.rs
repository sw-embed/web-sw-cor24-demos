use yew::prelude::*;

struct MakerLispLink {
    label: &'static str,
    url: &'static str,
    description: &'static str,
}

fn makerlisp_links() -> Vec<MakerLispLink> {
    vec![
        MakerLispLink {
            label: "COR24 Soft CPU",
            url: "https://www.makerlisp.com/cor24-soft-cpu-for-fpga",
            description: "The COR24 CPU architecture -- 24-bit RISC designed for FPGA embedded systems. \
                Runs at 101 MHz on Lattice MachXO2 FPGAs.",
        },
        MakerLispLink {
            label: "COR24 Dev Board",
            url: "https://www.makerlisp.com/cor24-test-board",
            description: "The COR24-TB development board with S2 button, D2 LED, UART connectors, \
                GPIO for SPI/I2C, and power/reset controls.",
        },
        MakerLispLink {
            label: "Technical Information",
            url: "https://www.makerlisp.com/download",
            description: "Datasheets, Verilog source, pinouts, and technical documentation for the \
                COR24 CPU and development board.",
        },
        MakerLispLink {
            label: "News",
            url: "https://www.makerlisp.com/news",
            description: "Latest updates from MakerLisp on COR24 development, new features, and \
                community announcements.",
        },
        MakerLispLink {
            label: "Contact",
            url: "https://www.makerlisp.com/contact-us",
            description: "Get in touch with MakerLisp about COR24 hardware, custom designs, or \
                collaboration opportunities.",
        },
    ]
}

fn link_card(link: &MakerLispLink) -> Html {
    html! {
        <a href={link.url} target="_blank" rel="noopener noreferrer" class="hw-card">
            <h3 class="hw-card-title">{link.label}</h3>
            <p class="hw-card-desc">{link.description}</p>
            <span class="hw-card-url">{link.url}</span>
        </a>
    }
}

fn spec_table() -> Html {
    html! {
        <div class="isa-table-wrap">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>{"Feature"}</th>
                        <th>{"Specification"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr><td>{"Word size"}</td><td>{"24 bits (3 bytes)"}</td></tr>
                    <tr><td>{"Address space"}</td><td>{"16 MB (24-bit addresses)"}</td></tr>
                    <tr><td>{"Registers"}</td><td>{"8 x 24-bit (3 GP, 5 special)"}</td></tr>
                    <tr><td>{"Instruction sizes"}</td><td>{"1, 2, or 4 bytes (variable-length)"}</td></tr>
                    <tr><td>{"Clock speed"}</td><td>{"101 MHz"}</td></tr>
                    <tr><td>{"FPGA"}</td><td>{"Lattice MachXO2 (LFE5U-25F)"}</td></tr>
                    <tr><td>{"SRAM"}</td><td>{"1 MB on-board"}</td></tr>
                    <tr><td>{"EBR"}</td><td>{"3 KB embedded block RAM (stack)"}</td></tr>
                    <tr><td>{"I/O"}</td><td>{"UART, GPIO (SPI/I2C), button, LED"}</td></tr>
                    <tr><td>{"Interrupts"}</td><td>{"UART RX interrupt, auto-entry via jal (iv)"}</td></tr>
                    <tr><td>{"Condition flags"}</td><td>{"Single C flag (set by compare, tested by branch)"}</td></tr>
                </tbody>
            </table>
        </div>
    }
}

#[function_component(HardwarePage)]
pub fn hardware_page() -> Html {
    let links = makerlisp_links();
    html! {
        <div class="page-section hw-page">
            <h1 class="section-heading">{"Hardware"}</h1>
            <p class="isa-intro">
                {"COR24 is a 24-bit RISC soft CPU designed by "}
                <a href="https://www.makerlisp.com" target="_blank" rel="noopener noreferrer">{"MakerLisp"}</a>
                {" for FPGA-based embedded systems. The CPU runs on a Lattice MachXO2 FPGA at 101 MHz, \
                with 1 MB SRAM and a development board exposing UART, GPIO, and user I/O."}
            </p>

            <section class="hw-section">
                <h2>{"Specifications"}</h2>
                {spec_table()}
            </section>

            <section class="hw-section">
                <h2>{"MakerLisp.com"}</h2>
                <p class="isa-intro">
                    {"Detailed hardware documentation, Verilog source, development board photos, \
                    and technical resources are available at MakerLisp.com."}
                </p>
                <div class="hw-grid">
                    {links.iter().map(link_card).collect::<Html>()}
                </div>
            </section>
        </div>
    }
}

mod tests {
    #[test]
    fn makerlisp_links_valid() {
        let links = super::makerlisp_links();
        assert_eq!(links.len(), 5);
        for link in &links {
            assert!(link.url.starts_with("https://www.makerlisp.com/"));
            assert!(!link.label.is_empty());
            assert!(!link.description.is_empty());
        }
    }
}
