# Dev-only: builds index.html from template.html by injecting generated SVG (bike poster + dimension drawing).
import math, pathlib
here = pathlib.Path(__file__).parent
S = 500
def P(x, y): return ((x + 0.9) * S, (1.02 - y) * S)
def f(v): return f"{v:.1f}"
def pt(x, y):
    X, Y = P(x, y); return f"{f(X)} {f(Y)}"

RA=(-0.52,0.35); FA=(0.53,0.35); BB=(-0.08,0.28)
ST_TOP=(-0.225,0.76); TT_R=(-0.208,0.705); HT_T=(0.352,0.79); HT_B=(0.40,0.64); TT_F=(0.362,0.755)
SADDLE=(-0.274,0.925)

def spokes(cx, cy, hub_r, n=28):
    out=[]
    for i in range(n):
        a = 2*math.pi*i/n
        side = 1 if i%2==0 else -1
        ha = a + side*0.55
        x1 = cx + hub_r*math.cos(ha); y1 = cy + hub_r*math.sin(ha)
        x2 = cx + 0.300*math.cos(a); y2 = cy + 0.300*math.sin(a)
        out.append(f'M{pt(x1,y1)}L{pt(x2,y2)}')
    return "".join(out)

def wheel(c, hub_r, motor):
    cx, cy = c; X, Y = P(cx, cy)
    s = []
    s.append(f'<path class="spk" d="{spokes(cx,cy,hub_r)}"/>')
    s.append(f'<circle class="rim" cx="{f(X)}" cy="{f(Y)}" r="{0.31*S:.1f}"/>')
    s.append(f'<circle class="tyre" cx="{f(X)}" cy="{f(Y)}" r="{0.335*S:.1f}"/>')
    s.append(f'<circle class="tread" cx="{f(X)}" cy="{f(Y)}" r="{0.352*S:.1f}"/>')
    s.append(f'<circle class="hub" cx="{f(X)}" cy="{f(Y)}" r="{(0.07 if motor else 0.03)*S:.1f}"/>')
    s.append(f'<circle class="axle" cx="{f(X)}" cy="{f(Y)}" r="{0.012*S:.1f}"/>')
    return "".join(s)

def belt_path():
    # outer tangent belt between BB pulley r=.1 and rear sprocket r=.055
    r1, r2 = 0.1, 0.055
    (x1,y1),(x2,y2)=BB,RA
    dx,dy=x2-x1,y2-y1; d=math.hypot(dx,dy); a=math.atan2(dy,dx); b=math.acos((r1-r2)/d)
    t1u=(x1+r1*math.cos(a+b), y1+r1*math.sin(a+b)); t2u=(x2+r2*math.cos(a+b), y2+r2*math.sin(a+b))
    t1l=(x1+r1*math.cos(a-b), y1+r1*math.sin(a-b)); t2l=(x2+r2*math.cos(a-b), y2+r2*math.sin(a-b))
    return (f'M{pt(*t1u)}L{pt(*t2u)}A{r2*S:.1f} {r2*S:.1f} 0 1 0 {pt(*t2l)}L{pt(*t1l)}A{r1*S:.1f} {r1*S:.1f} 0 1 0 {pt(*t1u)}Z')

def bike_svg(cls, title):
    rx, ry = P(*RA); fx, fy = P(*FA); bx, by = P(*BB)
    g=[]
    g.append(f'<svg class="{cls}" viewBox="0 0 900 530" role="img" aria-label="{title}" xmlns="http://www.w3.org/2000/svg">')
    g.append('<ellipse class="shadow" cx="452" cy="512" rx="380" ry="12"/>')
    g.append(wheel(RA,0.07,True)); g.append(wheel(FA,0.03,False))
    # mudguards (arcs)
    def arc(c, r, a0, a1):
        cx,cy=c; x0,y0=cx+r*math.cos(a0),cy+r*math.sin(a0); x1,y1=cx+r*math.cos(a1),cy+r*math.sin(a1)
        large = 1 if abs(a1-a0)>math.pi else 0
        return f'M{pt(x0,y0)}A{r*S:.1f} {r*S:.1f} 0 {large} 0 {pt(x1,y1)}'
    g.append(f'<path class="guard" d="{arc(RA,0.375,math.radians(170),math.radians(20))}"/>'.replace(' 0 0 0 ',' 0 0 1 '))
    g.append(f'<path class="guard" d="{arc(FA,0.375,math.radians(160),math.radians(-5))}"/>'.replace(' 0 0 0 ',' 0 0 1 '))
    # belt + pulleys
    g.append(f'<path class="belt" d="{belt_path()}"/>')
    g.append(f'<circle class="pulley" cx="{f(bx)}" cy="{f(by)}" r="{0.1*S:.1f}"/>')
    g.append(f'<circle class="pulley" cx="{f(rx)}" cy="{f(ry)}" r="{0.055*S:.1f}"/>')
    # stays (behind)
    g.append(f'<path class="stay" d="M{pt(*BB)}L{pt(*RA)}M{pt(-0.215,0.69)}L{pt(*RA)}"/>')
    # fork
    g.append(f'<path class="fork" d="M{pt(*HT_B)}Q{pt(0.47,0.47)} {pt(*FA)}"/>')
    # seat tube, top tube
    g.append(f'<path class="tube st" d="M{pt(*BB)}L{pt(*ST_TOP)}"/>')
    g.append(f'<path class="tube tt" d="M{pt(*TT_R)}L{pt(*TT_F)}"/>')
    # down tube (battery)
    g.append(f'<path class="tube dt" d="M{pt(*BB)}L{pt(*HT_B)}"/>')
    g.append(f'<path class="cells" d="M{pt(-0.02,0.325)}L{pt(0.34,0.595)}"/>')
    g.append(f'<path class="tube ht" d="M{pt(*HT_B)}L{pt(*HT_T)}"/>')
    # seatpost + saddle
    g.append(f'<path class="post" d="M{pt(*ST_TOP)}L{pt(*SADDLE)}"/>')
    sx, sy = P(*SADDLE)
    g.append(f'<path class="saddle" d="M{f(sx-62)} {f(sy-6)}C{f(sx-40)} {f(sy-18)} {f(sx+10)} {f(sy-14)} {f(sx+58)} {f(sy-4)}L{f(sx+60)} {f(sy+4)}C{f(sx+20)} {f(sy+2)} {f(sx-30)} {f(sy+10)} {f(sx-60)} {f(sy+8)}Z"/>')
    # stem + bars
    g.append(f'<path class="bar" d="M{pt(*HT_T)}L{pt(0.345,0.84)}L{pt(0.405,0.865)}C{pt(0.39,0.885)} {pt(0.32,0.89)} {pt(0.27,0.885)}"/>')
    gx, gy = P(0.27,0.885)
    g.append(f'<path class="grip" d="M{f(gx)} {f(gy)}l-26 2"/>')
    # cranks + pedal
    px, py = bx+0.17*S*math.cos(math.radians(60)), by+0.17*S*math.sin(math.radians(60))
    g.append(f'<path class="crank" d="M{f(bx)} {f(by)}L{f(px)} {f(py)}"/><rect class="pedal" x="{f(px-22)}" y="{f(py-6)}" width="44" height="12" rx="3"/>')
    g.append(f'<circle class="bbcap" cx="{f(bx)}" cy="{f(by)}" r="13"/>')
    # lights
    lx, ly = P(0.43,0.80)
    g.append(f'<rect class="lamp" x="{f(lx-6)}" y="{f(ly-12)}" width="26" height="22" rx="6"/><rect class="lens" x="{f(lx+14)}" y="{f(ly-8)}" width="6" height="14" rx="2"/>')
    tx, ty = P(-0.86,0.47)
    g.append(f'<rect class="tail" x="{f(tx-4)}" y="{f(ty-8)}" width="14" height="16" rx="3"/>')
    # kickstand
    g.append(f'<path class="stand" d="M{pt(-0.2,0.30)}L{pt(-0.32,0.0)}"/>')
    g.append('</svg>')
    return "".join(g)

def drawing_svg():
    base = bike_svg("spec-drawing__bike", "Side elevation of the Volta One with key dimensions")
    rx, ry = P(*RA); fx, fy = P(*FA); X0,_=P(-0.875,0); X1,_=P(0.885,0)
    dims = []
    yb = 524
    dims.append(f'<path class="dim" d="M{f(rx)} {f(ry+16)}V{yb+8}M{f(fx)} {f(fy+16)}V{yb+8}M{f(rx)} {yb}H{f(fx)}"/>')
    dims.append(f'<path class="dim-tick" d="M{f(rx-6)} {yb+6}l12 -12M{f(fx-6)} {yb+6}l12 -12"/>')
    dims.append(f'<text class="dim-t" x="{f((rx+fx)/2)}" y="{yb-10}" text-anchor="middle">Wheelbase 1,050 mm</text>')
    hx, hy = P(*HT_T)
    dims.append(f'<path class="dim" d="M{f(hx+30)} {f(hy)}H930M920 {f(hy)}V512M905 512H935"/>')
    dims.append(f'<text class="dim-t" x="944" y="{f((hy+512)/2-6)}" text-anchor="start">Stack</text>')
    dims.append(f'<text class="dim-t" x="944" y="{f((hy+512)/2+38)}" text-anchor="start">610 mm</text>')
    dims.append(f'<text class="dim-t" x="{f(rx)}" y="40" text-anchor="middle">700c × 35</text>')
    dims.append(f'<path class="dim" d="M{f(rx)} 50V{f(ry-0.35*S-6)}"/>')
    s = base.replace('viewBox="0 0 900 530"', 'viewBox="-20 0 1110 560"').replace('</svg>', "".join(dims) + '</svg>')
    return s

tpl = (here / "template.html").read_text(encoding="utf-8")
out = tpl.replace("<!--POSTER-->", bike_svg("poster-bike", "The Volta One electric bike, side view")).replace("<!--DRAWING-->", drawing_svg())
(here.parent / "index.html").write_text(out, encoding="utf-8")
print("built", len(out))

def roof():
    houses=[(0,200,52,'#F4B9C4'),(200,180,36,'#E3AA2E'),(380,220,58,'#ABDCC6'),(600,190,42,'#F6F2EA'),(790,210,28,'#B5603C'),(1000,200,48,'#56653A'),(1200,240,38,'#F4B9C4')]
    g=['<svg class="roofline" viewBox="0 0 1440 150" preserveAspectRatio="xMidYMax slice" aria-hidden="true">']
    for i,(x,w,top,c) in enumerate(houses):
        if i%2==0:
            cx=x+w-62
            g.append(f'<rect x="{cx}" y="{top-30}" width="40" height="32" fill="#1D4C52" stroke="#172126" stroke-width="3"/>')
            g.append(f'<rect x="{cx+5}" y="{top-42}" width="10" height="13" fill="#B5603C" stroke="#172126" stroke-width="2.5"/><rect x="{cx+24}" y="{top-40}" width="10" height="11" fill="#B5603C" stroke="#172126" stroke-width="2.5"/>')
        g.append(f'<rect x="{x}" y="{top}" width="{w}" height="{150-top}" fill="{c}" stroke="#172126" stroke-width="3"/>')
        g.append(f'<rect x="{x-2}" y="{top}" width="{w+4}" height="9" fill="#F6F2EA" stroke="#172126" stroke-width="3"/>')
        for wx in (x+30, x+w-30-46):
            g.append(f'<rect x="{wx}" y="{top+26}" width="46" height="{min(70,150-top-30)}" fill="#2A3A40" stroke="#F6F2EA" stroke-width="4"/>')
            g.append(f'<path d="M{wx+23} {top+26}v{min(70,150-top-30)}M{wx} {top+26+min(70,150-top-30)/2:.0f}h46" stroke="#F6F2EA" stroke-width="3"/>')
    g.append('<rect x="0" y="136" width="1440" height="14" fill="#1D4C52"/></svg>')
    return "".join(g)
out = out.replace("<!--ROOF-->", roof())
(here.parent / "index.html").write_text(out, encoding="utf-8")
print("built with roof", len(out))
