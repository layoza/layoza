"""Isometric card-model generator for the Fold page (dev only).
Each scene is a list of convex extruded solids. Output: inline SVG strings."""
import math, json, os

C30, S30 = math.cos(math.radians(30)), 0.5
VIEW = (1, 1, 1)
LIGHT = (-0.45, 0.62, 1.0)
_l = math.sqrt(sum(v * v for v in LIGHT)); LN = tuple(v / _l for v in LIGHT)

PAL = {
    "card": ((0xFB, 0xFA, 0xF7), (0xCF, 0xCB, 0xC2)),
    "red": ((0xB8, 0x4A, 0x30), (0x74, 0x2A, 0x1A)),
    "board": ((0xC8, 0xC3, 0xB8), (0x9E, 0x99, 0x8E)),
    "timber": ((0xE9, 0xD9, 0xBC), (0xB3, 0x9C, 0x77)),
}
INK = "#1C1B19"
VOID = "#3A3732"


def proj(p, s):
    x, y, z = p
    return ((x - y) * C30 * s, ((x + y) * S30 - z) * s)


def sub(a, b): return tuple(i - j for i, j in zip(a, b))
def dot(a, b): return sum(i * j for i, j in zip(a, b))
def cross(a, b): return (a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0])


def norm(v):
    l = math.sqrt(dot(v, v)) or 1
    return tuple(i / l for i in v)


def extrude(profile, axis, a0, a1):
    """profile: list of (u, z) points (convex, any winding) in the plane
    perpendicular to axis. axis 'x': u = y. axis 'y': u = x."""
    def P(u, z, a):
        return (a, u, z) if axis == "x" else (u, a, z)
    caps = [[P(u, z, a0) for u, z in profile], [P(u, z, a1) for u, z in profile]]
    faces = caps[:]
    n = len(profile)
    for i in range(n):
        (u0, z0), (u1, z1) = profile[i], profile[(i + 1) % n]
        faces.append([P(u0, z0, a0), P(u1, z1, a0), P(u1, z1, a1), P(u0, z0, a1)])
    return faces


def box(x0, x1, y0, y1, z0, z1):
    return extrude([(y0, z0), (y1, z0), (y1, z1), (y0, z1)], "x", x0, x1)


def gable_x(x0, x1, y0, y1, z0, ridge):
    m = (y0 + y1) / 2
    return extrude([(y0, z0), (y1, z0), (m, ridge)], "x", x0, x1)


def gable_y(x0, x1, y0, y1, z0, ridge):
    m = (x0 + x1) / 2
    return extrude([(x0, z0), (x1, z0), (m, ridge)], "y", y0, y1)


def mono_x(x0, x1, y0, y1, z0, zlow, zhigh):
    """monopitch block, high at y0, low at y1, extruded along x"""
    return extrude([(y0, z0), (y1, z0), (y1, zlow), (y0, zhigh)], "x", x0, x1)


def hip_x(x0, x1, y0, y1, z0, ridge, inset):
    """hipped roof as convex hull: ridge shortened by inset"""
    m = (y0 + y1) / 2
    pts = [(x0, y0, z0), (x1, y0, z0), (x1, y1, z0), (x0, y1, z0), (x0 + inset, m, ridge), (x1 - inset, m, ridge)]
    return hull_faces(pts, [[0, 1, 2, 3], [0, 3, 4], [1, 5, 2], [0, 4, 5, 1], [3, 2, 5, 4]])


def hull_faces(pts, idx):
    return [[pts[i] for i in f] for f in idx]


def centroid(pts):
    n = len(pts)
    return tuple(sum(p[i] for p in pts) / n for i in range(3))


def shade(material, nrm):
    lit, dark = PAL[material]
    t = max(0.0, dot(nrm, LN))
    t = 0.18 + 0.82 * t
    c = tuple(round(d + (l - d) * t) for l, d in zip(lit, dark))
    return "#%02X%02X%02X" % c


class Solid:
    def __init__(self, faces, material="card", windows=None, stroke=True, layer=0):
        self.layer = layer
        self.faces = faces
        self.material = material
        self.windows = windows or []  # list of (face_key, u0, u1, z0, z1, colour)
        allp = [p for f in faces for p in f]
        self.c = centroid(allp)
        self.pts = allp
        self.stroke = stroke


def face_normal(face, solid_c):
    n = norm(cross(sub(face[1], face[0]), sub(face[2], face[0])))
    fc = centroid(face)
    if dot(n, sub(fc, solid_c)) < 0:
        n = tuple(-i for i in n)
    return n


def window_poly(key, val, u0, u1, z0, z1):
    # key 'x' -> face at x=val, u is y ; key 'y' -> face at y=val, u is x
    if key == "x":
        return [(val, u0, z0), (val, u1, z0), (val, u1, z1), (val, u0, z1)]
    return [(u0, val, z0), (u1, val, z0), (u1, val, z1), (u0, val, z1)]


def convex_hull(points):
    pts = sorted(set(points))
    if len(pts) < 3: return pts
    def cr(o, a, b): return (a[0]-o[0])*(b[1]-o[1]) - (a[1]-o[1])*(b[0]-o[0])
    lo, up = [], []
    for p in pts:
        while len(lo) >= 2 and cr(lo[-2], lo[-1], p) <= 0: lo.pop()
        lo.append(p)
    for p in reversed(pts):
        while len(up) >= 2 and cr(up[-2], up[-1], p) <= 0: up.pop()
        up.append(p)
    return lo[:-1] + up[:-1]


def fmt(pts):
    return " ".join("%.1f,%.1f" % p for p in pts)


def render(solids, base=None, s=10, pad=14, title="", sw=1.1, name="m"):
    out = []
    allxy = []
    # base board
    shapes = []
    if base:
        x0, x1, y0, y1 = base
        shapes.append(Solid(box(x0, x1, y0, y1, -0.6, 0), "board"))
    # shadows on z=0
    shadow_polys = []
    for so in solids:
        sp = [(p[0] - p[2] * LN[0] / LN[2] * -1 * -1, p[1] - p[2] * LN[1] / LN[2], 0) for p in so.pts]
        # shadow falls away from light: p - z * L/Lz
        sp = [(p[0] - p[2] * LN[0] / LN[2], p[1] - p[2] * LN[1] / LN[2], 0) for p in so.pts]
        if max(p[2] for p in so.pts) <= 0.01: continue
        hull = convex_hull([proj(q, s) for q in sp])
        shadow_polys.append(hull)
    for so in shapes + sorted(solids, key=lambda q: dot(q.c, VIEW) + q.layer * 1000):
        for f in so.faces:
            for p in f: allxy.append(proj(p, s))
    minx = min(p[0] for p in allxy) - pad; maxx = max(p[0] for p in allxy) + pad
    miny = min(p[1] for p in allxy) - pad; maxy = max(p[1] for p in allxy) + pad
    W, H = maxx - minx, maxy - miny
    def tr(p): return (p[0] - minx, p[1] - miny)
    def draw(so):
        vis = []
        for f in so.faces:
            nrm = face_normal(f, so.c)
            if dot(nrm, VIEW) <= 1e-6: continue
            vis.append((f, nrm))
        for f, nrm in vis:
            pts = [tr(proj(p, s)) for p in f]
            st = ' stroke="%s" stroke-width="%s"' % (INK, sw) if so.stroke else ""
            out.append('<polygon points="%s" fill="%s"%s/>' % (fmt(pts), shade(so.material, nrm), st))
            for w in so.windows:
                key, u0, u1, z0, z1, col = w
                axis = 0 if key == "x" else 1
                val = f[0][axis]
                if not all(abs(p[axis] - val) < 1e-6 for p in f): continue
                if abs(nrm[axis]) < 0.99: continue
                # window belongs to the face only if inside its u range
                ui = 1 if key == "x" else 0
                us = [p[ui] for p in f]
                if not (min(us) - 1e-6 <= u0 and u1 <= max(us) + 1e-6): continue
                wp = [tr(proj(p, s)) for p in window_poly(key, val, u0, u1, z0, z1)]
                out.append('<polygon points="%s" fill="%s"/>' % (fmt(wp), col))
    for so in shapes: draw(so)
    if shadow_polys:
        x0, x1, y0, y1 = base
        top = [tr(proj(p, s)) for p in [(x0, y0, 0), (x1, y0, 0), (x1, y1, 0), (x0, y1, 0)]]
        out.append('<clipPath id="%s-c"><polygon points="%s"/></clipPath>' % (name, fmt(top)))
        out.append('<g fill="%s" opacity="0.13" clip-path="url(#%s-c)">' % (INK, name))
        for h in shadow_polys:
            out.append('<polygon points="%s"/>' % fmt([tr(p) for p in h]))
        out.append('</g>')
    for so in sorted(solids, key=lambda q: dot(q.c, VIEW) + q.layer * 1000):
        draw(so)
    body = "\n".join(out)
    return W, H, body


def svg(name, solids, base, s, title, desc, extra_class=""):
    W, H, body = render(solids, base, s, name=name)
    return ('<svg class="model %s" viewBox="0 0 %d %d" role="img" aria-labelledby="%s-t" '
            'stroke-linejoin="round"><title id="%s-t">%s</title>%s</svg>') % (
        extra_class, math.ceil(W), math.ceil(H), name, name, title, body)


def win_grid(key, val, u_start, u_end, cols, floors, f_h, w_w, w_h, sill, col=VOID, skip=()):
    ws = []
    span = (u_end - u_start) / cols
    for c in range(cols):
        if c in skip: continue
        uc = u_start + span * (c + 0.5)
        for fl in range(floors):
            z0 = fl * f_h + sill
            ws.append((key, uc - w_w / 2, uc + w_w / 2, z0, z0 + w_h, col))
    return ws


RED_W = "#8E3320"


def tenement(x0, x1, y0, y1, floors=4, fh=3.2, roof=2.6, bays=(), win_col=VOID, ycols=8, xcols=3, roof_mat="card"):
    h = floors * fh
    ws = win_grid("y", y1, x0, x1, ycols, floors, fh, 0.9, 1.7, 0.7, win_col)
    ws += win_grid("x", x1, y0, y1, xcols, floors, fh, 0.9, 1.7, 0.7, win_col)
    out = [Solid(box(x0, x1, y0, y1, 0, h), "card", ws),
           Solid(hip_x(x0, x1, y0, y1, h, h + roof, 1.6), roof_mat)]
    return out, h


def scenes():
    S = {}
    # HERO: tenement with chimneys + red folded-plate timber extension in the back court
    t, h = tenement(0, 26, 0, 9, floors=4, win_col=VOID, ycols=9, xcols=3)
    chim = [Solid(box(cx, cx + 1.4, 3.7, 5.3, h + 1.2, h + 4.4), "card", layer=1) for cx in (4, 12.3, 20.5)]
    nb, nh = tenement(26, 33, 0, 9, floors=3, ycols=3, xcols=3)
    ext = []
    for i, xs in enumerate((7, 10.2, 13.4)):
        ext.append(Solid(gable_y(xs, xs + 3.2, 9, 15, 0, 4.2) if False else extrude([(xs, 0), (xs + 3.2, 0), (xs + 3.2, 3.0), (xs + 1.6, 4.3), (xs, 3.0)], "y", 9, 15), "red",
                         [("y", xs + 0.5, xs + 2.7, 0.3, 2.6, "#5E2215")], layer=2))
    bays = []
    for bx in (1.2, 18.6):
        bw = [("y", bx + 0.55, bx + 1.65, fl * 3.2 + 0.7, fl * 3.2 + 2.4, VOID) for fl in range(4)]
        bw += [("x", 9.25, 9.95, fl * 3.2 + 0.7, fl * 3.2 + 2.4, VOID) for fl in range(4)]
        bays += [Solid(box(bx, bx + 2.2, 9, 10.2, 0, 12.8), "card", bw, layer=1), Solid(box(bx - 0.1, bx + 2.3, 8.9, 10.3, 12.8, 13.2), "card", layer=1)]
    shed = [Solid(box(19, 22.5, 12, 15, 0, 2.2), "red", [("y", 19.6, 21.9, 0.2, 1.9, "#5E2215")], layer=2)]
    S["hero"] = (t + chim + nb + nh * 0 + ext + shed if False else t + chim + nb + bays + ext + shed, (-2, 35, -2, 17), 13,
                 "Card model of a sandstone tenement with Fold's timber extension",
                 "")
    # 1 Close 14 retrofit: tenement with red (new) windows + red heat pump enclosure
    t1, h1 = tenement(0, 18, 0, 9, floors=4, win_col="#A8402A", ycols=6, xcols=3)
    hp = [Solid(box(6, 9, 11, 13, 0, 1.6), "red")]
    S["close14"] = (t1 + hp + [Solid(box(c, c + 1.3, 3.8, 5.2, h1 + 1.3, h1 + 4.2), "card", layer=1) for c in (3, 13.5)],
                    (-2, 20, -2, 15), 10, "Card model: tenement retrofit, new windows shown in red", "")
    # 2 Back-court library: two tenement slabs + red pavilion
    a, ha = tenement(0, 7, 0, 22, floors=3, ycols=2, xcols=7)
    b, hb = tenement(7, 26, 0, 6, floors=3, ycols=6, xcols=2)
    pav = [Solid(gable_y(13, 19, 11, 17, 0, 0) if False else extrude([(13, 0), (19, 0), (19, 3.0), (16, 5.6), (13, 3.0)], "y", 11, 17.5), "red",
                 [("y", 14, 18, 0.2, 2.6, "#5E2215")])]
    S["library"] = (a + b + pav, (-2, 28, -2, 24), 9, "Card model: reading room in a tenement back court", "")
    # 3 Kitchen on the green: villa + red folded extension
    villa = [Solid(box(0, 12, 0, 10, 0, 6.6), "card", win_grid("y", 10, 0, 12, 3, 2, 3.3, 1.1, 1.9, 0.6) + win_grid("x", 12, 0, 10, 3, 2, 3.3, 1.1, 1.9, 0.6)),
             Solid(hip_x(0, 12, 0, 10, 6.6, 10, 3), "card")]
    ext3 = [Solid(mono_x(1.5, 10.5, 10, 16, 0, 1.6, 3.6), "red", [("y", 2.5, 9.5, 0.2, 1.4, "#5E2215")])]
    S["kitchen"] = (villa + ext3, (-2, 15, -2, 19), 11, "Card model: villa with a low timber kitchen extension", "")
    # 4 Cottage row: four cottages, red new roofs
    cot = []
    for i in range(4):
        x0 = i * 6
        cot.append(Solid(box(x0, x0 + 6, 0, 5, 0, 2.8), "card", [("y", x0 + 1, x0 + 2, 0.2, 2.0, VOID), ("y", x0 + 3.4, x0 + 4.8, 0.9, 2.0, VOID)] + ([("x", 1.2, 3.8, 0.9, 2.0, VOID)] if i == 3 else [])))
        cot.append(Solid(gable_x(x0, x0 + 6, 0, 5, 2.8, 5.6), "red"))
        cot.append(Solid(box(x0 + 5.3, x0 + 6, 2.1, 2.9, 4.6, 6.6), "card"))
    S["cottages"] = (cot, (-2, 26, -2, 8), 9, "Card model: row of four cottages, new roofs shown in red", "")
    # 5 Waterside hall: tall red timber hall + white stone porch kept
    hall = [Solid(gable_x(0, 16, 0, 8, 0, 0) if False else extrude([(0, 0), (8, 0), (8, 4.5), (4, 9.5), (0, 4.5)], "x", 0, 16), "red",
                  [("y", 2 + 3 * k, 3 + 3 * k, 0.4, 4.0, "#5E2215") for k in range(5)] + [("x", 2.5, 5.5, 0.3, 6.4, "#5E2215")]),
            Solid(box(10, 14, 8, 11, 0, 3), "card", [("y", 11.3, 12.7, 0, 2.2, VOID)]),
            Solid(gable_y(10, 14, 8, 11, 3, 4.8), "card")]
    S["hall"] = (hall, (-2, 18, -2, 13), 10, "Card model: timber village hall with the old stone porch kept", "")
    # 6 Garden room: house far, red studio near end of a long garden
    house = [Solid(box(0, 9, 0, 7, 0, 6), "card", win_grid("y", 7, 0, 9, 3, 2, 3, 1, 1.7, 0.6) + win_grid("x", 9, 0, 7, 2, 2, 3, 1, 1.7, 0.6)),
             Solid(gable_x(0, 9, 0, 7, 6, 8.6), "card")]
    wall = [Solid(box(9, 9.3, 7, 27, 0, 1.4), "card"), Solid(box(-0.3, 0, 7, 27, 0, 1.4), "card")]
    studio = [Solid(mono_x(1, 8, 20, 25, 0, 2.4, 3.4), "red", [("y", 1.8, 5.6, 0.2, 2.1, "#5E2215"), ("x", 21, 24, 0.9, 2.1, "#5E2215")])]
    S["garden"] = (house + wall + studio, (-1.5, 11, -1.5, 28), 9, "Card model: garden studio at the end of a long garden", "")
    return S


if __name__ == "__main__":
    here = os.path.dirname(os.path.abspath(__file__))
    out = {}
    for k, (solids, base, s, title, _) in scenes().items():
        out[k] = svg(k, solids, base, s, title, "")
    with open(os.path.join(here, "models.json"), "w", encoding="utf-8") as f:
        json.dump(out, f)
    for k, v in out.items():
        print(k, len(v))
