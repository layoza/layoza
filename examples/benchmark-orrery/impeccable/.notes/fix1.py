p='index.html'; s=open(p,encoding='utf-8').read()
def rep(a,b):
    global s
    assert a in s, a[:70]; s=s.replace(a,b,1)
rep("font-size:clamp(2.5rem,1rem + 4.2vw,4.75rem);line-height:.92}","font-size:clamp(2.6rem,1rem + 4.7vw,5.25rem);line-height:.92}")
rep(".hero-copy{grid-column:1 / span 7}",".hero-copy{grid-column:1 / span 7;position:relative;z-index:0}\n.ovp{position:relative;mix-blend-mode:multiply}\n.ovp-disc{position:absolute;z-index:-1;width:1.02em;height:1.02em;border-radius:50%;background:var(--amber);right:-.32em;top:-.06em;pointer-events:none}")
rep("seat dome.</h1>","seat <span class=\"ovp\">dome.</span><span class=\"ovp-disc\" aria-hidden=\"true\"></span></h1>")
rep('<h1 class="display" id="heroTitle">','<h1 class="display" id="heroTitle" style="position:relative;isolation:isolate;width:max-content;max-width:100%">')
rep(".prog .h3{color:var(--ink)}",".prog .h3{color:var(--ink);font-stretch:70%;font-weight:850;font-size:clamp(1.9rem,1.3rem + 1.8vw,3rem);line-height:.95;letter-spacing:-.005em;text-transform:uppercase}")
rep("""  .tt li{grid-template-columns:5.5rem minmax(0,1fr) auto;row-gap:2px}
  .tt .t{font-size:1.6rem;grid-row:span 2}
  .tt .meta{grid-column:2;grid-row:2}
  .tt .seats{display:none}
  .tt .btn{grid-row:1 / span 2;grid-column:3}""","""  .tt li{grid-template-columns:4.75rem minmax(0,1fr) auto;grid-template-areas:"t show book" "t meta book";row-gap:2px;column-gap:var(--s3)}
  .tt .t{font-size:1.5rem;grid-area:t}
  .tt .show{grid-area:show}
  .tt .meta:not(.seats){grid-area:meta}
  .tt .seats{display:none}
  .tt .btn{grid-area:book}""")
rep("@media (max-width:760px){\n  .tt li","@media (max-width:900px){\n  .tt li")
rep("Open Tuesday to Sunday, 10:00 to 21:00.","Open Tuesday to Sunday from 10:00, Fridays until 23:00.")
rep("Shows every day on Bristol's harbourside","Shows Tuesday to Sunday on Bristol's harbourside")
rep("""    var dd = new Date(today); dd.setDate(today.getDate() + i);
    var o = document.createElement("option");""","""    var dd = new Date(today); dd.setDate(today.getDate() + i);
    if (dd.getDay() === 1) continue; // Mondays: schools only
    var o = document.createElement("option");""")
rep("""    var allPassed = SESSIONS.every(function(t){ return sessionPassed(iso(today), t); });""","""    var allPassed = SESSIONS.every(function(t){ return sessionPassed(iso(today), t); }) || today.getDay() === 1;""")
rep("""    var got = 0; for (var i=0;i<60 && got<n;i++) if (!taken[i]){ out[i] = true; got++; }
    return out;""","""    var pos = seatPositions(), free = [];
    for (var i=0;i<60;i++) if (!taken[i]) free.push(i);
    if (!free.length) return out;
    var front = function(k){ return Math.abs(pos[k].x - 300) + (600 - pos[k].y) * .2; };
    free.sort(function(a, b){ return front(a) - front(b); });
    var anchor = free[0];
    var dist = function(k){ return Math.hypot(pos[k].x - pos[anchor].x, pos[k].y - pos[anchor].y); };
    free.sort(function(a, b){ return dist(a) - dist(b); });
    free.slice(0, n).forEach(function(k){ out[k] = true; });
    return out;""")
rep("""      sl.innerHTML = "<b>" + Math.min(n, free) + "</b> " + (Math.min(n, free) === 1 ? "seat" : "seats") + " held for you · " + free + " of 60 free";""","""      var held = Math.min(n, free);
      sl.innerHTML = "<b>" + held + "</b> " + (held === 1 ? "seat" : "seats") + " held for you · " + (free - held) + " more free of 60";""")
rep("""  <defs>
    <symbol id="i-mark\"""","""  <defs>
    <pattern id="ht-ink" width="5" height="5" patternUnits="userSpaceOnUse"><circle cx="2.5" cy="2.5" r="1.25" fill="#1F2BB8"/></pattern>
    <symbol id="i-mark\"""")
rep('<path d="M148 42 L196 22 L196 104 L148 84 Z" fill="#1F2BB8" opacity=".18"/>','<path d="M148 42 L196 22 L196 104 L148 84 Z" fill="url(#ht-ink)"/>')
rep('<path d="M90 92 L100 120 L110 92Z" fill="#1F2BB8" opacity=".35"/>','<path d="M88 92 L100 122 L112 92Z" fill="url(#ht-ink)"/>')
rep('<rect x="80" y="106" width="40" height="3" fill="#1F2BB8" opacity=".5"/>','<rect x="72" y="105" width="56" height="10" fill="url(#ht-ink)"/>')
rep('<path d="M100 14 V112 M65 14 V112 M135 14 V112" stroke="#1F2BB8" stroke-width="1" opacity=".35"/>','<rect x="30" y="14" width="140" height="98" fill="url(#ht-ink)" opacity=".5"/><path d="M100 14 V112 M65 14 V112 M135 14 V112" stroke="#1F2BB8" stroke-width="1.5"/>')
rep(".success{border:2px solid var(--amber);padding:clamp(20px,3vw,32px);background:var(--ink-deep)}",
""".success{display:grid;grid-template-columns:minmax(0,1fr) 9.5rem;background:var(--ink-deep);border:2px solid var(--amber)}
.success .main{padding:clamp(20px,3vw,32px)}
.stub{position:relative;background:var(--amber);color:var(--ink-text);padding:20px 16px;display:flex;flex-direction:column;justify-content:center;gap:4px;text-align:center}
.stub::before{content:"";position:absolute;left:-6px;top:0;bottom:0;width:12px;background:radial-gradient(circle at 6px 6px,var(--ink-deep) 4px,transparent 4.5px) 0 0/12px 14px repeat-y}
.stub b{font-stretch:125%;font-weight:850;font-size:2rem;line-height:1}
.stub span{font-size:.8125rem;font-weight:700;letter-spacing:.06em;text-transform:uppercase}
@media (max-width:480px){.success{grid-template-columns:1fr}.stub{flex-direction:row;justify-content:space-between;align-items:center}.stub::before{left:0;right:0;top:-6px;bottom:auto;width:auto;height:12px;background:radial-gradient(circle at 7px 6px,var(--ink-deep) 4px,transparent 4.5px) 0 0/14px 12px repeat-x}}""")
rep(""".date-block{display:grid;place-items:center;width:5.25rem;height:5.25rem;background:var(--ink);color:var(--on-ink);text-align:center;line-height:1}""",
""".date-block{position:relative;display:grid;place-items:center;width:5.25rem;height:5.25rem;background:var(--ink);color:var(--on-ink);text-align:center;line-height:1}
.date-block::after{content:"";position:absolute;right:-5px;top:0;bottom:0;width:10px;background:radial-gradient(circle at 5px 5px,var(--stock) 3.4px,transparent 3.9px) 0 1px/10px 11px repeat-y}""")
rep('role="status"><h3>You','role="status"><div class="main"><h3>You')
rep("""Doors open 15 minutes before the show.</p></div>';""","""Doors open 15 minutes before the show.</p></div>' +
        '<div class="stub" aria-hidden="true"><span>' + DAY[d.getDay()] + ' ' + d.getDate() + ' ' + MON[d.getMonth()] + '</span><b>' + sessionVal() + '</b><span>' + n + (n === 1 ? ' seat' : ' seats') + '</span></div></div>';""")
open(p,'w',encoding='utf-8').write(s); print("done")
