import pathlib, http.server, threading, functools, time
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).resolve().parent.parent
out = root/'.notes'/'shots'
srv = http.server.ThreadingHTTPServer(('127.0.0.1', 0), functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(root)))
srv.RequestHandlerClass.log_message = lambda *a: None
threading.Thread(target=srv.serve_forever, daemon=True).start()
url = f'http://127.0.0.1:{srv.server_address[1]}/index.html'
with sync_playwright() as p:
    b = p.chromium.launch(args=['--use-gl=angle','--ignore-gpu-blocklist'])
    errs = []
    # mobile: menu, form, dialog
    pg = b.new_page(viewport={'width':375,'height':812}, is_mobile=True, has_touch=True)
    pg.on('pageerror', lambda e: errs.append(str(e))); pg.on('console', lambda m: errs.append(m.text) if m.type=='error' else None)
    pg.goto(url, wait_until='networkidle'); time.sleep(2)
    pg.click('.menu-toggle'); time.sleep(1); pg.screenshot(path=str(out/'i-menu.png'))
    print('menu open:', pg.get_attribute('.menu-toggle','aria-expanded'))
    pg.click('.menu-links a[href="#work"]'); time.sleep(1.5)
    print('after menu link scrollY', pg.evaluate('scrollY'), 'work top', pg.evaluate("document.getElementById('work').getBoundingClientRect().top"))
    pg.click('.proj--a .proj-open'); time.sleep(.6); print('dialog open:', pg.evaluate("document.getElementById('detail').open")); pg.screenshot(path=str(out/'i-dialog.png'))
    pg.keyboard.press('Escape'); time.sleep(.3); print('dialog closed:', not pg.evaluate("document.getElementById('detail').open"))
    # services accordion
    pg.evaluate("document.querySelectorAll('.svc-btn')[2].scrollIntoView()"); pg.click('.svc[data-i=\"2\"] .svc-btn'); time.sleep(.6)
    print('svc 2 expanded:', pg.get_attribute('.svc[data-i=\"2\"] .svc-btn','aria-expanded'))
    # form
    pg.evaluate("document.getElementById('brief').scrollIntoView()")
    pg.click('#brief button[type=submit]'); time.sleep(.3); print('empty submit status:', pg.inner_text('.form-status'))
    pg.fill('#f-name','Test Person'); pg.fill('#f-email','test@example.com'); pg.fill('#f-msg','A short test brief.')
    pg.evaluate("window.__href=null; const d=Object.getOwnPropertyDescriptor(window.location.__proto__||Location.prototype,'href');")
    pg.route('**/*', lambda r: r.continue_())
    hrefs=[]
    pg.on('request', lambda r: None)
    pg.evaluate("HTMLFormElement.prototype._x=1")
    pg.click('#brief button[type=submit]'); time.sleep(.5); print('valid submit status:', pg.inner_text('.form-status'))
    print('overflow 375:', pg.evaluate('document.documentElement.scrollWidth'))
    # reel controls
    pg.evaluate("document.querySelector('.reel-stage').scrollIntoView()"); time.sleep(.5)
    pg.click('.reel-tab[data-seq=\"1\"]'); pg.click('.reel-play'); time.sleep(.5)
    print('reel selected:', pg.get_attribute('.reel-tab[data-seq=\"1\"]','aria-selected'), 'paused:', pg.get_attribute('.reel-play','aria-pressed'))
    pg.screenshot(path=str(out/'i-reel.png'))
    # reduced motion desktop
    ctx = b.new_context(viewport={'width':1440,'height':900}, reduced_motion='reduce'); pr = ctx.new_page()
    pr.on('pageerror', lambda e: errs.append('RM '+str(e)))
    pr.goto(url, wait_until='networkidle'); time.sleep(1.5); pr.screenshot(path=str(out/'i-reduced-hero.png'))
    for y in [3000, 7000, 9500]:
        pr.evaluate(f'scrollTo(0,{y})'); time.sleep(.6); pr.screenshot(path=str(out/f'i-reduced-{y}.png'))
    print('errors:', errs or 'none')
    b.close()
srv.shutdown()
