---
name: layoza-accessibility
description: WCAG 2.2 AA accessibility engineering, relative luminance contrast verification, keyboard focus management, ARIA semantics, target size enforcement, and automated vs manual audit separation.
---

# Layoza Specialist Skill: Accessibility (`layoza-accessibility`)

Use this skill when auditing or remediating accessibility across components, color palettes, forms, modals, and interactive animations.

## 1. Non-Negotiable WCAG 2.2 AA Standards
1. **Color Contrast (SC 1.4.3 & 1.4.11)**:
   - Normal body text (`< 18pt` or `< 14pt bold`): **`>= 4.5:1`** contrast ratio.
   - Large text (`>= 18pt` or `>= 14pt bold`): **`>= 3.0:1`** contrast ratio.
   - Interactive UI boundaries and icons: **`>= 3.0:1`** contrast ratio.
2. **Keyboard Operability & Focus Visibility (SC 2.1.1, 2.4.7, 2.4.11)**:
   - Every interactive control must be reachable via `Tab` / `Shift+Tab` using native `<button>` or `<a href>` elements.
   - Never use `outline: none` without a high-contrast `:focus-visible` ring (`outline: 2px solid var(--color-primary); outline-offset: 2px`).
   - Modals and drawers must trap focus while open, close on `Escape`, and restore focus to the trigger on close.
3. **Target Size Minimum (SC 2.5.8)**:
   - Interactive touch/pointer targets must be at least `24x24px` CSS pixels (`44x44px` strongly recommended on mobile).
4. **Motion Actuation & Reduced Motion (SC 2.3.3)**:
   - Respect `@media (prefers-reduced-motion: reduce)` across all CSS and JS animations.

## 2. Automated Checks vs Manual Verification
Layoza explicitly separates what automated tools (`axe-core`, `layoza audit`) can prove from what requires human or keyboard-flow verification:
- **Automated (`axe-core` + token math)**: Contrast ratios, missing alt text, missing form labels, duplicate IDs, invalid ARIA roles, heading hierarchy skips, horizontal overflow.
- **Manual / Interactive Checklists**: Logical screen-reader reading order, focus trap restoration across multi-step flows, cognitive clarity of error recovery, custom WebGL canvas alternatives.

## 3. WCAG 2.2 additions to check on every app screen
- **2.4.11 Focus Not Obscured (AA):** a focused control is never fully hidden by a sticky header, footer, cookie bar or toast. Set `scroll-padding-top` / `scroll-margin` to the sticky bar's height.
- **2.5.7 Dragging Movements (AA):** every drag (reorder, slider, map pan, kanban) has a single-pointer alternative: move up/down buttons, a "Move to…" menu, or click-to-place.
- **2.5.8 Target Size Minimum (AA):** 24×24 CSS px, or 24px spacing circles that don't overlap; inline text links are exempt. Compact density shrinks visuals, never hit areas.
- **3.3.7 Redundant Entry (A):** never ask twice for data given earlier in the same flow; prefill it or offer "Same as billing".
- **3.3.8 Accessible Authentication (AA):** no cognitive test to log in; allow paste and password managers, offer passkeys, magic links or OTP with `autocomplete="one-time-code"`.
- **2.2.2 Pause, Stop, Hide (A):** anything moving, blinking or auto-updating for more than 5s (carousels, tickers, live feeds) has a pause control.

## 4. Async updates and data patterns
- **Live regions:** one persistent `aria-live="polite"` region per view, present in the DOM before content changes; announce results counts, "Saved", "3 items deleted, Undo". Use `role="alert"` only for blocking errors. Never announce every keystroke; debounce ~500ms.
- **Loading:** `aria-busy="true"` on the updating region; move focus only when the user asked for the new content.
- **Tables:** real `<table>` with `<caption>`, `<th scope="col|row">`, `aria-sort` on the sorted header's `<th>`; selection checkboxes labelled per row ("Select INV-2041").
- **Grids:** `role="grid"` only with the full APG keyboard model (one tab stop, arrows, Home/End, Enter/F2 to edit, Esc); virtual rows keep `aria-rowcount` and `aria-rowindex`.
