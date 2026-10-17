"""Build index.html from src.html, expanding <!--TICKS cx cy r cls--> into TABO protractor marks.

TABO scale: 0 deg at the examiner's right, 180 at the left, drawn over the top half of each cell.
"""
import math, re, pathlib

here = pathlib.Path(__file__).resolve().parent
src = (here / "src.html").read_text(encoding="utf-8")


def ticks(cx, cy, r, scale):
    """r = inner radius of the scale band. scale multiplies tick/label sizes."""
    out = []
    for deg in range(0, 181, 5):
        a = math.radians(deg)
        major = deg % 10 == 0
        big = deg % 30 == 0
        r1 = r + (2 if big else (5 if major else 8)) * scale
        r2 = r + 15 * scale
        x1, y1 = cx + r1 * math.cos(a), cy - r1 * math.sin(a)
        x2, y2 = cx + r2 * math.cos(a), cy - r2 * math.sin(a)
        w = (1.6 if big else 1.0) * scale
        out.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke-width="{w:.2f}"/>')
    labels = []
    for deg in range(0, 181, 30):
        a = math.radians(deg)
        rl = r - 7 * scale
        x, y = cx + rl * math.cos(a), cy - rl * math.sin(a)
        rot = 90 - deg
        labels.append(
            f'<text x="{x:.1f}" y="{y:.1f}" transform="rotate({rot} {x:.1f} {y:.1f})" '
            f'font-size="{8.5*scale:.1f}" text-anchor="middle" dominant-baseline="central">{deg}</text>'
        )
    return (
        '<g class="ticks" stroke="currentColor">' + "".join(out) + "</g>"
        + '<g class="tick-labels" fill="currentColor" stroke="none">' + "".join(labels) + "</g>"
    )


def knurl(cx, cy, r, n=28):
    out = []
    for i in range(int(n)):
        a = 2 * math.pi * i / n
        out.append(
            f'<line x1="{cx + (r-4)*math.cos(a):.1f}" y1="{cy + (r-4)*math.sin(a):.1f}" '
            f'x2="{cx + r*math.cos(a):.1f}" y2="{cy + r*math.sin(a):.1f}"/>'
        )
    return '<g stroke="currentColor" stroke-width="1">' + "".join(out) + "</g>"


def rail(x1, x2, y, step, h):
    out = []
    i = 0
    x = x1
    while x <= x2 + 0.01:
        hh = h if i % 5 == 0 else h * 0.55
        out.append(f'<line x1="{x:.1f}" y1="{y:.1f}" x2="{x:.1f}" y2="{y+hh:.1f}"/>')
        x += step
        i += 1
    return '<g stroke="currentColor" stroke-width="0.9">' + "".join(out) + "</g>"


def sub(m):
    kind, *args = m.group(1).split()
    nums = [float(a) for a in args]
    if kind == "TICKS":
        return ticks(*nums)
    if kind == "KNURL":
        return knurl(*nums)
    if kind == "RAIL":
        return rail(*nums)
    raise ValueError(kind)


html = re.sub(r"<!--((?:TICKS|KNURL|RAIL)[^>]*?)-->", sub, src)
out = here.parent / "index.html"
out.write_text(html, encoding="utf-8")
print("wrote", out, len(html), "bytes")
