"""Storyboard capture: real scrolling frames at a given width (dev-only)."""
import sys, os, pathlib
from playwright.sync_api import sync_playwright
here = pathlib.Path(__file__).parent
url = (here.parent / "index.html").as_uri()
w = int(sys.argv[1]) if len(sys.argv) > 1 else 1440
h = int(sys.argv[2]) if len(sys.argv) > 2 else 900
step = int(sys.argv[3]) if len(sys.argv) > 3 else 760
only = sys.argv[4] if len(sys.argv) > 4 else None
reduce = os.environ.get("REDUCE") == "1"
out = here / "shots" / f"{w}{'-rm' if reduce else ''}"
out.mkdir(parents=True, exist_ok=True)
for f in out.glob("*.png"): f.unlink()
with sync_playwright() as p:
    b = p.chromium.launch()
    pg = b.new_page(viewport={"width": w, "height": h}, device_scale_factor=1, reduced_motion="reduce" if reduce else "no-preference", has_touch=w < 768, is_mobile=w < 768)
    errs = []
    pg.on("console", lambda m: m.type in ("error", "warning") and errs.append(f"{m.type}: {m.text}"))
    pg.on("pageerror", lambda e: errs.append(f"pageerror: {e}"))
    pg.goto(url + "?t", wait_until="networkidle")
    pg.wait_for_timeout(2600)
    total = pg.evaluate("document.documentElement.scrollHeight")
    print("height", total)
    ys = [int(v) for v in only.split(",")] if only else list(range(0, total - h + step, step))
    for i, y in enumerate(ys):
        pg.evaluate(f"window.scrollTo(0,{y})")
        pg.wait_for_timeout(1300)
        pg.screenshot(path=str(out / f"f{i:02d}_{y}.png"))
    print("\n".join(errs) or "no console errors")
    b.close()
