"""Procedural architectural drawings for the Fold page. Each returns (ratio, svg inner markup).
viewBox height is 100; width = ratio * 100. One graphic language: filled ink poche for cut walls,
one non-scaling stroke weight for lines, light-grey tone for ground/stone, orange for new work."""

INK = "#18191b"
MID = "#8d9096"
LIGHT = "#c9cbce"
PALE = "#dfe1e3"
NEW = "#ff4a1f"
PAPER = "#eff0f1"


def f(v):
    return ("%.2f" % v).rstrip("0").rstrip(".")


def r(x, y, w, h, fill=INK, extra=""):
    return f'<rect x="{f(x)}" y="{f(y)}" width="{f(w)}" height="{f(h)}" fill="{fill}"{extra}/>'


def poly(pts, fill=INK):
    return f'<polygon points="{" ".join(f(a)+","+f(b) for a,b in pts)}" fill="{fill}"/>'


def line(pts, stroke=INK, w=1.1, fill="none"):
    return (f'<polyline points="{" ".join(f(a)+","+f(b) for a,b in pts)}" fill="{fill}" stroke="{stroke}" '
            f'stroke-width="{w}" vector-effect="non-scaling-stroke" stroke-linejoin="round"/>')


def circ(cx, cy, rad, fill=MID):
    return f'<circle cx="{f(cx)}" cy="{f(cy)}" r="{f(rad)}" fill="{fill}"/>'


def arc_door(x, y, size, dirx=1, diry=1):
    # quarter-circle door swing with its leaf; hinge at (x,y)
    ex, ey = x + size * dirx, y
    lx, ly = x, y + size * diry
    sweep = 1 if dirx * diry > 0 else 0
    return (line([(x, y), (lx, ly)], INK, 1.1) +
            f'<path d="M{f(lx)},{f(ly)} A{f(size)},{f(size)} 0 0 {sweep} {f(ex)},{f(ey)}" fill="none" stroke="{MID}" '
            f'stroke-width="1" vector-effect="non-scaling-stroke"/>')


def person(x, base, s=1.0, fill=MID):
    return circ(x, base - 15.5 * s, 1.7 * s, fill) + r(x - 1.6 * s, base - 13.4 * s, 3.2 * s, 13.4 * s, fill)


def tree(x, base, h=26, rad=9):
    return r(x - 0.8, base - h * 0.55, 1.6, h * 0.55, MID) + circ(x, base - h + rad * 0.7, rad, LIGHT) + circ(x + rad * 0.45, base - h + rad * 0.95, rad * 0.62, PALE)


def earth(W, y=88):
    out = r(0, y, W, 100 - y, LIGHT)
    for i in range(int(W // 7) + 1):
        out += line([(i * 7, 100), (i * 7 + 5, y + 1.5)], MID, 0.8)
    return out + r(0, y, W, 1.2, INK)


def bird(x, y):
    return line([(x - 2, y - 1), (x, y), (x + 2, y - 1.2)], INK, 1)


# ---------- tenement elevation ----------
def tenement_elevation(storeys=4, bays=3, new="dormer", ratio=1.5):
    W = ratio * 100
    x0, x1 = 10, W - 10
    fh = 15.5
    base = 90
    top = base - storeys * fh
    o = r(0, base, W, 10, LIGHT) + r(0, base, W, 1.2, INK)
    o += r(x0, top, x1 - x0, storeys * fh, PALE)
    # ashlar courses
    for i in range(1, int(storeys * fh / 3.9)):
        o += line([(x0, top + i * 3.9), (x1, top + i * 3.9)], LIGHT, 0.7)
    # string course and cornice
    o += r(x0 - 1.5, top - 2.5, x1 - x0 + 3, 2.5, INK)
    o += r(x0, base - fh + 0.5, x1 - x0, 1.4, MID)
    # roof
    o += poly([(x0 - 1, top - 2.5), (x0 + 7, top - 15), (x1 - 7, top - 15), (x1 + 1, top - 2.5)], INK)
    for cx in (x0 + 9, x1 - 13):
        o += r(cx, top - 21, 6, 7, MID) + r(cx + 0.6, top - 24, 1.6, 3, INK) + r(cx + 3.6, top - 24, 1.6, 3, INK)
    if new == "dormer":
        mx = (x0 + x1) / 2
        o += poly([(mx - 9, top - 3), (mx - 9, top - 12), (mx + 9, top - 12), (mx + 9, top - 3)], NEW)
        o += r(mx - 6, top - 10, 12, 5.5, INK)
    if new == "rooflights":
        for k in range(3):
            o += r(x0 + 22 + k * (x1 - x0 - 44) / 2 - 3, top - 11, 6, 6, NEW)
    o += bird(x1 - 24, top - 19)
    colw = (x1 - x0) / bays
    for b in range(bays):
        cx = x0 + colw * (b + 0.5)
        bay = (b == 0 or b == bays - 1) and bays > 2
        if bay:
            o += r(cx - colw * 0.36, top + 1, colw * 0.72, storeys * fh - 1, LIGHT)
        for s in range(storeys):
            y = top + s * fh + 3.5
            if s == storeys - 1 and b == bays // 2:
                # close entrance
                o += r(cx - 3.5, base - 12.5, 7, 12.5, INK)
                o += circ(cx, base - 12.5, 3.5, INK) + r(cx - 3.5, base - 12.5, 7, 0.6, INK)
                o += r(cx + 4.5, base - 11, 1.6, 1.6, NEW if new == "door" else MID)
                continue
            ww = 5.2 if not bay else 4.4
            xs = [cx - ww / 2] if not bay else [cx - ww * 1.6, cx - ww / 2, cx + ww * 0.6]
            for wx in xs:
                o += r(wx, y, ww, 9, INK)
                o += line([(wx, y + 4.5), (wx + ww, y + 4.5)], PAPER, 0.8)
                o += line([(wx + ww / 2, y), (wx + ww / 2, y + 9)], PAPER, 0.6)
                o += r(wx - 0.6, y + 9, ww + 1.2, 0.9, MID)
    o += person(x0 + 4, base, 0.75)
    return ratio, o


# ---------- tenement section ----------
def tenement_section(storeys=4, ratio=1.3, new="insulation"):
    W = ratio * 100
    x0, x1 = 16, W - 16
    fh = 15
    base = 88
    top = base - storeys * fh
    o = earth(W, base)
    o += r(x0 - 4, base, (x1 - x0) + 8, 4, INK)  # foundation
    mid = (x0 + x1) / 2
    # roof void
    o += poly([(x0 - 3, top), (mid, top - 17), (x1 + 3, top)], PALE)
    o += line([(x0 - 3, top), (mid, top - 17), (x1 + 3, top)], INK, 1.4)
    o += r(x1 - 15, top - 19, 5, 12, INK) + r(x1 - 14.4, top - 22, 1.4, 3, MID) + r(x1 - 12, top - 22, 1.4, 3, MID)
    if new == "insulation":
        o += line([(x0 + 1, top - 1.2), (mid, top - 15.5), (x1 - 1, top - 1.2)], NEW, 2.6)
    for s in range(storeys + 1):
        y = base - s * fh
        o += r(x0, y - 1.3, x1 - x0, 1.3, INK)
    for wx in (x0 - 3.6, x1):
        for s in range(storeys):
            y = base - (s + 1) * fh
            o += r(wx, y, 3.6, fh, INK)
            o += r(wx + 0.6, y + 4, 2.4, 7, PAPER)
            if new == "insulation":
                ix = wx + 3.6 if wx < mid else wx - 1.8
                o += r(ix, y + 0.2, 1.8, fh - 1.5, NEW)
    # close: the common stair in the middle bay
    sx0, sx1 = mid - 9, mid + 9
    for s in range(storeys):
        yb = base - s * fh - 1.3
        yt = base - (s + 1) * fh
        pts = []
        steps = 6
        for k in range(steps + 1):
            px = sx0 + (sx1 - sx0) * k / steps if s % 2 == 0 else sx1 - (sx1 - sx0) * k / steps
            py = yb - (yb - yt) * k / steps
            pts += [(px, py)]
        o += line(pts, INK, 1.1)
        o += r(sx0 - 1, yt, 1, fh, MID) if s % 2 else r(sx1, yt, 1, fh, MID)
    o += person(x0 + 9, base - 1.3, 0.75)
    o += person(x1 - 10, base - 2 * fh - 1.3, 0.75)
    return ratio, o


# ---------- generic plan ----------
def plan(ratio=1.3, walls=None, parts=(), doors=(), windows=(), new=(), extra=""):
    """walls: outer rect (x,y,w,h); parts: (x1,y1,x2,y2) partitions; new: rects in orange; windows: (x,y,w,h) gaps."""
    W = ratio * 100
    x, y, w, h = walls or (8, 8, W - 16, 84)
    t = 3.2
    o = r(x, y, w, h, PALE)
    o += r(x, y, w, t, INK) + r(x, y + h - t, w, t, INK) + r(x, y, t, h, INK) + r(x + w - t, y, t, h, INK)
    for (a, b, c, d) in parts:
        if a == c:
            o += r(a - 0.7, min(b, d), 1.4, abs(d - b), INK)
        else:
            o += r(min(a, c), b - 0.7, abs(c - a), 1.4, INK)
    for (wx, wy, ww, wh) in windows:
        o += r(wx, wy, ww, wh, PAPER)
        if ww > wh:
            o += line([(wx, wy + wh / 2), (wx + ww, wy + wh / 2)], MID, 0.8)
        else:
            o += line([(wx + ww / 2, wy), (wx + ww / 2, wy + wh)], MID, 0.8)
    for d in doors:
        o += arc_door(*d)
    for (nx, ny, nw, nh) in new:
        o += r(nx, ny, nw, nh, NEW)
    return ratio, o + extra


# ---------- wall detail ----------
def wall_detail(ratio=0.75, kind="stone"):
    W = ratio * 100
    o = ""
    if kind == "stone":
        sx, sw = 6, 30
        o += r(sx, 0, sw, 100, INK)
        for i in range(9):
            yy = i * 11.5 + (3 if i % 2 else 0)
            o += r(sx + 3 + (i % 3) * 2, yy + 2, sw - 9 - (i % 2) * 4, 8, MID)
        ix = sx + sw + 3
        o += r(ix, 0, 12, 100, PALE)  # battened service void
        for k in range(6):
            o += r(ix, k * 18 + 4, 12, 3, LIGHT)
        o += r(ix + 12, 0, 6, 100, NEW)  # new wood-fibre insulation
        o += r(ix + 18, 0, 3, 100, LIGHT) + r(ix + 21, 0, 1.2, 100, INK)
        o += r(ix + 22.2, 82, 4, 18, INK)  # skirting
    elif kind == "timber":
        o += r(6, 0, 4, 100, MID)  # larch boards
        for k in range(10):
            o += r(6, k * 10, 4, 0.8, INK)
        o += r(12, 0, 1.2, 100, INK)  # membrane
        o += r(15, 0, 22, 100, PALE)  # timber stud zone
        for k in range(5):
            o += r(15, k * 20 + 8, 22, 4, LIGHT)
        o += r(37, 0, 6, 100, NEW)   # new clt panel
        o += r(45, 0, 2, 100, LIGHT) + r(47, 0, 1.2, 100, INK)
    elif kind == "rooflight":
        o += poly([(0, 60), (W, 34), (W, 44), (0, 70)], LIGHT)
        o += poly([(0, 56), (W, 30), (W, 34), (0, 60)], INK)
        o += r(W * 0.32, 22, W * 0.36, 30, NEW)
        o += r(W * 0.36, 26, W * 0.28, 22, PAPER)
        o += line([(W * 0.36, 37), (W * 0.64, 37)], MID, 0.8)
        o += r(W * 0.28, 52, 3, 48, INK) + r(W * 0.69, 44, 3, 56, INK)
        o += person(W * 0.5, 100, 0.9)
    elif kind == "shelf":
        o += r(8, 4, 6, 92, INK)
        for k in range(6):
            yy = 10 + k * 14
            o += r(14, yy, 40, 2, NEW)
            for b in range(7):
                bh = 7 + (b * 37 % 5)
                o += r(16 + b * 5.4, yy - bh, 4.2, bh, MID if (b + k) % 3 else LIGHT)
        o += r(54, 4, 2.4, 92, NEW)
    elif kind == "sash":
        o += r(10, 6, 18, 88, INK)  # stone ingo
        o += r(W - 28, 6, 18, 88, INK)
        o += r(28, 10, W - 56, 40, PAPER) + r(28, 50, W - 56, 40, PAPER)
        o += r(28, 10, W - 56, 3, MID) + r(28, 47, W - 56, 3, MID) + r(28, 50, W - 56, 3, MID) + r(28, 87, W - 56, 3, MID)
        o += r(26, 6, 2, 88, NEW) + r(W - 28, 6, 2, 88, NEW)  # draught seals
        o += line([(W / 2, 13), (W / 2, 47)], MID, 0.8) + line([(W / 2, 53), (W / 2, 87)], MID, 0.8)
    return ratio, o


# ---------- extension section ----------
def extension_section(ratio=1.3, roof="mono", host_storeys=2):
    W = ratio * 100
    base = 86
    o = earth(W, base)
    hx = 14
    # existing house to the left
    o += r(0, base - host_storeys * 17 - 6, hx, host_storeys * 17 + 6, PALE)
    o += r(hx, base - host_storeys * 17 - 6, 4, host_storeys * 17 + 6, INK)
    o += r(hx, base - 17 + 3, 4, 11, PAPER)  # opening through old wall
    ex1 = W - 22
    if roof == "mono":
        rt, rb = base - 30, base - 22
        o += poly([(hx + 4, rt), (ex1 + 4, rb), (ex1 + 4, rb + 3), (hx + 4, rt + 3)], NEW)
        o += r(hx + 10, rt + 0.3, 12, 3, PAPER)  # rooflight
        o += r(ex1, rb + 3, 4, base - rb - 3, NEW)
        o += r(ex1 + 0.8, rb + 6, 2.4, base - rb - 8, PAPER)
    elif roof == "flat":
        rt = base - 24
        o += r(hx + 4, rt, ex1 - hx, 3.4, NEW)
        o += r(ex1, rt, 4, base - rt, NEW)
        o += r(ex1 + 0.8, rt + 4, 2.4, base - rt - 6, PAPER)
        o += r(hx + 30, rt - 3, 14, 3, PAPER) + line([(hx + 30, rt - 3), (hx + 44, rt - 3)], INK, 1)
    else:  # pitched
        mid = (hx + 4 + ex1 + 4) / 2
        o += poly([(hx + 4, base - 22), (mid, base - 36), (ex1 + 4, base - 22), (ex1 + 4, base - 19), (mid, base - 33), (hx + 4, base - 19)], NEW)
        o += r(ex1, base - 19, 4, 19, NEW) + r(ex1 + 0.8, base - 16, 2.4, 14, PAPER)
    o += r(hx + 4, base - 1.4, ex1 - hx, 1.4, INK)
    o += person((hx + ex1) / 2, base - 1.4, 0.8)
    o += tree(W - 8, base, 30, 8)
    return ratio, o


# ---------- block / site plan ----------
def site_plan(ratio=1.0, new=(55, 52, 10, 14)):
    W = ratio * 100
    o = r(0, 0, W, 100, PALE)
    # street
    o += r(0, 0, W, 10, LIGHT) + r(0, 90, W, 10, LIGHT)
    # perimeter block of tenements around a back court
    o += r(6, 14, W - 12, 16, INK) + r(6, 70, W - 12, 16, INK)
    o += r(6, 30, 14, 40, INK) + r(W - 20, 30, 14, 40, INK)
    # back court divisions
    for k in range(1, 4):
        xx = 20 + k * (W - 40) / 4
        o += line([(xx, 30), (xx, 70)], MID, 0.8)
    o += line([(20, 50), (W - 20, 50)], MID, 0.8)
    o += circ(32, 40, 4, LIGHT) + circ(W - 34, 60, 5, LIGHT) + circ(W - 40, 38, 3.5, LIGHT)
    o += r(*new, NEW)
    o += r(W / 2 - 2, 11, 4, 3, PAPER)  # close entry from street
    return ratio, o


# ---------- civic: hall section ----------
def hall_section(ratio=1.4):
    W = ratio * 100
    base = 88
    o = earth(W, base)
    x0, x1 = 12, W - 12
    top = 26
    o += r(x0 - 5, top, 5, base - top, INK) + r(x1, top, 5, base - top, INK)
    # arched windows cut in section shown as light openings
    for wx in (x0 - 4, x1 + 1):
        o += r(wx, 40, 3, 26, PAPER)
    # coffered ceiling
    o += r(x0, top, x1 - x0, 3, INK)
    for k in range(9):
        o += r(x0 + 3 + k * (x1 - x0 - 6) / 8 - 1, top + 3, 2, 3, MID)
    o += poly([(x0 - 6, top), ((x0 + x1) / 2, 12), (x1 + 6, top)], LIGHT)
    o += line([(x0 - 6, top), ((x0 + x1) / 2, 12), (x1 + 6, top)], INK, 1.4)
    # new mezzanine on slender steel + timber deck
    my = 60
    o += r(x0 + (x1 - x0) * 0.45, my, (x1 - x0) * 0.55, 2.6, NEW)
    o += r(x0 + (x1 - x0) * 0.47, my + 2.6, 1.4, base - my - 2.6, NEW)
    o += r(x0 + (x1 - x0) * 0.75, my + 2.6, 1.4, base - my - 2.6, NEW)
    o += line([(x0 + (x1 - x0) * 0.45, my - 6), (x0 + (x1 - x0) * 0.45, my)], NEW, 1.6)
    # stair to mezzanine
    pts = []
    sx = x0 + (x1 - x0) * 0.2
    for k in range(9):
        pts += [(sx + k * 3, base - k * 3.1), (sx + (k + 1) * 3, base - k * 3.1)]
    o += line(pts, NEW, 1.4)
    for k in range(4):
        o += r(x1 - 6 - k * 6, my - 12, 4.6, 12, MID)
    o += person(x0 + 10, base, 0.8) + person(x1 - 12, my, 0.8)
    o += r(x0, base - 1.4, x1 - x0, 1.4, INK)
    return ratio, o


def bank_elevation(ratio=1.0):
    W = ratio * 100
    base = 90
    o = r(0, base, W, 10, LIGHT) + r(0, base, W, 1.2, INK)
    x0, x1 = 12, W - 12
    o += r(x0, 22, x1 - x0, base - 22, PALE)
    o += poly([(x0 - 3, 22), (W / 2, 8), (x1 + 3, 22)], LIGHT)
    o += line([(x0 - 3, 22), (W / 2, 8), (x1 + 3, 22)], INK, 1.2)
    o += r(x0 - 3, 22, x1 - x0 + 6, 3, INK)
    cols = 4
    for k in range(cols):
        cx = x0 + 6 + k * (x1 - x0 - 12) / (cols - 1)
        o += r(cx - 2.2, 26, 4.4, base - 30, LIGHT) + r(cx - 3, 26, 6, 2, MID) + r(cx - 3, base - 5, 6, 3, MID)
    for k in range(cols - 1):
        cx = x0 + 6 + (k + 0.5) * (x1 - x0 - 12) / (cols - 1)
        if k == 1:
            o += r(cx - 5, 56, 10, base - 56, NEW)  # new accessible entrance
            o += r(cx - 3.6, 59, 7.2, base - 59, INK)
        else:
            o += r(cx - 4, 34, 8, 20, INK) + circ(cx, 34, 4, INK)
            o += r(cx - 4, 62, 8, 18, INK)
    o += r(x0 - 6, base - 3, 18, 3, NEW)  # new ramp
    o += person(x1 + 5, base, 0.75)
    return ratio, o


# ---------- civic: timber pavilion ----------
def pavilion_elevation(ratio=2.0):
    W = ratio * 100
    base = 80
    o = r(0, base, W, 20, LIGHT)
    for k in range(0, int(W), 16):
        o += r(k, base + 1, 8, 19, PALE)  # mown stripes of the green
    o += r(0, base, W, 1.2, INK)
    x0, x1 = 22, W - 22
    o += poly([(x0 - 10, 44), (x1 + 10, 44), (x1 + 10, 48), (x0 - 10, 48)], INK)
    o += r(x0, 48, x1 - x0, base - 48, MID)
    for k in range(int((x1 - x0) / 3)):
        o += r(x0 + k * 3, 48, 0.7, base - 48, INK)
    gx0 = x0 + (x1 - x0) * 0.3
    o += r(gx0, 52, (x1 - x0) * 0.4, base - 52, INK)
    for k in range(1, 5):
        o += r(gx0 + k * (x1 - x0) * 0.08, 52, 0.7, base - 52, PAPER)
    for px in (x0 - 9, x1 + 8):
        o += r(px, 48, 1.6, base - 48, NEW)  # glulam posts
    o += r(x0 - 10, 44, x1 - x0 + 20, 1.4, NEW)
    o += person(x0 - 4, base, 0.8) + person(x1 + 14, base, 0.8)
    o += circ(W - 9, base - 1.5, 1.4, INK) + circ(12, base - 1.5, 1.4, NEW)
    o += tree(10, base, 34, 10)
    return ratio, o


def pavilion_section(ratio=1.3):
    W = ratio * 100
    base = 84
    o = earth(W, base)
    x0, x1 = 16, W - 16
    mid = (x0 + x1) / 2
    o += r(x0, 46, 3, base - 46, INK) + r(x1 - 3, 46, 3, base - 46, INK)
    o += poly([(x0 - 10, 47), (mid, 24), (x1 + 10, 47), (x1 + 10, 50), (mid, 27), (x0 - 10, 50)], INK)
    # glulam truss, new
    o += line([(x0, 48), (mid, 30), (x1, 48), (x0, 48)], NEW, 2.2)
    o += line([(mid, 30), (mid, 48)], NEW, 1.6)
    o += line([(x0 + (mid - x0) / 2, 39), (mid, 48), (mid + (x1 - mid) / 2, 39)], NEW, 1.2)
    o += r(x0 + 3, 54, 18, 1.6, MID) + r(x0 + 3, 62, 18, 1.6, MID)
    o += person(mid - 8, base, 0.8) + person(mid + 10, base, 0.8)
    return ratio, o


# ---------- house elevations ----------
def cottage_elevation(ratio=1.7):
    W = ratio * 100
    base = 88
    o = r(0, base, W, 12, LIGHT) + r(0, base, W, 1.2, INK)
    x0, x1 = 12, W - 12
    o += r(x0, 46, x1 - x0, base - 46, PALE)
    o += r(x0, 46, x1 - x0, 6, LIGHT)
    o += poly([(x0 - 4, 46), (x0 + 14, 20), (x1 - 14, 20), (x1 + 4, 46)], INK)
    for cx in (x0 + 30, x1 - 34):
        o += r(cx, 12, 6, 9, MID)
    for k in range(4):
        cx = x0 + 14 + k * (x1 - x0 - 28) / 3
        o += r(cx - 6, 56, 12, 10, INK) + r(cx - 7, 54, 14, 2, NEW)  # new insulated window heads
        o += line([(cx, 56), (cx, 66)], PAPER, 0.7)
        o += r(cx - 6, 72, 12, 10, INK)
        o += line([(cx, 72), (cx, 82)], PAPER, 0.7)
    o += r(x0 - 1, 46, x1 - x0 + 2, 1.6, NEW)  # external insulation line at eaves
    o += person(x1 + 6, base, 0.75)
    o += tree(5, base, 30, 6)
    return ratio, o


def house_elevation(ratio=1.6, ext=True):
    W = ratio * 100
    base = 86
    o = r(0, base, W, 14, LIGHT) + r(0, base, W, 1.2, INK)
    hx0, hx1 = 10, W * 0.52
    o += r(hx0, 40, hx1 - hx0, base - 40, PALE)
    o += poly([(hx0 - 3, 40), (hx0 + 4, 24), (hx1 - 4, 24), (hx1 + 3, 40)], INK)
    for k in range(2):
        cx = hx0 + (hx1 - hx0) * (0.3 + 0.4 * k)
        o += r(cx - 6, 47, 12, 10, INK) + r(cx - 6, 66, 12, 14, INK)
        o += line([(cx, 47), (cx, 57)], PAPER, 0.7)
    if ext:
        ex1 = W - 14
        o += r(hx1, 58, ex1 - hx1, base - 58, MID)
        for k in range(int((ex1 - hx1) / 2.6)):
            o += r(hx1 + k * 2.6, 58, 0.6, base - 58, INK)
        o += r(hx1 + 6, 64, ex1 - hx1 - 12, base - 64, INK)
        for k in range(1, 4):
            o += r(hx1 + 6 + k * (ex1 - hx1 - 12) / 4, 64, 0.7, base - 64, PAPER)
        o += r(hx1, 56, ex1 - hx1 + 2, 2.4, NEW)
        o += r(ex1, 58, 1.6, base - 58, NEW)
    o += tree(W - 5, base, 36, 7)
    o += bird(hx0 + 22, 21)
    return ratio, o


def rear_elevation(ratio=0.9):
    W = ratio * 100
    base = 90
    o = r(0, base, W, 10, LIGHT) + r(0, base, W, 1.2, INK)
    x0, x1 = 6, W - 6
    o += r(x0, 8, x1 - x0, base - 8, PALE)
    for i in range(1, 21):
        o += line([(x0, 8 + i * 4), (x1, 8 + i * 4)], LIGHT, 0.6)
    for s in range(4):
        y = 12 + s * 15
        for cx in (x0 + 12, x1 - 12):
            o += r(cx - 3, y, 6, 9, INK)
            o += line([(cx - 3, y + 4.5), (cx + 3, y + 4.5)], PAPER, 0.7)
        o += r((x0 + x1) / 2 - 1.5, y + 2, 3, 12, MID)  # stair window of the close
    # single storey kitchen extension in the back court
    o += r(x0 + 4, 72, x1 - x0 - 8, base - 72, MID)
    for k in range(int((x1 - x0 - 8) / 2.4)):
        o += r(x0 + 4 + k * 2.4, 72, 0.6, base - 72, INK)
    o += r(x0 + 4, 70, x1 - x0 - 8, 2.4, NEW)
    o += r(x0 + 12, 76, x1 - x0 - 24, base - 76, INK)
    o += person(x1 - 3, base, 0.7)
    return ratio, o
