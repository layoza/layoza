p='index.html'; s=open(p,encoding='utf-8').read()
def rep(a,b):
    global s
    assert a in s, a[:70]; s=s.replace(a,b,1)
rep(".ovp{position:relative;mix-blend-mode:multiply}\n.ovp-disc{position:absolute;z-index:-1;width:1.02em;height:1.02em;border-radius:50%;background:var(--amber);right:-.32em;top:-.06em;pointer-events:none}",
".ovp{position:relative;display:inline-block}\n.ovp::after{content:\"\";position:absolute;left:1.02em;top:.16em;width:.62em;height:.62em;border-radius:50%;background:var(--amber);mix-blend-mode:multiply;pointer-events:none}")
rep('<span class="ovp">dome.</span><span class="ovp-disc" aria-hidden="true"></span>','<span class="ovp">dome.</span>')
rep(' style="position:relative;isolation:isolate;width:max-content;max-width:100%"','')
open(p,'w',encoding='utf-8').write(s); print("ok")
