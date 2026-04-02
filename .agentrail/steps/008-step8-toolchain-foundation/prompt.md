Phase 5, Step 8: Toolchain Data + Foundation Tools

Create the toolchain documentation with foundation tools.

Tasks:
1. Create src/data/tools.rs with:
   - ToolEntry struct (name, repo, description, language, target, repo_url, demo_url: Option, category)
   - ToolCategory enum (Foundation, CrossCompiler, PCode, NativeLanguage, SystemSoftware)
   - Static array for foundation tools:
     * sw-cor24-emulator: COR24 assembler and emulator (Rust, Host, has web demo)
     * sw-cor24-x-assembler: Cross-assembler library (Rust, Host)
     * sw-cor24-assembler: Native assembler (C, COR24)
     * sw-cor24-project: Ecosystem hub/portal (Docs, Host)
2. Create src/components/toolchain/mod.rs with ToolchainPage component
3. Create src/components/toolchain/tool_card.rs with ToolCard component:
   - Name and description
   - Language badge (Rust, C, Assembly, Docs)
   - Target badge (Host, COR24, COR24 via emulator)
   - GitHub link
   - 'Live Demo' link (if has_web_ui)
4. Create src/components/toolchain/category.rs with ToolCategorySection:
   - Category header with description
   - Grid of ToolCards within the category
5. Render foundation tools section on ToolchainPage
6. Verify: foundation tool cards render with correct links, language/target badges show, live demo links work