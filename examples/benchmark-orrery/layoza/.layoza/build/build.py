# Builds ../../index.html from template.html + gen.json (dev-only)
import json, math, os, random, datetime
here = os.path.dirname(os.path.abspath(__file__))
g = json.load(open(os.path.join(here, 'gen.json')))
random.seed(7)

# ---------- Harbourside skyline (Cliftonwood terraces, the Orrery loft with its roof dome, dockside cranes)
W, Q = 1440, 172
sil, win, refl, lines = [], [], [], []
# hill
hill = lambda x: 96 + (x/560)**1.2*76 if x < 560 else Q
pts = [(0, Q)] + [(x, hill(x)) for x in range(0, 561, 20)] + [(560, Q)]
sil.append("M" + "L".join(f"{x:.0f} {y:.1f}" for x, y in pts) + "Z")
# terraces stepping down the hill, three rows
for row, dy in enumerate((0, 26, 50)):
    x = 4 + row*9
    while x < 520 - row*20:
        w = random.choice((20, 22, 24, 26)); h = random.choice((24, 27, 30))
        base = hill(x + w/2) + dy + 4
        top = base - h
        roof = top - 9
        sil.append(f"M{x} {base:.1f}V{top:.1f}L{x+w/2:.1f} {roof:.1f}L{x+w} {top:.1f}V{base:.1f}Z")
        # chimney
        if random.random() < .5:
            sil.append(f"M{x+w-6} {top-4:.1f}h3v-8h-3Z")
        for wy in (top + 6, top + 15):
            if wy + 5 < base:
                for wx in (x + 4, x + w - 9):
                    if random.random() < .24:
                        win.append(f"M{wx} {wy:.1f}h5v6h-5Z")
        x += w + 1
# the Orrery loft: a brick warehouse with a rooftop observatory dome
bx, bw, btop = 600, 236, 92
sil.append(f"M{bx} {Q}V{btop+8}L{bx+18} {btop}H{bx+bw-18}L{bx+bw} {btop+8}V{Q}Z")
for c in range(9):
    for r_ in range(3):
        wx = bx + 16 + c*24; wy = btop + 20 + r_*22
        lit = (c + r_) % 3 != 1
        if lit:
            win.append(f"M{wx} {wy+12}V{wy+4}a5 5 0 0 1 10 0V{wy+12}Z")
dx, dr = bx + 150, 30
sil.append(f"M{dx-dr-4} {btop+1}h{2*dr+8}v-6h-{2*dr+8}Z")
sil.append(f"M{dx-dr} {btop-5}A{dr} {dr} 0 0 1 {dx+dr} {btop-5}Z")
# open slit + telescope tube in the dome
win.append(f"M{dx-3} {btop-5-dr+2}h7v{dr-3}h-7Z")
lines.append(f"M{dx+1} {btop-14}L{dx+13} {btop-dr-14}")
# M Shed-style dockside cranes along the quay
def crane(x, s=1.0, flip=False):
    k = -1 if flip else 1
    p = []
    # portal legs
    p.append(f"M{x-14*s:.1f} {Q}L{x-9*s:.1f} {Q-34*s:.1f}M{x+14*s:.1f} {Q}L{x+9*s:.1f} {Q-34*s:.1f}M{x-11*s:.1f} {Q-18*s:.1f}H{x+11*s:.1f}")
    # cab
    sil.append(f"M{x-12*s:.1f} {Q-34*s:.1f}h{24*s:.1f}v{-26*s:.1f}h{-24*s:.1f}Z")
    sil.append(f"M{x-8*s:.1f} {Q-60*s:.1f}l{4*s:.1f} {-8*s:.1f}h{8*s:.1f}l{4*s:.1f} {8*s:.1f}Z")
    win.append(f"M{x-8*s:.1f} {Q-54*s:.1f}h{6*s:.1f}v{6*s:.1f}h{-6*s:.1f}Z")
    # counterweight
    sil.append(f"M{x+k*12*s:.1f} {Q-50*s:.1f}h{k*14*s:.1f}v{12*s:.1f}h{-k*14*s:.1f}Z")
    # lattice jib
    ang = math.radians(32)
    L = 128*s
    x0, y0 = x - k*6*s, Q - 50*s
    x1, y1 = x0 - k*L*math.cos(ang), y0 - L*math.sin(ang)
    nx, ny = -math.sin(ang)*5*s*k, math.cos(ang)*5*s
    nx, ny = (math.sin(ang)*5*s, math.cos(ang)*5*s*-1) if False else (nx, ny)
    a = (x0, y0); b = (x1, y1); a2 = (x0 + nx, y0 - ny); b2 = (x1 + nx*0.4, y1 - ny*0.4)
    p.append(f"M{a[0]:.1f} {a[1]:.1f}L{b[0]:.1f} {b[1]:.1f}M{a2[0]:.1f} {a2[1]:.1f}L{b2[0]:.1f} {b2[1]:.1f}")
    zz = []
    n = 11
    for i in range(n + 1):
        t = i/n
        px = a[0] + (b[0]-a[0])*t; py = a[1] + (b[1]-a[1])*t
        qx = a2[0] + (b2[0]-a2[0])*t; qy = a2[1] + (b2[1]-a2[1])*t
        zz.append((px, py) if i % 2 == 0 else (qx, qy))
    p.append("M" + "L".join(f"{u:.1f} {v:.1f}" for u, v in zz))
    # hoist rope and hook
    p.append(f"M{x1:.1f} {y1:.1f}V{y1+44*s:.1f}M{x1-3*s:.1f} {y1+44*s:.1f}h{6*s:.1f}")
    # A-frame tie
    p.append(f"M{x:.1f} {Q-68*s:.1f}L{x1+ (x0-x1)*0.35:.1f} {y1 + (y0-y1)*0.35:.1f}")
    lines.extend(p)
for cx, s, fl in ((930, 1.0, False), (1060, 0.92, False), (1196, 1.05, False), (1330, 0.88, False)):
    crane(cx, s, fl)
# quay edge and bollards
sil.append(f"M0 {Q-2}H{W}V{Q+6}H0Z")
for bxp in range(560, 1440, 70):
    sil.append(f"M{bxp} {Q-2}v-5h6v5Z")
# water reflections under lit windows
for i in range(70):
    rx = random.randint(0, 1430); ry = random.randint(Q + 14, 214); rw = random.randint(10, 46)
    refl.append(f"M{rx} {ry}h{rw}")
skyline = (f'<svg class="skyline" viewBox="0 0 {W} 220" preserveAspectRatio="xMidYMax slice" aria-hidden="true" focusable="false">'
           f'<path class="sk-sil" d="{"".join(sil)}"/>'
           f'<path class="sk-line" d="{"".join(lines)}"/>'
           f'<path class="sk-win" d="{"".join(win)}"/>'
           f'<rect class="sk-water" x="0" y="{Q+6}" width="{W}" height="{220-Q-6}"/>'
           f'<path class="sk-refl" d="{"".join(refl)}"/></svg>')

# default (no-JS) rotation: Fri 16 Oct 2026, 21:00 BST = 20:00 UT
ut = datetime.datetime(2026, 10, 16, 20, 0)
jd = (ut - datetime.datetime(1970, 1, 1)).total_seconds()/86400 + 2440587.5
lst = (280.46061837 + 360.98564736629*(jd - 2451545) - 2.5879) % 360

t = open(os.path.join(here, 'template.html'), encoding='utf-8').read()
t = (t.replace('%%DISC%%', g['disc']).replace('%%OVER%%', g['over'])
      .replace('%%NAMED%%', json.dumps(g['named'], separators=(',', ':')))
      .replace('%%SKYLINE%%', skyline).replace('%%ROT%%', f"{-lst:.2f}"))
out = os.path.normpath(os.path.join(here, '..', '..', 'index.html'))
open(out, 'w', encoding='utf-8', newline='\n').write(t)
print('wrote', out, len(t), 'bytes; default rot', round(-lst, 2))
