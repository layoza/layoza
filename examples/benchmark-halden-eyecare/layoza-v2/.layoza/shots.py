"""Viewport screenshots at scroll positions (dev check). Usage: python shots.py [outdir]"""
import sys, pathlib
from playwright.sync_api import sync_playwright

here = pathlib.Path(__file__).resolve().parent
page_url = (here.parent / "index.html").as_uri()
out = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else here / "shots"
out.mkdir(parents=True, exist_ok=True)

def run(p, w, h, tag, stops, reduce=False):
    b = p.chromium.launch()
    pg = b.new_page(viewport={"width": w, "height": h}, reduced_motion="reduce" if reduce else "no-preference")
    pg.goto(page_url, wait_until="networkidle")
    pg.wait_for_timeout(1800)
    for name, js in stops:
        pg.evaluate(js)
        pg.wait_for_timeout(1300)
        pg.screenshot(path=str(out / f"{tag}-{name}.png"))
    b.close()

def at(sel, frac=0.0):
    return f"(()=>{{const el=document.querySelector('{sel}');const y=el.getBoundingClientRect().top+scrollY;window.scrollTo(0,y+{frac}*innerHeight)}})()"

with sync_playwright() as p:
    desk = [("00-hero", "window.scrollTo(0,0)")]
    # pinned story: the spacer starts at the story top; walk through it
    for i, f in enumerate([0.0, 0.45, 0.9, 1.35, 1.75]):
        desk.append((f"1{i}-story", at("#story", f)))
    for n, s in [("20-tests", "#tests"), ("30-book", "#book"), ("40-frames", "#frames"), ("50-lenses", "#lenses"),
                 ("60-about", "#about"), ("70-workshop", "#workshop"), ("80-voices", "#voices"), ("90-faq", "#faq"), ("95-foot", "#visit")]:
        desk.append((n, at(s, -0.05)))
    run(p, 1440, 900, "d", desk)
    mob = [("00-hero", "window.scrollTo(0,0)"), ("10-story", at("#story", 0.2)), ("11-story", at("#story", 0.8)),
           ("20-tests", at("#tests")), ("30-book", at("#book")), ("40-frames", at("#frames")), ("70-workshop", at("#workshop", 0.4))]
    run(p, 375, 812, "m", mob)
    run(p, 768, 1024, "t", [("00-hero", "window.scrollTo(0,0)")])
    run(p, 1440, 900, "rm", [("00-hero", "window.scrollTo(0,0)"), ("10-story", at("#story"))], reduce=True)
print("ok", out)
