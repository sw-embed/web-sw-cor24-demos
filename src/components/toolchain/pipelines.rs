use yew::prelude::*;

pub struct PipelineStep {
    pub label: &'static str,
    pub file: &'static str,
}

pub fn render_pipeline(title: &'static str, steps: &[PipelineStep]) -> Html {
    html! {
        <div class="pipeline">
            <h4 class="pipeline-title">{title}</h4>
            <div class="pipeline-flow">
                {steps.iter().enumerate().map(|(i, step)| {
                    let step_html = html! {
                        <div class="pipeline-step">
                            <span class="pipeline-file">{step.file}</span>
                            <span class="pipeline-label">{step.label}</span>
                        </div>
                    };
                    let arrow_html = if i < steps.len() - 1 {
                        html! { <span class="pipeline-arrow">{"\u{2192}"}</span> }
                    } else {
                        html! {}
                    };
                    html! {
                        <>
                            {step_html}
                            {arrow_html}
                        </>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}

static C_PIPELINE: [PipelineStep; 5] = [
    PipelineStep {
        label: "C source",
        file: ".c",
    },
    PipelineStep {
        label: "tc24r",
        file: "cross-compile",
    },
    PipelineStep {
        label: "as24",
        file: ".s \u{2192} .bin",
    },
    PipelineStep {
        label: "cor24-run",
        file: "emulator",
    },
    PipelineStep {
        label: "execute",
        file: "COR24 binary",
    },
];

static PASCAL_PIPELINE: [PipelineStep; 6] = [
    PipelineStep {
        label: "Pascal source",
        file: ".pas",
    },
    PipelineStep {
        label: "p24p",
        file: "compile",
    },
    PipelineStep {
        label: "pl24r",
        file: ".spc \u{2192} .p24",
    },
    PipelineStep {
        label: "pa24r",
        file: "assemble",
    },
    PipelineStep {
        label: "pvm",
        file: "P-code VM",
    },
    PipelineStep {
        label: "execute",
        file: "on COR24",
    },
];

static AOT_PIPELINE: [PipelineStep; 5] = [
    PipelineStep {
        label: "P-code",
        file: ".p24",
    },
    PipelineStep {
        label: "pc-aotc",
        file: "AOT compile",
    },
    PipelineStep {
        label: "as24",
        file: ".s \u{2192} .bin",
    },
    PipelineStep {
        label: "cor24-run",
        file: "emulator",
    },
    PipelineStep {
        label: "execute",
        file: "native binary",
    },
];

static RUST_PIPELINE: [PipelineStep; 5] = [
    PipelineStep {
        label: "Rust source",
        file: ".rs",
    },
    PipelineStep {
        label: "rustc",
        file: "MSP430 asm",
    },
    PipelineStep {
        label: "translate",
        file: "\u{2192} COR24 asm",
    },
    PipelineStep {
        label: "as24",
        file: ".s \u{2192} .bin",
    },
    PipelineStep {
        label: "execute",
        file: "COR24 binary",
    },
];

pub fn render_all_pipelines() -> Html {
    html! {
        <>
            {render_pipeline("C Pipeline", &C_PIPELINE)}
            {render_pipeline("Pascal Pipeline", &PASCAL_PIPELINE)}
            {render_pipeline("P-code AOT Pipeline", &AOT_PIPELINE)}
            {render_pipeline("Rust Pipeline", &RUST_PIPELINE)}
        </>
    }
}
