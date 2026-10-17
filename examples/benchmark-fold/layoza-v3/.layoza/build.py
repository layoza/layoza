"""Builds ../index.html from template.html + generated card-model SVGs (dev only)."""
import os, re
import models

here = os.path.dirname(os.path.abspath(__file__))
sc = models.scenes()
out = {}
for k, (solids, base, s, title, _) in sc.items():
    out[k] = models.svg(k, solids, base, s, title, "")
h = sc["hero"]
out["footer"] = models.svg("footer", h[0], h[1], h[2], h[3], "")
out["footer"] = out["footer"].replace('role="img" aria-labelledby="footer-t"', 'aria-hidden="true" focusable="false"')
tpl = open(os.path.join(here, "template.html"), encoding="utf-8").read()
html = re.sub(r"\{\{(\w+)\}\}", lambda m: out[m.group(1)], tpl)
open(os.path.join(here, "..", "index.html"), "w", encoding="utf-8").write(html)
print("index.html", len(html))
