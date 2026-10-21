// Original Layoza scroll-state evidence, on Layoza's own CDP client (scripts/lib/cdp.mjs).
import fs from 'node:fs';import path from 'node:path';
import {Browser,sleep} from './lib/cdp.mjs';
const args=process.argv.slice(2),url=args.shift(),opts={};for(let i=0;i<args.length;i+=2)opts[args[i].replace(/^--/,'')]=args[i+1];
if(!/^https?:\/\//.test(url||''))throw Error('Usage: node scripts/review-motion.mjs <URL> --out <directory>');
const out=path.resolve(opts.out||'.layoza/motion-review');fs.mkdirSync(out,{recursive:true});
const selected=opts.scenes?new Set(opts.scenes.split(',')):null;
const report={schema_version:1,status:'capturing',url,scope:selected?[...selected]:'all registered scenes',method:'Actual normal-motion viewport samples at 0/25/50/75/100 percent of each registered trigger, with native wheel input. Actual progress and scroll position are retained. No full-section capture or viewport resize is performed: those can alter pin and scroll states while rendering. Entry/exit frames may show neighboring content, exactly as a visitor sees it. Target properties separate authored transformation from ordinary page movement. This is evidence, not a beauty score.',viewports:[],images:[],observations:[]};
const browser=await Browser.launch();
try{for(const width of[1440,390]){const page=await browser.newPage({width,height:900,reducedMotion:false});
try{await page.goto(url,{settleMs:1500});await page.eval(()=>{document.querySelector('.intro-screen button')?.click();return document.fonts.ready.then(()=>true)});await sleep(900);
// Warm lazy media in normal document order before measuring trigger locations.
await page.eval(()=>{document.documentElement.style.scrollBehavior='auto';document.querySelectorAll('img[loading]').forEach(i=>i.loading='eager')});
for(let n=0;n<3;n++){await page.eval(n=>scrollTo(0,n*innerHeight),n);await sleep(100)}await page.eval(()=>scrollTo(0,0));await sleep(700);
const initial=await page.eval(()=>document.querySelector('.site')?.__layozaMotion?.snapshot()||null);
const viewport={width,status:initial?'instrumented':'uninstrumented',scenes:[]};report.viewports.push(viewport);if(!initial){report.observations.push(`${width}: no section inspector; no motion pass can be claimed.`);continue}
for(const scene of initial.scenes.filter(s=>!selected||selected.has(s.id))){const entry={id:scene.id,family:scene.family,kind:scene.kind,variant:scene.variant,samples:[],changed:null};viewport.scenes.push(entry);if(scene.kind!=='scroll'||scene.start===null)continue;
const safe=scene.id.replace(/[^a-z0-9_-]/gi,'_');for(const progress of[0,.25,.5,.75,1]){
await page.eval(({id,progress})=>{const scene=document.querySelector('.site').__layozaMotion.snapshot().scenes.find(s=>s.id===id);scrollTo({top:scene.start+(scene.end-scene.start)*progress,behavior:'instant'})},{id:scene.id,progress});await page.send('Input.dispatchMouseEvent',{type:'mouseWheel',x:width/2,y:450,deltaX:0,deltaY:1});await sleep(700);
const state=await page.eval(id=>({scrollY,snapshot:document.querySelector('.site').__layozaMotion.snapshot().scenes.find(s=>s.id===id)}),scene.id);
const file=`${safe}-${width}-${progress*100}.png`,viewportFile=`${safe}-${width}-${progress*100}-viewport.png`;const shot=await page.send('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});fs.writeFileSync(path.join(out,viewportFile),Buffer.from(shot.data,'base64'));
fs.copyFileSync(path.join(out,viewportFile),path.join(out,file));
report.images.push({viewport:width,scene:scene.id,progress,path:file,viewport_path:viewportFile});entry.samples.push({requested_progress:progress,...state,image:file,viewport_image:viewportFile});
}entry.changed=new Set(entry.samples.map(s=>JSON.stringify({targets:s.snapshot?.targets,observations:s.snapshot?.observations}))).size>1;if(!entry.changed)report.observations.push(`${scene.id}/${width}: no sampled target-property change; inspect whether this is intentional, unmeasured or broken.`);
}
}finally{await page.close()}}
report.status=report.viewports.length===2&&report.viewports.every(v=>v.status==='instrumented'&&v.scenes.some(s=>s.samples.length===5)&&v.scenes.filter(s=>s.kind==='scroll').every(s=>s.samples.length===5))?'captured':'incomplete';
}finally{await browser.close();fs.writeFileSync(path.join(out,'report.json'),JSON.stringify(report,null,2))}
const esc=s=>String(s).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('"','&quot;');
fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Section motion evidence</title><style>body{background:#101114;color:#eee;font:16px/1.5 system-ui;margin:24px}section{margin:50px 0}.frames{display:flex;gap:12px;overflow:auto}figure{margin:0;min-width:260px;width:28vw}img{width:100%;display:block}p{max-width:85ch}a{color:#c4ff60}</style><h1>Section motion: actual scroll states</h1><p>${esc(report.method)}</p><a href="${esc(url)}">Open working website</a>${report.observations.map(s=>`<p>${esc(s)}</p>`).join('')}${report.viewports.map(v=>v.scenes.filter(s=>s.samples.length).map(s=>`<section><h2>${esc(s.id)} / ${v.width} / ${esc(s.family)}</h2><p>Measured target change: ${s.changed}. Inspect playback and readable resolved state.</p><div class="frames">${s.samples.map(f=>`<figure><figcaption>${f.requested_progress*100}%</figcaption><a href="${f.image}"><img loading="lazy" src="${f.image}" alt="${esc(s.id)} at ${f.requested_progress*100}% scroll"></a></figure>`).join('')}</div></section>`).join('')).join('')}</html>`);
console.log(JSON.stringify({status:report.status,frames:report.images.length,report:path.join(out,'report.json')}));if(report.status!=='captured')process.exitCode=2;
