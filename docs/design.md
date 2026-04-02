# Design -- COR24 Ecosystem Landing Page

## Design Principles

1. **Content-first** -- The page is a documentation hub. Text, tables, and links are primary; decorative elements are minimal.
2. **Zero coupling** -- No path dependencies on other sw-embed repos. All content is compile-time embedded static data.
3. **Consistent identity** -- Catppuccin Mocha theme matching all other COR24 web demos. A visitor should immediately recognize this belongs to the same ecosystem.
4. **Scannable** -- Heavy use of tables, cards, and grids rather than walls of text.
5. **Incremental** -- Build the shell first, then fill in content section by section.

## Theme & Colors

Catppuccin Mocha palette (CSS custom properties):

```css
:root {
    --ctp-rosewater: #f5e0dc;
    --ctp-flamingo: #f2cdcd;
    --ctp-pink: #f5c2e7;
    --ctp-mauve: #cba6f7;
    --ctp-red: #f38ba8;
    --ctp-maroon: #eba0ac;
    --ctp-peach: #fab387;
    --ctp-yellow: #f9e2af;
    --ctp-green: #a6e3a1;
    --ctp-teal: #94e2d5;
    --ctp-sky: #89dceb;
    --ctp-sapphire: #74c7ec;
    --ctp-blue: #89b4fa;
    --ctp-lavender: #b4befe;
    --ctp-text: #cdd6f4;
    --ctp-subtext1: #bac2de;
    --ctp-subtext0: #a6adc8;
    --ctp-overlay2: #9399b2;
    --ctp-overlay1: #7f849c;
    --ctp-overlay0: #6c7086;
    --ctp-surface2: #585b70;
    --ctp-surface1: #45475a;
    --ctp-surface0: #313244;
    --ctp-base: #1e1e2e;
    --ctp-mantle: #181825;
    --ctp-crust: #11111b;
}
```

Role mapping:
- Background: `--ctp-base`
- Surface/cards: `--ctp-surface0`
- Borders: `--ctp-surface1`
- Body text: `--ctp-text`
- Headings: `--ctp-lavender`
- Links: `--ctp-blue`
- Accent (live badges): `--ctp-green`
- Warning (planned items): `--ctp-yellow`
- Code/mono: `--ctp-overlay2`

## Layout Design

### Header
- Fixed top bar
- Logo/title left: "COR24" in bold, "Ecosystem" in subtext
- Nav links right: Home | ISA | Demos | Toolchain | Ecosystem
- Hamburger menu on mobile (< 768px)

### Home Page
```
+--------------------------------------------------+
|  COR24 Ecosystem                                  |
|  24-bit RISC for FPGA embedded systems            |
|  [makerlisp.com]                                  |
|                                                   |
|  +---------+ +---------+ +---------+ +---------+ |
|  | Asm IDE | | P-code  | | Tiny C  | |  Lisp   | |
|  | [Live]  | | [Live]  | | [Live]  | | [Live]  | |
|  +---------+ +---------+ +---------+ +---------+ |
|  +---------+ +---------+ +---------+ +---------+ |
|  | Pascal  | |  APL    | |  Forth  | |  PL/SW  | |
|  | [Live]  | | [Live]  | | [Live]  | | [Live]  | |
|  +---------+ +---------+ +---------+ +---------+ |
|                                                   |
|  What is COR24?                                   |
|  A 24-bit RISC architecture designed for FPGA...  |
+--------------------------------------------------+
```

### ISA Reference Page
- Left sidebar with section navigation (collapsible on mobile)
- Main content area with tables and code blocks
- Each section has a table of contents

### Demos Page
- Filter bar at top (search + category dropdown)
- Grid of demo cards (3 columns desktop, 2 tablet, 1 mobile)
- Each card: name, description, status badge, tags, links

### Toolchain Page
- Grouped by category with section headers
- Each category is a list/grid of tool cards
- Pipeline diagrams as ASCII/text art or simple CSS diagrams

## Data Model

### Demo Entries (static data in Rust)

```rust
struct DemoEntry {
    name: &'static str,
    repo: &'static str,
    description: &'static str,
    live_url: &'static str,
    repo_url: &'static str,
    status: Status,       // Active | Beta | Planned
    tags: &'static [&'static str],
}

enum Status {
    Active,
    Beta,
    Planned,
}
```

### Tool Entries (static data in Rust)

```rust
struct ToolEntry {
    name: &'static str,
    repo: &'static str,
    description: &'static str,
    language: &'static str,   // "Rust", "C", "COR24 Assembly"
    target: &'static str,     // "COR24", "Host", "COR24 (via emulator)"
    repo_url: &'static str,
    demo_url: Option<&'static str>,
    category: ToolCategory,
}

enum ToolCategory {
    Foundation,
    CrossCompiler,
    PCode,
    NativeLanguage,
    SystemSoftware,
}
```

### ISA Data (static data in Rust)

```rust
struct Register {
    name: &'static str,
    alias: &'static str,
    purpose: &'static str,
    constraints: &'static str,
}

struct Instruction {
    mnemonic: &'static str,
    format: &'static str,
    encoding: &'static str,
    description: &'static str,
    category: InstrCategory,
}

struct MemoryRegion {
    name: &'static str,
    start: u32,
    end: u32,
    size: &'static str,
    description: &'static str,
}
```

## Component Design

### Router
- Simple enum-based routing (no yew-router dependency)
- `Route` enum with variants: Home, Isa, Demos, Toolchain, Ecosystem
- Rendered via `match` in App::view()

### Content Components
- Each page section is its own Yew component
- Data lives in separate `data/` modules (demo_data.rs, tool_data.rs, isa_data.rs)
- Components are pure rendering functions of their data props

### Responsive Design
- CSS Grid for card layouts
- `@media` queries at 768px and 1024px breakpoints
- Flexbox for navigation and single-row layouts

## File Structure (Planned)

```
src/
  main.rs              -- Yew entry point
  app.rs               -- Root component, routing
  components/
    header.rs          -- Top nav bar
    footer.rs          -- Page footer
    home/
      mod.rs
      hero.rs          -- Hero section
      demo_grid.rs     -- Quick-link demo cards
    isa/
      mod.rs
      registers.rs     -- Register reference table
      instructions.rs  -- Instruction set tables
      memory_map.rs    -- Memory map table
      calling_conv.rs  -- Calling convention docs
    demos/
      mod.rs
      demo_card.rs     -- Individual demo card
      demo_grid.rs     -- Filtered grid of demos
    toolchain/
      mod.rs
      tool_card.rs     -- Individual tool card
      category.rs      -- Tool category section
      pipelines.rs     -- Pipeline diagrams
    ecosystem/
      mod.rs
      repo_map.rs      -- Repository grid
  data/
    mod.rs
    demos.rs           -- DemoEntry static data
    tools.rs           -- ToolEntry static data
    isa.rs             -- Register/Instruction/MemoryRegion data
  styles/
    theme.rs           -- CSS custom properties (embedded)
    layout.css         -- Page structure styles
    components.css     -- Component-specific styles
index.html
scripts/
  serve.sh
  build-pages.sh
build.rs
```

## Build Configuration

### Cargo.toml Dependencies

```toml
[dependencies]
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["window", "document", "html_element"] }

[dependencies.web-sys]
version = "0.3"
features = [
    "Window",
    "Document",
    "Element",
    "HtmlElement",
    "History",
    "ScrollIntoViewOptions",
]

[profile.release]
opt-level = "z"
lto = true
```

### Trunk Configuration (in index.html)

```html
<!DOCTYPE html>
<html>
<head>
    <title>COR24 Ecosystem</title>
    <link data-trunk rel="css" href="src/styles/theme.css">
    <link data-trunk rel="css" href="src/styles/layout.css">
    <link data-trunk rel="css" href="src/styles/components.css">
</head>
<body>
    <link data-trunk rel="rust" data-wasm-opt="z" />
</body>
</html>
```
