import sys, pathlib, http.server, threading, functools, time
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).resolve().parent.parent
w = int(sys.argv[1]); js = sys.argv[2]
srv = http.server.ThreadingHTTPServer(('127.0.0.1', 0), functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(root)))
srv.RequestHandlerClass.log_message = lambda *a: None
threading.Thread(target=srv.serve_forever, daemon=True).start()
with sync_playwright() as p:
    b = p.chromium.launch(args=['--use-gl=angle','--ignore-gpu-blocklist'])
    pg = b.new_page(viewport={'width': w, 'height': 900 if w > 600 else 844}, is_mobile=w<600, has_touch=w<600)
    pg.goto(f'http://127.0.0.1:{srv.server_address[1]}/index.html', wait_until='networkidle'); time.sleep(2)
    print(pg.evaluate(js))
    b.close()
srv.shutdown()
