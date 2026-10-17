"""Browser QA for the ui-ux-pro-max arm. Usage: python qa.py <url> <outdir>"""
import sys, os, json
from playwright.sync_api import sync_playwright

url, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
report = {}

def scroll_shots(page, tag, positions):
    h = page.evaluate("document.documentElement.scrollHeight")
    for name, sel in positions:
        if sel.startswith('#') or sel.startswith('.'):
            page.evaluate(f"(() => {{ const e=document.querySelector('{sel}'); window.scrollTo(0, e.getBoundingClientRect().top + window.scrollY); }})()")
        else:
            page.evaluate(f"window.scrollTo(0, {sel})")
        page.wait_for_timeout(1600)
        page.screenshot(path=f"{out}/{tag}-{name}.png")
    return h

with sync_playwright() as p:
    b = p.chromium.launch(args=["--use-gl=angle", "--enable-webgl", "--ignore-gpu-blocklist"])
    for w, hgt in [(1440, 900), (768, 1024), (375, 812)]:
        ctx = b.new_context(viewport={"width": w, "height": hgt})
        page = ctx.new_page()
        errs = []
        page.on("console", lambda m: errs.append(f"{m.type}: {m.text}") if m.type in ("error", "warning") else None)
        page.on("pageerror", lambda e: errs.append(f"pageerror: {e} {getattr(e,'stack','')}"))
        page.goto(url, wait_until="networkidle")
        page.wait_for_timeout(2600)
        page.screenshot(path=f"{out}/{w}-hero.png")
        info = page.evaluate("""() => ({
          overflow: document.documentElement.scrollWidth - document.documentElement.clientWidth,
          ribbonReady: document.querySelector('.hero-visual').classList.contains('is-ready'),
          gsap: !!window.gsap, cursor: document.documentElement.classList.contains('has-cursor'),
          wide: [...document.querySelectorAll('body *')].filter(e => { const r = e.getBoundingClientRect(); return r.right > document.documentElement.clientWidth + 1 && getComputedStyle(e).position !== 'fixed' && !e.closest('.contact-marquee,.contact-ribbon,.foot-mark,.proj--full .art,.menu,.hero-visual,.art,svg'); }).slice(0,8).map(e => e.tagName + '.' + e.className)
        })""")
        sels = [("intro", "#about"), ("services", "#services"), ("services2", ".svc-track"), ("work", "#work"), ("work2", ".proj-pair"),
                ("philo", "#philosophy"), ("process", "#process"), ("reel", ".reel-stage"), ("insights", "#insights"), ("contact", "#contact"), ("footer", ".footer")]
        if w == 1440:
            # mid-sequence positions for sticky stories
            page.evaluate("window.scrollTo(0, document.querySelector('.svc-track').getBoundingClientRect().top + scrollY + innerHeight*1.6)"); page.wait_for_timeout(1500)
            page.screenshot(path=f"{out}/{w}-services-mid.png")
            page.evaluate("window.scrollTo(0, document.querySelector('#philosophy').getBoundingClientRect().top + scrollY + innerHeight*1.55)"); page.wait_for_timeout(1500)
            page.screenshot(path=f"{out}/{w}-philo-end.png")
            page.evaluate("window.scrollTo(0, document.querySelector('#philosophy').getBoundingClientRect().top + scrollY + innerHeight*0.45)"); page.wait_for_timeout(1500)
            page.screenshot(path=f"{out}/{w}-philo-start.png")
        scroll_shots(page, str(w), sels)
        if w == 375:
            page.evaluate("window.scrollTo(0,0)"); page.wait_for_timeout(500)
            page.click(".burger"); page.wait_for_timeout(900)
            page.screenshot(path=f"{out}/375-menu.png")
            info["menuOpen"] = page.evaluate("!document.getElementById('menu').hidden")
            page.keyboard.press("Escape"); page.wait_for_timeout(700)
            info["menuClosed"] = page.evaluate("document.getElementById('menu').hidden")
            page.evaluate("window.scrollTo(0, document.querySelector('#services').offsetTop)"); page.wait_for_timeout(600)
            page.click(".svc-item[data-i='2'] .svc-btn"); page.wait_for_timeout(900)
            page.screenshot(path=f"{out}/375-svc-accordion.png")
            info["accordion"] = page.evaluate("document.querySelector(\".svc-item[data-i='2']\").classList.contains('is-active')")
        # form validation
        page.evaluate("document.querySelector('#contact-form').scrollIntoView()"); page.wait_for_timeout(800)
        page.click("#contact-form button[type=submit]"); page.wait_for_timeout(300)
        info["formErrors"] = page.evaluate("[...document.querySelectorAll('#contact-form .err')].map(e=>e.textContent).filter(Boolean).length")
        page.click(".proj-view >> nth=0", force=True); page.wait_for_timeout(500)
        info["dialog"] = page.evaluate("document.getElementById('caseDialog').open")
        page.keyboard.press("Escape")
        info["errors"] = errs
        report[w] = info
        ctx.close()
    # reduced motion
    ctx = b.new_context(viewport={"width": 1440, "height": 900}, reduced_motion="reduce")
    page = ctx.new_page(); errs = []
    page.on("pageerror", lambda e: errs.append(str(e)))
    page.goto(url, wait_until="networkidle"); page.wait_for_timeout(1500)
    page.screenshot(path=f"{out}/reduced-hero.png")
    page.evaluate("window.scrollTo(0, document.querySelector('#philosophy').getBoundingClientRect().top + scrollY + innerHeight*0.5)"); page.wait_for_timeout(800)
    page.screenshot(path=f"{out}/reduced-philo.png")
    report["reduced"] = {"errors": errs, "anim": page.evaluate("document.documentElement.classList.contains('anim')")}
    b.close()
print(json.dumps(report, indent=1))
