Phase 1, Step 2: App Shell + Routing + Theme

Build the application shell with routing, navigation, and full Catppuccin Mocha theming.

Tasks:
1. Create src/app.rs with App component implementing enum-based routing (Route enum: Home, Isa, Demos, Toolchain, Ecosystem)
2. Implement simple hash-based or state-based routing (no yew-router dependency) using web-sys History API
3. Create src/components/header.rs - fixed top nav bar with:
   - COR24 logo/title on left
   - Nav links on right: Home | ISA | Demos | Toolchain | Ecosystem
   - Mobile hamburger menu (< 768px breakpoint)
   - Active section highlighting
4. Create src/components/footer.rs with copyright, makerlisp.com link, GitHub org link
5. Create src/styles/layout.css with:
   - Page grid layout
   - Responsive breakpoints (320px, 768px, 1024px+)
   - Content max-width and padding
6. Create src/styles/components.css with:
   - Card base styles (border, padding, hover)
   - Table styles (striped rows, headers)
   - Badge/tag styles (active=green, beta=yellow, planned=overlay)
   - Nav link hover/active states
7. Wire up all route components as placeholder stubs that show their section name
8. Verify: navigation between all 5 sections works, theme renders with Catppuccin Mocha colors, mobile hamburger menu works

Follow the component patterns from other web-sw-cor24-* demos (Component trait, Msg enum, html! macro).