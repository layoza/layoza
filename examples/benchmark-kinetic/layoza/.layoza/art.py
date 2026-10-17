"""Generates the authored SVG art for the Kinetic Signal page (dev-only)."""
import math, random, json

OBS, GRA, POR, INK = "#090B11", "#151923", "#F6F5F0", "#111318"
CO, CO2, LIME, LAV = "#4865FF", "#304EF0", "#D8FF57", "#A99BFF"


def f(v):
    return f"{v:.1f}".rstrip("0").rstrip(".")


def mix(a, b, t):
    a = [int(a[i:i + 2], 16) for i in (1, 3, 5)]
    b = [int(b[i:i + 2], 16) for i in (1, 3, 5)]
    return "#" + "".join(f"{round(x + (y - x) * t):02x}" for x, y in zip(a, b))


def ramp(stops, t):
    t = max(0, min(1, t))
    n = len(stops) - 1
    i = min(int(t * n), n - 1)
    return mix(stops[i], stops[i + 1], t * n - i)


def ribbon(center, width, theta, n, stops_front, stops_back, light=0.7):
    """2D projection of a twisting band. Returns svg polygons."""
    out = []
    pts = [center(i / n) for i in range(n + 1)]
    for i in range(n):
        u0, u1 = i / n, (i + 1) / n
        p0, p1 = pts[i], pts[i + 1]
        pp = pts[max(i - 1, 0)]
        tx, ty = p1[0] - pp[0], p1[1] - pp[1]
        L = math.hypot(tx, ty) or 1
        nx, ny = -ty / L, tx / L
        q = []
        for (p, u) in ((p0, u0), (p1, u1)):
            h = width(u) * math.cos(theta(u))
            q.append((p[0] + nx * h, p[1] + ny * h, p[0] - nx * h, p[1] - ny * h))
        th = theta((u0 + u1) / 2)
        c = math.cos(th)
        shade = 0.5 + 0.5 * math.sin(th + light)
        col = ramp(stops_front if c >= 0 else stops_back, shade)
        poly = [(q[0][0], q[0][1]), (q[1][0], q[1][1]), (q[1][2], q[1][3]), (q[0][2], q[0][3])]
        d = "M" + " L".join(f"{f(x)},{f(y)}" for x, y in poly) + "Z"
        out.append(f'<path d="{d}" fill="{col}" stroke="{col}" stroke-width=".7"/>')
    return "".join(out)


SILVER_F = ["#3B4252", "#8D95A6", "#E9ECF2", "#FFFFFF"]
SILVER_B = ["#1E2330", "#4865FF", "#9DAAF0", "#DCE1EE"]
COBALT_F = ["#1D2C9E", CO2, CO, "#8EA0FF"]
COBALT_B = ["#121C6B", "#2338C4", CO2, "#6F84FF"]


def hero_poster():
    c = lambda u: (60 + 700 * u + 90 * math.sin(2 * math.pi * u * 1.5),
                   360 + 170 * math.sin(2 * math.pi * u + 0.5) * (1 - 0.35 * u) - 60 * math.cos(4 * math.pi * u))
    w = lambda u: 78 * math.sin(math.pi * u) ** 0.55 + 4
    th = lambda u: 3.2 * math.pi * u + 0.4
    body = ribbon(c, w, th, 260, SILVER_F, SILVER_B, 0.9)
    return f'<svg class="hero__poster" viewBox="0 0 820 720" aria-hidden="true" focusable="false">{body}</svg>'


def contact_ribbon():
    c = lambda u: (-120 + 1940 * u, 330 + 150 * math.sin(2 * math.pi * u * 1.25 + 0.6) - 50 * math.sin(2 * math.pi * u * 3.1))
    w = lambda u: 70 + 50 * math.sin(math.pi * u)
    th = lambda u: 4.6 * math.pi * u + 0.2
    body = ribbon(c, w, th, 300, COBALT_F, COBALT_B, 0.5)
    return f'<svg class="finale__ribbon" viewBox="0 0 1700 660" preserveAspectRatio="xMidYMid slice" aria-hidden="true" focusable="false"><g class="finale__ribbon-g">{body}</g></svg>'


def wave_pts(fn, n=160, w=1000):
    return [(w * i / (n - 1), fn(i / (n - 1))) for i in range(n)]


def to_d(pts):
    return "M" + " L".join(f"{f(x)},{f(y)}" for x, y in pts)


def process_states():
    rnd = random.Random(7)
    ph = [rnd.uniform(0, 6.28) for _ in range(6)]
    fr = [rnd.uniform(3, 23) for _ in range(6)]
    am = [rnd.uniform(6, 26) for _ in range(6)]
    jit = [rnd.uniform(-1, 1) for _ in range(160)]
    s1 = lambda u, k=0: 150 + sum(a * math.sin(2 * math.pi * fq * u + p) for a, fq, p in zip(am, fr, ph)) * 0.75 + 12 * jit[min(int(u * 159), 159)]
    s2 = lambda u, k=0: 150 + 62 * math.sin(2 * math.pi * 2 * u + k)
    env = lambda u: 0.35 + 0.65 * math.sin(math.pi * u)
    s3 = lambda u, k=0: 150 + 92 * env(u) * math.sin(2 * math.pi * 4 * u + 1.2 * math.sin(2 * math.pi * 2 * u) + k)
    s4 = lambda u, k=0: 150 + (30 + 100 * u) * math.sin(2 * math.pi * 6 * u + k)
    main = [to_d(wave_pts(s)) for s in (s1, s2, s3, s4)]
    second = [to_d(wave_pts(s1)), to_d(wave_pts(s2)),
              to_d(wave_pts(lambda u: s3(u, 2.2))), to_d(wave_pts(lambda u: s4(u, 1.6)))]
    return main, second


def mini(d, d2, w=1000):
    return (f'<svg class="pstep__mini" viewBox="0 -10 1000 320" aria-hidden="true" focusable="false">'
            f'<path d="{d2}" fill="none" stroke="{LAV}" stroke-width="5" vector-effect="non-scaling-stroke"/>'
            f'<path d="{d}" fill="none" stroke="{CO2}" stroke-width="2.4" vector-effect="non-scaling-stroke"/></svg>')


def work_p1():
    W, H = 1600, 860
    g = []
    for i in range(44):
        y0 = 120 + i * 15
        pts = []
        for k in range(121):
            x = W * k / 120
            u = k / 120
            bump = math.exp(-((u - 0.62 + 0.004 * i) / 0.13) ** 2) * (90 + 3 * i) * math.sin(math.pi * i / 44)
            y = y0 - bump + 6 * math.sin(2 * math.pi * (u * 3 + i * 0.07))
            pts.append((x, y))
        op = 0.18 + 0.6 * math.sin(math.pi * i / 44)
        g.append(f'<path d="{to_d(pts)}" fill="none" stroke="{POR}" stroke-opacity="{f(op)}" stroke-width="1.3"/>')
    c = lambda u: (-60 + 1720 * u, 600 - 330 * math.sin(math.pi * u * 0.9) + 60 * math.sin(2 * math.pi * 2 * u))
    rib = ribbon(c, lambda u: 36 + 14 * math.sin(math.pi * u), lambda u: 3.3 * math.pi * u, 220, COBALT_F, COBALT_B, 0.4)
    route = ('<g stroke="#F6F5F0" stroke-width="2" fill="none" opacity=".9">'
             '<path d="M110 760 H420 Q450 760 470 740 L560 650 Q580 630 610 630 H760"/></g>'
             + "".join(f'<circle cx="{x}" cy="{y}" r="9" fill="{OBS}" stroke="{POR}" stroke-width="2"/>' for x, y in ((110, 760), (300, 760), (515, 695), (760, 630)))
             + f'<circle cx="760" cy="630" r="5" fill="{LIME}"/>')
    word = f'<text x="170" y="270" fill="{POR}" font-family="Space Grotesk, sans-serif" font-weight="700" font-size="132" letter-spacing="-6">HALCYON</text>'
    return (f'<svg viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid slice" aria-hidden="true" focusable="false">'
            f'<rect width="{W}" height="{H}" fill="{OBS}"/><g class="pl" data-depth="0.6">{"".join(g)}</g>'
            f'<g class="pl" data-depth="1.4">{rib}</g><g class="pl" data-depth="0.3">{word}{route}</g></svg>')


def work_p2():
    W, H = 1000, 1250
    def page(x, y, w, h, rot, variant):
        parts = [f'<rect width="{w}" height="{h}" rx="10" fill="{POR}"/>',
                 f'<rect x="28" y="26" width="{w-56}" height="3" fill="{INK}" opacity=".85"/>',
                 f'<rect x="28" y="44" width="70" height="10" rx="2" fill="{INK}"/>']
        parts += [f'<rect x="{w-28-40*k-30}" y="44" width="30" height="8" rx="2" fill="#9AA0AE"/>' for k in range(3)]
        if variant == 0:
            parts.append(f'<rect x="28" y="90" width="{w*0.62}" height="34" rx="3" fill="{INK}"/>')
            parts.append(f'<rect x="28" y="134" width="{w*0.48}" height="34" rx="3" fill="{INK}"/>')
            parts.append(f'<rect x="28" y="196" width="{w-56}" height="{h*0.36}" rx="6" fill="{OBS}"/>')
            cx, cy = 28 + (w - 56) / 2, 196 + h * 0.18
            parts.append(ribbon(lambda u: (40 + (w - 80) * u, cy + 50 * math.sin(2 * math.pi * u * 1.3)), lambda u: 22 * math.sin(math.pi * u) + 2,
                                lambda u: 3 * math.pi * u, 90, SILVER_F, SILVER_B))
            for k in range(6):
                parts.append(f'<rect x="28" y="{196+h*0.36+30+k*20}" width="{(w-56)*(0.92 if k%3 else 0.7)}" height="7" rx="3" fill="#9AA0AE"/>')
        else:
            for k in range(3):
                parts.append(f'<rect x="{28+k*((w-56)/3)}" y="92" width="{(w-56)/3-14}" height="{h*0.3}" rx="6" fill="{[CO, OBS, LAV][k]}"/>')
                for j in range(4):
                    parts.append(f'<rect x="{28+k*((w-56)/3)}" y="{92+h*0.3+22+j*16}" width="{(w-56)/3-30-j*8}" height="6" rx="3" fill="#9AA0AE"/>')
            parts.append(f'<rect x="28" y="{h*0.62}" width="{w*0.7}" height="30" rx="3" fill="{INK}"/>')
            for k in range(5):
                parts.append(f'<rect x="28" y="{h*0.62+50+k*18}" width="{(w-56)*(0.95 if k%2 else 0.8)}" height="7" rx="3" fill="#9AA0AE"/>')
        return (f'<g transform="translate({x} {y}) rotate({rot})" filter="url(#p2s)">' + "".join(parts) + '</g>')
    defs = (f'<defs><filter id="p2s" x="-10%" y="-10%" width="120%" height="130%"><feDropShadow dx="0" dy="24" stdDeviation="22" flood-color="#0a1250" flood-opacity=".45"/></filter></defs>')
    return (f'<svg viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid slice" aria-hidden="true" focusable="false">{defs}'
            f'<rect width="{W}" height="{H}" fill="{CO2}"/>'
            + '<g class="pl" data-depth="0.3">' + ribbon(lambda u: (-80 + 1160 * u, 1060 - 820 * u + 120 * math.sin(2 * math.pi * u * 1.4)), lambda u: 70 * math.sin(math.pi * u) + 8, lambda u: 3.4 * math.pi * u, 160, SILVER_F, SILVER_B, 0.8) + '</g>'
            f'<g class="pl" data-depth="0.5">{page(120, 140, 560, 760, -4, 1)}</g>'
            f'<g class="pl" data-depth="1.2">{page(330, 380, 560, 760, 3, 0)}</g></svg>')


def work_p3():
    W, H = 1000, 1250
    posters = []
    rnd = random.Random(3)
    cells = [(60, 70), (540, 120), (80, 660), (520, 710)]
    for idx, (x, y) in enumerate(cells):
        w, h = 400, 520
        bg = [POR, OBS, CO, POR][idx]
        fg = [INK, POR, POR, INK][idx]
        p = [f'<rect width="{w}" height="{h}" fill="{bg}" stroke="#2B303B" stroke-width="{0 if bg != OBS else 2}"/>']
        if idx == 0:
            p.append(f'<text x="24" y="300" font-family="Space Grotesk, sans-serif" font-weight="700" font-size="290" letter-spacing="-18" fill="{fg}">24</text>')
            p.append(f'<rect x="24" y="340" width="{w-48}" height="3" fill="{fg}"/>')
            p.append(f'<text x="24" y="390" font-family="Space Grotesk, sans-serif" font-weight="500" font-size="30" fill="{fg}">OPEN HOURS</text>')
        elif idx == 1:
            for k in range(60):
                a = 2 * math.pi * k / 60
                r1 = 150 if k % 5 else 132
                p.append(f'<line x1="{f(200+math.cos(a)*r1)}" y1="{f(230+math.sin(a)*r1)}" x2="{f(200+math.cos(a)*160)}" y2="{f(230+math.sin(a)*160)}" stroke="{POR}" stroke-width="{3 if k%5==0 else 1.2}"/>')
            p.append(f'<path d="M200 230 L200 110" stroke="{POR}" stroke-width="6" stroke-linecap="round"/><path d="M200 230 L290 270" stroke="{LIME}" stroke-width="6" stroke-linecap="round"/>')
            p.append(f'<text x="24" y="470" font-family="Space Grotesk, sans-serif" font-weight="600" font-size="34" fill="{fg}">After dark,</text>')
        elif idx == 2:
            p.append(ribbon(lambda u: (-20 + 440 * u, 260 + 120 * math.sin(2 * math.pi * u * 1.1 + 1)), lambda u: 40 * math.sin(math.pi * u) + 6,
                            lambda u: 2.6 * math.pi * u, 120, SILVER_F, SILVER_B))
            p.append(f'<text x="24" y="480" font-family="Space Grotesk, sans-serif" font-weight="700" font-size="40" fill="{fg}">still moving.</text>')
        else:
            for k in range(7):
                p.append(f'<text x="24" y="{80+k*62}" font-family="Space Grotesk, sans-serif" font-weight="700" font-size="58" letter-spacing="-2" fill="{fg if k!=3 else CO2}">{["MON","TUE","WED","THU","FRI","SAT","SUN"][k]} 07–24</text>')
        rot = [-2.5, 2, 1.5, -1.8][idx]
        posters.append(f'<g class="pl" data-depth="{0.4+idx*0.3:.1f}"><g transform="translate({x} {y}) rotate({rot} 200 260)" filter="url(#p3s)">{"".join(p)}</g></g>')
    defs = '<defs><filter id="p3s" x="-10%" y="-10%" width="120%" height="130%"><feDropShadow dx="0" dy="18" stdDeviation="16" flood-color="#000" flood-opacity=".5"/></filter></defs>'
    return (f'<svg viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid slice" aria-hidden="true" focusable="false">{defs}'
            f'<rect width="{W}" height="{H}" fill="{GRA}"/><g class="pl" data-depth="0.3">' + ribbon(lambda u: (-80 + 1160 * u, 200 + 860 * u + 110 * math.sin(2 * math.pi * u * 1.6)), lambda u: 80 * math.sin(math.pi * u) + 8, lambda u: 3.8 * math.pi * u, 160, COBALT_F, COBALT_B, 0.5) + f'</g>{"".join(posters)}</svg>')


def work_p4():
    W, H = 2400, 1000
    rings = []
    for i in range(34):
        r = 40 + i * 30
        pts = []
        for k in range(97):
            a = 2 * math.pi * k / 96
            rr = r * (1 + 0.06 * math.sin(3 * a + i * 0.35) + 0.04 * math.sin(5 * a - i * 0.2))
            pts.append((1500 + math.cos(a) * rr * 1.35, 500 + math.sin(a) * rr * 0.72))
        col = ramp([CO, LAV, "#2B303B"], i / 33)
        rings.append(f'<path d="{to_d(pts)}Z" fill="none" stroke="{col}" stroke-width="{f(2.2 - i*0.04)}" opacity="{f(1 - i/40)}"/>')
    c = lambda u: (-100 + 2600 * u, 560 + 230 * math.sin(2 * math.pi * u * 0.9 + 2.4) - 70 * math.sin(2 * math.pi * u * 2.6))
    rib = ribbon(c, lambda u: 54 + 20 * math.sin(math.pi * u), lambda u: 4 * math.pi * u + 0.7, 280, SILVER_F, SILVER_B, 1.1)
    return (f'<svg viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid slice" aria-hidden="true" focusable="false">'
            f'<rect width="{W}" height="{H}" fill="{OBS}"/><g class="pl" data-depth="0.5">{"".join(rings)}</g>'
            f'<g class="pl" data-depth="1.6">{rib}</g></svg>')


def insight_art(i):
    W, H = 360, 260
    body = []
    if i == 0:
        body.append(f'<rect width="{W}" height="{H}" fill="{OBS}"/>')
        body.append(ribbon(lambda u: (10 + 340 * u, 130 + 60 * math.sin(2 * math.pi * u * 1.2)), lambda u: 34 * math.sin(math.pi * u) + 3,
                           lambda u: 3 * math.pi * u, 120, COBALT_F, COBALT_B))
    elif i == 1:
        body.append(f'<rect width="{W}" height="{H}" fill="{CO2}"/>')
        for k in range(14):
            x = 24 + k * 23
            hh = 40 + 150 * abs(math.sin(k * 0.55))
            body.append(f'<rect x="{x}" y="{f(220-hh)}" width="10" height="{f(hh)}" rx="2" fill="{POR}" opacity="{f(0.35+0.65*k/13)}"/>')
        body.append(f'<rect x="24" y="226" width="310" height="2" fill="{POR}"/>')
    else:
        body.append(f'<rect width="{W}" height="{H}" fill="{GRA}"/>')
        for k in range(9):
            pts = wave_pts(lambda u, k=k: 130 + (10 + k * 9) * math.sin(2 * math.pi * (1 + k * 0.3) * u + k), 60, 360)
            body.append(f'<path d="{to_d(pts)}" fill="none" stroke="{ramp([LAV, CO, LIME], k/8)}" stroke-width="1.6" opacity="{f(0.4+0.06*k)}"/>')
    return f'<svg viewBox="0 0 {W} {H}" aria-hidden="true" focusable="false">{"".join(body)}</svg>'


def marketing_paths():
    a = to_d(wave_pts(lambda u: 330 - 180 * (u ** 1.4) - 40 * math.sin(2 * math.pi * 3 * u), 90, 520))
    b = to_d(wave_pts(lambda u: 360 - 120 * u - 30 * math.sin(2 * math.pi * 2 * u + 1), 90, 520))
    c = to_d(wave_pts(lambda u: 380 - 60 * u - 22 * math.sin(2 * math.pi * 5 * u + 2), 90, 520))
    return a, b, c


if __name__ == "__main__":
    pm, ps = process_states()
    out = {
        "HERO_POSTER": hero_poster(),
        "FINALE_RIBBON": contact_ribbon(),
        "P1": work_p1(), "P2": work_p2(), "P3": work_p3(), "P4": work_p4(),
        "I0": insight_art(0), "I1": insight_art(1), "I2": insight_art(2),
        "PROC_D0": pm[3], "PROC_S0": ps[3],
        "PROC_JSON": json.dumps({"m": pm, "s": ps}),
    }
    for i in range(4):
        out[f"MINI{i}"] = mini(pm[i], ps[i])
    ma, mb, mc = marketing_paths()
    out.update({"MK_A": ma, "MK_B": mb, "MK_C": mc})
    json.dump(out, open(__file__.replace("art.py", "art.json"), "w"))
    print({k: len(v) for k, v in out.items()})
