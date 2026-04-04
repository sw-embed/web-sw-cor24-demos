use yew::prelude::*;

pub fn render_tc24r_constraints() -> Html {
    html! {
        <section class="toolchain-section">
            <h2 class="toolchain-section-title">{"tc24r C Subset Constraints"}</h2>
            <p class="toolchain-section-desc">
                {"The Tiny C cross-compiler (tc24r) targets a restricted subset of C due to the \
                 COR24 architecture's 24-bit word size and simplified memory model."}
            </p>
            <div class="tc24r-constraints">
                <ul>
                    <li>{"No structs or unions (flat scalar types only)"}</li>
                    <li>{"No malloc/free or heap allocation"}</li>
                    <li>{"No standard C library (string.h, stdio.h, etc.)"}</li>
                    <li>{"24-bit int (no 32-bit or 64-bit types)"}</li>
                    <li>{"Single translation unit per compilation"}</li>
                    <li>{"Function pointers supported, no function recursion limit"}</li>
                </ul>
            </div>
        </section>
    }
}

pub fn render_pcode_vm() -> Html {
    html! {
        <section class="toolchain-section">
            <h2 class="toolchain-section-title">{"P-Code VM Architecture"}</h2>
            <p class="toolchain-section-desc">
                {"A stack-based virtual machine written in COR24 assembly. Pascal and BASIC compile \
                 to P-code bytecode and run on this VM."}
            </p>
            <div class="impl-detail">
                <h4>{"Dual-Stack Design"}</h4>
                <p>{"Separate eval stack (expression temporaries) and call stack (activation records), \
                 both growing upward. This keeps frame data safe from eval stack corruption and \
                 simplifies debugging."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"VM State"}</h4>
                <p>{"VM State: stored in a memory struct (not COR24 registers -- too few registers available): \
                 pc, esp (eval stack pointer), csp (call stack pointer), fp (frame pointer), gp \
                 (globals base), hp (heap pointer), and status flags. \
                 COR24 registers: r0=work, r1=VM state pointer, r2=scratch."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Call Frame Layout"}</h4>
                <p>{"[return PC | dynamic link | static link | procedure ID | args... | locals...] \
                 with 3-byte slots. Static links enable lexical scoping -- nested procedures access \
                 outer variables by walking the static link chain."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Instruction Set"}</h4>
                <p>{"~40+ opcodes, variable-length byte encoding (1-5 bytes). Categories: stack ops \
                 (push/dup/drop/swap), arithmetic (add/sub/mul/div/mod/neg), comparison (eq/ne/lt/le/gt/ge), \
                 control flow (jmp/jz/jnz/call/ret/halt/trap), variable access (load/store locals, \
                 globals, args, nonlocals via static link chain), and memory (load/store/byte/memcpy/memset). \
                  System calls via sys instruction (PUTC, GETC, LED, ALLOC, FREE, READ_SWITCH)."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Binary Format"}</h4>
                <p>{"<code>{\".p24\"}</code> files have an 18-byte header (magic P24\\0, version, entry point, \
                 code/data sizes, global count -- all 3-byte LE fields matching COR24 word size) \
                 followed by code bytes then data bytes."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Toolchain"}</h4>
                <p>{"pa24r (Rust): two-pass assembler, .spc to .p24. \
                 pl24r (Rust): text-level linker, merges .spc modules with \
                 .module/.export/.extern directives. pasm.s (COR24 asm): on-target \
                 assembler. Memory: call stack 4KB, eval stack 2KB, heap gets remaining SRAM (~900KB)."}</p>
            </div>
        </section>
    }
}

pub fn render_lisp_gc() -> Html {
    html! {
        <section class="toolchain-section">
            <h2 class="toolchain-section-title">{"Macro Lisp Implementation"}</h2>
            <p class="toolchain-section-desc">
                {"Lisp-1 interpreter with lexical scoping, defmacro, closures, and mark-sweep \
                 garbage collection. Written in C, cross-compiled with tc24r."}
            </p>
            <div class="impl-detail">
                <h4>{"Tagged Values (24-bit)"}</h4>
                <p>{"Single 24-bit word, 2 low bits = tag: 00 = fixnum (22-bit \
                 signed, range +/-2M), 01 = cons pointer, 10 = \
                 symbol pointer, 11 = other (closure, string, primitive, macro). \
                 NIL and T are interned symbols compared by pointer equality."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Heap and Cells"}</h4>
                <p>{"Fixed-size arena of Value cells. Cons cell = 6 bytes \
                 (two 24-bit tagged values: car + cdr). Closure = 9 bytes (params + body + env). \
                 Heap grows upward from BSS end; 64K cells = 384KB. Allocation is bump-style \
                 with mark-sweep fallback."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Garbage Collector (Mark-Sweep)"}</h4>
                <p>{"Roots: global environment, evaluation stack (temp roots), symbol table, \
                 reader/printer temporaries. Mark phase: recursive traversal from roots, setting \
                 mark bit on each reachable object. Sweep phase: linear scan of heap, free unmarked \
                 objects to free list. Triggered when allocation fails (heap exhausted). If still \
                 insufficient after GC, out-of-memory error."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Environment and Scoping"}</h4>
                <p>{"Lexical scope via chained environment frames: (symbol . value) \
                 pairs linked to parent frames. Lookup walks the chain outward. Tail-call optimization \
                 on if, begin, and closure application."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Special Forms"}</h4>
                <p>{"Special forms: quote, if, define, set!, lambda, defmacro, \
                 begin, quasiquote. Unhygienic defmacro with \
                 single-pass expand-then-eval."}</p>
            </div>
        </section>
    }
}

pub fn render_forth_dtc() -> Html {
    html! {
        <section class="toolchain-section">
            <h2 class="toolchain-section-title">{"Forth DTC Implementation"}</h2>
            <p class="toolchain-section-desc">
                {"Direct-threaded code Forth written in COR24 assembly. Clean-room implementation, \
                 not ported from any existing Forth."}
            </p>
            <div class="impl-detail">
                <h4>{"Threading Model"}</h4>
                <p>{"Each compiled word contains the address of the next word to execute (code field \
                 points to the next definition's code field). DTC was chosen for COR24's limited \
                 register set -- minimal indirection per word call."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Register Allocation"}</h4>
                <p>{"Four dedicated COR24 registers: r0 = W (working register), \
                 r1 = RSP (return stack pointer), r2 = IP \
                 (instruction pointer), sp = DSP (data stack pointer). \
                 Cell size is 3 bytes (native COR24 word)."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Self-Extending Architecture"}</h4>
                <p>{"The assembly kernel (~2600 lines) provides the core interpreter loop and \
                 primitives. Higher-level words are defined in Forth itself using : and ;. \
                 Built-in words cover arithmetic (+, -, *, AND, OR, XOR), \
                 stack manipulation (DUP, DROP, SWAP, OVER), memory (@, !, C@, C!), control \
                 (EXIT, EXECUTE), compiler (CREATE, WORD, FIND, IMMEDIATE), and I/O (EMIT, KEY, ., CR). \
                 The system also supports hex/decimal base switching, depth inspection (.S), and \
                 word listing (WORDS)."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"I/O"}</h4>
                <p>{"UART at address 0xFF0100 (memory-mapped). LED output via \
                 dedicated Forth word LED!. Switch input via system word."}</p>
            </div>
        </section>
    }
}

pub fn render_web_ui() -> Html {
    html! {
        <section class="toolchain-section">
            <h2 class="toolchain-section-title">{"Web UI Architecture"}</h2>
            <p class="toolchain-section-desc">
                {"All browser-based COR24 tools share a common architecture: the full emulator and \
                 assembler compile to WebAssembly, embedded in a Yew 0.21 single-page application."}
            </p>
            <div class="impl-detail">
                <h4>{"WASM Bridge"}</h4>
                 <p>{"Each web UI defines a WasmCpu struct with #[wasm_bindgen] \
                  that wraps EmulatorCore from the emulator crate. This is the single \
                  bridge between the Rust emulator and the browser. The full CPU emulator and assembler \
                  compile into the WASM binary via path dependencies on cor24-emulator \
                  and cor24-assembler."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Execution Control"}</h4>
                 <p>{"Three modes: step() (single instruction), run_batch() \
                 (animated execution via requestAnimationFrame), run() (run to halt, \
                 100K instruction limit). State inspection: registers, PC, memory, UART output, LEDs, \
                 switches, current instruction disassembly, and sparse SRAM/EBR views."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"I/O Simulation"}</h4>
                <p>{"UART TX busy cycles set to 0 (instant TX in WASM -- no busy-wait). Switch/button \
                  simulation via set_switches() / toggle_switch(). \
                 UART input via uart_send_char()."}</p>
            </div>
            <div class="impl-detail">
                <h4>{"Build"}</h4>
                 <p>{"crate-type = [\"cdylib\", \"rlib\"]. Release profile: \
                  opt-level = \"z\", lto = true for minimal WASM \
                  size. Trunk for WASM packaging and dev server. Catppuccin Mocha dark theme across \
                  all UIs for visual consistency."}</p>
            </div>
        </section>
    }
}
