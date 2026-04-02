Phase 1, Step 1: Rust/Yew/Trunk Project Init

Set up the complete project scaffold for the COR24 Ecosystem Landing Page.

Tasks:
1. Update Cargo.toml with Yew 0.21 (csr feature), wasm-bindgen 0.2, web-sys 0.3 (Window, Document, Element, HtmlElement, History, ScrollIntoViewOptions features)
2. Configure release profile: opt-level = 'z', lto = true
3. Create index.html with Trunk directives, Catppuccin Mocha theme link, and data-trunk rust/wasm-opt attributes
4. Create scripts/serve.sh (dev server with hot reload, using trunk serve with correct flags)
5. Create scripts/build-pages.sh (release build to dist/ then rsync to pages/, preserving .nojekyll)
6. Create build.rs embedding BUILD_SHA, BUILD_HOST, BUILD_TIMESTAMP via env vars
7. Update .gitignore to include target/, dist/
8. Create .github/workflows/pages.yml for GitHub Pages deployment of pages/ on push to main
9. Create src/styles/ directory with theme.css (Catppuccin Mocha CSS custom properties)
10. Create a minimal src/lib.rs with empty App component that renders 'COR24 Ecosystem'
11. Update src/main.rs to call yew::Renderer::<App>::new().render()

Reference other web-sw-cor24-* repos for script patterns:
- web-sw-cor24-assembler/scripts/serve.sh
- web-sw-cor24-plsw/scripts/build-pages.sh

Verify: cargo check --target wasm32-unknown-unknown passes, scripts/serve.sh starts successfully