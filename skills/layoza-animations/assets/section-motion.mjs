// Original Layoza section choreography host. The caller authors every scene;
// this module supplies responsive ownership, scroll binding, teardown and inspectable evidence.
export function createSectionMotion({root,gsap,ScrollTrigger,scenes=[],enabled=true}={}) {
  const inert={destroy(){},snapshot:()=>({status:'static',scenes:[]})};
  if(typeof window==='undefined'||!root||!enabled)return inert;
  if(!gsap?.timeline||!ScrollTrigger?.create)throw Error('Section motion needs injected GSAP and ScrollTrigger.');
  gsap.registerPlugin(ScrollTrigger);
  const ids=new Set();for(const scene of scenes){if(!scene.id||ids.has(scene.id))throw Error('Section motion ids must be nonempty and unique.');ids.add(scene.id)}
  const records=[],previous=root.__layozaMotion;let destroyed=false;
  const snapshot=()=>({schema_version:1,status:destroyed?'destroyed':'mounted',viewport:innerWidth,scenes:records.map(r=>({id:r.spec.id,section:r.spec.section,family:r.spec.family,purpose:r.spec.purpose,kind:r.spec.kind||'scroll',variant:r.variant,start:r.trigger?.start??null,end:r.trigger?.end??null,progress:r.trigger?.progress??null,targets:(r.spec.targets||[]).map(selector=>({selector,nodes:[...root.querySelectorAll(selector)].map(e=>{const s=getComputedStyle(e);return{transform:s.transform,clipPath:s.clipPath,opacity:s.opacity,strokeDashoffset:s.strokeDashoffset}})})),observations:r.spec.measure?.()??null}))});
  root.__layozaMotion={snapshot};root.dataset.layozaMotionRoot='';
  // A scene with no resolve() still ends visible: clear whatever hides its targets (inside the media context, so it reverts with it).
  const settle=spec=>{if(spec.resolve)return spec.resolve();for(const selector of spec.targets||[])for(const el of root.querySelectorAll(selector)){const s=getComputedStyle(el),v={};if(+s.opacity<1)v.opacity=1;if(s.visibility==='hidden')v.visibility='visible';if(s.transform!=='none')v.transform='none';if(s.clipPath!=='none')v.clipPath='none';if(Object.keys(v).length)gsap.set(el,v)}};
  const media=gsap.matchMedia();
  media.add({wide:'(min-width: 1000px)',phone:'(max-width: 999px)',reduced:'(prefers-reduced-motion: reduce)'},context=>{
    records.length=0;const cleanup=[];
    if(context.conditions.reduced){for(const spec of scenes)settle(spec);return}
    const variant=context.conditions.wide?'desktop':'phone';
    // Register in document order so pins exist before later scroll measurements.
    const ordered=scenes.map(spec=>({spec,node:root.querySelector(spec.section)})).filter(r=>r.node).sort((a,b)=>a.node===b.node?0:a.node.compareDocumentPosition(b.node)&Node.DOCUMENT_POSITION_FOLLOWING?-1:1);
    for(const {spec,node}of ordered){const record={spec,node,variant,trigger:null};records.push(record);
      const builder=variant==='phone'?spec.phone:spec.desktop;
      if(spec.kind==='held'||spec.kind==='interaction'||!builder){settle(spec);continue}
      const timeline=gsap.timeline({paused:true,defaults:{ease:'none'}});
      const dispose=builder({timeline,gsap,node,root,variant});if(typeof dispose==='function')cleanup.push(dispose);
      if(timeline.duration()===0){timeline.revert();settle(spec);continue}
      const options=variant==='phone'?spec.phoneScroll:spec.desktopScroll;
      const trigger=ScrollTrigger.create({trigger:node,animation:timeline,start:'top 85%',end:'clamp(bottom 40%)',scrub:.45,invalidateOnRefresh:true,...options,onUpdate:self=>{node.dataset.layozaMotionProgress=self.progress.toFixed(4);spec.onProgress?.(self.progress,variant)}});
      record.trigger=trigger;node.dataset.layozaMotionId=spec.id;node.dataset.layozaMotionFamily=spec.family;
      cleanup.push(()=>{trigger.kill(true);timeline.revert();delete node.dataset.layozaMotionProgress;delete node.dataset.layozaMotionId;delete node.dataset.layozaMotionFamily;spec.resolve?.()});
    }
    return()=>{for(const dispose of cleanup.reverse())dispose();records.length=0};
  });
  let refreshFrame=0;
  const refresh=()=>{cancelAnimationFrame(refreshFrame);refreshFrame=requestAnimationFrame(()=>{if(!destroyed){ScrollTrigger.sort();ScrollTrigger.refresh()}})};
  const images=[...root.querySelectorAll('img')];for(const image of images)if(!image.complete){image.addEventListener('load',refresh);image.addEventListener('error',refresh)}
  document.fonts?.ready.then(refresh);refresh();
  return{snapshot,destroy(){if(destroyed)return;destroyed=true;cancelAnimationFrame(refreshFrame);media.revert();for(const image of images){image.removeEventListener('load',refresh);image.removeEventListener('error',refresh)}if(previous)root.__layozaMotion=previous;else{delete root.__layozaMotion;delete root.dataset.layozaMotionRoot}}};
}
