"""Dev-only: generates the crowd silhouettes and writes ../index.html from src.html."""
import random, pathlib

HERE = pathlib.Path(__file__).parent
rnd = random.Random(14082027)


def person(x, base, s, arm):
    w = 15 * s
    sh = base - 26 * s           # shoulder line
    r = 8.5 * s                  # head radius
    hy = sh - 6 * s - r
    d = (f"M{x-w:.1f} {base:.1f} C{x-w:.1f} {sh+4*s:.1f} {x-w*0.7:.1f} {sh-4*s:.1f} {x:.1f} {sh-5*s:.1f} "
         f"C{x+w*0.7:.1f} {sh-4*s:.1f} {x+w:.1f} {sh+4*s:.1f} {x+w:.1f} {base:.1f}Z ")
    d += f"M{x-r:.1f} {hy:.1f}a{r:.1f} {r:.1f} 0 1 0 {2*r:.1f} 0a{r:.1f} {r:.1f} 0 1 0 {-2*r:.1f} 0Z "
    if arm:
        side = rnd.choice([-1, 1])
        sx = x + side * w * 0.7
        tx = sx + side * rnd.uniform(4, 16) * s
        ty = sh - rnd.uniform(46, 62) * s
        aw = 3.6 * s
        d += (f"M{sx-aw:.1f} {sh+2*s:.1f} L{tx-aw*0.8:.1f} {ty:.1f} "
              f"Q{tx:.1f} {ty-5*s:.1f} {tx+aw*0.8:.1f} {ty:.1f} L{sx+aw:.1f} {sh+2*s:.1f}Z ")
    return d


def row(y, s, gap, arm_p, x0=-20, x1=1460):
    d = ""
    x = x0
    while x < x1:
        d += person(x + rnd.uniform(-4, 4), y + rnd.uniform(-3, 3) * s, s * rnd.uniform(0.92, 1.08), rnd.random() < arm_p)
        x += gap * rnd.uniform(0.85, 1.15)
    return d


back = row(494, 0.62, 19, 0.30, 430, 1010)
mid = row(542, 0.85, 26, 0.32)
front = row(608, 1.25, 38, 0.28)

src = (HERE / "src.html").read_text(encoding="utf-8")
out = (src.replace("{{CROWD_BACK}}", back.strip())
          .replace("{{CROWD_MID}}", mid.strip())
          .replace("{{CROWD_FRONT}}", front.strip()))
(HERE.parent / "index.html").write_text(out, encoding="utf-8")
print("wrote", len(out), "bytes")
