"""Dev helper: generates the axonometric card-model SVGs inlined in index.html.
Solid line = cut edge, dashed line = fold. Not shipped."""
import math, sys

C30 = math.cos(math.radians(30)); S30 = 0.5
TOP = "var(--card)"; LEFT = "var(--card-2)"; RIGHT = "var(--grey)"
BTOP = "var(--balsa-1)"; BLEFT = "var(--balsa-2)"; BRIGHT = "var(--balsa-3)"
OPEN = "var(--grey-dk)"

class Scene:
    def __init__(s, scale=10, ox=0, oy=0):
        s.k = scale; s.items = []; s.ox = ox; s.oy = oy; s.bb = [1e9, 1e9, -1e9, -1e9]
    def p(s, x, y, z):
        a, b = ((x - y) * C30 * s.k + s.ox, (x + y) * S30 * s.k - z * s.k + s.oy)
        s.bb = [min(s.bb[0], a), min(s.bb[1], b), max(s.bb[2], a), max(s.bb[3], b)]
        return a, b
    def poly(s, pts, fill, extra=""):
        d = " ".join(f"{a:.1f},{b:.1f}" for a, b in (s.p(*q) for q in pts))
        s.items.append(f'<polygon points="{d}" fill="{fill}"{extra}/>')
    def line(s, a, b, dash=False):
        (x1, y1), (x2, y2) = s.p(*a), s.p(*b)
        if dash:  # knock out the solid polygon edge, then score the fold
            s.items.append(f'<line class="knock" x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}"/>')
        cls = ' class="fold"' if dash else ''
        s.items.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}"{cls}/>')
    def box(s, x, y, z, w, d, h, balsa=False):
        t, l, r = (BTOP, BLEFT, BRIGHT) if balsa else (TOP, LEFT, RIGHT)
        # +y face (left on screen)
        s.poly([(x, y+d, z), (x+w, y+d, z), (x+w, y+d, z+h), (x, y+d, z+h)], l)
        # +x face (right on screen)
        s.poly([(x+w, y, z), (x+w, y+d, z), (x+w, y+d, z+h), (x+w, y, z+h)], r)
        s.poly([(x, y, z+h), (x+w, y, z+h), (x+w, y+d, z+h), (x, y+d, z+h)], t)
        if h > 1.5:
            s.line((x+w, y+d, z), (x+w, y+d, z+h), dash=True)
    def gable_x(s, x, y, z, w, d, rh, balsa=False):
        """Ridge runs along x at y+d/2."""
        t, l, r = (BTOP, BLEFT, BRIGHT) if balsa else (TOP, LEFT, RIGHT)
        ym = y + d/2
        s.poly([(x, y, z), (x+w, y, z), (x+w, ym, z+rh), (x, ym, z+rh)], t)
        s.poly([(x, ym, z+rh), (x+w, ym, z+rh), (x+w, y+d, z), (x, y+d, z)], "var(--balsa-1)" if balsa else "var(--roof)")
        s.poly([(x+w, y, z), (x+w, ym, z+rh), (x+w, y+d, z)], r)
        s.line((x, ym, z+rh), (x+w, ym, z+rh), dash=True)
        s.line((x, y+d, z), (x+w, y+d, z), dash=True)
    def gable_y(s, x, y, z, w, d, rh, balsa=False):
        """Ridge runs along y at x+w/2."""
        t, l, r = (BTOP, BLEFT, BRIGHT) if balsa else (TOP, LEFT, RIGHT)
        xm = x + w/2
        s.poly([(x, y, z), (xm, y, z+rh), (xm, y+d, z+rh), (x, y+d, z)], t)
        s.poly([(xm, y, z+rh), (x+w, y, z), (x+w, y+d, z), (xm, y+d, z+rh)], r)
        s.poly([(x, y+d, z), (xm, y+d, z+rh), (x+w, y+d, z)], l)
        s.line((xm, y, z+rh), (xm, y+d, z+rh), dash=True)
        s.line((x+w, y, z), (x+w, y+d, z), dash=True)
    def mono(s, x, y, z, w, d, rh, balsa=False):
        """Mono-pitch roof rising toward +y? rising toward -y (back)."""
        t, l, r = (BTOP, BLEFT, BRIGHT) if balsa else (TOP, LEFT, RIGHT)
        s.poly([(x, y, z+rh), (x+w, y, z+rh), (x+w, y+d, z), (x, y+d, z)], t)
        s.poly([(x+w, y, z), (x+w, y, z+rh), (x+w, y+d, z)], r)
    def win_y(s, x, y, z, w, h):
        """Opening on a +y face (plane y)."""
        s.poly([(x, y, z), (x+w, y, z), (x+w, y, z+h), (x, y, z+h)], OPEN)
    def win_x(s, x, y, z, d, h):
        s.poly([(x, y, z), (x, y+d, z), (x, y+d, z+h), (x, y, z+h)], OPEN)
    def slats_x(s, x, y0, y1, z0, z1, n):
        """Vertical cladding lines on a +x face."""
        for i in range(1, n):
            yy = y0 + (y1 - y0) * i / n
            a, b = s.p(x, yy, z0), s.p(x, yy, z1)
            s.items.append(f'<line class="slat" x1="{a[0]:.1f}" y1="{a[1]:.1f}" x2="{b[0]:.1f}" y2="{b[1]:.1f}"/>')
    def slats_y(s, y, x0, x1, z0, z1, n):
        for i in range(1, n):
            xx = x0 + (x1 - x0) * i / n
            a, b = s.p(xx, y, z0), s.p(xx, y, z1)
            s.items.append(f'<line class="slat" x1="{a[0]:.1f}" y1="{a[1]:.1f}" x2="{b[0]:.1f}" y2="{b[1]:.1f}"/>')
    def svg(s, vb, label, aspect=None, pad=6):
        x0, y0, x1, y1 = s.bb
        x0 -= pad; y0 -= pad; x1 += pad; y1 += pad
        w, h = x1 - x0, y1 - y0
        if aspect:
            if w / h < aspect:
                nw = h * aspect; x0 -= (nw - w) / 2; w = nw
            else:
                nh = w / aspect; y0 -= (nh - h) / 2; h = nh
        vb = f"{x0:.0f} {y0:.0f} {w:.0f} {h:.0f}"
        return (f'<svg viewBox="{vb}" role="img" aria-label="{label}" class="model">'
                + "".join(s.items) + "</svg>")

def plinth(sc, x, y, w, d, h=0.6):
    sc.poly([(x, y+d, -h), (x+w, y+d, -h), (x+w, y+d, 0), (x, y+d, 0)], "var(--board-2)")
    sc.poly([(x+w, y, -h), (x+w, y+d, -h), (x+w, y+d, 0), (x+w, y, 0)], "var(--board-3)")
    sc.poly([(x, y, 0), (x+w, y, 0), (x+w, y+d, 0), (x, y+d, 0)], "var(--board-1)")

def hero():
    sc = Scene(13, 330, 40)
    plinth(sc, -3, -3, 36, 26)
    # tenement block 4 storeys + attic
    sc.box(0, 0, 0, 28, 9, 13)
    for i in range(7):
        for f in range(4):
            sc.win_y(1.4 + i*3.8, 9, 1.2 + f*3.0, 1.4, 1.8)
    for j in range(2):
        for f in range(4):
            sc.win_x(28, 2 + j*4, 1.2 + f*3.0, 1.4, 1.8)
    sc.gable_x(0, 0, 13, 28, 9, 2.6)
    sc.box(5, 3.5, 15.6, 1.2, 1.2, 1.6)
    sc.box(19, 3.5, 15.6, 1.2, 1.2, 1.6)
    # rear (front-facing) timber extension
    sc.box(9, 9, 0, 10, 6, 3.8, balsa=True)
    sc.mono(9, 9, 3.8, 10, 6, 1.2, balsa=True)
    sc.slats_y(15, 9, 19, 0, 3.8, 14)
    sc.slats_x(19, 9, 15, 0, 3.8, 7)
    sc.win_y(11.5, 15, 0, 4, 2.9)
    return sc.svg(None, "Axonometric study model: a four-storey sandstone tenement in white card with a single-storey balsa timber extension at the back, on a greyboard base")

def p_tenement():
    sc = Scene(9, 170, 30); plinth(sc, -1, -1, 22, 12)
    sc.box(0, 0, 0, 20, 8, 14)
    for i in range(5):
        for f in range(4):
            sc.win_y(1 + i*3.9, 8, 1.4 + f*3.2, 1.5, 2)
    sc.win_x(20, 3, 1.4, 2, 2); sc.win_x(20, 3, 4.6, 2, 2); sc.win_x(20, 3, 7.8, 2, 2); sc.win_x(20, 3, 11, 2, 2)
    sc.gable_x(0, 0, 14, 20, 8, 2.4)
    # insulation layer shown as a fold line around base
    sc.line((0, 8, 0.01), (20, 8, 0.01), dash=True)
    return sc.svg(None, "Card model of a sandstone tenement block", 4/3)

def p_backcourt():
    sc = Scene(9, 170, 50); plinth(sc, -1, -1, 24, 16)
    for i in range(4):
        sc.box(i*5.5, 0, 0, 4.5, 6, 5)
        sc.gable_y(i*5.5, 0, 5, 4.5, 6, 2.2)
        sc.win_y(i*5.5 + 1.2, 6, 0, 1.6, 2.8)
    # drying green
    sc.poly([(2, 9, 0.02), (18, 9, 0.02), (18, 13.5, 0.02), (2, 13.5, 0.02)], "var(--mat-soft)")
    sc.line((2, 11.25, 0.03), (18, 11.25, 0.03), dash=True)
    return sc.svg(None, "Card model of four small gabled homes around a shared green", 4/3)

def p_reading():
    sc = Scene(9, 165, 40); plinth(sc, -1, -1, 22, 14)
    sc.box(0, 0, 0, 14, 11, 7)
    sc.win_y(2, 11, 0, 2.6, 4.6); sc.win_y(5.7, 11, 0, 2.6, 4.6); sc.win_y(9.4, 11, 0, 2.6, 4.6)
    sc.win_x(14, 2, 1.4, 2, 4); sc.win_x(14, 7, 1.4, 2, 4)
    sc.box(0, 0, 7, 14, 11, 0.6)
    sc.box(14, 4, 0, 6, 7, 3.6, balsa=True)
    sc.slats_x(20, 4, 11, 0, 3.6, 8)
    sc.mono(14, 4, 3.6, 6, 7, 0.8, balsa=True)
    return sc.svg(None, "Card model of a former bank hall with a small timber entrance pavilion", 4/3)

def p_garden():
    sc = Scene(10, 170, 30); plinth(sc, -1, -1, 20, 14)
    sc.box(0, 0, 0, 9, 7, 11)
    sc.gable_x(0, 0, 11, 9, 7, 3)
    sc.win_y(2.5, 7, 4.5, 1.6, 2.2); sc.win_y(2.5, 7, 7.6, 1.6, 2.2)
    sc.box(0, 7, 0, 9, 5, 3.4, balsa=True)
    sc.slats_y(12, 0, 9, 0, 3.4, 12)
    sc.mono(0, 7, 3.4, 9, 5, 1.0, balsa=True)
    sc.box(9, 7, 0, 6, 5, 2.2, balsa=True)
    sc.slats_x(15, 7, 12, 0, 2.2, 6)
    sc.win_y(10.5, 12, 0, 3.2, 1.9)
    return sc.svg(None, "Card model of a villa with a larch-clad kitchen extension stepping down to the garden", 4/3)

def p_hall():
    sc = Scene(9, 150, 40); plinth(sc, -1, -1, 24, 13)
    sc.box(0, 0, 0, 16, 9, 5, balsa=True)
    sc.gable_x(0, 0, 5, 16, 9, 3.4, balsa=True)
    sc.slats_y(9, 0, 16, 0, 5, 20)
    sc.slats_x(16, 0, 9, 0, 5, 10)
    sc.box(16, 1.5, 0, 6, 7.5, 3.6)
    sc.mono(16, 1.5, 3.6, 6, 7.5, 0.8)
    sc.win_y(17.2, 9, 0, 3.6, 2.6)
    return sc.svg(None, "Model of a 1930s timber hall in balsa with a new white card extension", 4/3)

def p_attic():
    sc = Scene(10, 165, 40); plinth(sc, -1, -1, 18, 15)
    sc.box(0, 0, 0, 10, 8, 7)
    sc.gable_x(0, 0, 7, 10, 8, 3.6)
    sc.box(3, 5.2, 7, 4, 2.4, 2.2, balsa=True)
    sc.win_y(3.8, 7.6, 7.3, 2.4, 1.4)
    sc.win_y(1.5, 8, 3.8, 1.6, 2); sc.win_y(6.8, 8, 3.8, 1.6, 2)
    sc.box(0, 8, 0, 10, 4.5, 3.2, balsa=True)
    sc.slats_y(12.5, 0, 10, 0, 3.2, 12)
    sc.mono(0, 8, 3.2, 10, 4.5, 0.9, balsa=True)
    sc.win_y(2.5, 12.5, 0, 5, 2.6)
    return sc.svg(None, "Card model of a terraced house with a timber dormer and rear extension", 4/3)

out = {"hero": hero(), "tenement": p_tenement(), "backcourt": p_backcourt(), "reading": p_reading(),
       "garden": p_garden(), "hall": p_hall(), "attic": p_attic()}
import json
json.dump(out, open(sys.argv[1], "w"), indent=0)
