"""Assembles ../index.html from src.html + src.css + src.js + art.json (dev-only)."""
import json, os, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
subprocess.run([sys.executable, os.path.join(here, "art.py")], check=True, stdout=subprocess.DEVNULL)
art = json.load(open(os.path.join(here, "art.json")))
rd = lambda n: open(os.path.join(here, n), encoding="utf-8").read()

PLUS = '<span class="plus" aria-hidden="true"><svg width="12" height="12" viewBox="0 0 12 12"><path d="M6 1v10M1 6h10" stroke="currentColor" stroke-width="1.6"/></svg></span>'
projects = [
    ("a", "P1", "Halcyon Line", "Identity &amp; wayfinding", "A demonstration identity for a coastal ferry route: one continuous line that becomes the logo, the timetable and the signage on every pier.",
     [("Brief", "Make a small ferry line feel as dependable as a city metro."), ("Idea", "The route itself is the mark: a single line that bends with the coast."), ("Work", "Identity, pier signage, timetable system, ticket app screens.")]),
    ("b", "P2", "Field Notes", "Editorial platform", "A demonstration publishing platform where layouts rearrange around the story instead of the template.",
     [("Brief", "Give a long-form magazine a site that reads like print and loads like an app."), ("Idea", "Modular page layouts that reflow by story type."), ("Work", "Design system, CMS templates, front-end build.")]),
    ("c", "P3", "Open Hours", "Campaign", "A demonstration poster and social campaign for a venue that stays open late: time, written as type.",
     [("Brief", "Tell a city that the venue is open after dark, every day."), ("Idea", "The opening hours are the headline."), ("Work", "Poster series, motion cut-downs, social templates.")]),
    ("d", "P4", "Tidal", "Motion system &amp; 3D", "A demonstration motion language: rings and ribbons that slow down and speed up like water, built for screens of every size.",
     [("Brief", "Give a brand a way to move that no one could mistake."), ("Idea", "Timing borrowed from tides: long swells, short breaks."), ("Work", "Motion rules, 3D toolkit, launch film, real-time web hero.")]),
]
import re
def zoom(svg):
    i = svg.index('>') + 1
    j = svg.rindex('</svg>')
    return svg[:i] + '<g class="zm">' + svg[i:j] + '</g>' + svg[j:]
pm = []
for i, (k, key, name, cat, desc, outline) in enumerate(projects):
    dl = "".join(f"<dt>{a}</dt><dd>{b}</dd>" for a, b in outline)
    pm.append(f'''<article class="proj proj--{k}" aria-labelledby="pj-{i}">
        <button class="proj__media" type="button" aria-expanded="false" aria-controls="pm-{i}" aria-label="View project: {name}"><span class="proj__art">{zoom(art[key])}</span></button>
        <div class="proj__meta">
          <h3 class="proj__name" id="pj-{i}">{name}</h3>
          <div class="proj__info">
            <p class="proj__cat">{cat}, <em>demonstration project</em></p>
            <p>{desc}</p>
            <button class="proj__view" type="button" aria-expanded="false" aria-controls="pm-{i}">{PLUS}View project</button>
            <div class="proj__more" id="pm-{i}"><div><dl>{dl}<dt>Note</dt><dd>Demonstration content. Replace with a real case study and verified results.</dd></dl></div></div>
          </div>
        </div>
      </article>''')
rings = "".join(f'<circle class="ring" cx="960" cy="150" r="{r}"/>' for r in (34, 62, 92, 124))

html = rd("src.html")
html = html.replace("{{CSS}}", rd("src.css")).replace("{{JS}}", rd("src.js"))
html = html.replace("{{PROJECTS}}", "\n      ".join(pm)).replace("{{RINGS}}", rings)
for k, v in art.items():
    html = html.replace("{{" + k + "}}", v.replace("'", "&#39;") if k == "PROC_JSON" else v)
assert "{{" not in html, html[html.index("{{"):html.index("{{") + 40]
out = os.path.join(here, "..", "index.html")
open(out, "w", encoding="utf-8").write(html)
print("wrote", os.path.normpath(out), len(html) // 1024, "KB")
