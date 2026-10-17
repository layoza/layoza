# Fold: design plan (frontend-design skill, two passes)

Subject: a 6-person Glasgow practice doing tenement/housing retrofits, small civic buildings, timber extensions.
Audience: housing associations, community groups, homeowners. Primary job: show the work, invite an enquiry.
(Skill says confirm with client; running unattended, so I decided and noted it here.)

## Pass 1
- Color: paper #F2F3F0 (cool tracing paper), slate #1E2629 (ink / Scottish slate), graphite #5B6569 (existing-fabric lines, secondary text),
  rule #C8CCC7, larch #8A5A2B (new work). 5 values.
- Type: Schibsted Grotesk only, 400/500/600. Scale 14 / 16 / 18 / 21 / 28 / 36 / 56.
- Layout: left-aligned 12-col grid. Hero statement left 7 cols, drawing right 5 cols. Work 3x2 grid of drawings + text. Approach 2x2. Studio paragraph + people list. Form + details.
- Principles: quiet, drawn, nothing decorative.

## Review against brief / generic defaults
- "Statement left, illustration right" hero is the default I'd produce for any studio. REVISED: the hero drawing becomes a full-bleed
  street elevation (tenement row, timber extension, small hall with a folded-plate roof) running under the statement like a horizon.
  It is the one bold thing on the page.
- Accent used only as decoration is a default. REVISED: use the drafting convention existing = grey line, new work = coloured line.
  Larch colour therefore always means "what we added". Same rule in all six project drawings; a two-item legend under the hero states it.
- Single-family grotesque on off-white could drift into generic Swiss. Kept, but the paper is cool (not cream), no all-caps eyebrows,
  no mono labels, no numbered markers (principles are not a sequence), no arrows on links, no middle-dot meta strings.
- Cards with shadows avoided: projects sit directly on the paper, separated by space; drawings framed by a single thin rule like a sheet.
- Motion: none except hover/focus colour/underline changes (client instruction). Menu and filter change state instantly.

## Decisions taken without client
- All projects, people, address, phone (Ofcom drama range 0141 496 0xxx) and email (.example domain) are fictional.
- Social links point to "#" placeholders.

## Self-critique passes
1. Desktop 1440x900: street elevation was cut by the fold. Widened h1 measure (20ch -> 24ch, 5 lines -> 4), trimmed hero/drawing spacing; full elevation + key now sit above the fold.
2. Mobile: elevation crop shifted (-72% -> -90%) so the crop shows the retrofitted tenement, cottage and timber extension.
Checks (Playwright, Chromium): no horizontal scroll at 1440/768/375; filter (Civic -> 2 projects, live-region text); empty submit -> 5 errors, focus to first; valid submit -> success with name/email, focus moved; menu opens, Escape closes; 0 console errors.
