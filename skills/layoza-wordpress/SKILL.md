---
name: layoza-wordpress
description: First-class WordPress Gutenberg Block Theme engineering (theme.json v3, block patterns, templates, parts) plus Elementor and Bricks builder JSON export adapters.
---

# Layoza Specialist Skill: WordPress & Builder Engineering (`layoza-wordpress`)

Use this skill when building, extending, or modernizing WordPress websites, Gutenberg Full Site Editing (FSE) Block Themes, or exporting structured layouts for Elementor and Bricks Builder.

## 1. Native WordPress Block Themes First (`WordPressGutenberg`)
Always prefer native WordPress Gutenberg Block Themes (`theme.json` schema v3 + semantic block markup) unless the project already uses Elementor or Bricks:
1. **`theme.json` (v3)**:
   - Centralize all `SITE-DNA.json` color palettes, fluid typography sizes (`clamp()`), spacing scales, layout content/wide widths (`contentSize`, `wideSize`), and custom CSS variables under `settings`.
   - Prefer v3 presets over hand-written `clamp()`: `settings.typography.fluid: true` with per-size `fluid: { min, max }`, and `settings.spacing.spacingScale` (or explicit `spacingSizes`) so blocks get `var:preset|spacing|50` steps that scale with the viewport. Set `defaultFontSizes` / `defaultSpacingSizes: false` to hide core presets that clash with the brand.
   - Configure global element styles (`button`, `heading`, `link`) under `styles`.
2. **Block Patterns (`patterns/*.php`)**:
   - Author clean `<!-- wp:group -->`, `<!-- wp:columns -->`, `<!-- wp:heading -->`, and `<!-- wp:buttons -->` block grammar referencing preset slugs (`var:preset|spacing|40`, `has-primary-background-color`) so patterns remain 100% editable inside the WordPress Site Editor.
3. **Templates & Template Parts (`templates/index.html`, `parts/header.html`, `parts/footer.html`)**:
   - Keep templates declarative and free of hardcoded inline hex values.

## 2. Elementor & Bricks Compatibility Adapters
When inspecting (`layoza inspect`) an existing WordPress project that uses Elementor or Bricks:
- Preserve the existing builder and generate compatible container/element JSON schemas via:
  ```bash
  layoza generate --component hero --framework elementor --title "Headline" --subtitle "Subheadline"
  layoza generate --component hero --framework bricks --title "Headline" --subtitle "Subheadline"
  ```
- Never inject conflicting CSS frameworks into an active page-builder site.

## 3. wp-admin and Settings UI
- Build plugin admin screens with `@wordpress/components` (`Panel`, `Card`, `TextControl`, `ToggleControl`, `SelectControl`, `Notice`, `Button`, `Modal`) inside a React root from `@wordpress/element`; load with `wp_enqueue_script` and the `*.asset.php` dependencies from `@wordpress/scripts`.
- For list screens use `DataViews` / `DataForm` (`@wordpress/dataviews`) where the site's WordPress and Gutenberg versions ship them; fall back to `WP_List_Table` patterns otherwise. Never ship a custom table framework into wp-admin.
- Persist settings with `register_setting` + REST (`show_in_rest`), save via `@wordpress/api-fetch`, and show success/error with `Notice` or snackbars. Follow [`layoza-product-ui`](../layoza-product-ui/SKILL.md) for states and the explicit-save vs autosave rule; never restyle core admin colours.

## 4. Elementor Editor Controls
- Expose design choices as widget controls bound to Global Colors and Global Fonts (`'global' => ['default' => Global_Colors::COLOR_PRIMARY]`), not hard-coded values.
- Use responsive controls (`add_responsive_control`) for spacing and typography, `selectors` with `{{WRAPPER}}` scoping, and group controls (`Group_Control_Typography`, `Group_Control_Border`) instead of custom CSS fields.
- Group controls into Content / Style / Advanced tabs with sections named by task; give every control a default so a freshly dropped widget already looks finished.
