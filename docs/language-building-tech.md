# Language-Building Tech Across sw-embed's sw-cor24-*

A survey and cross-comparison of the compiler, interpreter, VM, assembler,
emulator, debugger, and web-frontend implementations targeting the COR24 ISA.
Written after reading through ~40 repos in the `sw-embed` GitHub org; concrete
claims anchor to file paths where relevant.

Repositories are identified by their short name (e.g. `sw-cor24-apl` for the
APL interpreter, `web-sw-cor24-apl` for its browser UI). Worker clones live
under `/disk1/github/softwarewrighter/devgroup/work/d<xx>/github/sw-embed/`.

---

## Contents

1. [The COR24 ISA](#1-the-cor24-isa)
2. [Project goals and design philosophy](#2-project-goals-and-design-philosophy)
3. [Development process](#3-development-process)
4. [Language groups](#4-language-groups)
5. [Tooling: precursor to an embedded OS](#5-tooling-precursor-to-an-embedded-os)
6. [Bootstrapping and porting strategy](#6-bootstrapping-and-porting-strategy)
7. [Lexing and tokenizing](#7-lexing-and-tokenizing)
8. [Parsing](#8-parsing)
9. [ASTs and intermediate representations](#9-asts-and-intermediate-representations)
10. [Lowering and codegen](#10-lowering-and-codegen)
11. [The P-code VM as a substrate](#11-the-p-code-vm-as-a-substrate)
12. [Memory allocation](#12-memory-allocation)
13. [Register strategy for a tight register file](#13-register-strategy-for-a-tight-register-file)
14. [Garbage collection](#14-garbage-collection)
15. [Memory-mapped I/O](#15-memory-mapped-io)
16. [Integer-only arithmetic](#16-integer-only-arithmetic)
17. [Software divide](#17-software-divide)
18. [Stack sizes, EBR vs SRAM](#18-stack-sizes-ebr-vs-sram)
19. [Compiler / interpreter pipeline](#19-compiler--interpreter-pipeline)
20. [Linker and loader](#20-linker-and-loader)
21. [Emulator and debugger](#21-emulator-and-debugger)
22. [Memory loading strategies](#22-memory-loading-strategies)
23. [Web frontends](#23-web-frontends-web-sw-cor24-)
24. [Status summary](#24-status-summary)
25. [Design commitments](#25-design-commitments)

---

## 1. The COR24 ISA

COR24 is a 24-bit integer-only RISC designed for practical use. Its
characteristics — named-register set limited to `r0`–`r2` plus
`fp`/`sp`/`iv`/`ir`, no floating-point, no hardware divide — shape
how language implementations that target it are built. This section
records the facts about the ISA as relevant to that.

### 1.1 Registers

The assembler recognizes **seven symbolic register names**:

- **`r0`, `r1`, `r2`** — general-purpose registers, 24 bits. Also used
  for argument-passing and return-value slots per the calling
  convention (§1.4).
- **`fp`** — frame pointer.
- **`sp`** — stack pointer.
- **`iv`** — interrupt-vector register (address of the current ISR).
- **`ir`** — interrupt-return register.

Machine-code encodings reference registers by number, and the opcode
format has bit-room for `r3..r7`; those slots are the underlying
hardware positions of `fp`, `sp`, `iv`, `ir` (and one unused slot).
The **assembler does not accept `r3..r7` as symbolic names**; that
restriction is intentional (per project design).

There is also a hardware **zero constant `z`**, used by comparison
and some arithmetic forms. `z` is **not a register**: it has no
storage, it has no bits, and it can never be the destination of a
write.

**Condition flag `C`** is set by the `ceq` / `cls` / `clu` compare
instructions and consumed by `brt` / `brf`. There are no separate
overflow, sign, or carry flags.

### 1.2 Memory

- **16 MB address space** (24-bit addresses; words are 24 bits = 3 bytes).
- **1 MB SRAM** at `0x000000–0x0FFFFF` — the working-memory workhorse. Heap,
  stack growth room, program images land here.
- **3 KB EBR** (embedded block RAM) mapped in an 8 KB window at
  `0xFEE000–0xFEEC00`. Fast path for tight inner loops, data stack in Forth,
  small hot caches. Addresses beyond the populated 3 KB return zero.
- **Unmapped region** `0x100000–0xFDFFFF` reads as zero — no fault. This is
  a safety property several interpreters rely on.
- **MMIO** at `0xFF0000` and above:
  - `0xFF0000` — LED `D2` (write, single bit) and switch `S2` (read,
    single bit) share this address. The line has a pull-up (~5 V);
    **writing `0` lights the LED**, and **reading returns `0` while
    the switch is depressed**. A canonical smoke-test loop copies
    `S2` to `D2` — the LED follows the switch. The LED bit is
    write-only (you cannot interrogate the LED's own state); the
    switch bit is read-only.
  - `0xFF0100` UART data register (RX/TX)
  - `0xFF0101` UART status (bit 7 TX busy, bit 0 RX ready, bit 2 RX
    overflow)
  - `0xFF0010` interrupt enable (bit 0 = UART RX)

### 1.3 Instruction encoding

Three variable-length formats, selected by opcode:

- **1 byte** — register-only operations (e.g. `mov r1, r2`).
- **2 bytes** — register plus 8-bit immediate or branch offset.
- **4 bytes** — register plus 24-bit absolute address (loads/stores, long
  branches, `jal`).

The hardware decode ROM takes the first byte (opcode + two register fields)
and yields a 12-bit internal decoded tuple (see
`cor24-rs/docs/isa-reference.md:129`). **32 opcodes**, **211 concrete
instruction forms** after accounting for all register-pair and immediate
variants.

Notable opcodes: `Add{Reg,Imm}`, `Sub`, `SubSp` (stack-adjust),
`And/Or/Xor`, `Shl/Sra/Srl`, `Mul` (low-24-bit result only),
`Ceq/Cls/Clu` (compare), `Bra/Brt/Brf`, `Jmp/Jal`, `La/Lb/Lbu/Lc/Lcu/Lw`,
`Sb/Sw`, `Push/Pop`, `Sxt/Zxt`. The encoding `0xFF` is `nop`; this
was added after the original encoding tables, so some older docs may
predate it.

### 1.4 Calling convention

COR24 does not mandate a calling convention at the ISA level — it's
up to each tool / language impl. The widely-shared convention is the
one produced by `tc24r` for C cross-compilation; other producers
(PL/SW, native `.s` tools, Forth) may use their own. Canonical
source: `cor24-rs/docs/research/` (example C sources with their
generated `.s` output); individual repos for their local ABI
choices.

Words are 24 bits (3 bytes). Exact push/pop semantics, frame layout,
and caller/callee save rules: **look to the source.** I have not
verified a single common convention across every project.

### 1.5 Interrupts

The interrupt mechanism uses `iv` (interrupt vector) and `ir`
(interrupt return): a hardware interrupt invokes the ISR via
`jal ir, (iv)` and the ISR returns with `jmp (ir)`.

Today, UART-RX-ready is the only interrupt source. Ongoing work adds
bit-banged I²C and SPI peripherals; how interrupts will be used for
those is not yet decided.

There is **no software `syscall` / `trap` instruction**. A software
trap could, in principle, be faked by a debugger or tool that
replaces an opcode at a known PC with a branch into its own handler
and restores the opcode on return — but nothing in the ecosystem
does this today.

### 1.6 Execution oddities

- **No hardware divide.** Every language that needs division ships
  or uses a software routine (typically repeated subtraction or
  shift-subtract).
- **No floating-point.** All numerics are integer. Fixed-point or
  arbitrary-precision libraries are planned but not yet implemented.
- **Halting a program.** The conventional halt is a one-instruction
  self-loop, typically labelled `halt: bra halt`. On the emulator,
  this is detected and execution stops; on the FPGA, the CPU simply
  spins on that instruction until reset. The observable effect is
  the same: the program stops making progress. (Note: COR24 has not
  yet been tested on the FPGA; the emulator is the current source of
  truth for runtime behaviour.)

---

## 2. Project goals and design philosophy

The sw-embed org exists to **build a complete language stack against
COR24** from the ISA up, and to **use the tools it produces to build
the next tools**. COR24 itself is the author's (mike's) proving
ground for tools and techniques meant to be ported to *other*
FPGA-based systems — the motivation is to run tools and applications
on emulated historical and modern ISAs (IBM 1130, 360, 390, RCA
1802, RISC-V I32, and others). COR24 is where the techniques are
worked out; the other ISAs are where they'll be applied.

Four threads run through the work.

### 2.1 Dogfooding

Every production language eventually compiles itself, or at least its
runtime. The Pascal runtime (`sw-cor24-pascal`) is written in Pascal from
phase 1. PL/SW's compiler (`sw-cor24-plsw`) is already self-hosting. The
P-code assembler has two implementations — `pasm.s` written in COR24 asm and
`pa24r` written in Rust — that must stay byte-identical, and the native one
is the definition of done.

### 2.2 Historically-inspired layering

Implementations consciously pick the period-accurate abstract machine:

| Language      | Abstract machine / precedent                               |
|---------------|------------------------------------------------------------|
| SNOBOL4       | SIL (SNOBOL Implementation Language) — bytecode + heap AM  |
| Prolog        | **LAM**, a WAM-inspired "Logic Abstract Machine" with 24 opcodes |
| Pascal        | P-code VM (Niklaus Wirth's original teaching approach)     |
| OCaml subset  | Tree-walking interp *on* the P-code VM (compound layering) |
| Forth         | Direct Threaded Code (classical, not ITC or subroutine)    |
| BASIC         | Teletype UNIVAC-1100-style, tokenized lines on P-code VM   |
| APL           | Right-to-left recursive descent, APL\360-style             |
| HLASM         | Macro assembler, IBM HLASM lineage                         |
| RPG-II        | Fixed-column specification → generated code                |
| FORTRAN       | Fixed-form FTI-0 integer subset, card-image era            |

These aren't cosmetic choices: the abstract machine shapes what's easy,
what's hard, and what can be self-hosted later.

### 2.3 Matching the implementation language to the target (with historical precedent)

Each language's implementation language is chosen to echo its historical
lineage, not just to get the job done. The pattern is: *look at how this
language was first made real, and pick something with the same shape on
our stack.*

- **Forth in COR24 assembler.** Forth has always been written in
  assembler — Moore's original, FIG-Forth, the inner interpreters of
  every Forth since. Writing `forth.s` in `.s` isn't a concession to
  the target; it's the point.
- **MacroLisp in C.** Later Lisps (Emacs Lisp, Guile, MacLisp
  descendants) are classic C projects — the cons-cell + mark-sweep
  pattern maps cleanly onto C's `malloc`/struct idioms. `tc24r` makes
  C feasible on COR24, so MacroLisp follows that tradition.
- **APL in C.** APL\360's assembler origins are hard to reproduce on a
  small target; every subsequent APL (NARS, A+, Dyalog's core) moved
  to C. C's pointer arithmetic and array-slice idioms line up with APL's
  vector operations.
- **SNOBOL4 in PL/SW — because of SIL.** The original SNOBOL4 was
  implemented in **SIL (SNOBOL4 Implementation Language)**, a
  macro-heavy systems language Griswold et al. designed expressly for
  SNOBOL4's abstract machine. PL/SW has compile-time macros
  (`MACRODEF`/`GEN`), records, and inline assembly — the same shape as
  SIL. Using PL/SW here is a deliberate echo of the SIL choice.
- **Prolog in PL/SW — because of WAM.** Prolog's canonical implementation
  is the **Warren Abstract Machine**, a register-based VM. That design
  assumes a systems language with records and structured code. PL/SW
  fits, and the project prefers PL/SW over C for this kind of
  abstract-machine work. The COR24 **LAM** (Logic Abstract Machine) is
  WAM-shaped: dedicated argument and temp register slots in tagged-cell
  frames.
- **Fortran in SNOBOL4 — because Bell Labs did it.** Bell Labs
  experimented with Fortran compilers written in SNOBOL4 in the
  mid-1970s. `sw-cor24-fortran` deliberately re-runs that experiment
  on COR24: write the compiler in SNOBOL4, emit PL/SW. The fact that
  this makes Fortran a third-generation dependent (Fortran → SNOBOL4
  → PL/SW) is part of the point — the full lineage has to be real
  to count.
- **RPG-II possibly in HLASM.** Macro assemblers and RPG are natural
  company: RPG-II's fixed-column spec sheets translate cleanly into
  code via macro templates. Today `rpg2.s` is raw COR24 assembly; the
  HLASM route would layer it on `hlasm.s` once HLASM solidifies —
  another echo of historical practice, where RPG implementations on
  IBM systems often rode on top of the macro assembler.
- **Pascal in C initially, runtime in Pascal.** UCSD Pascal's
  P-code-targeted compiler bootstrapped through smaller dialects of
  itself. Our stack compresses that: the compiler is C (via `tc24r`)
  to get going, the runtime is Pascal from phase 1. BASIC and OCaml
  then sit on the same Pascal/P-code substrate because Pascal's
  syntactic simplicity makes it a reasonable implementation host for
  tree-walking interpreters over other AST shapes.
- **PL/SW bootstrapped via C, self-hosted after.** The PL/I family
  (PL/I, PL/M, IBM's PL/S) started in assembler or bare metal. PL/SW's
  bootstrap from C via `tc24r` is the modernized version of that
  story; v1 is already self-compiling.

The through-line is curiosity about *how historical languages were
actually implemented* — and how to bootstrap language tools for a
range of ISAs (not just COR24). Where a language has a canonical
implementation technique or a well-known implementation language,
the project honours it; where it doesn't, the choice defaults to
mike's preferred stack (Rust on Linux/Mac for host tools, PL/SW or
HLASM for native COR24 tools, C used only where dictated by
convention or necessity).

### 2.4 Clean-room integer subsets; software float is future work

Every language in the ecosystem is a **clean-room clone of an integer
subset** of its reference. "Clean-room" because implementations follow
published semantics but don't lift code; "integer subset" because COR24
has no floating-point unit, and the ISA isn't going to grow one.

In scope today:

- Integer arithmetic at COR24's native word width (24-bit), with 8/16-bit
  accessors where the ISA supports them.
- APL over integer arrays; no floor/ceiling FP, no domino, no complex.
- FORTRAN restricted to its **FTI-0** subset: integers, simple arrays;
  no `REAL`, `DOUBLE PRECISION`, `COMPLEX`, `EQUIVALENCE`, `FORMAT`.
- Pascal without `Real`.
- OCaml without `float`.
- Lisp fixnums only (22-bit signed, two tag bits).
- BASIC's numeric type is 24-bit integer; no single-precision, no
  double-precision.
- SNOBOL4 string patterns over integer byte sequences; no FP arithmetic.

**Future work: a software floating-point library.** Extending the
ISA with hardware FP is not planned. A portable software-FP package
(IEEE-754 subset or Q-format, TBD) is on the roadmap; once it
exists, each language chooses whether to layer FP on top. The
library itself would likely live in PL/SW (systems language with
inline asm for tight loops), callable from all four language groups.

### 2.5 Personal preferences that shape the ecosystem

Several choices across the project reflect mike's own tastes rather
than strictly technical necessity. Worth stating plainly — they
explain a lot about why the ecosystem looks the way it does.

- **Metaprogramming and macros are first-class building blocks.**
  This is the reason **MacroLisp, PL/SW, and HLASM** are preferred
  as implementation hosts. C has `m4`, but the C preprocessor is
  not in the same league as Lisp macros, PL/SW's `MACRODEF` / `GEN`,
  or HLASM's macro facilities. Native tools lean on the
  metaprogramming-rich languages wherever they can.
- **APL is mike's first programming language.** `sw-cor24-apl`
  exists in part because mike wanted his own APL to play with.
- **Adjacent (but separate) work**: mike is developing a new
  programming language that **extends APL for machine-learning
  applications**. That project is not part of the sw-embed / COR24
  ecosystem — it's a separate effort — but it shares the APL
  lineage and the metaprogramming sensibility that shows up here.

---

## 3. Development process

Four conventions shape how every repo in the ecosystem grows: test-
driven development, demo-driven scope, regression testing with
`reg-rs`, and incremental deliverables tracked by saga/step logs in
`agentrail-rs`. They form a closed loop — **agentrail names the next
step → TDD writes a failing test → the feature is made to match a
canonical demo → the demo is added to reg-rs as a regression golden
→ the next step is queued.**

### 3.1 Test-driven development

Features land behind a failing test first. The test stays in tree
forever as a regression net. Standard red/green TDD, applied across
C/PL/SW/Pascal/assembly impl languages alike.

Testing tools vary with the impl language:

- **Rust host tools** (`tc24r`, `pa24r`, `pl24r`, the emulator): native
  `cargo test` plus snapshot tests against reference `.s` outputs.
- **Compiler tests for C/PL/SW/Pascal**: test programs compiled against
  known-good expected outputs (text emitted on UART), run on the
  emulator, output diffed.
- **`.s`-based tools** (Forth, HLASM, native assembler): runtime-assert
  tests embedded in the source; pass/fail observed via UART output or
  halt state.

### 3.2 Demo-driven scope

For each language, the project **researches the canonical demos** —
what programs people actually wrote in that language at its peak —
and treats those as acceptance criteria. "Implement enough of the
language to run these demos" is the scope definition; once the demos
work, the phase is done.

Example demo targets per language (partial):

- **APL** — `iota n`, primes via outer product, matrix operations,
  horse-race simulation, small interactive toys.
- **BASIC** — `99 Bottles`, Hamurabi, Star Trek, guess-a-number,
  robot-chase.
- **MacroLisp** — 99 Bottles, lazy sequences, anaphora macros,
  threading macros (`→`, `→>`).
- **Forth** — LED-blink loops, arithmetic drills, self-extending
  dictionary demos.
- **SNOBOL4** — fizzbuzz, factorial, dating-game pattern demo, small
  tokenizers.
- **Fortran** — integer hello-world, loops, simple formulae (FTI-0
  only).
- **Pascal / OCaml** — recursive list ops, pattern matching (OCaml),
  record types (Pascal).
- **Prolog** — `ancestor/2`, `append/3`, small puzzle demos.

Implementing "enough" of a language has a clean termination condition:
demo runs, output matches, full regression suite passes. This prevents
scope creep ("should we support all of ANSI X?") and keeps each
language's phase boundary explicit.

### 3.3 Regression testing with `reg-rs`

Once a demo works, it becomes a regression test. The project's shared
harness is **`reg-rs`** — a Rust utility that runs each demo input
through its compiler / interpreter, captures output, and diffs against
a saved golden file. A failure blocks merges.

Two usage styles:

- **Per-demo**: input program + expected output; `reg-rs` drives the
  language's toolchain, captures UART output, diffs against the
  golden.
- **Per-feature battery**: groups of demos for a specific feature
  (e.g. "pattern matching", "vector reduction") whose collective pass
  is the feature's acceptance signal.

The effect: **prior demos continue to work** across every subsequent
feature addition. Languages accrete capability monotonically. If a
change breaks an earlier demo, the PR doesn't merge. This discipline
is what makes multi-agent parallel development on ~40 repos
tractable.

### 3.4 Saga / step tracking with `agentrail-rs`

Work is structured into **sagas** (named threads of related
development) broken into **steps** (single-sitting deliverables).
Tracked by **`agentrail-rs`**, each step has its own directory under
`.agentrail/<saga>/steps/NNN-<slug>/` containing:

- `prompt.md` — the task, as originally defined.
- `step.toml` — metadata (status, depends-on, expected artifacts).
- `summary.md` — what actually happened, written at step close.

When a saga completes, its directory moves to `.agentrail-archive/`
(preserves history without cluttering the live view). `sw-cor24-apl`
shows the pattern well: 81 steps archived under
`.agentrail-archive/cor24-apl-interpreter-<timestamp>/`, each step a
discrete, committed increment with tests that run under `reg-rs`.

Why it matters:

- **Incremental deliverables.** Each step is a small, reviewable,
  independently-runnable change. Nothing is "partially done."
- **Parallel-agent traceability.** A coordinator can see exactly what
  each d\* worker is working on (current `.agentrail/<saga>/steps/`)
  without reading code.
- **Rationale preserved.** `prompt.md` captures *why* the step was
  defined; `summary.md` captures *why* the result is the shape it is.
  The step directory is the project's memory.

### 3.5 Cross-agent coordination with `wiki-rs`

`wiki-rs` started as a demo: a Rust-backed Markdown wiki that
multiple agents could read and update. As parallel agent work grew,
mike added **agent-specific APIs on top of it** — structured
request/response primitives so agent B could ask agent A for a
feature or a fix, and agent A could indicate when the request was
done.

In practice the protocol still required mike to **poke** agents to
get them to ask, respond, or check the wiki for status — the API
defined the shape of messages but no agent polled on its own. That
poking-by-hand is what motivated the next layer.

### 3.6 Multi-agent orchestration: `all-together-now`

**`all-together-now`** (sometimes `atn`) is the project's
coordination layer:
[github.com/sw-vibe-coding/all-together-now](https://github.com/sw-vibe-coding/all-together-now).

It replaces mike's manual poking with a **coordinator agent** that
polls the other agents via a `wiki-rs`-like API with **mailboxes**
added. The web UI surfaces one pty per agent so mike can still
observe and intervene when needed, but the routine asking, answering,
and status-checking is handled agent-to-agent.

Main capabilities:

- **Per-agent pty** — each worker agent has a terminal surfaced in
  the UI; outputs and prompts are visible in one place.
- **Coordinator role** — one agent acts as coordinator, delegating
  tasks to workers and polling them on a cadence.
- **Mailboxes** — asynchronous inter-agent messaging (a primitive
  `wiki-rs` didn't have on its own).
- **Shared wiki pages** — the same `wiki-rs` surface, now written by
  the coordinator and worker agents as part of their routine instead
  of only by mike.

### 3.7 Vendoring for parallel progress

The concrete problem vendoring solves: agent A is mid-flight adding a
feature to tool A, so tool A is temporarily broken. Agent B, working
on tool B that depends on tool A, tries to build and gives up.

With vendoring, agent B holds a **stable copy of tool A under its own
`vendor/`**. Agent A can rebuild a newer tool A at its own pace
without breaking agent B. When tool A's new version is published,
agent B can opt in to vendoring that newer version if and when it
needs new features — otherwise it stays on the pinned stable copy.

The pattern:

- Each consumer repo keeps a vendored copy of each producer tool
  under `vendor/`.
- A `vendor/active.env` file records the pinned version; the
  consumer's build references that, not the producer's `HEAD`.
- Producers publish new stable versions when they're done; consumers
  bump their `vendor/active.env` individually.

The `sw-cor24-assembler` repo has a canonical example: it vendors
the `cor24-rs` emulator + cross-assembler under
`vendor/sw-em24/v0.1.0/` with binary-source fallbacks declared in
its manifest.

Net effect: **parallel progress is promoted**. Agents don't block on
each other, and upgrades are opt-in rather than forced.

Together, `wiki-rs` + `all-together-now` + vendoring are the layers
that make parallel AI-agent development on interdependent repos
workable. The git branching model (`feat/` → `pr/` → `dev` →
`main`) handles **intra-repo** discipline; these three handle
**inter-repo** coordination.

---

## 4. Language groups

The project's repos naturally split into **four parallel lineages**,
each with its own entry tool, its own self-hosting trajectory, and its
own family of languages that fall out once the entry tool works. Groups
are roughly orthogonal — a worker can make progress on one without
blocking the others — which is why the fleet of d\* agents can
parallelize effectively.

Each group illustrates a different answer to the question *"given a
lower layer, what language does that make easy to build next?"* The
grouping is reflected operationally in the `family` column of
`scripts/dev-users.tsv`: every worker is tagged `c-lisp`,
`pascal-ocaml`, `plsw-prolog`, `assembler`, or `project` (the tooling
bucket — see §5).

### Group 1 — **c-to-lisp**: imperative tree-walking, written in C

**Repos**: `sw-cor24-x-tinyc` (`tc24r`) → native C → `sw-cor24-apl`,
`sw-cor24-macrolisp`.

The entry tool is `tc24r`, the Rust-host C cross-compiler. Once it
produces reliable COR24 `.s`, anything that looks like "a C program
with malloc and an interpreter loop" becomes available:

- **APL** — recursive-descent (right-to-left) tree-walking interpreter
  in C, with a bump allocator for per-iteration temporaries.
- **MacroLisp** — cons-cell tree-walking evaluator in C with a
  mark-sweep GC and a macro system.

**Why grouped**: C is a well-known, well-understood bootstrap tool;
it's the natural implementation language for tree-walking
interpreters whose core data structure is a tagged-cell tree. The
project's long-term preference is to move native tools toward HLASM
and PL/SW rather than C, but C is the pragmatic starting point for
this group, especially for languages (Lisp, APL) whose historical
implementations followed the same path.

**Bootstrapping story**: `tc24r` stays in Rust (it's a host-side
development tool, not a COR24 artifact). The languages it produces
run natively on COR24. Progression is linear — once APL and
MacroLisp work to their demo criteria, the group's job is done for
now.

### Group 2 — **pascal-ocaml**: stack-VM compilation, written in Pascal

**Repos**: `sw-cor24-pascal` (compiler + runtime) + `sw-cor24-pcode`
(P-code VM + assembler + linker) → `sw-cor24-basic` + `sw-cor24-ocaml`
+ (planned) `sw-cor24-x-pcode-aot`.

The entry artifact is an entire **stack VM ecosystem**: the Pascal
compiler (`p24p.s`, originally C-cross-compiled via `tc24r`), the
Pascal runtime (written in Pascal — dogfooded from phase 1), the
P-code VM (`pvm.s`, native COR24 assembly), the P-code assembler
(two impls, `pasm.s` native and `pa24r` in Rust, kept bug-compatible),
and the linker (`pl24r` in Rust).

Once that's in place, two languages layer cleanly on top:

- **BASIC** — a tokenized-line interpreter written in Pascal, running
  on the P-code VM. The VM abstracts COR24's register scarcity; BASIC
  never sees the ISA.
- **OCaml (integer subset)** — a tree-walking evaluator written in
  Pascal on the P-code VM. Compound layering: OCaml AST evaluated by
  Pascal code compiled to P-code interpreted by `pvm.s` running on the
  emulator.

**Why grouped**: P-code is the glue. Everything here either produces
P-code, consumes P-code, or layers on top of it. The stack VM absorbs
the ISA pain so individual languages can be written in a high-level
style.

**Bootstrapping story**: Pascal's compiler starts as C (via `tc24r`);
the runtime self-hosts early. The long-term plan is `x-pcode-aot`
(currently empty) to compile `.p24` bytecode directly to `.s`,
removing the interpretation tax on production P-code programs. That
would let this whole group stay high-level without ever paying the VM
cost.

### Group 3 — **plsw-prolog**: pattern-oriented, abstract-machine style

**Repos**: `sw-cor24-plsw` → `sw-cor24-snobol4`, `sw-cor24-prolog`,
`sw-cor24-fortran`.

PL/SW is a PL/I-flavored systems language with records, compile-time
macros (`MACRODEF`, `GEN`), and inline assembly. It's first
cross-compiled via `tc24r` (C), then self-hosts (v1 complete, 40
steps across 12 phases).

Once PL/SW is real, pattern-heavy languages become tractable:

- **SNOBOL4** — lexer, bytecode compiler, and a **SIL-inspired
  abstract machine** with heap-backed evaluation/backtrack/rollback
  stacks. Implemented in PL/SW + small inline assembly for
  performance-critical primitives.
- **Prolog** — lexer, parser, and a **LAM (Logic Abstract Machine)**
  with WAM-style tagged cells, 8 argument + 8 temp registers,
  unification, trail, and choice-point stacks. Also implemented in
  PL/SW.

**Why grouped**: PL/SW is mike's preferred native systems language;
its records and macros map well onto pattern-oriented abstract
machines (SIL, WAM). SNOBOL4 and Prolog both land here because
their canonical implementations (SIL; WAM-family systems-language
hosts) have the same shape PL/SW provides, and because writing
them in PL/SW is more to mike's taste than writing them in C.

**Bootstrapping story**: PL/SW is the answer to "what do we write
abstract-machine interpreters in." Prolog and SNOBOL4 validate that
answer. **Fortran is second-generation within this group**: its
compiler is *written in SNOBOL4* and *targets PL/SW text*, so it can't
start until SNOBOL4 is itself real. This gives the group a
three-layer spine: PL/SW at the bottom, SNOBOL4/Prolog as its peers,
Fortran on top of SNOBOL4. If a future language needs a similar style
— logic variants, term-rewriting systems, unification engines — it
should follow this template.

### Group 4 — **assembler**: operating at the bottom of the stack

**Repos**: `sw-cor24-x-assembler` (Rust, reference) →
`sw-cor24-assembler` (native `.s`), plus `sw-cor24-forth`,
`sw-cor24-hlasm`, `sw-cor24-rpg-ii`.

The entry tool is the pair of assemblers: the Rust-host `x-assembler`
(the specification) and the native `sw-cor24-assembler` (the target,
currently at nop-only smoke-test stage). Output must be byte-identical
— the native one proves self-hosting.

From there, three tools live entirely at the assembler level:

- **Forth** — direct-threaded-code kernel in ~2,600 lines of COR24
  `.s`. Once the dictionary and inner interpreter exist, Forth extends
  itself.
- **HLASM** — a macro assembler in `.s`, generating plain `.s`. Its
  job is to make writing raw COR24 less painful by adding macros,
  conditionals, structured control.
- **RPG-II** — a report-program-generator that reads fixed-column
  specifications and emits `.s`. Also written in `.s`.

**Why grouped**: these are all tools that *talk to the ISA directly*.
Their implementation language is COR24 assembly. They're the absolute
floor of the stack — there's nothing below them except the silicon
(or the emulator). The `x-assembler` stays in Rust as a bridge and a
reference; everything else converges on native `.s`.

**Bootstrapping story**: this is the most self-contained group. Once
the native assembler reaches parity with `x-assembler`, no cross-host
tool is required for any member of this group — you can edit `.s` in
`yocto-ed`, assemble with native `sw-cor24-assembler`, run on
hardware. Forth especially benefits because its tradition is "the
Forth system is the development environment"; here that tradition is
respected.

### Cross-group observations

- **Every group has an entry tool written in Rust** (or produced via
  `tc24r` from C). Nothing starts on COR24 from zero — the Rust host
  is the backstop for all four.
- **Groups are orthogonal.** Group-1's APL doesn't block Group-3's
  Prolog; a worker can advance each independently.
- **Shared infrastructure cuts across groups**: the COR24 emulator,
  the monitor/loader, the editor, the script interpreter. These
  aren't in any group because they don't produce new languages — they
  *serve* all four. They are covered in §5.

---

## 5. Tooling: precursor to an embedded OS

Alongside the four language lineages, seven repos are **tools** — they
don't implement a new language, they make the existing stack usable.
Taken together, they cover roughly the user-facing surface that a
minimal embedded operating system would need to provide.

| Repo                      | Impl language        | Role                                               |
|---------------------------|----------------------|----------------------------------------------------|
| `sw-cor24-emu` (cor24-rs) | Rust                 | CPU + peripheral emulator; native `cor24-run` CLI and WASM |
| `sw-cor24-debugger`       | Rust                 | `cor24-dbg` instruction-level stepper; talks to emu via snapshot |
| `sw-cor24-monitor`        | (future) native `.s` | Cold-boot ROM monitor — placeholder today          |
| `sw-cor24-script` (sws)   | C (via `tc24r`)      | Tcl-shaped shell: `$var`, `{}` blocks, `(cmd)` subst, `run` for subprocesses |
| `sw-cor24-yocto-ed` (swye)| C (via `tc24r`)      | Modal gap-buffer editor; 3-line display            |
| `sw-cor24-x-rust`         | (empty placeholder)  | Planned Rust→COR24 cross-compiler; tool for writing future tools |
| `sw-cor24-project`        | (markdown)           | Org-wide meta-docs: roadmap, ISA reference home    |

**Tooling that lives inside other repos** (not standalone):

- **Loader**: `cor24-run --load-binary <image> --entry <label>` (in
  `sw-cor24-emu`). No dedicated repo; it's an emulator flag.
- **Linker**: `pl24r` (in `sw-cor24-pcode`). P-code-specific.
- **Image relocation**: `relocate_p24.py` (in `sw-cor24-pcode`).
  Host-side Python that rewrites absolute data references for a known
  load address.

### 5.1 Why this is an OS precursor

Lined up against what a minimal embedded OS would provide:

| Embedded-OS function        | Today's tooling analogue                                     |
|-----------------------------|--------------------------------------------------------------|
| Boot / bootloader           | `sw-cor24-monitor` (future); `cor24-run --load-binary` (now) |
| Shell                       | `sws`                                                        |
| Process launch / pipeline   | `sws`'s `run` command captures child stdout into `$rc`       |
| Editor (`ed`, `vi`)         | `swye` (yocto-ed)                                            |
| Debugger (`gdb`)            | `cor24-dbg`                                                  |
| Syscall surface             | MMIO (UART, LED) + UART-RX interrupt — that's the entire API |
| Filesystem                  | Stubbed in `sws`; no real implementation                     |
| Process isolation / MMU     | None — the ISA has no MMU, single-process by design          |
| Memory allocator (shared)   | None — each language's runtime brings its own                |
| Device drivers              | Raw MMIO in each language; no kernel abstraction             |

What's **deliberately missing** (an embedded OS would need these):

- A filesystem layer. `sws` has command stubs that will eventually hit
  this.
- Multitasking or cooperative scheduling. Today the machine runs
  exactly one program at a time.
- A syscall / trap instruction. All I/O is MMIO; interrupts come from
  hardware only.
- A standard ABI across languages. Every language has its own calling
  convention; inter-language composition happens via UART and `sws`
  pipe chains, not via in-process linkage.

### 5.2 How tooling depends on the language groups

- `sws` and `swye` are **C programs**, so they're downstream of Group
  1's `tc24r`. They exist because you need an editor and a shell to
  work on anything else on COR24 natively.
- The **emulator and debugger** are Rust-host tools. They don't
  depend on any group; they *enable* all four. The web variants in
  §23 are the emulator delivered as WASM.
- The **monitor** (when built) will be native `.s`, i.e. downstream
  of Group 4's assembler.
- `x-rust`, once implemented, gives future tooling authors a
  Rust→COR24 pipeline — widening Group 1's reach without replacing
  `tc24r`.

### 5.3 Trajectory

If these utilities keep accreting capability — a filesystem in `sws`,
a boot sequence in `sw-cor24-monitor`, a source-level debugger in
`cor24-dbg`, a tasking model in anything — they become the shape of a
Group 5: the embedded OS. The repos that would grow into that OS
(monitor, shell, editor, debugger) are already reserved as standalone
projects, which is why the tooling family is sized deliberately
larger than today's feature surface requires.

---

## 6. Bootstrapping and porting strategy

The bootstrapping story is the load-bearing architecture of the whole
org — the how-do-we-get-from-nothing-to-self-hosted question. The
language groups in §4 are *what* gets built; this section is *how*.

### 6.1 The cross-host bridge

**Everything starts on a Rust development host.** Two Rust-authored
tools bridge from normal laptops into COR24:

- **`tc24r`** (`sw-cor24-x-tinyc`) — a C→COR24 cross-compiler,
  TinyCC-shaped (recursive-descent parser, direct codegen, no IR). 14
  components, ~50 internal crates, 61 demos, 81 tests. Generates
  human-readable `.s` with comments mapping back to the C source.
  **This is the single most important tool in the ecosystem** — if
  you can write it in C, `tc24r` brings it to COR24.
- **`cor24-run`** (`sw-cor24-emu` / `cor24-rs`) — a Rust-hosted
  emulator with built-in assembler that can assemble-and-run `.s`
  directly, load pre-assembled binaries, stream UART, step
  instructions, snapshot state.

### 6.2 The language ladder

From `tc24r`, everything else bootstraps. Target native binary on
the left, build path on the right:

```
tc24r (Rust, host)
  ├── p24p.s     (Pascal compiler, C source → tc24r → COR24 .s)
  ├── plsw.s     (PL/SW compiler, C → tc24r → .s; later self-hosts)
  ├── rpg2.s     (RPG-II compiler, .s direct)
  ├── hlasm.s    (HLASM macro-asm, .s direct; self-bootstraps macros)
  ├── sws.s      (sws script interpreter, C → tc24r → .s)
  ├── swye.s     (yocto-ed editor, C → tc24r → .s)
  └── ...
```

The **cross-assembler** `sw-cor24-x-assembler` (Rust) is the
reference for the native one: the in-progress `sw-cor24-assembler`
(COR24 `.s`, currently bootstrapping at nop-only smoke-test stage)
must emit byte-identical output.

Two "cross-tool" slots are currently **empty placeholders**, reserved
for eventual Rust-host implementations:

- `sw-cor24-x-pcode-aot` — AOT compile `.p24` bytecode to native
  `.s`, bypassing the P-code interpreter.
- `sw-cor24-x-rust` — Rust-to-COR24, the hypothetical complete
  native-Rust pipeline.

### 6.3 Porting strategy

The canonical progression is "**write it in Rust/C first, then port
to something native**":

1. **Rust host** (pure dev convenience; fast, familiar, no COR24
   dependency).
2. **C (cross-compiled)** via `tc24r`. Now runs on actual COR24.
3. **PL/SW** — once it's stable, rewrite tools in PL/SW (self-hosting
   story).
4. **Prolog / SNOBOL4** — pattern-heavy tools naturally move to these
   once the semantic machines (LAM, SIL) are on the machine.
5. **`.s` direct** — the absolute limit. Only the lowest-level tools
   (assembler, loader, Forth kernel) live here long-term.

The ladder is explicit in the project docs and reflected in which
repos are "x-" (cross, Rust-host) versus bare name (native target).

---

## 7. Lexing and tokenizing

Everything is **hand-written**. Not a single yacc/bison/PEG generator
in the tree — which makes the lexers readable and the code size
minimal.

- **APL**: right-to-left tokenizer, feeds directly into the parser
  via a token stream (no lookahead buffer, right-to-left simplifies
  things).
- **BASIC**: tokenizer rewrites source into a packed sorted buffer of
  `line# | length | tokens`. No AST at all — execution dispatches on
  tokens directly.
- **MacroLisp**: hand-written reader emits S-expressions (cons
  pairs). Lexer is part of the reader, there's no separate token
  type.
- **Forth**: `WORD` + `NUMBER` primitives. The "tokenizer" is itself
  Forth.
- **SNOBOL4**: PL/SW-written; classifies statements, handles
  continuation lines, distinguishes label/body, strips comments.
- **Fortran**: SNOBOL4-written; fixed-form normalizer (columns 1–6
  labels, 7 continuation, 72 card limit).
- **Pascal, OCaml, Prolog, PL/SW, TinyC**: hand-written
  recursive-descent style lexers emitting token streams, no external
  tooling.

---

## 8. Parsing

Style varies with the language:

- **Recursive descent** dominates. APL parses **right-to-left**
  recursive descent, which is APL-idiomatic (no operator precedence);
  most others parse left-to-right with standard precedence climbing
  or explicit per-precedence levels (TinyC's chibicc lineage).
- **Statement-level dispatch** in BASIC: no parse tree, each
  tokenized line is interpreted by looking at its leading keyword.
- **Token-threaded** in Forth: the "parser" is dictionary lookup,
  and execution is the AST.
- **PEG / GLR / table-driven parsers**: none present.
- **Pattern-matching directed** in SNOBOL4 and Prolog: source maps
  almost directly to bytecode because the surface language is already
  close to the abstract machine.

---

## 9. ASTs and intermediate representations

The IR story sorts projects into clear buckets:

| Project      | IR                                                           |
|--------------|--------------------------------------------------------------|
| APL          | AST of `NODE_*` kinds (monad, dyad, reduce, bracket, lit)    |
| MacroLisp    | Cons-cell trees (tagged 24-bit words: fixnum/cons/symbol/ext)|
| Forth        | **No AST** — dictionary entries + threaded IP               |
| BASIC        | **No AST** — sorted tokenized-line buffer                   |
| Pascal       | Parse tree → P-code (`.spc` assembler text)                 |
| OCaml        | Evaluated as Pascal-hosted AST on P-code VM                 |
| Prolog       | Parser output → LAM bytecode (24 opcodes)                   |
| SNOBOL4      | Abstract-machine bytecode, SIL-shaped                       |
| Fortran      | Source-to-source: normalized statements → PL/SW text        |
| PL/SW        | Direct-emission to `.s` with source comments                |
| TinyC        | Expression/statement trees → direct assembly (no IR layer)  |
| HLASM        | Macro expansion → plain `.s` text                           |

### 9.1 Tagged cells — a recurring trick

MacroLisp and Prolog both use **3-bit tag, 21-bit payload** within a
24-bit cell:

- MacroLisp: `fixnum` (22-bit signed, 2 tag bits), `cons`, `symbol`,
  `extended`. GC scans cells, distinguishes pointers from immediates
  by tag.
- Prolog LAM: `REF` (unbound variable), `INT`, `ATOM` (interned),
  `STR` (structure functor+args), `LIST`, `FUN` (functor header).
  Six tags, same encoding space.

The small word size forces this; native 64-bit pointer tagging
wouldn't translate.

### 9.2 P-code as a second IR level

Pascal emits P-code, which is its own dense bytecode with a stack VM
definition (`pvm.s`, native COR24 assembly). BASIC, OCaml, and PL/SW
also target P-code in various stages — it's the de facto mid-level
IR of the ecosystem. See §11.

---

## 10. Lowering and codegen

Four distinct lowering patterns live in the ecosystem:

### 10.1 Direct to COR24 `.s` (no IR)

**TinyC (`tc24r`)** is the clearest example: the parser drives the
codegen directly. No intermediate form, no optimizer. Source comments
are preserved inline in the emitted assembly. **PL/SW** follows the
same pattern with added macro expansion. Characteristic of compilers
whose source language maps cleanly onto the target's abstractions.

### 10.2 Through P-code

Pascal → `.spc` (P-code text) → `pl24r` (link) → `pa24r` (assemble)
→ `.p24` bytecode → `pvm.s` (interpret).

BASIC, OCaml, and (in progress) PL/SW module-linking also target
P-code. The advantage: the language's codegen never sees COR24
registers directly — the stack-based P-code abstracts them away
entirely. The tradeoff is an interpretation dispatch cost. An
`x-pcode-aot` cross-tool is planned to compile `.p24` straight to
`.s` for production.

### 10.3 Through an abstract machine (WAM/SIL)

**Prolog (LAM, WAM-shaped)**: 8 argument registers `A0..A7` + 8 temp
registers `X0..X7` on top of the tagged-cell heap. Unification,
trail, choice points, environment frames — every Prolog hallmark.
Bytecode opcodes: `CALL`, `EXECUTE`, `PROCEED`, `FAIL`, `CUT`, `TRY`,
`RETRY`, `TRUST`, `PUT_CONST`, `GET_VAR`, `ALLOCATE`, `DEALLOCATE`,
and peers.

**SNOBOL4 (SIL-inspired)**: heap-backed evaluation stack, backtrack
stack, rollback stack, all kept separate from the machine stack
because pattern matching needs to rewind deeply. Mark-sweep GC
planned.

### 10.4 Threaded code

**Forth** is direct threaded code: each word is a list of code-field
addresses (CFAs). `IP` (COR24 `r2`) walks the list; each CFA jumps
into its word. No AST, no IR, no codegen step — parsing *is*
execution.

---

## 11. The P-code VM as a substrate

`sw-cor24-pcode` deserves its own section because so many other
languages sit on it.

### 11.1 Shape

Stack machine, syscall-driven:

- `HALT`, `PUTC`, `GETC`, `LED`, `ALLOC`, `FREE` are the minimal
  kernel surface.
- 24-bit stack cells. Argument/result passing via the data stack.
- Code relocated to load address `0x010000` by `relocate_p24.py` so
  `push <data_ref>` operands resolve absolutely.

### 11.2 Two assemblers in lockstep

- `pasm.s` — COR24-native assembler for P-code (the self-hosting
  proof).
- `pa24r` — Rust-hosted assembler (4,447× faster, used for everyday
  development).

These **must stay bug-compatible**. Any change to opcode encoding or
directive syntax has to land in both simultaneously. This kind of
dual-impl invariant is a recurring pattern in the org (e.g.
`sw-cor24-x-assembler` vs. native `sw-cor24-assembler`) and serves as
a living specification.

### 11.3 Languages on the P-code VM

| Language | Implementation language | Role of P-code                         |
|----------|-------------------------|-----------------------------------------|
| Pascal   | C (via tc24r)            | primary target of pascal compiler       |
| BASIC    | Pascal                   | interpret tokenized BASIC as p-code     |
| OCaml    | Pascal                   | tree-walk OCaml AST using P-code ops    |
| PL/SW    | C (via tc24r), native    | module linking goes through P-code      |

That compound layering — OCaml interpreter in Pascal compiled to
P-code interpreted on `pvm.s` — is the most baroque stack in the
org, and an explicit demonstration that the levels compose.

---

## 12. Memory allocation

Each language picks an allocator that matches its typical object
lifetime:

| Project     | Allocator                                                      |
|-------------|----------------------------------------------------------------|
| APL         | Bump allocator on a 4 KB heap; per-iteration reclamation for temps |
| MacroLisp   | 8 KB fixed-cell heap → free list → mark-sweep GC               |
| Forth       | Dictionary grows linearly in SRAM; data stack on EBR; no GC    |
| Pascal      | P-code VM manages heap via `ALLOC`/`FREE` syscalls             |
| BASIC       | Fixed-size scalar/array tables, no dynamic alloc               |
| SNOBOL4     | Heap-backed semantic stacks; mark-sweep GC planned             |
| Prolog      | Heap, trail, choice-point stack, environment stack; no GC yet  |
| PL/SW / TinyC | Stack-frame and static-data layout; no GC (C-style ownership) |

The **bump allocator** is used for short-lived temporaries (APL,
tree-walk interpreters); **mark-sweep** is used where there are
long-lived heap graphs (Lisp, Prolog, SNOBOL4). No generational or
concurrent GC anywhere.

---

## 13. Register strategy for a tight register file

With 3 truly general-purpose registers, the interesting question is
*how* each project avoids pretending it has more.

- **Stack machines avoid the question.** Pascal/BASIC/OCaml/PL/SW
  target the P-code VM which is stack-based; the language's codegen
  never sees a register. Spilling doesn't exist because values are
  never in a register long enough to spill.
- **Forth embraces stack discipline.** Dedicated register
  assignments (`r0=W`, `r1=RSP`, `r2=IP`, `sp=DSP`) plus the data
  stack on EBR. The rest of the ISA is free for primitive
  implementations.
- **SNOBOL4 pushes to the heap.** Evaluation stack, backtrack stack,
  and rollback stack are all heap-allocated rings rather than
  fighting for a real-register "working set." Assumes only 3
  registers usable at any time.
- **Prolog's LAM borrows WAM's abstraction.** 8 argument regs + 8
  temp regs are simulated — they're slots in the environment frame,
  not real COR24 registers. Unification and backtracking happen
  entirely in the tagged-cell heap.
- **TinyC / PL/SW do real register allocation** (for expression-level
  temporaries only, with stack spill for anything that doesn't fit).
  No graph-coloring — chibicc-style "stack everything, peephole
  later."
- **Lisp / APL** use C's calling convention via `tc24r` and inherit
  whatever `tc24r`'s backend gives them.

No project implements linear-scan or graph-coloring allocation. The
design bet is that on this machine size, **abstract-machine approaches
dominate register-aware codegen**.

---

## 14. Garbage collection

- **MacroLisp** — conservative mark-sweep. Scans the C stack
  (implementation language) as a root set. 8 KB of cells (48 KB
  total).
- **SNOBOL4** — mark-sweep planned; current implementation leaks by
  design because programs are short-lived.
- **Prolog** — no GC yet. Trail and choice points are reclaimed on
  cut, giving a backtracking-scoped arena that mostly avoids the
  problem.
- **APL / Forth / BASIC / Pascal / OCaml / PL/SW / TinyC** — no GC.
  Arena reclamation (APL), stack-scoped lifetimes (PL/SW, TinyC,
  Pascal), or explicit `ALLOC/FREE` (P-code).

No generational, no concurrent, no incremental GC anywhere in the
ecosystem.

---

## 15. Memory-mapped I/O

All I/O goes through MMIO addresses above `0xFF0000`. Every language
has at least a minimal "banging on the UART" path, and some have
richer conveniences:

- **APL** — quad variables `qled`, `qsw`, `qsvo` give user-level
  access to the LED, switch, and shared-variable-offer (MMIO-as-value).
- **Forth** — `EMIT`, `KEY`, and user-definable hardware words;
  UART/LED wrapped as first-class words.
- **BASIC** — `PEEK(addr)` / `POKE(addr, val)` give direct MMIO
  access.
- **MacroLisp** — LED and UART wrappers in the prelude.
- **Pascal** — `PUTC`/`GETC`/`LED` P-code syscalls; higher-level IO
  units on top.
- **TinyC / PL/SW** — `volatile` pointer dereference or inline `asm`
  clauses; the usual embedded-C patterns.
- **SNOBOL4, Prolog, Fortran** — delegate to the PL/SW runtime's IO.

The **ISR-on-UART-RX** path is rare in user languages but visible in
the Forth kernel and TinyC's `__attribute__((interrupt))` support.

---

## 16. Integer-only arithmetic

No floating point anywhere. Consequences observed in the codebases:

- **APL** — documented integer subset; arrays of 24-bit signed
  integers only.
- **Fortran (FTI-0 subset)** — explicitly excludes `REAL`,
  `DOUBLE PRECISION`, `COMPLEX`. A future fixed-point extension is
  mentioned but not built.
- **MacroLisp** — 22-bit signed fixnums (2 bits consumed by tags).
- **Pascal** — no `Real` type.
- **TinyC / PL/SW** — no `float`/`double` keywords accepted by the
  parser.

Software fixed-point routines exist in a few places as private
helpers; there is no shared fixed-point library (yet) in the org.
A shared software-FP library is future work — see §2.4.

---

## 17. Software divide

Every language that needs division either:

1. Calls a shared runtime `__divu` / `__divs` / `__modu` routine
   (produced by `tc24r` for C; hand-written for languages that call
   it directly).
2. Avoids division — Forth canonically does only when the user
   explicitly asks; APL defers to its fixed-word-size interpreter.
3. Pre-multiplies and tracks the scale (fixed-point pattern).

`mul` is in hardware but returns only the low 24 bits — high-precision
multiplication also requires a software routine.

---

## 18. Stack sizes, EBR vs SRAM

| Stack                     | Where                   | Size                        |
|---------------------------|-------------------------|-----------------------------|
| COR24 call stack (sp)     | SRAM, grows downward    | Configurable; default ≥8 KB |
| Forth data stack          | EBR                     | ~1 KB (fast path)           |
| Forth return stack        | SRAM (r1-backed, grows down from ~0x0F0000) | program-sized |
| P-code VM eval stack      | SRAM                    | ~4 KB                       |
| SNOBOL4 eval/backtrack    | Heap (in SRAM)          | bounded only by heap        |
| Prolog choice/env/trail   | Heap (in SRAM)          | bounded only by heap        |
| BASIC GOSUB / FOR stacks  | SRAM, fixed-size        | 64 / 16 entries             |

**EBR is reserved for the hot path.** Forth puts its data stack
there because every primitive touches it. No other language currently
uses EBR heavily — in part because the 3 KB budget is hard to use
well, in part because the emulator-first development flow makes EBR
vs SRAM behaviorally identical until you're targeting real hardware.

---

## 19. Compiler / interpreter pipeline

Full pipeline for a typical native-path program (Pascal example):

```
source.pas                      (text)
    ↓ p24p.s                    (Pascal compiler, running on COR24 emulator
                                 via cor24-run, or AOT-produced native binary)
source.spc                      (P-code assembly text)
    ↓ pl24r (Rust host) or native equivalent
combined.spc                    (linked P-code text)
    ↓ pa24r or pasm.s
program.p24                     (P-code bytecode)
    ↓ relocate_p24.py
program.p24 (relocated)         (addresses fixed at load addr 0x010000)
    ↓ cor24-run --load-binary
[execution on pvm.s P-code VM, which itself runs on cor24-run emulator]
```

Full pipeline for a C-to-native program:

```
source.c
    ↓ tc24r (Rust host)
source.s                        (COR24 assembly, with source comments)
    ↓ cor24-run --assemble  or sw-cor24-assembler (once self-hosted)
program.bin                     (loadable binary)
    ↓ cor24-run --load-binary
[execution]
```

`tc24r` and `pvm.s` are the two backstops — one makes anything
C-shaped real on COR24, the other makes anything stack-machine-shaped
real on COR24. Most of the ecosystem fits in one or the other.

---

## 20. Linker and loader

- **`pl24r`** (Rust host) — linker for P-code `.spc` modules. Merges
  modules, resolves symbols, emits a combined `.spc` that `pa24r` can
  assemble. Module format: `.proc`, `.end`, `.module`, `.export`
  directives.
- **`relocate_p24.py`** (host-side Python) — relocates a `.p24`
  binary so `push <data_ref>` operands become absolute addresses.
  Usually target `0x010000`, configurable via `--load-addr`.
- **`cor24-run --load-binary`** — generic image loader at arbitrary
  address. Used for every payload type (P-code images, Forth blocks,
  TinyC-compiled programs).
- **Entry-point resolution**: default `0x0`, override via `--entry
  <label>` or explicit address. For P-code images, the entry is
  discovered by resolving `code_ptr` in the VM and patching at load
  time.

No `sw-cor24-linker` repo exists as a separate artifact — the linker
lives inside `sw-cor24-pcode`.

---

## 21. Emulator and debugger

### 21.1 Emulator (`sw-cor24-emu` / `cor24-rs`)

Rust implementation, shared by native CLI (`cor24-run`) and browser
(WASM). Characteristics:

- **Instruction-by-instruction** execution; no JIT, no basic-block
  caching.
- **UART simulation** with configurable busy countdown (default 10
  cycles) and bit 2 overflow flag. `--terminal` bridges stdin/stdout
  to the UART.
- **Tracing**: last N instructions printed on halt.
- **Snapshots**: CPU registers, I/O state, non-zero memory regions.
- **Halt detection** for the `bra halt` one-instruction self-loop.
- **Load modes**: assemble-from-source, load pre-built binary at
  address, load with entry-point label.

### 21.2 Debugger (`sw-cor24-debugger`)

- **Instruction-level** (not source-level — no DWARF).
- Step, break-on-address, memory inspection, UART I/O.
- Invoked as `cor24-dbg`, talks to the emulator's snapshot interface.
- The `.lgo` (MakerLisp load-and-go) format is recognized.
- Status: **experimental**. Most debugging is done today via
  `cor24-run --step --dump`.

A source-level debugger is future work; the blocker is generating
DWARF or an equivalent line-table from `tc24r` / the Pascal compiler.

---

## 22. Memory loading strategies

How code and data get into the machine:

- **Monitor** (`sw-cor24-monitor`, placeholder) — the eventual
  cold-boot ROM story. Not yet populated.
- **Script** — `cor24-run --load-binary image.bin --entry label` on
  the host, scripted through Makefiles / `justfile`s per repo.
- **Shell / interpreter** — `sw-cor24-script` (`sws`, Tcl-shaped) is
  the on-machine shell. Tcl-style command-and-word syntax, `$var`,
  `(cmd)` substitution, `{}` blocks, and structured `$rc` from child
  commands. It can launch other binaries (`swye.s` the editor, other
  compilers) via `run` and capture their UART output.
- **Editor** (`sw-cor24-yocto-ed`, `swye.s`) — modal 3-line-display
  editor with a gap buffer. C source, compiled by `tc24r`. Dogfoods
  the whole native stack (C → `.s` → `cor24-run`).
- **Running commands** — any binary emitted by the P-code or native
  pipelines can be launched directly from `sws`.

UART has a 4 KB buffer in the emulator, which matters: source files
fed into compilers are usually preloaded via `-u <file>"\x04"`
(EOT-terminated) rather than the interactive `--terminal` mode.

---

## 23. Web frontends (`web-sw-cor24-*`)

Sixteen web repos (10 working, 6 planned/scaffolded) give every tool
a browser UI. The architecture is strikingly uniform:

### 23.1 Common stack

- **Yew 0.21** client-side rendering, `crate-type = ["cdylib",
  "rlib"]`.
- **Trunk** (not wasm-pack) as the build tool.
- **wasm-bindgen** for the Rust↔JS boundary.
- **Catppuccin Mocha** theme, plain HTML + CSS (no Monaco/Ace/
  CodeMirror).
- **Release profile**: `opt-level = "z"`, `lto = true` for WASM size.
- **Deployment**: GitHub Pages (build target `pages/`, `trunk build
  --release --public-url /web-sw-cor24-<name>/`).

### 23.2 No reimplementation — path dependencies

Every web repo depends on its native counterpart as a Rust crate via
path:

```toml
cor24-emulator = { path = "../sw-cor24-emulator", default-features = false }
cor24-isa      = { path = "../sw-cor24-emulator/isa", features = ["serde"] }
# or
pa24r = { path = "../sw-cor24-pcode/assembler" }
pl24r = { path = "../sw-cor24-pcode/linker" }
```

Interpreter images (`.bin`, `.p24`, `.p24m`) are **embedded at build
time via `build.rs`** — the browser ships a frozen snapshot of the
compiled interpreter, not a live-built one.

### 23.3 Two execution models in the browser

| Flavor          | Repos                               | How                                     |
|-----------------|-------------------------------------|-----------------------------------------|
| **Emulate COR24** | APL, Forth, MacroLisp, SNOBOL4      | `cor24-emulator` compiled to WASM, runs the native `.bin` interpreter image |
| **P-code VM in Rust/WASM** | BASIC, Pascal, OCaml, PL/SW | `pv24t` port: Rust reimplementation of `pvm.s`, directly interprets `.p24` bytecode — skips COR24 emulation entirely |

The second pattern is a performance choice: P-code has its own stack
semantics, so emulating COR24 underneath the P-code VM would double
the interpretation tax. Instead, the web variant replaces the bottom
level only.

### 23.4 Cross-tool web frontends — not yet

`web-sw-cor24-x-assembler`, `web-sw-cor24-x-rust`,
`web-sw-cor24-x-tinyc` are **not yet clones under `/disk1`** —
they're placeholders. Once filled in, the value proposition is
clear: "try the COR24 cross-toolchain in a browser without
installing Rust."

---

## 24. Status summary

| Repo                           | Phase / status                        | Self-hosting              |
|--------------------------------|---------------------------------------|---------------------------|
| `sw-cor24-emu` (cor24-rs)       | Production (Rust)                     | N/A (stays host-side)     |
| `sw-cor24-debugger`             | Experimental, instruction-level       | N/A                       |
| `sw-cor24-assembler` (native)   | Relaunch saga; nop-only smoke test    | Target state              |
| `sw-cor24-x-assembler` (Rust)   | Complete; reference for native        | N/A                       |
| `sw-cor24-x-tinyc` (`tc24r`)    | Functional (61 demos, 81 tests)       | N/A                       |
| `sw-cor24-x-pcode-aot`          | **Empty**; placeholder                 | —                         |
| `sw-cor24-x-rust`               | **Empty**; placeholder                 | —                         |
| `sw-cor24-pcode` (`pvm.s`)      | Full; assembler+linker+VM             | `pasm.s` native ✓         |
| `sw-cor24-pascal`               | Phases 0–1 complete                   | Runtime self-hosted       |
| `sw-cor24-ocaml`                | Phases 0–3; 34 tests                  | N/A (tree-walk on P-code) |
| `sw-cor24-plsw`                 | v1 complete; 40 steps / 12 phases     | Yes                       |
| `sw-cor24-prolog`               | LAM core; 15 tests                    | Partial (PL/SW impl)      |
| `sw-cor24-snobol4`              | Core + demos; pattern matching ok     | No (SIL planned)          |
| `sw-cor24-fortran`              | Scaffold; normalize+classify working   | No                        |
| `sw-cor24-apl`                  | Complete; 81 agentrail steps archived  | No (C impl)               |
| `sw-cor24-basic`                | v1 end-to-end functional              | No                        |
| `sw-cor24-macrolisp`            | REPL feature-complete; mark-sweep GC   | No                        |
| `sw-cor24-forth`                | Phases 2–4 working                    | Native `.s`               |
| `sw-cor24-hlasm`                | Bootstrap subset; macros planned       | Native `.s`               |
| `sw-cor24-rpg-ii`               | Early phase                           | Native `.s`               |
| `sw-cor24-script` (`sws`)       | v0.1 feature-complete                 | No (C impl)               |
| `sw-cor24-yocto-ed` (`swye`)    | Core; 7 regression tests              | No (C impl)               |
| `sw-cor24-monitor`              | Bare status                           | —                         |
| `web-sw-cor24-*` (10 live, 6 planned) | See §23                          | N/A                       |

---

## 25. Project commitments

The author's stated commitments — not inferred by this doc, but set
by mike directly:

1. **Learn by educating others.** The project is about history and
   technique; every piece of it is meant to explain how some class
   of language implementation actually works.
2. **Build an ecosystem for FPGA-based systems.** COR24 is the first
   target, but the intent is to port the same tools and techniques
   to additional FPGA-hosted ISAs (IBM 1130, 360, 390, RCA 1802,
   RISC-V I32, others).

Patterns that emerge from those commitments, visible in the codebase:

- **Two-implementation invariants as living specs.** `pasm.s` ↔
  `pa24r`, `sw-cor24-assembler` ↔ `sw-cor24-x-assembler` — the
  second impl is the specification.
- **Cross-first, native-later.** Tools start as Rust cross-hosted
  (or C via `tc24r`) and earn their native form once the lower
  layers are stable enough.
- **Abstract machines over clever codegen.** P-code, LAM (WAM-style),
  SIL, threaded Forth — historically-grounded abstract machines are
  preferred over ambitious register-allocator work on COR24's tight
  register file.
- **Monotonic growth, per-demo regression gating.** TDD + canonical
  demos + `reg-rs` goldens + `agentrail-rs` saga/step tracking keep
  prior capability from breaking silently.
- **The web as a delivery channel.** Web repos are thin Yew wrappers
  over the same Rust crates the native tooling uses. No fork; WASM
  gets zero-install demos of the actual tools.

---

*Written after reading CLAUDE.md, README.md, and surface-level source
across ~40 repos in the sw-embed org.*
