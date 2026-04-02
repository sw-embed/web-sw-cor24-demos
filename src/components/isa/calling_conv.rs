use yew::prelude::*;

use crate::data::isa::{calling_convention_rules, stack_frame_layout};

fn frame_row(entry: &crate::data::isa::StackFrameEntry) -> Html {
    let is_sp = entry.offset == "sp";
    let is_fp = entry.offset == "fp";
    let row_class = if is_fp {
        "stack-frame-row stack-frame-fp"
    } else if is_sp {
        "stack-frame-row stack-frame-sp"
    } else {
        "stack-frame-row"
    };
    html! {
        <tr class={row_class}>
            <td class="stack-offset"><code>{entry.offset}</code></td>
            <td class="stack-contents"><code>{entry.contents}</code></td>
            <td class="stack-desc">{entry.description}</td>
        </tr>
    }
}

fn stack_frame_table() -> Html {
    let layout = stack_frame_layout();
    html! {
        <div class="isa-table-wrap">
            <table class="data-table stack-frame-table">
                <thead>
                    <tr>
                        <th>{"Offset"}</th>
                        <th>{"Contents"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {layout.iter().map(frame_row).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}

fn convention_row(rule: &crate::data::isa::CallingConventionRule) -> Html {
    html! {
        <tr>
            <td class="conv-topic"><strong>{rule.topic}</strong></td>
            <td class="conv-detail">{rule.detail}</td>
        </tr>
    }
}

fn convention_table() -> Html {
    let rules = calling_convention_rules();
    html! {
        <div class="isa-table-wrap">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>{"Rule"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {rules.iter().map(convention_row).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}

fn prologue_example() -> Html {
    html! {
        <div class="isa-details">
            <h3>{"Prologue / Epilogue Example"}</h3>
            <div class="code-example">
                <pre><code>{"; Prologue (callee entry)
push fp        ; save caller's frame pointer
mov fp, sp     ; establish new frame
sub sp, 6      ; allocate 2 locals (2 x 3 bytes)

; ... function body ...

; Epilogue (callee return)
mov sp, fp     ; deallocate locals
pop fp         ; restore caller's frame pointer
jmp (r1)       ; return to caller (jal saved addr in r1)"}</code></pre>
            </div>
        </div>
    }
}

#[function_component(CallingConventionsSection)]
pub fn calling_conventions_section() -> Html {
    html! {
        <section id="calling-conventions" class="isa-section">
            <h2 class="section-heading">{"Calling Conventions"}</h2>
            <p class="isa-intro">
                {"COR24 uses a stack-based calling convention. Arguments are passed on the stack, \
                return values come back in r0, and the frame pointer (fp) provides stable access to \
                locals and arguments. The link register (r1) is implicitly written by jal."}
            </p>
            <h3 class="isa-subheading">{"Stack Frame Layout"}</h3>
            <p class="isa-intro">
                {"The stack grows downward. Each push/pop moves the stack pointer by 3 bytes (one 24-bit word)."}
            </p>
            {stack_frame_table()}
            {convention_table()}
            {prologue_example()}
        </section>
    }
}
