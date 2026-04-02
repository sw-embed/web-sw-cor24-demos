use yew::prelude::*;

fn interrupt_sequence() -> Html {
    html! {
        <div class="isa-details">
            <h3>{"Interrupt Sequence"}</h3>
            <ol class="isa-steps">
                <li>
                    <strong>{"Trigger:"}</strong>
                    {"UART RX receives a byte while interrupt enable bit 0 is set and no interrupt is in service."}
                </li>
                <li>
                    <strong>{"Acknowledge:"}</strong>
                    {"CPU automatically executes "}
                    <code>{"jal r7, (r6)"}</code>
                    {" \u{2014} saves PC to r7 (ir) and jumps to the address in r6 (iv)."}
                </li>
                <li>
                    <strong>{"Service:"}</strong>
                    {"ISR reads the received byte from 0xFF0100 (auto-acknowledges RX ready)."}
                </li>
                <li>
                    <strong>{"Return:"}</strong>
                    {"ISR executes "}
                    <code>{"jmp (r7)"}</code>
                    {" which clears the intis flag and returns to the interrupted code."}
                </li>
            </ol>
        </div>
    }
}

fn interrupt_register_table() -> Html {
    html! {
        <div class="isa-table-wrap">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>{"Register"}</th>
                        <th>{"Alias"}</th>
                        <th>{"Purpose"}</th>
                        <th>{"Notes"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>{"iv"}</code></td>
                        <td><code>{"r6"}</code></td>
                        <td>{"Interrupt vector address"}</td>
                        <td>{"Set with "}
                            <code>{"mov iv, r0"}</code>
                            {". CPU jumps here on interrupt."}
                        </td>
                    </tr>
                    <tr>
                        <td><code>{"ir"}</code></td>
                        <td><code>{"r7"}</code></td>
                        <td>{"Interrupt return (saved PC)"}</td>
                        <td>{"CPU saves PC here automatically. "}
                            <code>{"jmp (ir)"}</code>
                            {" returns from ISR."}
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[function_component(InterruptsSection)]
pub fn interrupts_section() -> Html {
    html! {
        <section id="interrupts" class="isa-section">
            <h2 class="section-heading">{"Interrupts"}</h2>
            <p class="isa-intro">
                {"COR24 supports a single UART RX interrupt. The interrupt vector register (iv) holds \
                the ISR address, and the interrupt return register (ir) automatically saves the \
                interrupted PC. Nested interrupts are prevented by the intis (interrupt-in-service) flag."}
            </p>
            <h3 class="isa-subheading">{"Interrupt Registers"}</h3>
            {interrupt_register_table()}
            {interrupt_sequence()}
            <div class="isa-details">
                <h3>{"Key Constraints"}</h3>
                <ul class="isa-detail-list">
                    <li>
                        <strong>{"No nesting:"}</strong>
                        {"The intis flag prevents a second interrupt from firing while one is being serviced. \
                         It is cleared only by "}
                        <code>{"jmp (ir)"}</code>
                        {"."}
                    </li>
                    <li>
                        <strong>{"Single source:"}</strong>
                        {"Only UART RX can trigger interrupts. Enable via "}
                        <code>{"IO_INTENABLE (0xFF0010)"}</code>
                        {" bit 0."}
                    </li>
                    <li>
                        <strong>{"Register use:"}</strong>
                        {"iv and ir are hardware-managed. iv is set by "}
                        <code>{"mov iv, r0"}</code>
                        {". ir is written automatically by the interrupt mechanism and read by "}
                        <code>{"jmp (ir)"}</code>
                        {"."}
                    </li>
                    <li>
                        <strong>{"ISR must save:"}</strong>
                        {"The ISR should push any registers it uses (r0, r1, r2) and restore them before \
                         returning."}
                    </li>
                </ul>
            </div>
        </section>
    }
}
