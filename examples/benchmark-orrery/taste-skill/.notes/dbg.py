from playwright.sync_api import sync_playwright
url = 'examples/benchmark-orrery/taste-skill/index.html'
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={'width':1440,'height':900})
    pg.goto(url); pg.wait_for_timeout(1500)
    pg.evaluate("document.querySelector('#telescope').scrollIntoView()"); pg.wait_for_timeout(2000)
    print(pg.evaluate("(()=>{const s=document.querySelector('.slip');const r=s.getBoundingClientRect();return [s.className, getComputedStyle(s).clipPath.slice(0,60), r.top, r.height, getComputedStyle(s).display]})()"))
    b.close()
