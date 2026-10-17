# Existing-Site DNA Inspection & Preservation Workflows (Modes B & C)

One of the most common failure modes of AI coding agents is destroying an existing website's brand identity when asked to add a new section, pricing table, or page. Layoza solves this with its **Existing Site DNA Engine** (`layoza inspect`).

---

## Step-by-Step Workflow for Mode B (Extend Existing Website)

### Step 1: Inspect the Project Before Writing Code
```bash
cargo run --bin layoza -- inspect --path /path/to/project --write-artifacts
```
Layoza scans:
- `package.json`, `astro.config.*`, `next.config.*`, `theme.json`, `style.css`
- CSS custom properties (`--color-*`, `--font-*`, `--space-*`, `--radius-*`)
- Tailwind configuration and theme tokens
- Existing component directories (`src/components`, `patterns/`, `sections/`)
- Existing animation libraries (`gsap`, `motion`, `three`, `lenis`)

### Step 2: Review the 6 Generated Artifacts
1. **`DESIGN.md`**: Human- and agent-readable design system contract compatible with Google's `DESIGN.md` format.
2. **`BRAND.md`**: Brand voice, tone, industry positioning, and anti-patterns.
3. **`MOTION.md`**: Allowed easing curves, duration scales, and reduced-motion rules.
4. **`SITE-DNA.json`**: Machine-readable tokens tagged with provenance:
   - `ObservedSource` (extracted directly from source files, confidence `0.90–1.0`)
   - `ObservedBrowser` (measured from live Chromium DOM computed styles)
   - `InferredRule` (synthesized from layout rhythm)
   - `UserApprovedDecision` (explicitly confirmed by the user)
5. **`COMPONENT-INVENTORY.json`**: Discovered reusable components and file paths.
6. **`DESIGN-DECISIONS.md`**: Architectural decision log and baseline hash.

### Step 3: Generate Framework-Native Extensions
```bash
cargo run --bin layoza -- generate --component pricing --framework wordpress-gutenberg --path /path/to/project
```
The adapter reads `SITE-DNA.json` and emits code in the project's native framework without adding conflicting CSS libraries.

### Step 4: Verify in Live Chromium
```bash
node scripts/browser-qa.mjs --file /path/to/project/index.html --out .layoza/after.json
cargo run --bin layoza -- verify --snapshot .layoza/after.json
```
