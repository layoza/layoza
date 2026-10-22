// Original Layoza scroll-state evidence, on Layoza's own CDP client (scripts/lib/cdp.mjs).
import fs from 'node:fs';import path from 'node:path';
import {Browser,sleep} from './lib/cdp.mjs';
// A bare second argument is the output directory, as in review-composition.mjs; --out still works.
// --click <selector> clicks one element after load (for example an intro or consent screen); nothing site-specific is clicked otherwise.
const args=process.argv.slice(2),url=args.shift(),opts={};if(args[0]&&!args[0].startsWith('--'))opts.out=args.shift();for(let i=0;i<args.length;i+=2)opts[args[i].replace(/^--/,'')]=args[i+1];
if(!/^https?:\/\//.test(url||''))throw Error('Usage: node scripts/review-motion.mjs <URL> [--out <directory>] [--scenes <id,id>] [--click <selector>]');
const out=path.resolve(opts.out||'.layoza/motion-review');fs.mkdirSync(out,{recursive:true});
const selected=opts.scenes?new Set(opts.scenes.split(',')):null;
const report={schema_version:1,status:'capturing',url,scope:selected?[...selected]:'all registered scenes',method:'Actual normal-motion viewport samples at 0/25/50/75/100 percent of each registered trigger, with native wheel input. Requested and actual progress and scroll position are retained; a scene whose final sample does not reach progress 0.99 is marked stuck. One reduced-motion viewport still per width shows the resolved state. No full-section capture or viewport resize is performed: those can alter pin and scroll states while rendering. Entry/exit frames may show neighboring content, exactly as a visitor sees it. Target properties separate authored transformation from ordinary page movement. This is evidence, not a beauty score.',viewports:[],images:[],observations:[]};
// The motion root is whichever element carries __layozaMotion: the marked root first, then a generic scan.
const findRoot=()=>{window.__lzRoot=()=>[...document.querySelectorAll('[data-layoza-motion-root]'),document.documentElement,document.body,...document.querySelectorAll('body *')].find(el=>el?.__layozaMotion)||null;return true};
const open=async(page,width)=>{await page.goto(url,{settleMs:1500});await page.eval(findRoot);
if(opts.click&&!await page.eval(sel=>{const el=document.querySelector(sel);el?.click();return!!el},opts.click))report.observations.push(`${width}: --click ${opts.click} matched nothing.`);
await page.eval(()=>document.fonts.ready.then(()=>true));await sleep(900);
// Warm lazy media in normal document order before measuring trigger locations.
await page.eval(()=>{document.documentElement.style.scrollBehavior='auto';document.querySelectorAll('img[loading]').forEach(i=>i.loading='eager')});
for(let n=0;n<3;n++){await page.eval(n=>scrollTo(0,n*innerHeight),n);await sleep(100)}await page.eval(()=>scrollTo(0,0));await sleep(700)};
let reducedFailed=false;
const browser=await Browser.launch();
try{for(const width of[1440,390]){const page=await browser.newPage({width,height:900,reducedMotion:false});
try{await open(page,width);
const initial=await page.eval(()=>__lzRoot()?.__layozaMotion.snapshot()||null);
const viewport={width,status:initial?'instrumented':'uninstrumented',scenes:[],reduced_still:null};report.viewports.push(viewport);if(!initial){report.observations.push(`${width}: no section inspector; no motion pass can be claimed.`);continue}
for(const scene of initial.scenes.filter(s=>!selected||selected.has(s.id))){const entry={id:scene.id,family:scene.family,kind:scene.kind,variant:scene.variant,samples:[],changed:null,stuck:false};viewport.scenes.push(entry);if(scene.kind==='scroll'&&scene.start===null)report.observations.push(`${scene.id}/${width}: scroll scene has no trigger (empty timeline or missing section); nothing to sample.`);if(scene.kind!=='scroll'||scene.start===null)continue;
const safe=scene.id.replace(/[^a-z0-9_-]/gi,'_');for(const progress of[0,.25,.5,.75,1]){
await page.eval(({id,progress})=>{const scene=__lzRoot().__layozaMotion.snapshot().scenes.find(s=>s.id===id);scrollTo({top:scene.start+(scene.end-scene.start)*progress,behavior:'instant'})},{id:scene.id,progress});await page.send('Input.dispatchMouseEvent',{type:'mouseWheel',x:width/2,y:450,deltaX:0,deltaY:1});await sleep(700);
const state=await page.eval(id=>({scrollY,snapshot:__lzRoot()?.__layozaMotion.snapshot().scenes.find(s=>s.id===id)??null}),scene.id);
const actual=typeof state.snapshot?.progress==='number'?state.snapshot.progress:null;
const file=`${safe}-${width}-${progress*100}.png`,viewportFile=`${safe}-${width}-${progress*100}-viewport.png`;const shot=await page.send('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});fs.writeFileSync(path.join(out,viewportFile),Buffer.from(shot.data,'base64'));
fs.copyFileSync(path.join(out,viewportFile),path.join(out,file));
report.images.push({viewport:width,scene:scene.id,progress,actual_progress:actual,path:file,viewport_path:viewportFile});entry.samples.push({requested_progress:progress,actual_progress:actual,progress_delta:actual===null?null:+(actual-progress).toFixed(4),...state,image:file,viewport_image:viewportFile});
}entry.changed=new Set(entry.samples.map(s=>JSON.stringify({targets:s.snapshot?.targets,observations:s.snapshot?.observations}))).size>1;if(!entry.changed)report.observations.push(`${scene.id}/${width}: no sampled target-property change; inspect whether this is intentional, unmeasured or broken.`);
const last=entry.samples.at(-1);if(!(last?.actual_progress>=.99)){entry.stuck=true;report.observations.push(`${scene.id}/${width}: stuck; requested progress 1 reached ${last?.actual_progress??'nothing'} (end ${scene.end}, scroll ${last?.scrollY}). The trigger cannot finish on this page.`)}
}
}finally{await page.close()}
// One reduced-motion still per width: the resolved state a visitor with reduced motion sees first.
const still=await browser.newPage({width,height:900,reducedMotion:true});const reducedFile=`reduced-${width}.png`;
try{await open(still,width);const shot=await still.send('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});fs.writeFileSync(path.join(out,reducedFile),Buffer.from(shot.data,'base64'));report.images.push({viewport:width,reduced_motion:true,path:reducedFile});const v=report.viewports.find(v=>v.width===width);if(v)v.reduced_still=reducedFile}
catch(e){reducedFailed=true;report.observations.push(`${width}: reduced-motion still failed: ${e.message}`)}finally{await still.close()}}
report.status=!reducedFailed&&report.viewports.length===2&&report.viewports.every(v=>v.status==='instrumented'&&v.reduced_still&&v.scenes.some(s=>s.samples.length===5)&&v.scenes.filter(s=>s.kind==='scroll').every(s=>s.samples.length===5&&!s.stuck))?'captured':'incomplete';
}finally{await browser.close();fs.writeFileSync(path.join(out,'report.json'),JSON.stringify(report,null,2))}
const esc=s=>String(s).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('"','&quot;');
fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Section motion evidence</title><style>body{background:#101114;color:#eee;font:16px/1.5 system-ui;margin:24px}section{margin:50px 0}.frames{display:flex;gap:12px;overflow:auto}figure{margin:0;min-width:260px;width:28vw}img{width:100%;display:block}p{max-width:85ch}a{color:#c4ff60}.stuck{color:#ff8a7a}</style><h1>Section motion: actual scroll states</h1><p>${esc(report.method)}</p><a href="${esc(url)}">Open working website</a>${report.observations.map(s=>`<p>${esc(s)}</p>`).join('')}${report.viewports.map(v=>(v.reduced_still?`<section><h2>Reduced motion / ${v.width}</h2><div class="frames"><figure><a href="${v.reduced_still}"><img loading="lazy" src="${v.reduced_still}" alt="Reduced-motion still at ${v.width}px"></a></figure></div></section>`:'')+v.scenes.filter(s=>s.samples.length).map(s=>`<section><h2>${esc(s.id)} / ${v.width} / ${esc(s.family)}</h2><p>Measured target change: ${s.changed}.${s.stuck?' <strong class="stuck">Stuck: final progress below 0.99.</strong>':''} Inspect playback and readable resolved state.</p><div class="frames">${s.samples.map(f=>`<figure><figcaption>${f.requested_progress*100}% requested, ${f.actual_progress===null?'n/a':(f.actual_progress*100).toFixed(1)+'%'} actual</figcaption><a href="${f.image}"><img loading="lazy" src="${f.image}" alt="${esc(s.id)} at ${f.requested_progress*100}% scroll"></a></figure>`).join('')}</div></section>`).join('')).join('')}</html>`);
console.log(JSON.stringify({status:report.status,frames:report.images.length,report:path.join(out,'report.json')}));if(report.status!=='captured')process.exitCode=2;
