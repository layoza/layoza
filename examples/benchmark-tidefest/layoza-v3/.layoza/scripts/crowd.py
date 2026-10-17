import random
random.seed(7)
def crowd(y0, scale, xstep, seed, arms_every):
    random.seed(seed)
    d = [f"M-20 1000 L-20 {y0+40}"]
    arms = []
    x = -20
    i = 0
    while x < 1640:
        w = xstep*random.uniform(0.85,1.2)
        h = random.uniform(-14,14)*scale
        sy = y0 + h            # shoulder line
        hr = 15*scale*random.uniform(0.9,1.1)   # head radius
        cx = x + w/2
        # shoulder slope up, neck, head (two arcs), neck, shoulder down
        d.append(f"C{x+w*0.08:.0f} {sy+10*scale:.0f} {x+w*0.18:.0f} {sy:.0f} {cx-hr*0.7:.0f} {sy-2*scale:.0f}")
        d.append(f"L{cx-hr*0.55:.0f} {sy-hr*0.6:.0f}")
        d.append(f"A{hr:.0f} {hr*1.12:.0f} 0 1 1 {cx+hr*0.55:.0f} {sy-hr*0.6:.0f}")
        d.append(f"L{cx+hr*0.7:.0f} {sy-2*scale:.0f}")
        d.append(f"C{x+w*0.82:.0f} {sy:.0f} {x+w*0.92:.0f} {sy+10*scale:.0f} {x+w:.0f} {sy+16*scale:.0f}")
        if i % arms_every == random.randint(0,arms_every-1):
            side = random.choice([-1,1])
            sx = cx + side*w*0.32
            ex = sx + side*random.uniform(4,22)*scale
            ey = sy - random.uniform(70,95)*scale
            mx = sx + side*random.uniform(0,10)*scale
            my = (sy+ey)/2
            arms.append(f"M{sx:.0f} {sy+4:.0f} Q{mx:.0f} {my:.0f} {ex:.0f} {ey:.0f}")
        x += w; i += 1
    d.append("L1640 1000 Z")
    return " ".join(d), " ".join(arms)
b, ba = crowd(905, 0.8, 46, 3, 3)
f, fa = crowd(950, 1.25, 74, 11, 3)
stars = []
random.seed(5)
for _ in range(46):
    x=random.uniform(0,1600); y=random.uniform(20,430); r=random.choice([1.2,1.6,2,2.6])
    stars.append(f'M{x-r:.1f} {y:.0f}a{r} {r} 0 1 0 {2*r} 0a{r} {r} 0 1 0 {-2*r} 0')
open(__file__.replace('crowd.py','crowd.out.txt'),'w').write(f'BACK\n{b}\nBACKARMS\n{ba}\nFRONT\n{f}\nFRONTARMS\n{fa}\nSTARS\n<path d="{"".join(stars)}"/>\n')
