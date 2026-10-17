<!-- Layoza original. Keyboard maps follow the W3C WAI-ARIA Authoring Practices Guide patterns (Grid, Table, Combobox, Dialog); form rules adapt ideas from vercel-labs/web-interface-guidelines (MIT). Written in Layoza's own words. See THIRD_PARTY_NOTICES.md, "Product UI ideas". -->

# Tables, lists and forms

## Table: pick the pattern first

| Need | Markup | Keyboard |
| :--- | :--- | :--- |
| Read-only data, links/buttons in cells | `<table>` + `<th scope>` | Tab through the controls only |
| Cell-by-cell navigation, inline edit, spreadsheet feel | `role="grid"` | One tab stop; arrows move cells (APG Grid) |
| Selectable list of records | `<table>` + row checkboxes | Tab to row, Space toggles, Shift+click range |

Use `role="grid"` only when you implement its full keyboard model: arrows,
Home/End, Ctrl+Home/End, Page Up/Down, Enter or F2 to edit, Esc to cancel.
A half-built grid is worse than a plain table.

## Table anatomy

- Toolbar: search (debounced 200–300ms), filters as removable chips, column
  visibility, density switch, export. Filters, sort and page write to the URL.
- Header: `position: sticky; top: <shell offset>`; sortable `<th>` holds a
  `<button>` and carries `aria-sort="ascending|descending"` only on the sorted
  column. Sort icon shows direction, not just "sortable".
- First column sticky on horizontal scroll, with a 1px divider or shadow that
  appears only while scrolled.
- Cells: numbers right-aligned with tabular numerals; text left; status as
  dot + label; empty cell shows "—" with an accessible "No value".
- Truncation: `text-overflow: ellipsis` with `min-width: 0` on the cell
  content; the full value in a tooltip on hover and focus, or in the detail
  drawer. Never truncate IDs a user must copy; add a copy button instead.
- Row count and range ("1–50 of 1,284") stay visible; announce changes
  politely.
- Selection: header checkbox has three states (none, some = `indeterminate`,
  all on page) and offers "Select all 1,284" separately. The bulk bar replaces
  the toolbar in place (no layout jump) and holds "N selected", actions, Clear.
- Row actions: a kebab `<button aria-label="Actions for INV-2041">` per row;
  Enter on a focused row opens its detail; Delete/Backspace triggers the
  destructive action with confirm or undo.
- Virtualise above ~200 rows (fixed row height from the density token makes
  it cheap); keep `aria-rowcount` / `aria-rowindex` on virtual rows.

## Tables below 768px

Choose per table, decided by what users scan for:

1. **Priority columns:** keep the identifier plus 1–2 key values; the rest
   opens in a detail view. Horizontal scroll inside the table container only,
   with the first column sticky and a visible scroll affordance.
2. **Card list:** each row becomes a card: title line, 2–3 label/value pairs,
   status, one action. Sorting and filtering move to a sheet.

At 375px: no page-level horizontal scroll, tap targets ≥44px, bulk actions in
a bottom bar.

## Forms

| Field | `type` | `inputmode` | `autocomplete` |
| :--- | :--- | :--- | :--- |
| Email | `email` | (implied) | `email` |
| Phone | `tel` | (implied) | `tel` |
| One-time code | `text` | `numeric` | `one-time-code` |
| Amount | `text` | `decimal` | `off` or `transaction-amount` |
| Postcode | `text` | `text` (UK) / `numeric` (US) | `postal-code` |
| New password | `password` | | `new-password` |
| Search | `search` | | `off` |

- Layout: one column; related short fields (city, postcode) may pair. Label
  above, help text under the label, error under the field.
- Required vs optional: mark whichever is the minority ("Optional").
- Validation timing: on blur for format, on submit for everything; server
  errors map back to their fields. Never validate an empty field the user has
  not left yet.
- Error copy says what to do ("Enter a date after 1 March"), not what went
  wrong in code terms.
- Submit: enabled until the request starts; then `disabled` + `aria-busy`,
  spinner after ~150–300ms, label "Saving…". On failure, re-enable and focus
  the first error or the error summary.
- Never block paste, never disable autofill, never hide the password without a
  show toggle. Allow 1Password/Bitwarden to fill (stable `name`/`id`).
- Mobile: inputs ≥16px, `enterkeyhint` set, the keyboard does not cover the
  focused field (scroll it into view).
- Long forms: save drafts; warn before leaving with unsaved changes; step
  indicator for 3+ steps; never ask twice for data already given.

## Settings pages

- Group by task, not by database table; each group has a heading and a
  one-line description.
- Declare the save model once, at the top or in the footer:
  - **Autosave:** toggles and selects apply on change; text fields on blur;
    show "Saved · 14:32" near the group; errors revert the control and say so.
  - **Explicit save:** sticky footer with Save (primary) and Cancel appears
    once something is dirty; `beforeunload` warning; Save stays enabled.
- Dangerous settings (delete workspace, rotate keys) live in a separated
  "Danger zone" with type-to-confirm for irreversible actions.
