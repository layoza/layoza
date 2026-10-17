"""Screenshot + console check. usage: python shoot.py <width> <height> <out_prefix> [reduce] [stops]"""
import sys, pathlib, http.server, threading, functools, time
from playwright.sync_api import sync_playwright

root = pathlib.Path(__file__).resolve().parent.parent
w, h, prefix = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3]
reduce = len(sys.argv) > 4 and sys.argv[4] == 'reduce'
stops = int(sys.argv[5]) if len(sys.argv) > 5 else 0
out = root / '.notes' / 'shots'; out.mkdir(parents=True, exist_ok=True)

Handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(root))
class Quiet(Handler.func):
    def log_message(self, *a): pass
srv = http.server.ThreadingHTTPServer(('127.0.0.1', 0), functools.partial(Quiet, directory=str(root)))
threading.Thread(target=srv.serve_forever, daemon=True).start()
url = f'http://127.0.0.1:{srv.server_address[1]}/index.html'

with sync_playwright() as p:
    b = p.chromium.launch(args=['--use-gl=angle', '--enable-webgl', '--ignore-gpu-blocklist'])
    mobile = w < 600
    ctx = b.new_context(viewport={'width': w, 'height': h}, reduced_motion='reduce' if reduce else 'no-preference',
                        is_mobile=mobile, has_touch=mobile, device_scale_factor=1)
    pg = ctx.new_page()
    logs = []
    pg.on('console', lambda m: logs.append(f'{m.type}: {m.text}') if m.type in ('error', 'warning') else None)
    pg.on('pageerror', lambda e: logs.append(f'PAGEERROR: {e}'))
    pg.goto(url, wait_until='networkidle')
    time.sleep(2.6)
    pg.screenshot(path=str(out / f'{prefix}-00.png'))
    total = pg.evaluate('document.documentElement.scrollHeight')
    sw = pg.evaluate('document.documentElement.scrollWidth')
    print('scrollHeight', total, 'scrollWidth', sw, 'viewport', w)
    if stops:
        y = 0; i = 1
        while y < total - h and i <= stops:
            y += int(h * 0.9)
            pg.mouse.wheel(0, 0)
            pg.evaluate(f'window.KSLenis ? window.KSLenis.scrollTo({y}, {{immediate:true}}) : window.scrollTo(0,{y})')
            time.sleep(1.1)
            pg.screenshot(path=str(out / f'{prefix}-{i:02d}.png'))
            i += 1
            total = pg.evaluate('document.documentElement.scrollHeight')
    print('\n'.join(logs) or 'no console errors')
    b.close()
srv.shutdown()
