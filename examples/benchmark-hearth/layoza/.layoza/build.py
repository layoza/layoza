# Dev-only generator: renders authored SVG art + curve data into template.html -> ../index.html
import json, math, random, pathlib

HERE = pathlib.Path(__file__).parent
random.seed(7)

# ---------- oven day curve (illustrative hearth temperatures) ----------
PTS = [(4.0, 30), (4.5, 40), (5.0, 170), (5.5, 330), (5.85, 345), (6.0, 305), (7, 272), (8, 245), (9, 222),
       (10, 205), (11, 192), (12, 183), (13, 176), (14, 170), (15, 166), (15.5, 190), (16, 300), (16.5, 385),
       (17, 425), (17.5, 438), (18.5, 445), (19.5, 440), (20.5, 432), (21.5, 420), (22, 375), (22.5, 300),
       (23, 255), (24, 215)]
H0, H1, TMAX = 4.0, 24.0, 480.0

def catmull(points, steps=12):
    out = []
    p = [points[0]] + points + [points[-1]]
    for i in range(1, len(p) - 2):
        p0, p1, p2, p3 = p[i - 1], p[i], p[i + 1], p[i + 2]
        for s in range(steps):
            t = s / steps
            t2, t3 = t * t, t * t * t
            x = 0.5 * ((2 * p1[0]) + (-p0[0] + p2[0]) * t + (2 * p0[0] - 5 * p1[0] + 4 * p2[0] - p3[0]) * t2 + (-p0[0] + 3 * p1[0] - 3 * p2[0] + p3[0]) * t3)
            y = 0.5 * ((2 * p1[1]) + (-p0[1] + p2[1]) * t + (2 * p0[1] - 5 * p1[1] + 4 * p2[1] - p3[1]) * t2 + (-p0[1] + 3 * p1[1] - 3 * p2[1] + p3[1]) * t3)
            out.append((x, y))
    out.append(points[-1])
    return out

DENSE = catmull(PTS)

def curve_path(w, h, pad_top, pad_bot):
    def X(hr): return (hr - H0) / (H1 - H0) * w
    def Y(t): return pad_top + (1 - t / TMAX) * (h - pad_top - pad_bot)
    d = "M" + " L".join(f"{X(x):.1f} {Y(y):.1f}" for x, y in DENSE)
    area = d + f" L{w} {h} L0 {h} Z"
    return d, area

# samples every 5 min for JS lookup
samples = []
for i in range(int((H1 - H0) * 12) + 1):
    hr = H0 + i / 12
    # find bracket in DENSE
    for j in range(len(DENSE) - 1):
        a, b = DENSE[j], DENSE[j + 1]
        if a[0] <= hr <= b[0]:
            f = 0 if b[0] == a[0] else (hr - a[0]) / (b[0] - a[0])
            samples.append(round(a[1] + (b[1] - a[1]) * f))
            break
    else:
        samples.append(DENSE[-1][1])

# ---------- the oven (front elevation) ----------
def oven_svg():
    W, Hh = 640, 600
    s = []
    s.append(f'<svg class="oven" viewBox="0 0 {W} {Hh}" role="img" aria-labelledby="ovenT" preserveAspectRatio="xMidYMax meet">')
    s.append('<title id="ovenT">The wood-fired oven: a brick dome with an arched mouth, fire burning at the back and today\'s bake on the hearth</title>')
    s.append('''<defs>
<clipPath id="facade"><path d="M40 600 L40 214 C40 128 160 92 320 90 C480 92 600 128 600 214 L600 600 Z"/></clipPath>
<clipPath id="mouth"><path d="M178 452 L178 332 A142 142 0 0 1 462 332 L462 452 Z"/></clipPath>
<radialGradient id="glow" cx="0.34" cy="0.78" r="0.85"><stop offset="0" stop-color="#F6B64B"/><stop offset="0.28" stop-color="#E06A2B"/><stop offset="0.62" stop-color="#6E2316"/><stop offset="1" stop-color="#171B19" stop-opacity="0"/></radialGradient>
<linearGradient id="hearthG" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#2A1A14"/><stop offset="1" stop-color="#5A2A18"/></linearGradient>
</defs>''')
    # flue and hood
    s.append('<g class="flue"><path d="M296 0 H344 V64 H296 Z" fill="#2B312E"/><path d="M296 0 V64" stroke="#4A524C" stroke-width="3"/>'
             '<path d="M248 92 L392 92 L364 58 L276 58 Z" fill="#171B19"/><path d="M276 58 L364 58" stroke="#4A524C" stroke-width="2"/>'
             '<circle cx="290" cy="80" r="2.2" fill="#4A524C"/><circle cx="320" cy="80" r="2.2" fill="#4A524C"/><circle cx="350" cy="80" r="2.2" fill="#4A524C"/></g>')
    # bricks
    s.append('<g clip-path="url(#facade)"><rect x="40" y="80" width="560" height="520" fill="#E3D6C9"/>')
    shades = ["#7A2A1A", "#6E2316", "#843322", "#69251A", "#8C3A26", "#5F1F14"]
    bh, bw = 19, 46
    y = 80
    row = 0
    bricks = {}
    while y < 600:
        off = -bw / 2 if row % 2 else 0
        x = 40 + off
        while x < 600:
            c = random.choice(shades)
            bricks.setdefault(c, []).append(f"M{x+1.5:.1f} {y+1.5:.1f}h{bw-3}v{bh-3}h-{bw-3}z")
            x += bw
        y += bh
        row += 1
    for c, ds in bricks.items():
        s.append(f'<path fill="{c}" d="{"".join(ds)}"/>')
    s.append('</g>')
    # facade outline + soot stain above mouth
    s.append('<path d="M40 600 L40 214 C40 128 160 92 320 90 C480 92 600 128 600 214 L600 600" fill="none" stroke="#171B19" stroke-width="4"/>')
    s.append('<path d="M214 196 C250 150 390 150 426 196 C400 120 240 120 214 196 Z" fill="#171B19" opacity="0.32"/>')
    # voussoirs
    cx, cy, r1, r2 = 320, 332, 142, 184
    n = 17
    for i in range(n):
        a0 = math.pi - i * math.pi / n
        a1 = math.pi - (i + 1) * math.pi / n
        g = 0.006
        pts = [(cx + r1 * math.cos(a0 - g), cy - r1 * math.sin(a0 - g)), (cx + r2 * math.cos(a0 - g), cy - r2 * math.sin(a0 - g)),
               (cx + r2 * math.cos(a1 + g), cy - r2 * math.sin(a1 + g)), (cx + r1 * math.cos(a1 + g), cy - r1 * math.sin(a1 + g))]
        c = "#4E1A10" if i == n // 2 else random.choice(["#7A2A1A", "#843322", "#6E2316", "#8C3A26"])
        d = f"M{pts[0][0]:.1f} {pts[0][1]:.1f} L{pts[1][0]:.1f} {pts[1][1]:.1f} A{r2} {r2} 0 0 1 {pts[2][0]:.1f} {pts[2][1]:.1f} L{pts[3][0]:.1f} {pts[3][1]:.1f} A{r1} {r1} 0 0 0 {pts[0][0]:.1f} {pts[0][1]:.1f} Z"
        s.append(f'<path d="{d}" fill="{c}" stroke="#E3D6C9" stroke-width="2"/>')
    # jamb soldiers
    for side in (0, 1):
        x0 = 136 if side == 0 else 462
        for k in range(6):
            yy = 332 + k * 20
            s.append(f'<rect x="{x0+1}" y="{yy+1}" width="40" height="18" fill="{random.choice(shades)}" stroke="#E3D6C9" stroke-width="2"/>')
    # interior
    s.append('<g clip-path="url(#mouth)"><rect x="170" y="180" width="300" height="280" fill="#171B19"/>')
    s.append('<rect class="glow" x="170" y="180" width="300" height="280" fill="url(#glow)"/>')
    s.append('<path d="M206 400 A114 98 0 0 1 434 400" fill="none" stroke="#3A221A" stroke-width="3"/>')
    # hearth floor (perspective)
    s.append('<path d="M178 452 L462 452 L432 398 L208 398 Z" fill="url(#hearthG)"/>')
    for k in range(1, 6):
        xa = 178 + k * (284 / 6); xb = 208 + k * (224 / 6)
        s.append(f'<path d="M{xa:.1f} 452 L{xb:.1f} 398" stroke="#171B19" stroke-width="1.2" opacity="0.6"/>')
    s.append('<path d="M193 425 L447 425" stroke="#171B19" stroke-width="1.2" opacity="0.5"/>')
    # fire: logs + flames (back left)
    s.append('<g class="fire">')
    s.append('<path d="M196 412 L268 394 L272 404 L200 422 Z" fill="#3B2216"/><path d="M268 394 a5 5 0 0 1 4 10" fill="#C9874A"/>')
    s.append('<path d="M206 392 L262 414 L258 423 L202 401 Z" fill="#4A2B1B"/><path d="M202 401 a5 5 0 0 1 4 -9" fill="#C9874A"/>')
    s.append('<g class="flames">')
    s.append('<path class="fl fl1" d="M200 404 C188 372 214 352 210 316 C232 344 240 330 236 304 C262 336 272 370 262 404 Z" fill="#E06A2B"/>')
    s.append('<path class="fl fl2" d="M210 404 C204 380 222 366 222 342 C236 360 244 352 244 336 C258 362 260 384 252 404 Z" fill="#F08A3C"/>')
    s.append('<path class="fl fl3" d="M220 404 C218 388 228 378 230 364 C238 376 246 380 244 404 Z" fill="#FBE3A0"/>')
    s.append('</g>')
    s.append('<g class="refire-logs"><path d="M214 396 L280 384 L282 392 L216 404 Z" fill="#5A3420"/><path d="M280 384 a4 4 0 0 1 2 8" fill="#D9A06A"/></g>')
    s.append('<g class="embers-bed"><ellipse cx="232" cy="410" rx="44" ry="7" fill="#E06A2B" opacity="0.85"/><ellipse cx="226" cy="409" rx="22" ry="3.5" fill="#F6B64B"/></g>')
    s.append('</g>')
    # loads
    # bread: boule, rye tin, boule
    s.append('<g class="load" data-load="bread">'
             '<path d="M300 432 C300 404 360 404 360 432 Z" fill="#8A5528"/><path d="M300 432 C300 404 360 404 360 432" fill="none" stroke="#5A3418" stroke-width="2"/>'
             '<path d="M314 418 C324 410 338 410 348 418" stroke="#E9D2B0" stroke-width="2.5" fill="none" stroke-linecap="round"/>'
             '<path d="M368 434 L374 412 L422 412 L428 434 Z" fill="#2B312E"/><path d="M372 414 C380 398 416 398 424 414 Z" fill="#5A3418"/>'
             '<path d="M384 408 L392 404 M402 403 L410 407" stroke="#A9733F" stroke-width="2" stroke-linecap="round"/>'
             '<path d="M272 440 C272 420 300 418 302 440 Z" fill="#9A6230"/><path d="M280 432 L294 426" stroke="#E9D2B0" stroke-width="2" stroke-linecap="round"/>'
             '</g>')
    # pastry tray
    knots = "".join(f'<path d="M{x} 418 c-7 0 -9 -9 -1 -11 c7 -2 11 5 6 9 c-4 3 -9 0 -7 -4" fill="none" stroke="#C07A3A" stroke-width="4" stroke-linecap="round"/>' for x in (318, 344, 370, 396))
    s.append(f'<g class="load" data-load="pastry"><path d="M296 430 L428 430 L420 422 L304 422 Z" fill="#2B312E"/>{knots}</g>')
    # pots
    s.append('<g class="load" data-load="pots"><path d="M316 438 C316 404 392 404 392 438 Z" fill="#2B312E"/><path d="M310 410 H398 V416 H310 Z" fill="#171B19"/>'
             '<path d="M346 404 h16 v-6 h-16 Z" fill="#171B19"/><path d="M308 420 h-8 M400 420 h8" stroke="#171B19" stroke-width="5" stroke-linecap="round"/>'
             '<path class="steam" d="M338 392 c-6 -10 6 -14 0 -26 M362 392 c-6 -10 6 -14 0 -26" stroke="#B9C7BC" stroke-width="2" fill="none" stroke-linecap="round" opacity="0.7"/></g>')
    # service: fish on an iron dish + flatbread on peel entering
    s.append('<g class="load" data-load="service"><path d="M300 436 C300 422 420 422 420 436 C420 446 300 446 300 436 Z" fill="#2B312E"/>'
             '<path d="M318 432 C340 414 384 414 398 428 L414 418 L410 434 L414 446 L398 438 C384 450 340 450 318 432 Z" fill="#C9A27A"/>'
             '<path d="M330 431 C346 424 370 424 386 430" stroke="#8A5528" stroke-width="1.6" fill="none"/><circle cx="328" cy="430" r="2.2" fill="#171B19"/>'
             '<path d="M338 426 C341 431 341 435 338 440" stroke="#8A5528" stroke-width="1.6" fill="none"/>'
             '<path d="M354 418 l4 -3 M364 417 l4 -3 M374 418 l4 -3" stroke="#6E2316" stroke-width="2" stroke-linecap="round"/></g>')
    # banked: lidded pot of grains
    s.append('<g class="load" data-load="banked"><path d="M336 436 C336 412 384 412 384 436 Z" fill="#2B312E"/><path d="M332 416 H388 V421 H332 Z" fill="#171B19"/><path d="M354 410 h12 v-5 h-12 Z" fill="#171B19"/></g>')
    s.append('<canvas-slot/>')
    s.append('</g>')
    # sill
    s.append('<path d="M150 452 H490 V462 H150 Z" fill="#CFC7BB"/><path d="M144 462 H496 V476 H144 Z" fill="#A79E92"/><path d="M144 476 H496" stroke="#171B19" stroke-width="2"/>')
    # log store in the base
    s.append('<path d="M86 600 V552 A40 40 0 0 1 166 552 V600 Z" fill="#171B19"/><path d="M474 600 V552 A40 40 0 0 1 554 552 V600 Z" fill="#171B19"/>')
    for (bx, by) in [(104, 586), (126, 588), (148, 586), (114, 566), (138, 566), (126, 548), (492, 586), (514, 588), (536, 586), (502, 566), (526, 566)]:
        r = 10 + random.random() * 1.5
        s.append(f'<circle cx="{bx}" cy="{by}" r="{r:.1f}" fill="#C9874A"/><circle cx="{bx}" cy="{by}" r="{r*0.62:.1f}" fill="none" stroke="#8A5528" stroke-width="1.2"/><circle cx="{bx}" cy="{by}" r="{r*0.28:.1f}" fill="none" stroke="#8A5528" stroke-width="1"/>')
    # pyrometer gauge
    s.append('<g class="gauge" transform="translate(522 196)"><circle r="36" fill="#171B19"/><circle r="30" fill="#E9ECE3"/>')
    for k in range(11):
        a = math.radians(-210 + k * 24)
        r_in = 22 if k % 5 else 18
        s.append(f'<path d="M{r_in*math.cos(a):.1f} {r_in*math.sin(a):.1f} L{27*math.cos(a):.1f} {27*math.sin(a):.1f}" stroke="#171B19" stroke-width="{2 if k%5==0 else 1.2}"/>')
    s.append('<path d="M18 -12 A22 22 0 0 1 22 6" stroke="#C42B17" stroke-width="3" fill="none"/>')
    s.append('<g class="needle"><path d="M0 3 L25 0 L0 -3 Z" fill="#C42B17"/></g><circle r="4" fill="#171B19"/></g>')
    # peel leaning at the right
    s.append('<g class="peel"><path d="M606 600 L586 300" stroke="#9A6230" stroke-width="9" stroke-linecap="round"/><path d="M606 600 L586 300" stroke="#C9874A" stroke-width="3" stroke-linecap="round"/>'
             '<path d="M560 298 C556 246 562 206 582 204 C604 202 612 244 610 294 Z" fill="#B88A55" stroke="#5A3418" stroke-width="2.5"/>'
             '<path d="M572 290 C570 250 574 222 584 220" stroke="#D9B585" stroke-width="2" fill="none"/></g>')
    s.append('</svg>')
    return "".join(s)

# ---------- breads ----------
BREADS = {
 "rye": '<path d="M14 66 L22 30 L98 30 L106 66 Z" fill="#2B312E"/><path d="M20 34 C30 6 90 6 100 34 Z" fill="#5A3418"/><path d="M36 22 L48 17 M58 14 L70 16 M80 19 L90 25" stroke="#A9733F" stroke-width="3" stroke-linecap="round"/><g fill="#C9A27A"><circle cx="44" cy="27" r="1.4"/><circle cx="62" cy="22" r="1.4"/><circle cx="76" cy="27" r="1.4"/><circle cx="54" cy="30" r="1.2"/></g>',
 "boule": '<path d="M12 64 C10 22 110 22 108 64 Z" fill="#8A5528"/><path d="M12 64 C10 22 110 22 108 64" fill="none" stroke="#5A3418" stroke-width="2.5"/><path d="M30 44 C46 30 74 30 90 44" stroke="#E9D2B0" stroke-width="4" fill="none" stroke-linecap="round"/><path d="M44 54 C54 48 66 48 76 54" stroke="#C9A27A" stroke-width="2" fill="none" stroke-linecap="round"/>',
 "oat": '<path d="M8 58 C10 30 110 30 112 58 C110 70 10 70 8 58 Z" fill="#6A3E1C"/><g fill="#E9D2B0"><ellipse cx="30" cy="44" rx="4" ry="1.8" transform="rotate(-20 30 44)"/><ellipse cx="48" cy="40" rx="4" ry="1.8" transform="rotate(15 48 40)"/><ellipse cx="66" cy="39" rx="4" ry="1.8" transform="rotate(-10 66 39)"/><ellipse cx="84" cy="42" rx="4" ry="1.8" transform="rotate(25 84 42)"/><ellipse cx="58" cy="48" rx="4" ry="1.8" transform="rotate(-30 58 48)"/><ellipse cx="40" cy="50" rx="4" ry="1.8"/><ellipse cx="96" cy="50" rx="4" ry="1.8" transform="rotate(10 96 50)"/></g>',
 "knot": '<path d="M60 64 C26 66 22 30 46 26 C70 22 84 46 66 54 C52 60 42 46 52 40 C60 35 68 42 62 46" fill="none" stroke="#C07A3A" stroke-width="13" stroke-linecap="round"/><path d="M60 64 C26 66 22 30 46 26 C70 22 84 46 66 54" fill="none" stroke="#8A5528" stroke-width="2" stroke-linecap="round"/><g fill="#3B2216"><circle cx="40" cy="40" r="1.3"/><circle cx="56" cy="30" r="1.3"/><circle cx="72" cy="44" r="1.3"/></g>',
 "eccles": '<ellipse cx="60" cy="50" rx="46" ry="16" fill="#A9733F"/><ellipse cx="60" cy="46" rx="46" ry="16" fill="#C9874A"/><path d="M42 46 l8 -6 M56 48 l8 -6 M70 50 l8 -6" stroke="#3B2216" stroke-width="3" stroke-linecap="round"/><g fill="#F3EBDD"><circle cx="34" cy="44" r="1.3"/><circle cx="50" cy="52" r="1.3"/><circle cx="66" cy="40" r="1.3"/><circle cx="84" cy="48" r="1.3"/><circle cx="76" cy="54" r="1.2"/></g>',
 "focaccia": '<path d="M8 40 L112 34 L114 62 L10 66 Z" fill="#D9A659"/><path d="M8 40 L112 34 L114 40 L8 46 Z" fill="#B88440"/><g fill="#9A6230"><circle cx="26" cy="54" r="2.5"/><circle cx="44" cy="50" r="2.5"/><circle cx="62" cy="55" r="2.5"/><circle cx="80" cy="49" r="2.5"/><circle cx="98" cy="54" r="2.5"/><circle cx="36" cy="60" r="2.2"/><circle cx="90" cy="60" r="2.2"/></g><path d="M50 58 l6 -8 M54 56 l4 2 M86 54 l6 -8 M90 52 l4 2" stroke="#3F5A2E" stroke-width="2" stroke-linecap="round"/>',
}

def bread_svg(key, label):
    return f'<svg viewBox="0 0 120 80" width="120" height="80" aria-hidden="true" focusable="false">{BREADS[key]}</svg>'

# ---------- chef's table plan ----------
def plan_svg():
    s = ['<svg class="plan" viewBox="0 0 640 420" role="img" aria-labelledby="planT"><title id="planT">Plan of the chef&#8217;s table: the oven dome at the top, the kitchen, then one long counter with ten place settings and ten stools facing the oven mouth</title>']
    # oven dome in plan: brick ring with mouth opening at the bottom
    s.append('<path d="M190 0 A130 130 0 0 0 450 0" fill="#6E2316"/>')
    s.append('<path d="M220 0 A100 100 0 0 0 420 0" fill="#2B1A14"/>')
    # brick joints on the ring
    for k in range(1, 12):
        a = math.pi * k / 12
        x1, y1 = 320 - 100 * math.cos(a), 100 * math.sin(a)
        x2, y2 = 320 - 130 * math.cos(a), 130 * math.sin(a)
        s.append(f'<path d="M{x1:.1f} {y1:.1f} L{x2:.1f} {y2:.1f}" stroke="#171B19" stroke-width="1.5"/>')
    s.append('<path d="M296 96 H344 V134 H296 Z" fill="#2B1A14"/><path d="M286 134 H354 V146 H286 Z" fill="#A79E92"/>')
    s.append('<path d="M250 40 C262 22 286 26 284 46 C280 62 256 62 250 40 Z" fill="#F08A3C"/><path d="M258 42 C264 32 276 34 275 45 C272 53 262 53 258 42 Z" fill="#FBE3A0"/>')
    s.append('<path d="M190 0 A130 130 0 0 0 450 0" fill="none" stroke="#E9ECE3" stroke-width="2"/>')
    # kitchen: prep bench and peel rack
    s.append('<rect x="70" y="40" width="96" height="120" fill="none" stroke="#E9ECE3" stroke-width="1.5"/><path d="M70 70 H166 M70 100 H166 M70 130 H166" stroke="#E9ECE3" stroke-width="1" opacity=".5"/>')
    s.append('<rect x="474" y="40" width="96" height="120" fill="none" stroke="#E9ECE3" stroke-width="1.5"/><circle cx="500" cy="70" r="12" fill="none" stroke="#E9ECE3" stroke-width="1.2"/><circle cx="544" cy="70" r="12" fill="none" stroke="#E9ECE3" stroke-width="1.2"/><circle cx="500" cy="118" r="12" fill="none" stroke="#E9ECE3" stroke-width="1.2"/><circle cx="544" cy="118" r="12" fill="none" stroke="#E9ECE3" stroke-width="1.2"/>')
    s.append('<path d="M196 200 L268 156" stroke="#C9874A" stroke-width="4" stroke-linecap="round"/><path d="M262 146 C276 138 290 150 280 162 L272 168 C262 170 254 158 262 146 Z" fill="#B88A55"/>')
    # the pass
    s.append('<rect x="50" y="236" width="540" height="56" fill="#222825" stroke="#E9ECE3" stroke-width="2.5"/>')
    s.append('<path d="M50 252 H590" stroke="#E9ECE3" stroke-width="1" opacity=".35"/>')
    seats = []
    for i in range(10):
        x = 50 + 27 + i * (486 / 9)
        seats.append(x)
        s.append(f'<circle cx="{x:.1f}" cy="272" r="12" fill="#E9ECE3"/><circle cx="{x:.1f}" cy="272" r="7.5" fill="none" stroke="#B9C7BC" stroke-width="1"/>')
        s.append(f'<path d="M{x-18:.1f} 264 v16 M{x+18:.1f} 264 v16" stroke="#B9C7BC" stroke-width="1.6" stroke-linecap="round"/>')
    for x in seats:
        s.append(f'<path d="M{x:.1f} 312 L320 146" stroke="#F08A3C" stroke-width="1" opacity=".28" stroke-dasharray="2 5"/>')
    for x in seats:
        s.append(f'<circle cx="{x:.1f}" cy="326" r="16" fill="#171B19" stroke="#F08A3C" stroke-width="2.5"/><circle cx="{x:.1f}" cy="326" r="5" fill="#F08A3C"/>')
    s.append('<path d="M50 372 H590" stroke="#E9ECE3" stroke-width="1" stroke-dasharray="6 6" opacity=".5"/><path d="M50 366 v12 M590 366 v12" stroke="#E9ECE3" stroke-width="1.2"/>')
    s.append('</svg>')
    return "".join(s)

strip_d, strip_area = curve_path(1200, 120, 8, 4)
story_d, story_area = curve_path(1200, 460, 20, 0)
def range_path(h0, h1, w, h, tlo, thi):
    pts = [(x, y) for x, y in DENSE if h0 <= x <= h1]
    def X(hr): return (hr - h0) / (h1 - h0) * w
    def Y(t): return (1 - (t - tlo) / (thi - tlo)) * h
    return "M" + " L".join(f"{X(x):.1f} {Y(y):.1f}" for x, y in pts)
dinner_d = range_path(17.0, 22.5, 600, 160, 280, 460)

tpl = (HERE / "template.html").read_text(encoding="utf-8")
oven = oven_svg().replace("<canvas-slot/>", "")
out = (tpl.replace("{{OVEN}}", oven)
          .replace("{{STRIP_D}}", strip_d).replace("{{STRIP_AREA}}", strip_area)
          .replace("{{STORY_D}}", story_d).replace("{{STORY_AREA}}", story_area)
          .replace("{{CURVE_JSON}}", json.dumps(samples, separators=(",", ":")))
          .replace("{{PLAN}}", plan_svg()).replace("{{DINNER_D}}", dinner_d))
for k in BREADS:
    out = out.replace("{{BREAD_" + k.upper() + "}}", bread_svg(k, k))
(HERE.parent / "index.html").write_text(out, encoding="utf-8")
print("ok", len(out))
