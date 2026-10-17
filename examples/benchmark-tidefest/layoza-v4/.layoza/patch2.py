import pathlib, re
HERE = pathlib.Path(__file__).parent
p = HERE / 'src.html'; s = p.read_text(encoding='utf-8')
def rep(a, b):
    global s
    assert s.count(a) == 1, (a[:60], s.count(a)); s = s.replace(a, b)

# ---- cave: layered rock, stalactites, the Cave stage glowing inside
rep('''      <path d="M470 720C470 560 492 420 540 318C578 238 640 176 700 160C740 150 790 158 834 196C892 246 936 330 958 430C976 520 980 620 978 720" fill="none" stroke="#C8124F" stroke-width="3" opacity=".7"/>
    </svg>''', '''      <path d="M470 720C470 560 492 420 540 318C578 238 640 176 700 160C740 150 790 158 834 196C892 246 936 330 958 430C976 520 980 620 978 720" fill="none" stroke="#C8124F" stroke-width="3" opacity=".7"/>
      <defs>
        <linearGradient id="cbeam" x1="0" y1="1" x2="0" y2="0"><stop offset="0" stop-color="#FF4F9A" stop-opacity=".8"/><stop offset="1" stop-color="#FF4F9A" stop-opacity="0"/></linearGradient>
        <radialGradient id="cglow" cx=".5" cy="1" r=".7"><stop offset="0" stop-color="#E0115F" stop-opacity=".75"/><stop offset="1" stop-color="#E0115F" stop-opacity="0"/></radialGradient>
      </defs>
      <path fill-rule="evenodd" fill="#14101F" d="M470 720C470 560 492 420 540 318C578 238 640 176 700 160C740 150 790 158 834 196C892 246 936 330 958 430C976 520 980 620 978 720Z M548 720C552 590 574 480 612 400C648 324 698 276 744 270C790 266 836 306 866 372C900 448 916 560 916 720Z"/>
      <g fill="none" stroke-linecap="round">
        <path d="M500 520C530 500 548 470 560 430M512 640C540 610 560 580 566 540M940 520C912 496 900 468 890 430M952 640C926 610 910 580 904 540" stroke="#3A2A4C" stroke-width="3"/>
        <path d="M556 420C566 384 580 350 598 322M904 420C896 388 884 356 868 330" stroke="#7A1E4E" stroke-width="2" opacity=".8"/>
      </g>
      <path fill="#1B1626" d="M640 196l10 46 8-52ZM690 166l6 38 7-40ZM760 162l5 30 8-32ZM812 182l8 44 6-48ZM858 226l4 26 9-22Z"/>
      <path fill="#14101F" d="M664 292l6 30 6-34ZM818 300l5 26 7-28Z"/>
      <ellipse cx="732" cy="720" rx="210" ry="150" fill="url(#cglow)"/>
      <g transform="translate(732 648)"><polygon points="-4,0 4,0 34,-230 -34,-230" fill="url(#cbeam)" transform="rotate(-22)"/><polygon points="-4,0 4,0 34,-230 -34,-230" fill="url(#cbeam)" transform="rotate(22)"/></g>
      <rect x="652" y="632" width="160" height="22" fill="#E0115F"/>
      <rect x="652" y="632" width="160" height="22" fill="url(#pix)"/>
      <path d="M612 654h240v14H612Z" fill="#0F0B16"/>
      <path d="M716 654h32l-4-16h-24Z" fill="#0F0B16"/>
      <circle cx="732" cy="628" r="6" fill="#0F0B16"/>
      <path d="M600 720c0-14 6-20 13-20s13 6 13 20Zm34 0c0-16 7-22 14-22s14 6 14 22Zm36 0c0-14 6-20 13-20s13 6 13 20Zm34 0c0-16 7-22 14-22s14 6 14 22Zm36 0c0-14 6-20 13-20s13 6 13 20Zm34 0c0-16 7-22 14-22s14 6 14 22Zm36 0c0-14 6-20 13-20s13 6 13 20Z" fill="#0F0B16"/>
      <circle cx="619" cy="688" r="7" fill="#0F0B16"/><circle cx="655" cy="686" r="7" fill="#0F0B16"/><circle cx="689" cy="688" r="7" fill="#0F0B16"/><circle cx="725" cy="686" r="7" fill="#0F0B16"/><circle cx="759" cy="688" r="7" fill="#0F0B16"/><circle cx="795" cy="686" r="7" fill="#0F0B16"/><circle cx="829" cy="688" r="7" fill="#0F0B16"/>
    </svg>''')
rep('.cave-mouth{position:relative;height:clamp(460px,72vh,720px);', '.cave-mouth{position:relative;height:clamp(440px,64vh,640px);')
rep('.cave-title{position:relative;z-index:2;text-align:center;padding:0 24px;max-width:min(560px,62vw);margin-top:9vh}', '.cave-title{position:relative;z-index:2;text-align:center;padding:0 24px;max-width:min(560px,62vw);margin-top:-2vh}')
rep('  .cave-title{max-width:78vw;margin-top:12vh}', '  .cave-title{max-width:78vw;margin-top:-4vh}')

# ---- camping: a drawn night map from station to tide line
rep('''<section class="camp" id="camping" data-clock="02:00" data-label="Small hours" aria-labelledby="camp-h">
  <svg class="stars" viewBox="0 0 760 280" aria-hidden="true">''', '''<section class="camp" id="camping" data-clock="02:00" data-label="Small hours" aria-labelledby="camp-h">
  <svg class="map" viewBox="0 0 1440 300" preserveAspectRatio="xMidYMax slice" aria-hidden="true">
    <path d="M1236 58a26 26 0 1 0 22 40 22 22 0 1 1-22-40Z" fill="#F6E3BC"/>
    <path d="M0 230c90-26 170-30 260-20s140-22 230-26 160 18 250 12 120-24 210-22 150 20 240 14 120-14 210-8V300H0Z" fill="#0E3D4A"/>
    <g fill="#082F3A" stroke="#4E8A8E" stroke-width="2" stroke-linejoin="round">
      <path d="M90 236v-58h128v58Z"/><path d="M80 180l74-30 74 30Z"/><path d="M218 206h120v-6H218Z"/>
    </g>
    <g fill="#FFB238"><rect x="108" y="194" width="16" height="18" rx="2"/><rect x="144" y="194" width="16" height="18" rx="2"/><rect x="180" y="194" width="16" height="18" rx="2"/></g>
    <g transform="translate(232 214)"><rect width="190" height="26" rx="8" fill="#0B4650" stroke="#4E8A8E" stroke-width="2"/><g fill="#F6E3BC" opacity=".85"><rect x="14" y="7" width="20" height="9" rx="2"/><rect x="44" y="7" width="20" height="9" rx="2"/><rect x="74" y="7" width="20" height="9" rx="2"/><rect x="104" y="7" width="20" height="9" rx="2"/><rect x="134" y="7" width="20" height="9" rx="2"/></g></g>
    <path d="M200 242H460" stroke="#4E8A8E" stroke-width="3"/>
    <path class="map-road" d="M330 262C430 262 470 236 560 236S700 262 800 254 920 226 1010 230 1120 256 1190 252" fill="none" stroke="#FFB238" stroke-width="4" stroke-linecap="round" stroke-dasharray="2 12"/>
    <path d="M640 300c30-46 70-70 120-72s90 30 130 30 70-34 120-34 80 40 110 76Z" fill="#0A5262"/>
    <g fill="#F6E3BC" stroke="#082F3A" stroke-width="2" stroke-linejoin="round">
      <path d="M730 262l20-30 20 30Z"/><path d="M790 270l18-26 18 26Z"/><path d="M856 254l22-32 22 32Z"/><path d="M930 264l18-26 18 26Z"/><path d="M990 252l20-30 20 30Z"/><path d="M1052 268l16-24 16 24Z"/>
    </g>
    <g fill="#FFB238"><path d="M746 262l4-8 4 8Z"/><path d="M874 254l4-8 4 8Z"/><path d="M1006 252l4-8 4 8Z"/></g>
    <path d="M1150 300c40-22 90-36 150-40s110 2 140 8V300Z" fill="#C9A86E"/>
    <rect x="1320" y="246" width="120" height="54" fill="#071E33"/>
    <path d="M1320 246c30 6 70 6 120 0" stroke="#4E8A8E" stroke-width="2" fill="none"/>
    <g fill="#082F3A"><rect x="1236" y="214" width="66" height="8"/><rect x="1238" y="222" width="6" height="40"/><rect x="1294" y="222" width="6" height="40"/><rect x="1230" y="258" width="78" height="8"/></g>
    <polygon points="1266,214 1272,214 1300,90 1250,90" fill="url(#beamW)" opacity=".35"/>
  </svg>
  <svg class="stars" viewBox="0 0 760 280" aria-hidden="true">''')
rep('background-image:linear-gradient(180deg,var(--cave) 0,var(--sea) 140px);padding:clamp(110px,10vw,160px) 0', 'background-image:linear-gradient(180deg,var(--cave) 0,var(--sea) 140px);padding:0 0')
rep('.stars{position:absolute;top:24px;', '.map{display:block;width:100%;height:clamp(200px,22vw,320px);margin-bottom:clamp(36px,4vw,56px)}\n.stars{position:absolute;top:24px;')

# ---- dawn: lit clouds, the rig on the sand facing the sun
rep('''      <path d="M0 362c160-10 320 6 480-2s320-10 480-2 320 8 480 0V400H0Z" fill="#F7C77E"/>''', '''      <g fill="#FF9A6B" opacity=".9"><path d="M120 120c60-10 140-8 220-2s160 6 220-2c-40 16-120 22-220 22s-170-6-220-18Z"/><path d="M880 92c70-8 150-6 230 0s130 4 190-4c-50 18-120 24-210 22s-160-8-210-18Z"/></g>
      <g fill="#FFC27A" opacity=".85"><path d="M300 190c60-8 140-6 200 0s120 2 170-4c-40 14-110 20-190 18s-140-6-180-14Z"/><path d="M960 196c50-6 120-4 180 0s100 2 140-4c-30 12-90 16-160 16s-130-4-160-12Z"/></g>
      <path d="M0 362c160-10 320 6 480-2s320-10 480-2 320 8 480 0V400H0Z" fill="#F7C77E"/>
      <g fill="#2A1630"><rect x="1100" y="318" width="120" height="8"/><rect x="1104" y="326" width="8" height="36"/><rect x="1208" y="326" width="8" height="36"/><rect x="1096" y="356" width="128" height="8"/>
        <path d="M200 400c0-14 6-22 14-22s14 8 14 22Zm40 0c0-18 7-26 15-26s15 8 15 26Zm44 0c0-14 6-22 14-22s14 8 14 22Zm42 0c0-16 7-24 15-24s15 8 15 24Z"/>
        <circle cx="214" cy="370" r="8"/><circle cx="255" cy="366" r="8"/><circle cx="298" cy="370" r="8"/><circle cx="341" cy="368" r="8"/><path d="M262 372l20-34 5 3-17 34Z"/>
      </g>''')
rep('.dawn-strip{position:relative;height:clamp(300px,46vh,500px);', '.dawn-strip{position:relative;height:clamp(280px,42vh,460px);')

# ---- wristbands: one woven row; leaner empty till
s = re.sub(r'\s*<span class="band-weave b" aria-hidden="true">[^<]*</span>', '', s)
rep('.pips{display:flex;flex-wrap:wrap;gap:6px;min-height:34px;margin-top:16px}', '.pips{display:flex;flex-wrap:wrap;gap:6px;margin-top:16px}\n.pips:empty{display:none}')
rep('.till-lines{margin-top:14px;color:var(--muted);font-size:1rem;min-height:3em}', '.till-lines{margin-top:12px;color:var(--muted);font-size:1rem}')
rep('.band-weave{position:absolute;left:64px;right:0;top:6px;font-size:.6875rem;font-weight:800;font-stretch:125%;letter-spacing:.3em;opacity:.38;', '.band-weave{position:absolute;left:64px;right:0;top:7px;font-size:.75rem;font-weight:800;font-stretch:125%;letter-spacing:.2em;opacity:.38;')

# ---- hero scrub: make the change unmistakable earlier
rep("'--beam-op': .75, '--led-on': .6, duration: .5 }, 0)", "'--beam-op': 1, '--led-on': 1, duration: .45 }, 0)")
rep("--led-on:.15;--beam-op:.42;", "--led-on:.15;--beam-op:.38;")
rep(".to('.sun', { attr: { cy: 430 }, duration: .7 }, 0)", ".to('.sun', { attr: { cy: 430 }, duration: .55 }, 0)")
rep(".to(beamsR, { rotation: i => fan[i] * 1.45, duration: 1, ease: 'sine.inOut' }, 0);", ".to(beamsR, { rotation: i => fan[i] * 1.6, duration: .6, ease: 'sine.inOut' }, 0)\n        .to(beamsR, { rotation: i => fan[i] * .7, duration: .4, ease: 'sine.inOut' }, .6);")
p.write_text(s, encoding='utf-8'); print('ok')
