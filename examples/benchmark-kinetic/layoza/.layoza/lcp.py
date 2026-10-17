import pathlib, json, sys
from playwright.sync_api import sync_playwright
url=(pathlib.Path(__file__).parent.parent/"index.html").as_uri()
init="""window.__l=[];new PerformanceObserver(l=>{for(const e of l.getEntries())window.__l.push([Math.round(e.renderTime||e.startTime),e.size,e.element&&(e.element.tagName+'.'+e.element.className+':'+(e.element.textContent||'').slice(0,30))])}).observe({type:'largest-contentful-paint',buffered:true});
new PerformanceObserver(l=>{for(const e of l.getEntries())window.__l.push([e.name,Math.round(e.startTime)])}).observe({type:'paint',buffered:true});"""
with sync_playwright() as p:
    b=p.chromium.launch()
    for w,h in ((375,812),):
        c=b.new_context(viewport={"width":w,"height":h}); c.add_init_script(init); pg=c.new_page()
        pg.goto(url,wait_until="load"); pg.wait_for_timeout(2500)
        print(w, json.dumps(pg.evaluate("window.__l")))
        print(' resources', json.dumps(pg.evaluate("performance.getEntriesByType('resource').map(r=>[r.name.slice(-40),Math.round(r.startTime),Math.round(r.responseEnd)])")))
    b.close()
