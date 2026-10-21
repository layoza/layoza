// MIT. Optional disclosure-header controller. Brand styling remains caller-owned.
// Markup: data-header-root contains menu-toggle/mobile-menu test hooks, a
// .menu-label, .desktop-motion-slot, .mobile-motion-slot and motion-toggle hook.
// Keep the supplied breakpoint synchronized with the component's actual CSS.
export function mountResponsiveHeader(root,{breakpoint=900,labels={open:'Menu',close:'Close'}}={}){
 const doc=root.ownerDocument,abort=new AbortController(),signal=abort.signal;
 const button=root.querySelector('[data-testid=menu-toggle]'),menu=root.querySelector('[data-testid=mobile-menu]'),utility=root.querySelector('[data-testid=motion-toggle]');
 const regions=[...doc.querySelectorAll('main,body > footer')],priorInert=new Map(regions.map(el=>[el,el.inert]));
 const media=matchMedia(`(max-width:${breakpoint}px)`);
 function setMenu(open,focus=false){menu.hidden=!open;button.setAttribute('aria-expanded',String(open));button.querySelector('.menu-label').textContent=open?labels.close:labels.open;doc.documentElement.dataset.menuOpen=String(open);regions.forEach(el=>el.inert=open||priorInert.get(el));if(focus)button.focus();}
 button.addEventListener('click',()=>setMenu(menu.hidden),{signal});
 menu.addEventListener('click',e=>{if(e.target.closest('a'))setMenu(false);},{signal});
 doc.addEventListener('keydown',e=>{if(menu.hidden)return;if(e.key==='Escape')setMenu(false,true);if(e.key==='Tab'){const targets=[...root.querySelectorAll('a,button')].filter(el=>!el.disabled&&el.getBoundingClientRect().width>0),first=targets[0],last=targets.at(-1);if(e.shiftKey&&doc.activeElement===first){e.preventDefault();last.focus();}else if(!e.shiftKey&&doc.activeElement===last){e.preventDefault();first.focus();}}},{signal});
 function arrange(){setMenu(false);root.querySelector(media.matches?'.mobile-motion-slot':'.desktop-motion-slot').append(utility);}
 media.addEventListener('change',arrange,{signal});arrange();
 return{close:()=>setMenu(false),dispose(){setMenu(false);abort.abort();}};
}
