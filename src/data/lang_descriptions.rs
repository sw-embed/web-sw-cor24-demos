#[derive(Clone, PartialEq, Debug)]
pub struct LangSummary {
    pub id: &'static str,
    pub label: &'static str,
    pub inspired_by: &'static str,
    pub one_liner: &'static str,
    pub repo: &'static str,
    pub section_id: &'static str,
}

pub fn summaries() -> &'static [LangSummary] {
    &SUMMARIES
}

pub fn all_details() -> &'static [LangDetail] {
    &DETAILS
}

#[derive(Clone, PartialEq, Debug)]
pub struct GlyphRow {
    pub latin: &'static str,
    pub glyph: &'static str,
    pub monadic: &'static str,
    pub dyadic: &'static str,
}

#[derive(Clone, PartialEq, Debug)]
pub struct KeywordRow {
    pub keyword: &'static str,
    pub glyph: &'static str,
    pub usage: &'static str,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LangDetail {
    pub id: &'static str,
    pub label: &'static str,
    pub inspired_by: &'static str,
    pub section_id: &'static str,
    pub history: &'static str,
    pub purpose: &'static str,
    pub usage: &'static str,
    pub pros: &'static [&'static str],
    pub cons: &'static [&'static str],
    pub glyph_table: Option<&'static [GlyphRow]>,
    pub keyword_table: Option<&'static [KeywordRow]>,
}

static SUMMARIES: [LangSummary; 15] = [
    LangSummary {
        id: "a24",
        label: "a24-sw",
        inspired_by: "Assembler",
        one_liner: "COR24 native assembly language, the foundation all compilers target",
        repo: "sw-cor24-assembler",
        section_id: "lang-assembler",
    },
    LangSummary {
        id: "apl",
        label: "apl-sw",
        inspired_by: "APL",
        one_liner: "Array-oriented language with concise notation for vector/matrix operations",
        repo: "sw-cor24-apl",
        section_id: "lang-apl",
    },
    LangSummary {
        id: "basic",
        label: "basic-sw",
        inspired_by: "BASIC",
        one_liner: "Classic beginner-friendly language with line-numbered imperative style",
        repo: "sw-cor24-basic",
        section_id: "lang-basic",
    },
    LangSummary {
        id: "forth",
        label: "forth-sw",
        inspired_by: "FORTH",
        one_liner: "Stack-based extensible language running via direct threaded code",
        repo: "sw-cor24-forth",
        section_id: "lang-forth",
    },
    LangSummary {
        id: "fortran",
        label: "fortran-sw",
        inspired_by: "FORTRAN",
        one_liner: "Numeric computing language; compiler in SNOBOL4, runtime in PL/SW",
        repo: "sw-cor24-fortran",
        section_id: "lang-fortran",
    },
    LangSummary {
        id: "hlasm",
        label: "hlasm-sw",
        inspired_by: "IBM HLASM",
        one_liner: "High-level assembler with structured macros and symbolic expressions for COR24",
        repo: "sw-cor24-hlasm",
        section_id: "lang-hlasm",
    },
    LangSummary {
        id: "lisp",
        label: "macrolisp-sw",
        inspired_by: "Lisp",
        one_liner: "Macro Lisp with first-class functions, closures, and mark-sweep GC",
        repo: "sw-cor24-macrolisp",
        section_id: "lang-lisp",
    },
    LangSummary {
        id: "ocaml",
        label: "ocaml-sw",
        inspired_by: "OCaml",
        one_liner: "Functional language compiling to P-code via the Pascal P-code VM pipeline",
        repo: "sw-cor24-ocaml",
        section_id: "lang-ocaml",
    },
    LangSummary {
        id: "pascal",
        label: "pascal-sw",
        inspired_by: "Pascal",
        one_liner: "Structured language compiling to P-code; strong typing with procedures",
        repo: "sw-cor24-pascal",
        section_id: "lang-pascal",
    },
    LangSummary {
        id: "plsw",
        label: "PL/SW",
        inspired_by: "PL/I",
        one_liner: "PL/I-inspired systems language with rich types, pointers, and inline ASM",
        repo: "sw-cor24-plsw",
        section_id: "lang-plsw",
    },
    LangSummary {
        id: "prolog",
        label: "prolog-sw",
        inspired_by: "Prolog",
        one_liner: "Logic programming with WAM-like 8+8 register VM implemented in PL/SW",
        repo: "sw-cor24-prolog",
        section_id: "lang-prolog",
    },
    LangSummary {
        id: "rpg-ii",
        label: "rpg-ii-sw",
        inspired_by: "RPG II",
        one_liner: "Simplified RPG-II report generator, compiled via HLASM for COR24",
        repo: "sw-cor24-rpg-ii",
        section_id: "lang-rpg-ii",
    },
    LangSummary {
        id: "smalltalk",
        label: "smalltalk-sw",
        inspired_by: "Smalltalk",
        one_liner: "Object-oriented messaging environment, implemented in COR24 BASIC",
        repo: "sw-cor24-smalltalk",
        section_id: "lang-smalltalk",
    },
    LangSummary {
        id: "snobol4",
        label: "SNOBOL4",
        inspired_by: "SNOBOL4",
        one_liner: "Pattern-matching language for string processing, written in PL/SW",
        repo: "sw-cor24-snobol4",
        section_id: "lang-snobol4",
    },
    LangSummary {
        id: "sws",
        label: "sws",
        inspired_by: "Tcl",
        one_liner: "Tcl-like scripting language for quick automation and glue code",
        repo: "sw-cor24-script",
        section_id: "lang-sws",
    },
];

static APL_GLYPH_TABLE: [GlyphRow; 28] = [
    GlyphRow {
        latin: "*",
        glyph: "\u{00d7}",
        monadic: "signum",
        dyadic: "multiply",
    },
    GlyphRow {
        latin: "+",
        glyph: "+",
        monadic: "N/A",
        dyadic: "add",
    },
    GlyphRow {
        latin: "-",
        glyph: "\u{2212}",
        monadic: "negate",
        dyadic: "subtract",
    },
    GlyphRow {
        latin: "/",
        glyph: "\u{00f7}",
        monadic: "N/A",
        dyadic: "divide",
    },
    GlyphRow {
        latin: "=",
        glyph: "=",
        monadic: "N/A",
        dyadic: "equal (0 or 1)",
    },
    GlyphRow {
        latin: "<",
        glyph: "<",
        monadic: "N/A",
        dyadic: "less than (0 or 1)",
    },
    GlyphRow {
        latin: ">",
        glyph: ">",
        monadic: "N/A",
        dyadic: "greater than (0 or 1)",
    },
    GlyphRow {
        latin: "<=",
        glyph: "\u{2264}",
        monadic: "N/A",
        dyadic: "less than or equal (0 or 1)",
    },
    GlyphRow {
        latin: ">=",
        glyph: "\u{2265}",
        monadic: "N/A",
        dyadic: "greater than or equal (0 or 1)",
    },
    GlyphRow {
        latin: "!=",
        glyph: "\u{2260}",
        monadic: "N/A",
        dyadic: "not equal (0 or 1)",
    },
    GlyphRow {
        latin: "_",
        glyph: "\u{00af}",
        monadic: "negate each element",
        dyadic: "N/A",
    },
    GlyphRow {
        latin: "and",
        glyph: "\u{2227}",
        monadic: "N/A",
        dyadic: "bitwise AND",
    },
    GlyphRow {
        latin: "cap",
        glyph: "\u{2229}",
        monadic: "N/A",
        dyadic: "intersection",
    },
    GlyphRow {
        latin: "cat",
        glyph: ",",
        monadic: "ravel (flatten to 1D)",
        dyadic: "catenate (join arrays)",
    },
    GlyphRow {
        latin: "ceil",
        glyph: "\u{2308}",
        monadic: "N/A",
        dyadic: "ceiling / max",
    },
    GlyphRow {
        latin: "compress",
        glyph: "/",
        monadic: "N/A",
        dyadic: "boolean compress (select)",
    },
    GlyphRow {
        latin: "cup",
        glyph: "\u{222a}",
        monadic: "unique (remove duplicates)",
        dyadic: "union",
    },
    GlyphRow {
        latin: "drop",
        glyph: "\u{2193}",
        monadic: "N/A",
        dyadic: "drop first N elements/rows",
    },
    GlyphRow {
        latin: "floor",
        glyph: "\u{230a}",
        monadic: "N/A",
        dyadic: "floor / min",
    },
    GlyphRow {
        latin: "fmt",
        glyph: "\u{2355}",
        monadic: "format (int to char vector)",
        dyadic: "N/A",
    },
    GlyphRow {
        latin: "iota",
        glyph: "\u{2373}",
        monadic: "index generator (\u{03b9}N)",
        dyadic: "N/A",
    },
    GlyphRow {
        latin: "not",
        glyph: "\u{223c}",
        monadic: "bitwise complement",
        dyadic: "N/A",
    },
    GlyphRow {
        latin: "or",
        glyph: "\u{2228}",
        monadic: "N/A",
        dyadic: "bitwise OR",
    },
    GlyphRow {
        latin: "pick",
        glyph: "\u{2283}",
        monadic: "N/A",
        dyadic: "index (pick element)",
    },
    GlyphRow {
        latin: "rev",
        glyph: "\u{233d}",
        monadic: "reverse",
        dyadic: "N/A",
    },
    GlyphRow {
        latin: "rho",
        glyph: "\u{2374}",
        monadic: "shape-of",
        dyadic: "reshape",
    },
    GlyphRow {
        latin: "roll",
        glyph: "?",
        monadic: "random integer 1..N",
        dyadic: "N/A",
    },
    GlyphRow {
        latin: "take",
        glyph: "\u{2191}",
        monadic: "N/A",
        dyadic: "take first N elements/rows",
    },
];

static APL_KEYWORD_TABLE: [KeywordRow; 10] = [
    KeywordRow {
        keyword: "assign",
        glyph: "\u{2190}",
        usage: "Assignment: NAME assign EXPR",
    },
    KeywordRow {
        keyword: "comment",
        glyph: "\u{235d}",
        usage: "Line or inline comment: comment text here",
    },
    KeywordRow {
        keyword: "del",
        glyph: "\u{2207}",
        usage: "Function definition: del FN(A) ...",
    },
    KeywordRow {
        keyword: "goto",
        glyph: "\u{2192}",
        usage: "Branch to label: goto LABEL",
    },
    KeywordRow {
        keyword: "qio",
        glyph: "\u{2395}IO",
        usage: "Index origin: qio (read), qio assign 0 or 1",
    },
    KeywordRow {
        keyword: "qled",
        glyph: "\u{2395}LED",
        usage: "LED output (write-only): qled assign 0 or 1",
    },
    KeywordRow {
        keyword: "qrl",
        glyph: "\u{2395}RL",
        usage: "PRNG seed: qrl (read), qrl assign N",
    },
    KeywordRow {
        keyword: "quad",
        glyph: "\u{2395}\u{2190}",
        usage: "Output: quad assign EXPR (print to UART)",
    },
    KeywordRow {
        keyword: "qsvo",
        glyph: "\u{2395}SVO",
        usage: "Shared-variable offer: NAME qsvo ADDR",
    },
    KeywordRow {
        keyword: "qsw",
        glyph: "\u{2395}SW",
        usage: "Switch input (read-only): read via qsvo",
    },
];

static DETAILS: [LangDetail; 15] = [
    LangDetail {
        id: "a24",
        label: "a24-sw",
        inspired_by: "Assembler",
        section_id: "lang-assembler",
        history: "The COR24 assembler (as24) is the native language of the processor. Written in C, it runs directly on COR24 hardware as part of the self-hosting toolchain. The cross-assembler (x-assembler) provides a Rust API for host-side assembly.",
        purpose: "Direct hardware control and maximum performance. Every higher-level language ultimately targets this layer. Understanding assembly is essential for systems programming, debugging, and runtime development on COR24.",
        usage: "Write .s source files with mnemonic instructions (LDA, STA, ADD, CMP, BNE, JSR, etc.), assemble with as24, and run the resulting binary in the emulator or on hardware.",
        pros: &[
            "Complete hardware control",
            "Zero runtime overhead",
            "Foundation for all other languages",
        ],
        cons: &[
            "Verbose for complex logic",
            "No abstraction or type safety",
            "Manual register and memory management",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "apl",
        label: "apl-sw",
        inspired_by: "APL",
        section_id: "lang-apl",
        history: "Originally created by Kenneth Iverson in the 1960s as a concise mathematical notation for teaching and computation. COR24 APL uses ASCII surface syntax with lowercase keywords (rho, iota, take, drop) and uppercase user identifiers, adapted for the constrained COR24 environment.",
        purpose: "Array-oriented computation. APL excels at expressing operations on entire vectors and matrices in a single line, making it ideal for linear algebra, data transformation, and numeric exploration.",
        usage: "Write expressions directly in the REPL; results display automatically. Vectors are first-class: 2 3 rho iota 6 creates a 2x3 matrix. Shared variables (SVO) provide hardware I/O access.",
        pros: &[
            "Extremely concise for array operations",
            "No boilerplate; results print automatically",
            "Powerful reduction and scan operators",
        ],
        cons: &[
            "No user-defined functions (deferred)",
            "Integer-only arithmetic",
            "Steep learning curve for glyph syntax",
        ],
        glyph_table: Some(&APL_GLYPH_TABLE),
        keyword_table: Some(&APL_KEYWORD_TABLE),
    },
    LangDetail {
        id: "basic",
        label: "basic-sw",
        inspired_by: "BASIC",
        section_id: "lang-basic",
        history: "Beginner's All-purpose Symbolic Instruction Code, created in 1964 at Dartmouth College. The COR24 port follows classic line-numbered BASIC style, compiled to P-code via the Pascal compiler toolchain.",
        purpose: "An accessible entry point for programming on COR24. BASIC's simple English-like syntax makes it approachable for beginners while still supporting structured programming constructs like FOR/NEXT loops and GOSUB/RETURN.",
        usage: "Write programs with line numbers, use LET for assignment, PRINT for output, FOR/NEXT for loops, and IF/THEN for conditionals. GOSUB/RETURN provides simple subroutine support.",
        pros: &[
            "Easy to learn and read",
            "Immediate feedback in interactive mode",
            "Familiar to generations of programmers",
        ],
        cons: &[
            "No local variables or scope",
            "Limited to 26 scalar variables (A-Z)",
            "No user-defined functions",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "forth",
        label: "forth-sw",
        inspired_by: "FORTH",
        section_id: "lang-forth",
        history: "Created by Charles Moore in the 1970s, Forth is a stack-based language known for its extreme extensibility. The COR24 implementation uses direct threaded code on native hardware and is explored along four axes of self-hosting: (1) the original all-asm kernel forth.s (~2600 lines, every word in COR24 assembly); (2) forth-in-forth, a tiered kernel where control flow, stack helpers, comments, and diagnostics live in .fth source on top of a slimmed asm kernel; (3) forth-on-forthish, a minimal-primitive kernel (~22 irreducible asm primitives: threading, +, NAND, memory, I/O, SP@/RP@) with :, ;, WORD, FIND, NUMBER, and the outer loop written in Forth; and (4) forth-from-forth, a future self-hosted bootstrap where a Forth cross-compiler emits the kernel .s as a build artifact.",
        purpose: "Interactive systems programming and hardware exploration. Forth's word-based architecture lets you extend the language itself at runtime, making it ideal for building domain-specific vocabularies for hardware control. The four-approach progression doubles as a teaching artifact for how much of a Forth system can be Forth itself.",
        usage: "Define words with colon definitions (: SQUARE DUP * ;), test them immediately in the REPL. Use BEGIN..WHILE..REPEAT or DO..LOOP for loops, IF..THEN..ELSE for conditionals. IN@ and OUT@ provide hardware I/O. The web UI exposes tabs for each self-hosting tier so the same demos run on forth.s, forth-in-forth, and forth-on-forthish kernels side-by-side.",
        pros: &[
            "Extremely fast compilation and execution",
            "Infinitely extensible at runtime",
            "Minimal memory footprint",
        ],
        cons: &[
            "Stack-based paradigm is unfamiliar",
            "No type safety",
            "Difficult to read complex programs",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "fortran",
        label: "fortran-sw",
        inspired_by: "FORTRAN",
        section_id: "lang-fortran",
        history: "Formula Translation, the first high-level language (1957). The COR24 Fortran compiler is written in SNOBOL4 with a PL/SW runtime, targeting COR24 assembly for scientific and numeric computing workloads.",
        purpose: "Scientific and engineering computation. Fortran excels at numeric-heavy programs, matrix operations, and simulations where performance on constrained hardware matters.",
        usage: "In development. Compiler written in SNOBOL4 translates Fortran source to COR24 assembly, with runtime support provided by PL/SW.",
        pros: &[
            "Optimized for numeric computation",
            "Well-suited to COR24's integer architecture",
            "Mature language with proven patterns",
        ],
        cons: &[
            "Currently in development",
            "Verbose syntax for simple tasks",
            "Limited string handling",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "hlasm",
        label: "hlasm-sw",
        inspired_by: "IBM HLASM",
        section_id: "lang-hlasm",
        history: "HLASM (High-Level Assembler) extends the COR24 assembly language with structured macros, symbolic expressions, and higher-level constructs. Written in COR24 assembly, it runs natively on the target platform as part of the self-hosting toolchain.",
        purpose: "Bridge the gap between raw assembly and higher-level languages. HLASM provides structured programming constructs (IF/ELSE, DO/WHILE), macro definitions with parameters, and symbolic expressions that make COR24 assembly more readable and maintainable.",
        usage: "Write .s source files with HLASM directives and macros. HLASM expands macros and evaluates symbolic expressions, producing pure COR24 assembly for the native assembler.",
        pros: &[
            "Structured programming in assembly",
            "Powerful macro system with parameters",
            "Runs on COR24 hardware",
        ],
        cons: &[
            "Currently in development",
            "Adds complexity over raw assembly",
            "Requires understanding of both HLASM and base assembly",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "lisp",
        label: "macrolisp-sw",
        inspired_by: "Lisp",
        section_id: "lang-lisp",
        history: "List Processing, created by John McCarthy in 1958. The COR24 Macro Lisp implementation provides first-class functions, closures, macros with quasiquote, and a conservative mark-sweep garbage collector, compiled to native COR24 assembly.",
        purpose: "Symbolic computation and metaprogramming. Lisp's homoiconic structure (code is data) makes it uniquely powerful for writing programs that write programs, building interpreters, and exploring recursive algorithms.",
        usage: "Define functions with defun, use lambda for anonymous functions. Built-in list operations (cons, car, cdr), higher-order functions (map, filter, reduce), and macros (defmacro) for metaprogramming. peek/poke and set-leds for hardware I/O.",
        pros: &[
            "First-class functions and closures",
            "Macros and quasiquote for metaprogramming",
            "Mark-sweep GC handles memory automatically",
        ],
        cons: &[
            "Parenthesis-heavy syntax",
            "Performance overhead from GC",
            "Integer-only on COR24 (no floating point)",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "ocaml",
        label: "ocaml-sw",
        inspired_by: "OCaml",
        section_id: "lang-ocaml",
        history: "Originally created by INRIA in 1996, OCaml combines functional, imperative, and \
         object-oriented programming. The COR24 port is implemented in Pascal and compiles a subset \
         of OCaml to P-code bytecode, reusing the Pascal P-code VM infrastructure on COR24.",
        purpose: "Functional programming with type inference and pattern matching on COR24. OCaml's \
         strong static type system and algebraic data types make it well-suited for writing correct, \
         expressive programs on constrained hardware.",
        usage: "In development. Implemented in Pascal, compiles OCaml source to .spc P-code via the \
         Pascal compiler toolchain, running on the P-code VM on COR24.",
        pros: &[
            "Type inference reduces annotations",
            "Pattern matching is natural for COR24's integer arithmetic",
            "Reuses proven P-code VM infrastructure",
        ],
        cons: &[
            "P-code layer adds runtime overhead",
            "Subset only — full OCaml is too large for COR24",
            "No live demo yet",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "pascal",
        label: "pascal-sw",
        inspired_by: "Pascal",
        section_id: "lang-pascal",
        history: "Created by Niklaus Wirth in 1970 as a teaching language emphasizing structured programming. The COR24 compiler (p24p) is written in C and compiles Pascal source to P-code bytecode, executed by the P-code VM on COR24.",
        purpose: "Structured systems programming with strong typing. Pascal enforces good practices with explicit variable declarations, typed procedures and functions, and clear block structure.",
        usage: "Declare variables with var blocks, define procedures and functions with typed parameters. Use for..to/downto, while, and repeat..until for loops. Compiles to .spc P-code, linked and assembled for the P-code VM.",
        pros: &[
            "Strong typing catches errors early",
            "Clean structured syntax",
            "Compiles to portable P-code bytecode",
        ],
        cons: &[
            "P-code layer adds runtime overhead",
            "Limited string support",
            "No pointer types",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "plsw",
        label: "PL/SW",
        inspired_by: "PL/I",
        section_id: "lang-plsw",
        history: "PL/SW is a PL/I-inspired language designed specifically for the COR24 ecosystem. It combines PL/I's rich type system with systems programming features like inline assembly, pointers, and level-based data declarations.",
        purpose: "Systems-level programming with high-level abstractions. PL/SW targets the gap between low-level assembly and high-level languages, providing rich types (BIT, BYTE, WORD, INT, CHAR, PTR) alongside direct hardware access.",
        usage: "Declare variables with DCL and type specifiers. Use PROC/END for procedures, DO WHILE and counted DO for loops. Inline ASM blocks (ASM DO; ... END;) provide direct hardware control. MACRODEF and ?MACRO() for metaprogramming.",
        pros: &[
            "Rich type system (6 types)",
            "Inline assembly for hardware access",
            "Macro system with GEN blocks",
        ],
        cons: &[
            "Complex syntax for simple tasks",
            "Steeper learning curve than BASIC/SWS",
            "Type declarations add verbosity",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "prolog",
        label: "prolog-sw",
        inspired_by: "Prolog",
        section_id: "lang-prolog",
        history: "Prolog (Programming in Logic) was created by Alain Colmerauer and Robert Kowalski in 1972. \
         The COR24 implementation uses a WAM-like (Warren Abstract Machine) architecture with an 8+8 \
         register virtual machine built in PL/SW, providing logic programming on COR24.",
        purpose: "Logic programming and symbolic AI on COR24. Prolog's declarative style — defining \
         what is true rather than how to compute it — makes it ideal for rule-based systems, \
         symbolic computation, and natural language processing.",
        usage: "In development. Will define facts and rules, query with goals, and use unification \
         and backtracking via the WAM-like VM. The 8+8 register architecture (8 argument + 8 \
         temporary registers) maps efficiently to COR24 hardware.",
        pros: &[
            "Declarative style is concise for rule-based problems",
            "WAM-like VM is well-understood and efficient",
            "Built on PL/SW, leveraging existing infrastructure",
        ],
        cons: &[
            "Implemented in PL/SW (two-language stack)",
            "No live demo yet",
            "Backtracking can be expensive on constrained hardware",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "rpg-ii",
        label: "rpg-ii-sw",
        inspired_by: "RPG II",
        section_id: "lang-rpg-ii",
        history: "RPG II (Report Program Generator) was created by IBM in the 1960s for business data \
         processing on minicomputers. The COR24 implementation is a simplified RPG-II that compiles \
         through HLASM, targeting report generation and file processing workloads on COR24.",
        purpose: "Business data processing and report generation. RPG-II's fixed-format specifications \
         (input, calculation, output) provide a structured approach to file processing, making it \
         well-suited for data transformation and reporting tasks.",
        usage: "In development. Define file descriptions, input specifications, calculation specifications, \
         and output specifications in RPG-II fixed-format source. Compiles through HLASM to COR24 \
         assembly for the assembler.",
        pros: &[
            "Excellent for report generation",
            "Fixed-format specifications are self-documenting",
            "Compiles through HLASM on COR24",
        ],
        cons: &[
            "Currently in development",
            "Fixed-format syntax is rigid",
            "Limited to batch/file processing",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "smalltalk",
        label: "smalltalk-sw",
        inspired_by: "Smalltalk",
        section_id: "lang-smalltalk",
        history: "Smalltalk was developed at Xerox PARC in the 1970s by Alan Kay, Dan Ingalls, \
         and Adele Goldberg, popularizing object-oriented programming, message passing, and the \
         modern integrated development environment. The COR24 implementation is a small Smalltalk \
         environment written in COR24 BASIC, ride-sharing with the BASIC/Pascal P-code stack rather \
         than introducing a new bytecode VM.",
        purpose: "Bring object-oriented programming and message passing to COR24 in a low-effort way. \
         Building on top of BASIC keeps the implementation tractable while still exposing the \
         classic Smalltalk feel: everything is an object, computation is messages, and code is \
         live and explorable.",
        usage: "In development. Source files use familiar Smalltalk syntax: keyword messages \
         (anObject doSomething: arg), unary messages (5 squared), and blocks ([:x | x * x]). \
         Programs are compiled by the BASIC-hosted Smalltalk frontend, then run on the P-code VM \
         the same way a BASIC program would.",
        pros: &[
            "Uniform message-passing model is small and elegant",
            "Reuses the existing BASIC + P-code stack \u{2014} no new VM to maintain",
            "Closures (blocks) come for free from the Smalltalk model",
        ],
        cons: &[
            "Newer project \u{2014} feature set is small and evolving",
            "Two-language stack (Smalltalk on top of BASIC on top of P-code) adds overhead",
            "Subset only \u{2014} no metaclass tower or full image semantics",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "snobol4",
        label: "SNOBOL4",
        inspired_by: "SNOBOL4",
        section_id: "lang-snobol4",
        history: "SNOBOL4 (String Oriented Symbolic Language) is a classic pattern-matching language from the 1960s. This implementation runs on COR24 and is written in PL/SW, demonstrating PL/SW as a host language for building interpreters.",
        purpose: "String processing and pattern matching. SNOBOL4 excels at text manipulation, parsing, and symbolic computation using its powerful pattern-matching primitives.",
        usage: "Assign with =, print with OUTPUT = expr. Pattern match with subject PAT . capture :F(label). Use SPAN, BREAK, REM, and literal patterns. Branch on success/failure with :S(label) and :F(label). Loop with :(label). Arrays with ARRAY() and <> indexing.",
        pros: &[
            "Powerful pattern matching (SPAN, BREAK, REM)",
            "Success/failure flow control",
            "String concatenation is implicit",
        ],
        cons: &[
            "No user-defined functions yet",
            "Unfamiliar control flow (explicit GOTO with success/failure branching)",
            "Terse pattern syntax has a steep learning curve",
        ],
        glyph_table: None,
        keyword_table: None,
    },
    LangDetail {
        id: "sws",
        label: "sws",
        inspired_by: "Tcl",
        section_id: "lang-sws",
        history: "SWS is a Tcl-like scripting language designed for quick automation and glue code on COR24. Implemented in C and compiled to COR24 assembly via tc24r, it provides dynamic typing with integer and string values.",
        purpose: "Rapid scripting and automation. SWS fills the role of shell scripting on COR24: quick one-liners, configuration, glue code between subsystems, and interactive testing of hardware interfaces.",
        usage: "Use set/expr for variables and expressions, if/while for control flow, echo for output, and proc for procedure definitions. Tcl-style substitution with $var and [expr {...}] for embedded expressions.",
        pros: &[
            "Quick to write and test interactively",
            "Dynamic typing is flexible",
            "Tcl-style substitution is powerful",
        ],
        cons: &[
            "No user-defined functions yet (v0.1)",
            "No local variable scope",
            "Interpreted execution is slower than compiled",
        ],
        glyph_table: None,
        keyword_table: None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summaries_alphabetical() {
        let labels: Vec<&str> = summaries().iter().map(|s| s.label).collect();
        let mut sorted = labels.clone();
        sorted.sort_by_key(|a| a.to_lowercase());
        assert_eq!(labels, sorted, "summaries are not in alphabetical order");
    }

    #[test]
    fn details_alphabetical() {
        let labels: Vec<&str> = all_details().iter().map(|d| d.label).collect();
        let mut sorted = labels.clone();
        sorted.sort_by_key(|a| a.to_lowercase());
        assert_eq!(labels, sorted, "details are not in alphabetical order");
    }

    #[test]
    fn summaries_count() {
        assert_eq!(summaries().len(), 15);
    }

    #[test]
    fn details_count() {
        assert_eq!(all_details().len(), 15);
    }
}
