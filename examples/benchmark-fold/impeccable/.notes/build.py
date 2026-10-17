"""Dev helper: injects generated SVG artwork into template.html -> ../index.html. Not shipped."""
import json, os
here = os.path.dirname(os.path.abspath(__file__))
m = json.load(open(os.path.join(here, "models.json")))

INK = "#1D1C1A"
def section(stage, label):
    """Wall section drawn progressively across the four approach steps. viewBox 0 0 200 120."""
    p = []
    # ground line
    p.append(f'<line x1="4" y1="104" x2="196" y2="104" stroke="{INK}"/>')
    p.append('<g stroke="#8C877E" stroke-width="0.8">' + "".join(
        f'<line x1="{x}" y1="104" x2="{x-6}" y2="112"/>' for x in range(14, 196, 12)) + "</g>")
    # stone wall with hatch
    p.append(f'<rect x="40" y="14" width="26" height="90" fill="url(#hatch)" stroke="{INK}"/>')
    # existing floor joist + wall head
    if stage >= 2:
        p.append(f'<rect x="66" y="56" width="58" height="6" fill="var(--balsa-2)" stroke="{INK}"/>')
        p.append(f'<rect x="34" y="8" width="38" height="6" fill="var(--card)" stroke="{INK}"/>')
    if stage == 1:
        # dimension line
        p.append(f'<g stroke="{INK}"><line x1="40" y1="4" x2="66" y2="4"/><line x1="40" y1="0" x2="40" y2="8"/><line x1="66" y1="0" x2="66" y2="8"/></g>')
        p.append('<text x="80" y="8" class="dim">600</text>')
        p.append(f'<g stroke="{INK}"><line x1="80" y1="14" x2="80" y2="104" stroke-dasharray="3 3"/></g>')
        p.append('<text x="86" y="62" class="dim">survey</text>')
    if stage == 2:
        p.append('<text x="128" y="62" class="dim">kept</text>')
    if stage >= 3:
        # insulation (zigzag) inside face
        zz = "M70 14 " + " ".join(f"L{78 if i % 2 else 70} {14 + i*5}" for i in range(1, 19))
        p.append(f'<rect x="66" y="14" width="16" height="90" fill="var(--card)" stroke="{INK}"/>')
        p.append(f'<path d="{zz}" fill="none" stroke="{INK}" stroke-width="0.8"/>')
        p.append(f'<rect x="82" y="56" width="42" height="6" fill="var(--balsa-2)" stroke="{INK}"/>')
        p.append(f'<line x1="86" y1="14" x2="86" y2="104" stroke="{INK}" stroke-dasharray="3 3"/>')
        if stage == 3:
            p.append('<text x="92" y="40" class="dim">airtight line</text>')
    if stage >= 4:
        # timber addition: frame outside the wall (to the left)
        p.append(f'<path d="M4 50 L40 40" fill="none" stroke="{INK}"/>')
        p.append(f'<rect x="8" y="52" width="32" height="52" fill="var(--balsa-1)" stroke="{INK}"/>')
        p.append('<g stroke="#7A6743" stroke-width="0.7">' + "".join(
            f'<line x1="{x}" y1="52" x2="{x}" y2="104"/>' for x in range(14, 40, 6)) + "</g>")
        p.append(f'<rect x="4" y="48" width="38" height="4" fill="var(--balsa-2)" stroke="{INK}"/>')
        p.append('<text x="2" y="-4" class="dim">new timber room</text>')
    defs = (f'<defs><pattern id="hatch" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">'
            f'<rect width="6" height="6" fill="var(--card-2)"/><line x1="0" y1="0" x2="0" y2="6" stroke="#8C877E" stroke-width="1"/></pattern></defs>')
    return (f'<svg viewBox="-2 -12 204 128" role="img" aria-label="{label}" class="secdraw">' + (defs if stage == 1 else "")
            + "".join(p) + "</svg>")

def plan():
    s = []
    W, H = 560, 250
    s.append(f'<svg viewBox="0 0 {W} {H}" role="img" aria-label="Plan of the studio table with six seats, labelled with each person\'s initials" class="plan">')
    s.append(f'<rect x="20" y="20" width="520" height="210" fill="none" stroke="{INK}" stroke-width="3"/>')
    # window openings on top wall
    for x in (70, 230, 390):
        s.append(f'<rect x="{x}" y="16" width="100" height="8" fill="var(--ground)" stroke="{INK}"/>')
        s.append(f'<line x1="{x}" y1="20" x2="{x+100}" y2="20" stroke="{INK}" stroke-width="0.8"/>')
    # door swing bottom right
    s.append(f'<path d="M470 230 L470 182 A48 48 0 0 1 518 230" fill="none" stroke="{INK}" stroke-dasharray="3 3"/>')
    s.append(f'<rect x="468" y="226" width="52" height="8" fill="var(--ground)" stroke="none"/>')
    # table
    s.append(f'<rect x="90" y="100" width="380" height="64" fill="var(--card)" stroke="{INK}"/>')
    # model on table
    s.append(f'<rect x="246" y="116" width="68" height="32" fill="var(--board-1)" stroke="{INK}"/>')
    s.append(f'<rect x="258" y="122" width="30" height="16" fill="var(--card)" stroke="{INK}"/>')
    s.append(f'<rect x="288" y="128" width="16" height="12" fill="var(--balsa-2)" stroke="{INK}"/>')
    s.append(f'<line x1="258" y1="130" x2="288" y2="130" stroke="{INK}" stroke-dasharray="3 2"/>')
    ini = ["IB", "RK", "AH", "CD", "MT", "SL"]
    xs = [130, 250, 370]
    for i, x in enumerate(xs):
        s.append(f'<rect x="{x}" y="64" width="44" height="30" rx="3" fill="var(--card-2)" stroke="{INK}"/>')
        s.append(f'<text x="{x+22}" y="84" text-anchor="middle">{ini[i]}</text>')
        s.append(f'<rect x="{x}" y="170" width="44" height="30" rx="3" fill="var(--card-2)" stroke="{INK}"/>')
        s.append(f'<text x="{x+22}" y="190" text-anchor="middle">{ini[i+3]}</text>')
    # north arrow + scale
    s.append(f'<g transform="translate(46 186)"><circle r="12" fill="none" stroke="{INK}"/><path d="M0 -10 L5 6 L0 2 L-5 6 Z" fill="{INK}"/></g>')
    s.append('<text x="40" y="160" text-anchor="start">N</text>')
    s.append("</svg>")
    return "".join(s)

t = open(os.path.join(here, "template.html"), encoding="utf-8").read()
for k, v in m.items():
    t = t.replace("{{" + k + "}}", v)
labels = [
    "Section through a stone wall with its thickness dimensioned during the survey",
    "The same wall with the existing timber floor and wall head kept",
    "Insulation and an airtight line added to the inside face of the wall",
    "A new timber room built against the outside of the insulated wall",
]
for i in range(4):
    t = t.replace("{{sec%d}}" % (i + 1), section(i + 1, labels[i]))
t = t.replace("{{plan}}", plan())
assert "{{" not in t, "unreplaced placeholder"
open(os.path.join(here, "..", "index.html"), "w", encoding="utf-8").write(t)
print("built", len(t))
