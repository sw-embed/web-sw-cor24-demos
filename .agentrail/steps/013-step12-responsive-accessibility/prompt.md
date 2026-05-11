Phase 7, Step 12: Responsive Design + Accessibility

Polish all pages for responsive design and accessibility.

Tasks:
1. Test and fix all pages at three breakpoints:
   - Mobile: 320px-767px (single column, hamburger menu)
   - Tablet: 768px-1023px (two columns)
   - Desktop: 1024px+ (full layout)
2. Fix specific responsive issues:
   - Header nav collapses to hamburger on mobile
   - ISA sidebar becomes top nav or collapsible on mobile
   - Demo/tool grids stack to single column on mobile
   - Pipeline diagrams wrap or scroll horizontally on small screens
   - Tables get horizontal scroll wrapper on mobile
   - Font sizes scale appropriately
3. Accessibility fixes:
   - Verify h1 -> h2 -> h3 heading hierarchy across all pages
   - Add aria-label to interactive elements (hamburger button, filter dropdowns)
   - Ensure all links have descriptive text (no 'click here')
   - Check color contrast ratios (Catppuccin Mocha should be fine, but verify)
   - Add skip-to-content link for keyboard navigation
   - Ensure focus styles are visible
4. Test keyboard navigation:
   - Tab through all interactive elements
   - Verify Enter/Space activates buttons and links
   - Escape closes any open menus/dropdowns
5. Verify: pages render correctly at all breakpoints, no horizontal overflow, keyboard navigation works