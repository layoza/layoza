import pathlib, json, sys
from playwright.sync_api import sync_playwright
root = pathlib.Path(__file__).resolve().parents[2]
with sync_playwright() as p:
    b = p.chromium.launch(); pg = b.new_page(viewport={'width':375,'height':812})
    pg.add_init_script("""window.__ls=[];new PerformanceObserver(l=>{for(const e of l.getEntries()){window.__ls.push({v:e.value,t:Math.round(e.startTime),src:(e.sources||[]).map(s=>(s.node&&s.node.nodeName+'.'+(s.node.className&&s.node.className.baseVal===undefined?s.node.className:''))+' '+JSON.stringify(s.previousRect)+'->'+JSON.stringify(s.currentRect))})}}).observe({type:'layout-shift',buffered:true});
    window.__lcp=[];new PerformanceObserver(l=>{for(const e of l.getEntries()){window.__lcp.push([Math.round(e.startTime),e.element&&e.element.nodeName+'.'+e.element.className,e.size])}}).observe({type:'largest-contentful-paint',buffered:true});""")
    pg.goto((root/'index.html').as_uri()); pg.wait_for_timeout(3000)
    for y in range(0,8000,700): pg.evaluate(f'scrollTo(0,{y})'); pg.wait_for_timeout(250)
    print(json.dumps(pg.evaluate('window.__ls'),indent=0)[:3000]); print(pg.evaluate('window.__lcp'))
    b.close()
