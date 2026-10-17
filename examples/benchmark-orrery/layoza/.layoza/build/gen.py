# Generates the planisphere SVG symbols and star data for index.html (dev-only)
import math, json, datetime, os
LAT = 51.45
DECMIN = -50.0
RS = 250.0
def rad(d): return d*math.pi/180
def rproj(dec): return RS*(90-dec)/(90-DECMIN)
def hm(h, m): return (h + m/60)*15

S = {}
def st(key, h, m, dec, mag, name=None): S[key]=(hm(h,m), dec, mag, name)
# Ursa Major
st('dubhe',11,3.7,61.75,1.8,'Dubhe'); st('merak',11,1.8,56.38,2.4); st('phecda',11,53.8,53.69,2.4); st('megrez',12,15.4,57.03,3.3)
st('alioth',12,54.0,55.96,1.8); st('mizar',13,23.9,54.93,2.2); st('alkaid',13,47.5,49.31,1.9)
# Ursa Minor
st('polaris',2,31.8,89.26,2.0,'Polaris'); st('kochab',14,50.7,74.16,2.1); st('pherkad',15,20.7,71.83,3.0); st('yildun',17,32.2,86.59,4.4)
st('epsumi',16,46.0,82.04,4.2); st('zetumi',15,44.1,77.79,4.3); st('etaumi',16,17.5,75.76,5.0)
# Cassiopeia
st('caph',0,9.2,59.15,2.3); st('schedar',0,40.5,56.54,2.2,'Schedar'); st('gamcas',0,56.7,60.72,2.2); st('ruchbah',1,25.8,60.24,2.7); st('segin',1,54.4,63.67,3.4)
# Cygnus
st('deneb',20,41.4,45.28,1.25,'Deneb'); st('sadr',20,22.2,40.26,2.2); st('gienah',20,46.2,33.97,2.5); st('delcyg',19,45.0,45.13,2.9); st('albireo',19,30.7,27.96,3.1)
# Lyra
st('vega',18,36.9,38.78,0.03,'Vega'); st('sheliak',18,50.1,33.36,3.5); st('sulafat',18,59.0,32.69,3.2); st('zetlyr',18,44.8,37.6,4.3)
# Aquila
st('altair',19,50.8,8.87,0.77,'Altair'); st('tarazed',19,46.3,10.61,2.7); st('alshain',19,55.3,6.41,3.7); st('zetaql',19,5.4,13.86,3.0); st('delaql',19,25.5,3.11,3.4); st('lamaql',19,6.2,-4.88,3.4)
# Orion
st('betel',5,55.2,7.41,0.5,'Betelgeuse'); st('rigel',5,14.5,-8.20,0.13,'Rigel'); st('bellatrix',5,25.1,6.35,1.6); st('mintaka',5,32.0,-0.30,2.2)
st('alnilam',5,36.2,-1.20,1.7); st('alnitak',5,40.8,-1.94,1.8); st('saiph',5,47.8,-9.67,2.1); st('meissa',5,35.1,9.93,3.4)
# Taurus
st('aldebaran',4,35.9,16.51,0.85,'Aldebaran'); st('elnath',5,26.3,28.61,1.65); st('zettau',5,37.6,21.14,3.0); st('pleiades',3,47.5,24.11,2.9)
st('the2tau',4,28.7,15.87,3.4); st('gamtau',4,19.8,15.63,3.7); st('epstau',4,28.6,19.18,3.5); st('lamtau',4,0.7,12.49,3.5)
# Gemini
st('castor',7,34.6,31.89,1.6,'Castor'); st('pollux',7,45.3,28.03,1.15,'Pollux'); st('alhena',6,37.7,16.40,1.9); st('mebsuta',6,43.9,25.13,3.0); st('tejat',6,22.9,22.51,2.9); st('wasat',7,20.1,21.98,3.5)
# Auriga
st('capella',5,16.7,46.0,0.08,'Capella'); st('menkalinan',5,59.5,44.95,1.9); st('theaur',5,59.7,37.21,2.6); st('iotaur',4,57.0,33.17,2.7); st('epsaur',5,2.0,43.82,3.0)
# Canis Major / Minor
st('sirius',6,45.1,-16.72,-1.46,'Sirius'); st('mirzam',6,22.7,-17.96,2.0); st('adhara',6,58.6,-28.97,1.5); st('wezen',7,8.4,-26.39,1.8); st('aludra',7,24.1,-29.30,2.4)
st('procyon',7,39.3,5.22,0.34,'Procyon'); st('gomeisa',7,27.2,8.29,2.9)
# Leo
st('regulus',10,8.4,11.97,1.35,'Regulus'); st('denebola',11,49.1,14.57,2.1); st('algieba',10,20.0,19.84,2.0); st('zosma',11,14.1,20.52,2.6); st('chertan',11,14.2,15.43,3.3)
st('etaleo',10,7.3,16.76,3.5); st('adhafera',10,16.7,23.42,3.4); st('rasalas',9,52.8,26.0,3.9); st('epsleo',9,45.9,23.77,3.0)
# Virgo
st('spica',13,25.2,-11.16,0.97,'Spica'); st('porrima',12,41.7,-1.45,2.7); st('vindem',13,2.2,10.96,2.8); st('delvir',12,55.6,3.40,3.4); st('zavijava',11,50.7,1.76,3.6)
# Bootes
st('arcturus',14,15.7,19.18,-0.05,'Arcturus'); st('izar',14,45.0,27.07,2.4); st('muphrid',13,54.7,18.40,2.7); st('seginus',14,32.1,38.31,3.0); st('nekkar',15,1.9,40.39,3.5); st('delboo',15,15.5,33.31,3.5); st('rhoboo',14,31.8,30.37,3.6)
# Corona Borealis
st('alphecca',15,34.7,26.71,2.2); st('betcrb',15,27.8,29.11,3.7); st('thecrb',15,32.9,31.36,4.1); st('gamcrb',15,42.7,26.30,3.8); st('delcrb',15,49.6,26.07,4.6); st('epscrb',15,57.6,26.88,4.2)
# Pegasus / Andromeda
st('markab',23,4.8,15.21,2.5); st('scheat',23,3.8,28.08,2.4); st('algenib',0,13.2,15.18,2.8); st('alpheratz',0,8.4,29.09,2.1); st('enif',21,44.2,9.88,2.4); st('homam',22,41.5,10.83,3.4); st('thepeg',22,10.2,6.20,3.5)
st('mirach',1,9.7,35.62,2.1); st('almach',2,3.9,42.33,2.1); st('deland',0,39.3,30.86,3.3)
# Perseus
st('mirfak',3,24.3,49.86,1.8); st('algol',3,8.2,40.96,2.1); st('gamper',3,4.8,53.51,2.9); st('delper',3,42.9,47.79,3.0); st('epsper',3,57.9,40.01,2.9); st('zetper',3,54.1,31.88,2.9)
# Scorpius
st('antares',16,29.4,-26.43,1.0,'Antares'); st('graffias',16,5.4,-19.81,2.6); st('dschubba',16,0.3,-22.62,2.3); st('pisco',15,58.9,-26.11,2.9); st('sigsco',16,21.2,-25.59,2.9)
st('tausco',16,35.9,-28.22,2.8); st('epssco',16,50.2,-34.29,2.3); st('musco',16,52.0,-38.05,3.0); st('shaula',17,33.6,-37.10,1.6)
# Sagittarius
st('kausaus',18,24.2,-34.38,1.8); st('kausmed',18,21.0,-29.83,2.7); st('kausbor',18,28.0,-25.42,2.8); st('nunki',18,55.3,-26.30,2.0); st('ascella',19,2.6,-29.88,2.6)
st('tausgr',19,6.9,-27.67,3.3); st('phisgr',18,45.7,-26.99,3.2); st('alnasl',18,5.8,-30.42,3.0)
# Hercules
st('zether',16,41.3,31.60,2.8); st('etaher',16,42.9,38.92,3.5); st('piher',17,15.0,36.81,3.2); st('epsher',17,0.3,30.93,3.9); st('kornephoros',16,30.2,21.49,2.8); st('rasalgethi',17,14.6,14.39,3.1)
# Draco
st('eltanin',17,56.6,51.49,2.2); st('rastaban',17,30.4,52.30,2.8); st('grumium',17,53.5,56.87,3.7); st('nudra',17,32.2,55.18,4.9); st('deldra',19,12.6,67.66,3.1)
st('epsdra',19,48.2,70.27,3.8); st('zetdra',17,8.8,65.71,3.2); st('etadra',16,24.0,61.51,2.7); st('iotdra',15,24.9,58.97,3.3); st('thuban',14,4.4,64.38,3.7); st('kapdra',12,33.5,69.79,3.9); st('lamdra',11,31.4,69.33,3.8)
# Cepheus
st('alderamin',21,18.6,62.59,2.5); st('alfirk',21,28.7,70.56,3.2); st('errai',23,39.3,77.63,3.2); st('iotcep',22,49.7,66.20,3.5); st('zetcep',22,10.9,58.20,3.4)
# Aries and singles
st('hamal',2,7.2,23.46,2.0); st('sheratan',1,54.6,20.81,2.6); st('mesarthim',1,53.5,19.29,3.9)
st('fomalhaut',22,57.6,-29.62,1.16,'Fomalhaut'); st('diphda',0,43.6,-17.99,2.0); st('menkar',3,2.3,4.09,2.5); st('alphard',9,27.6,-8.66,2.0); st('rasalhague',17,34.9,12.56,2.1); st('corcaroli',12,56.0,38.32,2.9)

L = [
 'dubhe merak phecda megrez dubhe','megrez alioth mizar alkaid',
 'polaris yildun epsumi zetumi kochab pherkad etaumi zetumi',
 'caph schedar gamcas ruchbah segin',
 'deneb sadr albireo','delcyg sadr gienah',
 'vega zetlyr sheliak sulafat zetlyr',
 'zetaql tarazed altair alshain','altair delaql lamaql',
 'betel bellatrix','meissa betel','meissa bellatrix','betel alnitak saiph rigel mintaka bellatrix','mintaka alnilam alnitak',
 'zettau aldebaran the2tau gamtau epstau elnath','gamtau lamtau',
 'castor mebsuta tejat','pollux wasat alhena','castor pollux',
 'capella menkalinan theaur elnath iotaur epsaur capella',
 'mirzam sirius wezen adhara','wezen aludra','procyon gomeisa',
 'regulus etaleo algieba adhafera rasalas epsleo','algieba zosma denebola chertan regulus','zosma chertan',
 'spica porrima zavijava','porrima delvir vindem',
 'arcturus izar delboo nekkar seginus rhoboo arcturus','arcturus muphrid',
 'thecrb betcrb alphecca gamcrb delcrb epscrb',
 'markab scheat alpheratz algenib markab','markab homam thepeg enif',
 'alpheratz deland mirach almach',
 'gamper mirfak delper epsper zetper','mirfak algol',
 'graffias dschubba pisco','dschubba sigsco antares tausco epssco musco shaula',
 'kausaus kausmed kausbor phisgr nunki tausgr ascella phisgr','kausmed alnasl kausaus','ascella kausaus',
 'zether etaher piher epsher zether','zether kornephoros',
 'eltanin rastaban nudra grumium eltanin','grumium deldra epsdra','deldra zetdra etadra iotdra thuban kapdra lamdra',
 'alderamin alfirk errai iotcep zetcep alderamin','alfirk iotcep',
 'hamal sheratan mesarthim',
]

def disc_xy(ra, dec):
    r = rproj(dec); return (-r*math.sin(rad(ra)), r*math.cos(rad(ra)))
def P(pts): return "M"+"L".join(f"{x:.1f} {y:.1f}" for x,y in pts)

out = []
out.append(f'<circle r="{rproj(0):.1f}" class="pl-eq"/>')
for d in (30,60):
    out.append(f'<circle r="{rproj(d):.1f}" class="pl-grid"/>')
grid=[]
for h in range(24):
    x1,y1 = disc_xy(h*15, 80); x2,y2 = disc_xy(h*15, DECMIN)
    grid.append(f'M{x1:.1f} {y1:.1f}L{x2:.1f} {y2:.1f}')
out.append(f'<path class="pl-grid" d="{"".join(grid)}"/>')
# Milky Way: galactic equator band
aN, dN, lN = rad(192.85948), rad(27.12825), rad(122.93192)
pts=[]
for i in range(0,361,3):
    l = rad(i)
    sd = math.cos(dN)*math.cos(lN-l)
    dec = math.degrees(math.asin(sd))
    ra = math.degrees(aN + math.atan2(math.sin(lN-l), -math.sin(dN)*math.cos(lN-l)))%360
    pts.append(disc_xy(ra, max(dec, DECMIN-15)))
out.append(f'<path class="pl-mw" d="{P(pts)}"/>')
# ecliptic
eps = rad(23.44); pts=[]
for i in range(0,361,4):
    lam = rad(i)
    dec = math.degrees(math.asin(math.sin(eps)*math.sin(lam)))
    ra = math.degrees(math.atan2(math.cos(eps)*math.sin(lam), math.cos(lam)))%360
    pts.append(disc_xy(ra,dec))
out.append(f'<path class="pl-ecl" d="{P(pts)}"/>')
segs=[P([disc_xy(*S[k][:2]) for k in line.split()]) for line in L]
out.append(f'<path class="pl-lines" d="{"".join(segs)}"/>')
dots=[]
for k,(ra,dec,mag,name) in S.items():
    x,y = disc_xy(ra,dec); r = max(1.0, 3.4-0.62*mag)
    dots.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{r:.2f}"/>')
out.append('<g class="pl-dots">'+"".join(dots)+'</g>')
for k,(ra,dec,mag,name) in S.items():
    if name and name not in ('Polaris','Castor'):
        x,y = disc_xy(ra,dec)
        out.append(f'<text class="pl-name" x="{x+5:.1f}" y="{y-4:.1f}">{name}</text>')
stars_svg = "".join(out)

def gmst0(d):
    j2000 = datetime.datetime(2000,1,1,12)
    days = (datetime.datetime(d.year,d.month,d.day) - j2000).total_seconds()/86400
    return (280.46061837 + 360.98564736629*days) % 360
ticks=[]; labels=[]
d0 = datetime.date(2026,1,1)
for i in range(365):
    d = d0+datetime.timedelta(days=i); ra = gmst0(d)
    ln = 13 if d.day==1 else (8 if d.day in (10,20) else (6 if d.day%5==0 else 3.5))
    r1=257; r2=r1+ln
    s,c = math.sin(rad(ra)), math.cos(rad(ra))
    ticks.append(f'M{-r1*s:.1f} {r1*c:.1f}L{-r2*s:.1f} {r2*c:.1f}')
    if d.day==15:
        r=286; x,y=-r*s, r*c; th=math.degrees(math.atan2(y,x))
        labels.append(f'<text transform="translate({x:.1f} {y:.1f}) rotate({th+90:.1f})" dy="0.35em">{d.strftime("%B").upper()}</text>')
    if d.day in (10,20):
        r=272; x,y=-r*s, r*c; th=math.degrees(math.atan2(y,x))
        labels.append(f'<text class="pl-dnum" transform="translate({x:.1f} {y:.1f}) rotate({th+90:.1f})" dy="0.35em">{d.day}</text>')
datering = (f'<circle r="300" class="pl-dateband"/><circle r="299" class="pl-rim"/><circle r="256.5" class="pl-rim"/>'
            f'<path class="pl-ticks" d="{"".join(ticks)}"/><g class="pl-months">{"".join(labels)}</g>')
disc_symbol = (f'<symbol id="pl-disc" viewBox="-310 -310 620 620">'
   f'{datering}<circle r="{RS+3:.0f}" class="pl-sky"/>'
   f'<g class="pl-stars">{stars_svg}</g></symbol>')

win=[]
for A in range(0,360,3):
    a=rad(A); n=math.cos(a); e=math.sin(a)
    dec=math.degrees(math.asin(n*math.cos(rad(LAT))))
    H = math.atan2(-e, -n*math.sin(rad(LAT))); r=rproj(dec)
    win.append((r*math.sin(H), r*math.cos(H)))
winpath = P(win)+"Z"
R0=RS+3.5
over_path = f'M{-R0} 0A{R0} {R0} 0 1 0 {R0} 0A{R0} {R0} 0 1 0 {-R0} 0Z' + winpath
hl=[]; ht=[]
for q in range(96):
    t = q/4; H = rad(t*15); r1=R0-1; ln = 7 if q%4==0 else 3
    ht.append(f'M{r1*math.sin(H):.1f} {r1*math.cos(H):.1f}L{(r1-ln)*math.sin(H):.1f} {(r1-ln)*math.cos(H):.1f}')
    if q%4==0:
        h=int(t); r=R0-15; x,y=r*math.sin(H), r*math.cos(H); th=math.degrees(math.atan2(y,x))
        rot = th-90 if y>1 else th+90
        hl.append(f'<text transform="translate({x:.1f} {y:.1f}) rotate({rot:.1f})" dy="0.35em">{h:02d}</text>')
def hor(A, alt=0):
    a=rad(A); n=math.cos(a)*math.cos(rad(alt)); e=math.sin(a)*math.cos(rad(alt)); u=math.sin(rad(alt))
    dec=math.degrees(math.asin(n*math.cos(rad(LAT))+u*math.sin(rad(LAT))))
    H=math.atan2(-e, -n*math.sin(rad(LAT))+u*math.cos(rad(LAT))); r=rproj(dec)
    return r*math.sin(H), r*math.cos(H)
cx = sum(p[0] for p in win)/len(win); cy=sum(p[1] for p in win)/len(win)
comp=[]
for A,lab in ((0,'N'),(90,'E'),(180,'S'),(270,'W')):
    x,y=hor(A); dx,dy=x-cx,y-cy; Ln=math.hypot(dx,dy) or 1
    off = 9 if A!=180 else 0
    if A==180: y -= 10
    comp.append(f'<text x="{x+dx/Ln*off:.1f}" y="{y+dy/Ln*off:.1f}" dy="0.35em">{lab}</text>')
zx,zy = hor(0,90)
overlay_symbol = (f'<symbol id="pl-over" viewBox="-310 -310 620 620">'
  f'<path class="pl-card" fill-rule="evenodd" d="{over_path}"/>'
  f'<path class="pl-winedge" d="{winpath}"/>'
  f'<circle r="{R0:.1f}" class="pl-cardedge"/>'
  f'<path class="pl-hticks" d="{"".join(ht)}"/><g class="pl-hours">{"".join(hl)}</g>'
  f'<g class="pl-compass">{"".join(comp)}</g>'
  f'<path class="pl-zen" d="M{zx-5:.1f} {zy:.1f}h10M{zx:.1f} {zy-5:.1f}v10"/>'
  f'<circle r="7" class="pl-rivet"/><circle r="2.2" class="pl-rivet-in"/>'
  f'</symbol>')

named = [[n, round(ra,2), dec, mag] for k,(ra,dec,mag,n) in S.items() if n]
def moon_age(dt):
    ref = datetime.datetime(2000,1,6,18,14)
    return ((dt-ref).total_seconds()/86400) % 29.530588853
phases = {d: round(moon_age(datetime.datetime(2026,10,d,21)),1) for d in (16,23,30)}
here = os.path.dirname(os.path.abspath(__file__))
json.dump({'disc':disc_symbol,'over':overlay_symbol,'named':named,'phases':phases}, open(os.path.join(here,'gen.json'),'w'))
print(len(disc_symbol), len(overlay_symbol), phases, 'centroid', round(cx,1), round(cy,1), 'zenith', round(zx,1), round(zy,1))
