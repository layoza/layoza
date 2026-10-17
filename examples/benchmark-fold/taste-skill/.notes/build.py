"""Builds ../index.html: inline CSS/JS, SVG drawings as <symbol>s reused by the hero sheet and work strips."""
import os, sys, html
sys.path.insert(0, os.path.dirname(__file__))
from drawings import *  # noqa

OUT = os.path.join(os.path.dirname(__file__), "..", "index.html")

# ---------------- sample projects (fictional; to be confirmed by the client) ----------------
PROJECTS = [
    dict(id="p1", name="Hyndland close", place="Hyndland, Glasgow", year="2024", type="housing", typel="Housing retrofit",
         desc="Eight flats in a red sandstone tenement, insulated from the inside with the sash windows kept and sealed.",
         frames=[
             ("elev", "Street elevation", tenement_elevation(4, 3, "dormer", 1.5)),
             ("sec", "Section through the close", tenement_section(4, 1.3)),
             ("det", "Wall detail", wall_detail(0.75, "stone")),
             ("plan", "Second floor flat plan", plan(1.4, None,
                 parts=[(56, 8, 56, 40), (56, 50, 56, 92), (8, 52, 44, 52), (76, 8, 76, 30), (76, 40, 76, 92), (76, 56, 132, 56)],
                 doors=[(44, 52, 8, 1, 1), (56, 40, 9, 1, 1), (76, 30, 9, 1, 1)],
                 windows=[(16, 8, 14, 3.2), (34, 8, 10, 3.2), (92, 8, 16, 3.2), (16, 88.8, 14, 3.2), (96, 88.8, 12, 3.2)],
                 new=[(11.2, 11.2, 1.8, 77.6), (127, 11.2, 1.8, 77.6), (100, 56, 1.6, 33)])),
             ("site", "Block plan with the back court", site_plan(1.0, (44, 14, 12, 4))),
         ]),
    dict(id="p2", name="Four cottage flats", place="Dennistoun, Glasgow", year="2023", type="housing", typel="Housing retrofit",
         desc="A 1930s four-in-a-block brought to a low-energy standard while all four households stayed at home.",
         frames=[
             ("elev", "Front elevation", cottage_elevation(1.7)),
             ("sec", "Section, upper and lower flats", tenement_section(2, 1.1)),
             ("plan", "Lower flat plan", plan(1.2, None,
                 parts=[(50, 8, 50, 44), (50, 54, 50, 92), (8, 50, 40, 50), (50, 62, 112, 62)],
                 doors=[(40, 50, 9, 1, 1), (50, 44, 9, 1, 1), (80, 62, 8, -1, 1)],
                 windows=[(18, 8, 16, 3.2), (66, 8, 18, 3.2), (20, 88.8, 14, 3.2)],
                 new=[(8, 4.4, 104, 2.2), (8, 93.4, 104, 2.2), (4.4, 8, 2.2, 84), (113.4, 8, 2.2, 84)])),
             ("det", "Window head detail", wall_detail(0.75, "sash")),
         ]),
    dict(id="p3", name="Pollokshaws reading room", place="Pollokshaws, Glasgow", year="2025", type="civic", typel="Civic",
         desc="An empty bank hall made into a reading room and meeting space for a local community trust.",
         frames=[
             ("elev", "Street elevation with the new entrance", bank_elevation(1.0)),
             ("sec", "Section through the hall and mezzanine", hall_section(1.4)),
             ("plan", "Ground floor plan", plan(1.3, None,
                 parts=[(90, 8, 90, 50), (90, 62, 90, 92), (90, 40, 122, 40)],
                 doors=[(90, 50, 10, -1, 1), (106, 40, 9, 1, -1)],
                 windows=[(20, 8, 12, 3.2), (44, 8, 12, 3.2), (68, 8, 12, 3.2), (8, 30, 3.2, 14), (8, 58, 3.2, 14)],
                 new=[(46, 50, 42, 2.2), (46, 52.2, 2.2, 36), (30, 88.8, 16, 3.2)])),
             ("det", "Shelving and mezzanine edge", wall_detail(0.8, "shelf")),
         ]),
    dict(id="p4", name="Bowling club pavilion", place="Shawlands, Glasgow", year="2022", type="civic", typel="Civic",
         desc="A timber clubhouse on the footprint of the old one, with a long window onto the green.",
         frames=[
             ("elev", "Elevation from the green", pavilion_elevation(2.0)),
             ("sec", "Section through the glulam truss", pavilion_section(1.3)),
             ("plan", "Plan", plan(1.5, None,
                 parts=[(104, 8, 104, 50), (104, 62, 104, 92), (104, 50, 142, 50)],
                 doors=[(104, 50, 10, -1, 1)],
                 windows=[(24, 88.8, 70, 3.2), (24, 8, 18, 3.2), (116, 8, 14, 3.2)],
                 new=[(12, 12, 1.6, 76), (60, 12, 1.6, 76)])),
             ("det", "Cladding and wall build-up", wall_detail(0.75, "timber")),
         ]),
    dict(id="p5", name="Back-court kitchen", place="Partick, Glasgow", year="2024", type="extensions", typel="Extension",
         desc="A larch-clad kitchen built out into a tenement back court, lit by one long roof light.",
         frames=[
             ("site", "Back court plan", site_plan(1.0, (54, 30, 12, 12))),
             ("sec", "Section through the kitchen", extension_section(1.3, "mono", 3)),
             ("elev", "Rear elevation of the tenement", rear_elevation(0.9)),
             ("plan", "Kitchen plan", plan(1.2, None,
                 parts=[(40, 8, 40, 36), (40, 48, 40, 92)],
                 doors=[(40, 36, 10, 1, 1)],
                 windows=[(56, 88.8, 40, 3.2), (12, 8, 14, 3.2)],
                 new=[(40, 8, 72, 3.2), (108.8, 8, 3.2, 84), (40, 88.8, 16, 3.2), (96, 88.8, 16, 3.2), (60, 30, 30, 10)])),
             ("det", "Roof light detail", wall_detail(0.75, "rooflight")),
         ]),
    dict(id="p6", name="Garden room", place="Bearsden", year="2023", type="extensions", typel="Extension",
         desc="A cross-laminated timber room at the end of a 1960s house, open to the garden on three sides.",
         frames=[
             ("elev", "Garden elevation", house_elevation(1.6)),
             ("plan", "Plan of house and garden room", plan(1.1, None,
                 parts=[(56, 8, 56, 40), (56, 52, 56, 92)],
                 doors=[(56, 40, 10, 1, 1)],
                 windows=[(70, 8, 28, 3.2), (98.8, 22, 3.2, 54), (70, 88.8, 28, 3.2), (20, 8, 14, 3.2)],
                 new=[(56, 8, 46, 2.4), (99.6, 8, 2.4, 84), (56, 89.6, 46, 2.4)])),
             ("sec", "Section", extension_section(1.3, "flat", 2)),
             ("det", "Wall build-up", wall_detail(0.75, "timber")),
         ]),
]

PEOPLE = [
    ("Morag Lindsay", "Director, architect"),
    ("Callum Reid", "Director, architect"),
    ("Priya Nair", "Architect, retrofit"),
    ("Euan Baird", "Architect, timber"),
    ("Ailsa McKenzie", "Architectural assistant"),
    ("Tomasz Wójcik", "Technologist"),
]

# ---------------- symbols ----------------
symbols = []
frames_all = []  # (symbol id, ratio, label, project)
for p in PROJECTS:
    for key, label, (ratio, inner) in p["frames"]:
        sid = f"d-{p['id']}-{key}"
        symbols.append(f'<symbol id="{sid}" viewBox="0 0 {f(ratio*100)} 100">{inner}</symbol>')
        frames_all.append((sid, ratio, label, p))


def svg_use(sid, ratio, label=None, cls=""):
    a11y = f'role="img" aria-label="{html.escape(label)}"' if label else 'aria-hidden="true" focusable="false"'
    return (f'<svg class="{cls}" viewBox="0 0 {f(ratio*100)} 100" style="aspect-ratio:{f(ratio)}" {a11y}>'
            f'<use href="#{sid}"/></svg>')


# ---------------- hero sheet ----------------
# deal frames to columns round-robin, rotating start so projects mix
order = []
for i in range(5):
    for p in PROJECTS:
        fr = [x for x in frames_all if x[3] is p]
        if i < len(fr):
            order.append(fr[i])
cols = {k: [] for k in ("c1", "c2", "c3t", "c4t", "c3b", "c4b", "c5", "c6")}
plan_seq = ["c1", "c2", "c5", "c6", "c3t", "c4t", "c1", "c2", "c5", "c6", "c3b", "c4b", "c1", "c2", "c5", "c6", "c3t", "c4t",
            "c1", "c2", "c5", "c6", "c3b", "c4b", "c1", "c2", "c5", "c6", "c3t", "c4t"]
for idx, fr in enumerate(order + order[:6]):
    if idx >= len(plan_seq):
        break
    cols[plan_seq[idx]].append(fr)

strip_i = [0]


def hero_frame(fr):
    sid, ratio, label, p = fr
    s = strip_i[0] % 4
    strip_i[0] += 1
    short = label.split(",")[0].split(" with")[0].lower()
    return (f'<div class="fr"><div class="pp">{svg_use(sid, ratio)}</div>'
            f'<span class="tag t{s}">{html.escape(p["name"])}, {html.escape(short)}</span></div>')


def col(name, cls):
    return f'<div class="col {cls}" aria-hidden="true">' + "".join(hero_frame(fr) for fr in cols[name]) + "</div>"


hero_html = f"""
<h1 id="hero-title" class="display">Retrofit, civic and timber work</h1>
<div class="sheet">
  {col('c1', 'k1')}
  {col('c2', 'k2')}
  <div class="mid">
    <div class="pair top">{col('c3t', 'k3')}{col('c4t', 'k4')}</div>
    <div class="panel">
      <p>Fold is six architects in Glasgow. We bring tenements and houses up to date, design small public buildings and add timber extensions. In our drawings, orange is the new work.</p>
      <div class="panel-foot">
        <a class="btn" href="#work">See the work</a>
        <p class="jump"><a href="#work" data-jump="housing">Housing</a><a href="#work" data-jump="civic">Civic</a><a href="#work" data-jump="extensions">Extensions</a></p>
      </div>
    </div>
    <div class="pair bot">{col('c3b', 'k3')}{col('c4b', 'k4')}</div>
  </div>
  {col('c5', 'k5')}
  {col('c6', 'k6')}
</div>"""

# ---------------- work ----------------
proj_html = []
for p in PROJECTS:
    fr = [x for x in frames_all if x[3] is p]
    frames = "".join(f'<figure class="wf">{svg_use(sid, ratio, p["name"] + ": " + label)}</figure>' for sid, ratio, label, _ in fr)
    proj_html.append(f"""
    <li class="proj" data-type="{p['type']}">
      <div class="strip" role="group" aria-label="Drawings of {html.escape(p["name"])}">{frames}</div>
      <div class="cap">
        <h3>{html.escape(p['name'])}<span class="place">{html.escape(p['place'])}</span></h3>
        <p class="type">{p['typel']}</p>
        <p class="year">{p['year']}</p>
      </div>
      <p class="desc">{html.escape(p['desc'])}</p>
    </li>""")

people_html = "".join(
    f'<li class="tile v{i % 4}"><span class="nm">{html.escape(n)}</span><span class="rl">{html.escape(r_)}</span></li>'
    for i, (n, r_) in enumerate(PEOPLE))

PRINCIPLES = [
    ("Keep what is there", "Most of the carbon in a building was spent the day it was built. Every job starts with a survey of what can stay, and the design works around it."),
    ("Draw the new work in orange", "Every drawing we issue shows the existing building in grey and our work in one colour, so owners, builders and planners can see exactly what changes."),
    ("Build in timber where it fits", "Timber frames and cross-laminated panels are light enough for back courts and old floors, and quick to put up when neighbours are living through the work."),
    ("Stay until it works", "The architect who drew the project visits site every week until handover, and comes back after a year to see how the building is being used."),
]
principles_html = "".join(
    f'<li class="band"><h3>{html.escape(t)}</h3><p>{html.escape(d)}</p></li>' for t, d in PRINCIPLES)

CSS = open(os.path.join(os.path.dirname(__file__), "page.css"), encoding="utf-8").read()
JS = open(os.path.join(os.path.dirname(__file__), "page.js"), encoding="utf-8").read()

page = f"""<!doctype html>
<html lang="en-GB">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Fold Architects, Glasgow</title>
<meta name="description" content="Fold is a six-person architecture studio in Glasgow working on housing retrofits, small civic buildings and timber extensions.">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Hanken+Grotesk:wght@400;500&display=swap" rel="stylesheet">
<style>
{CSS}
</style>
</head>
<body>
<a class="skip" href="#main">Skip to content</a>
<svg width="0" height="0" style="position:absolute" aria-hidden="true" focusable="false"><defs>{''.join(symbols)}</defs></svg>

<header class="bar">
  <a class="mark" href="#top" aria-label="Fold Architects, home">
    <svg viewBox="0 0 24 24" aria-hidden="true" focusable="false"><path d="M2 2h14l6 6v14H2z" fill="#eff0f1"/><path d="M16 2v6h6z" fill="#ff4a1f"/></svg>
    <span>Fold</span>
  </a>
  <button class="menu-btn" type="button" aria-expanded="false" aria-controls="site-nav">Menu</button>
  <nav id="site-nav" class="nav" aria-label="Main">
    <ul>
      <li><a href="#work">Work</a></li>
      <li><a href="#studio">Studio</a></li>
      <li><a href="#approach">Approach</a></li>
      <li><a href="#contact">Contact</a></li>
    </ul>
  </nav>
</header>

<main id="main">
<section class="hero" id="top" aria-labelledby="hero-title">{hero_html}
</section>

<section class="work" id="work" aria-labelledby="work-title">
  <div class="rail">
    <h2 id="work-title">Work</h2>
    <div class="filters" role="group" aria-label="Filter projects by type">
      <button type="button" data-filter="all" aria-pressed="true">All</button>
      <button type="button" data-filter="housing" aria-pressed="false">Housing</button>
      <button type="button" data-filter="civic" aria-pressed="false">Civic</button>
      <button type="button" data-filter="extensions" aria-pressed="false">Extensions</button>
    </div>
    <p class="count" aria-live="polite">Six projects</p>
    <p class="key"><span class="sw" aria-hidden="true"></span>Drawings, not photographs. Orange marks our new work.</p>
  </div>
  <ul class="list">{''.join(proj_html)}
  </ul>
</section>

<section class="studio" id="studio" aria-labelledby="studio-title">
  <div class="row1">
    <div class="facts">
      <h2 id="studio-title">Studio</h2>
      <dl>
        <div><dt>Based in</dt><dd>Glasgow</dd></div>
        <div><dt>People</dt><dd>Six, in one room</dd></div>
        <div><dt>Work</dt><dd>Retrofit, civic, timber</dd></div>
      </dl>
    </div>
    <p class="wide">Fold is small on purpose. The people below draw, cost and visit every project themselves, so the person you meet at the first site visit is the person who signs off the last detail. Most of our clients are householders, housing associations and community groups within an hour of the studio.</p>
  </div>
  <h3 class="team-title">Six people</h3>
  <ul class="team">{people_html}</ul>
</section>

<section class="approach" id="approach" aria-labelledby="approach-title">
  <div class="intro">
    <h2 id="approach-title">Approach</h2>
    <p>Four rules we work to on every job, from a single sash window to a new public building.</p>
  </div>
  <ul class="bands">{principles_html}</ul>
</section>

<section class="contact" id="contact" aria-labelledby="contact-title">
  <div class="card">
    <div class="face a">
      <h2 id="contact-title">Start a project</h2>
      <p>Tell us about the building, what you would like to change and roughly when. One of the six of us replies within five working days.</p>
      <p class="direct"><a href="mailto:studio@fold-architects.co.uk">studio@fold-architects.co.uk</a><br><a href="tel:+441414960732">0141 496 0732</a></p>
    </div>
    <div class="face b">
      <form id="enquiry" novalidate>
        <div class="f">
          <label for="f-name">Name</label>
          <input id="f-name" name="name" type="text" autocomplete="name" required aria-describedby="e-name">
          <p class="err" id="e-name"></p>
        </div>
        <div class="f">
          <label for="f-email">Email</label>
          <input id="f-email" name="email" type="email" autocomplete="email" required aria-describedby="e-email">
          <p class="err" id="e-email"></p>
        </div>
        <div class="f half">
          <label for="f-type">Project type</label>
          <select id="f-type" name="type" required aria-describedby="e-type">
            <option value="">Choose one</option>
            <option>Housing retrofit</option>
            <option>Civic building</option>
            <option>Timber extension</option>
            <option>Something else</option>
          </select>
          <p class="err" id="e-type"></p>
        </div>
        <div class="f half">
          <label for="f-budget">Budget range</label>
          <select id="f-budget" name="budget" required aria-describedby="e-budget">
            <option value="">Choose one</option>
            <option>Under £50,000</option>
            <option>£50,000 to £150,000</option>
            <option>£150,000 to £500,000</option>
            <option>Over £500,000</option>
            <option>Not sure yet</option>
          </select>
          <p class="err" id="e-budget"></p>
        </div>
        <div class="f">
          <label for="f-msg">About the building</label>
          <textarea id="f-msg" name="message" rows="4" required aria-describedby="e-msg"></textarea>
          <p class="err" id="e-msg"></p>
        </div>
        <button class="btn submit" type="submit">Send enquiry</button>
      </form>
      <div class="done" id="done" tabindex="-1" hidden></div>
    </div>
  </div>
</section>
</main>

<footer class="foot">
  <p class="addr">Fold Architects<br>Studio 3, 21 Kelvin Lane, Glasgow</p>
  <ul class="links">
    <li><a href="mailto:studio@fold-architects.co.uk">studio@fold-architects.co.uk</a></li>
    <li><a href="tel:+441414960732">0141 496 0732</a></li>
    <li><a href="#" rel="me">Instagram</a></li>
    <li><a href="#" rel="me">LinkedIn</a></li>
  </ul>
  <p class="legal">© 2026 Fold Architects <a href="#">Privacy</a></p>
</footer>
<script>
{JS}
</script>
</body>
</html>
"""
with open(OUT, "w", encoding="utf-8", newline="\n") as fh:
    fh.write(page)
print("wrote", os.path.abspath(OUT), len(page) // 1024, "KB")
