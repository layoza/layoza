p='index.html'; s=open(p,encoding='utf-8').read()
def rep(a,b):
    global s
    assert a in s, a[:70]; s=s.replace(a,b,1)
rep('\n.ovp{position:relative;display:inline-block}\n.ovp::after{content:"";position:absolute;left:1.02em;top:.16em;width:.62em;height:.62em;border-radius:50%;background:var(--amber);mix-blend-mode:multiply;pointer-events:none}','')
rep('<span class="ovp">dome.</span>','dome.')
rep(".seat{transition:fill .35s var(--ease-out),r .35s var(--ease-out)}",".seat{transition:fill .35s var(--ease-out),r .35s var(--ease-out)}\n.seat-ovp{mix-blend-mode:multiply;transition:opacity .35s var(--ease-out)}")
rep("""    var seats = seatPositions().map(function(p, i){
      return el("circle", {"class":"seat", cx:p.x.toFixed(1), cy:p.y.toFixed(1), r:7.5, fill:"#F1F3F6", "data-i":i}, seatsG);
    });""","""    var seats = seatPositions().map(function(p, i){
      var c = el("circle", {"class":"seat", cx:p.x.toFixed(1), cy:p.y.toFixed(1), r:7.5, fill:"#F1F3F6", "data-i":i}, seatsG);
      // amber overprint ring: shown only on held seats, multiplied into the ultramarine ring
      c._ovp = el("circle", {"class":"seat-ovp", cx:(p.x + 2).toFixed(1), cy:(p.y + 2).toFixed(1), r:13, fill:"#FFB000", opacity:0}, seatsG);
      seatsG.insertBefore(c._ovp, c);
      return c;
    });""")
rep("""      s.setAttribute("fill", fill); s.setAttribute("r", rr);
    });
    var sl""","""      s.setAttribute("fill", fill); s.setAttribute("r", rr);
      if (s._ovp) s._ovp.setAttribute("opacity", mine[i] && !(taken && taken[i]) ? 1 : 0);
    });
    var sl""")
open(p,'w',encoding='utf-8').write(s); print("ok")
