// Setup lines are copied from the README "add the MCP server by hand" section.
export const AGENTS = [
  { name: 'Claude Code', where: '.claude/skills · .mcp.json', cmd: 'claude mcp add layoza -- npx -y -p layoza layoza-mcp' },
  { name: 'Cursor', where: '.cursor/mcp.json', cmd: '{ "mcpServers": { "layoza": { "command": "npx", "args": ["-y", "-p", "layoza", "layoza-mcp"] } } }' },
  { name: 'Codex', where: 'codex mcp', cmd: 'codex mcp add layoza -- npx -y -p layoza layoza-mcp' },
  { name: 'OpenCode', where: 'opencode.json', cmd: '{ "mcp": { "layoza": { "type": "local", "command": ["npx", "-y", "-p", "layoza", "layoza-mcp"], "enabled": true } } }' },
  { name: 'Windsurf', where: '~/.codeium/windsurf/mcp_config.json', cmd: '{ "mcpServers": { "layoza": { "command": "npx", "args": ["-y", "-p", "layoza", "layoza-mcp"] } } }' },
  { name: 'Antigravity', where: '.agents/skills · .agents/plugins/layoza', cmd: 'npx layoza init' },
];

const GH = 'https://github.com/layoza/layoza/blob/main/';

export const MCP_GROUPS = [
  { tools: ['inspect_project', 'extract_site_dna', 'inspect_site', 'discover_components'], what: 'Read an existing codebase or live URL into design tokens and a component inventory' },
  { tools: ['get_design_system', 'recommend_layout'], what: 'Industry read, dials and three creative directions for a brief' },
  { tools: ['deal_mechanics', 'get_mechanic', 'recommend_animation'], what: 'Niche-aware motion mechanics and animation recipes' },
  { tools: ['capture_screenshots', 'audit_accessibility', 'audit_performance', 'verify_functional'], what: 'Live Chromium QA at three widths' },
  { tools: ['compare_before_after', 'safe_rollback', 'export_report'], what: 'Baselines, regression checks and reports' },
];

export const SKILL_GROUPS = [
  { h: 'Workflows', s: ['layoza-new-website', 'layoza-extend-website', 'layoza-refine', 'layoza-redesign'] },
  { h: 'Design', s: ['layoza-site-dna', 'layoza-design-system', 'layoza-art-direction', 'layoza-creative-direction'] },
  { h: 'Building', s: ['layoza-components', 'layoza-product-ui', 'layoza-figma', 'layoza-wordpress'] },
  { h: 'Motion', s: ['layoza-animations', 'layoza-threejs-webgl'] },
  { h: 'Checks', s: ['layoza-accessibility', 'layoza-performance', 'layoza-visual-qa', 'layoza-discovery-audit'] },
];

export const MODES = [
  { k: 'A', h: 'New site', when: 'Blank project', cmd: 'layoza design-system "<brief>" --mode new', note: 'Three directions, each a rendered hero sketch, before any code is written.' },
  { k: 'B', h: 'Extend', when: 'Add a page or section to a live site', cmd: 'layoza inspect . --write', note: 'No new directions. Layoza copies your look; it does not impose its own.' },
  { k: 'C', h: 'Improve', when: 'Fix design, usability or accessibility without changing the brand', cmd: 'layoza baseline create pre-improve → layoza audit .', note: 'Keeps the brand, fixes what fails.' },
  { k: 'D', h: 'Redesign', when: 'New creative direction, keeping content and function', cmd: 'layoza baseline create pre-redesign', note: 'A baseline is taken first, so you can roll back.' },
];

export const DOCS = [
  ['Installation', GH + 'docs/INSTALLATION.md'],
  ['CLI reference', GH + 'docs/CLI-REFERENCE.md'],
  ['MCP reference', GH + 'docs/MCP-REFERENCE.md'],
  ['Skills guide', GH + 'docs/SKILLS-GUIDE.md'],
  ['Motion recipes', GH + 'docs/MOTION-RECIPES.md'],
  ['Existing-site workflows', GH + 'docs/EXISTING-SITE-WORKFLOWS.md'],
  ['Example prompts', GH + 'docs/EXAMPLE-PROMPTS.md'],
  ['Architecture', GH + 'docs/ARCHITECTURE.md'],
  ['Security', GH + 'docs/SECURITY.md'],
  ['Troubleshooting', GH + 'docs/TROUBLESHOOTING.md'],
  ['Benchmarks', GH + 'docs/CROSS-TYPE-BENCHMARK-2026-10.md'],
];

export const LIMITS = [
  'Kinetic is a tie, not a win: taste-skill was as strong, and the lead stayed inside judge noise.',
  'Older single-brief rounds: impeccable beat Layoza on a simple page with no animation; ui-ux-pro-max beat it in an earlier kinetic round.',
  'One judge per round, judging screenshots. Motion itself is not judged.',
  'Weak spots judges keep naming: kinetic creativity is good, not striking; headings cut by the sticky header in mid-scroll stills; small hero art on phones; a 3D hero whose lab LCP is well over 2.5 s.',
];

export const STACKS = ['React', 'Next.js', 'Astro', 'Vue', 'Nuxt', 'SvelteKit', 'HTML/CSS/JS', 'WordPress block themes', 'Elementor JSON', 'Bricks JSON', 'Shopify Liquid'];
