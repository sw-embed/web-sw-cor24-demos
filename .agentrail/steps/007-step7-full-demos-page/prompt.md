Phase 4, Step 7: Full Demos Page

Build the complete demos directory page with filtering.

Tasks:
1. Create src/components/demos/mod.rs with DemosPage component
2. Create src/components/demos/demo_grid.rs with filter functionality:
   - Search input field (filter by name/description)
   - Category dropdown filter: All, Compiler, Interpreter, IDE, Debugger, Docs
   - Status dropdown filter: All, Active, Beta, Planned
   - Count indicator showing 'X of Y demos'
3. Reuse DemoCard component from step 3 (import from home module or extract to shared)
4. Add planned/future demo entries to data/demos.rs:
   - web-sw-cor24-debugger: Source-level Debugger (planned, debugger)
   - web-sw-cor24-basic: BASIC Interpreter (planned, interpreter)
   - web-sw-cor24-fortran: Fortran Compiler (planned, compiler)
   - web-sw-cor24-script: SWS Scripting Environment (planned, interpreter)
5. Implement filtering logic in DemosPage update() method
6. Show 'No demos match your filter' message when filter returns empty
7. Verify: all filters work independently and combined, planned demos show with yellow badge, responsive on mobile