import {spawn} from 'node:child_process';
export async function connect(executable,cwd,{timeoutMs=60000}={}){
 const child=spawn(executable,['mcp'],{cwd,stdio:['pipe','pipe','pipe'],env:{...process.env,LAYOZA_LEDGER:'off'}});let seq=0,buffer='',error='';const pending=new Map();
 child.stderr.on('data',d=>error+=d);child.stdout.on('data',d=>{buffer+=d;let at;while((at=buffer.indexOf('\n'))>=0){const line=buffer.slice(0,at);buffer=buffer.slice(at+1);if(!line.trim())continue;const r=JSON.parse(line);const p=pending.get(r.id);if(p){pending.delete(r.id);clearTimeout(p.timer);r.error?p.reject(Error(JSON.stringify(r.error))):p.resolve(r.result)}}});
 child.on('error',e=>{for(const p of pending.values())p.reject(e)});
 const request=(method,params)=>new Promise((resolve,reject)=>{const id=++seq,timer=setTimeout(()=>{pending.delete(id);reject(Error(`${method} timeout ${error}`))},timeoutMs);pending.set(id,{resolve,reject,timer});child.stdin.write(JSON.stringify({jsonrpc:'2.0',id,method,params})+'\n')});
 await request('initialize',{protocolVersion:'2025-06-18',capabilities:{},clientInfo:{name:'layoza-design-lab',version:'1'}});child.stdin.write(JSON.stringify({jsonrpc:'2.0',method:'notifications/initialized'})+'\n');
 return {request,call:async(name,args)=>{const r=await request('tools/call',{name,arguments:args});if(r.isError)throw Error(r.content?.[0]?.text||name);const text=r.content?.[0]?.text||'';try{return JSON.parse(text)}catch{return text}},close:()=>new Promise(resolve=>{child.once('exit',resolve);child.stdin.end()})};
}
