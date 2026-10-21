// Verify the actual packaged stdio transport and the bytes of installed skills.
// This proves this invocation, not discovery by a different or already-running host.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawn} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const walk=p=>fs.readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(path.join(p,e.name)):[path.join(p,e.name)]);
const source=path.join(root,'skills');
const installed=path.resolve(process.argv[2]||path.join(root,'.agents/skills'));
const files=walk(source).map(p=>{const rel=path.relative(source,p),dest=path.join(installed,rel);return {file:rel,source:sha(p),installed:fs.existsSync(dest)?sha(dest):null}});
const child=spawn(process.execPath,[path.join(root,'bin/layoza-mcp.mjs')],{cwd:root,stdio:['pipe','pipe','pipe']});
let seq=0,buffer='',stderr='';const pending=new Map();
child.stderr.on('data',d=>stderr+=d);
child.stdout.on('data',d=>{buffer+=d;let n;while((n=buffer.indexOf('\n'))>=0){const line=buffer.slice(0,n);buffer=buffer.slice(n+1);if(!line.trim())continue;const r=JSON.parse(line),p=pending.get(r.id);if(p){clearTimeout(p.timer);pending.delete(r.id);r.error?p.reject(Error(JSON.stringify(r.error))):p.resolve(r.result)}}});
const request=(method,params={})=>new Promise((resolve,reject)=>{const id=++seq,timer=setTimeout(()=>reject(Error(`MCP timeout: ${stderr}`)),30000);pending.set(id,{resolve,reject,timer});child.stdin.write(JSON.stringify({jsonrpc:'2.0',id,method,params})+'\n')});
try{
 const initialized=await request('initialize',{protocolVersion:'2025-06-18',capabilities:{},clientInfo:{name:'layoza-activation-verifier',version:'1'}});
 child.stdin.write(JSON.stringify({jsonrpc:'2.0',method:'notifications/initialized'})+'\n');
 const listed=await request('tools/list');
 const prompt=await request('prompts/get',{name:'art-direct',arguments:{brief:'Architecture portfolio with exact content. '.repeat(140)}});
 const tools=listed.tools.map(t=>t.name),current=files.every(f=>f.source===f.installed);
 console.log(JSON.stringify({time:new Date().toISOString(),source,installed,skillsCurrent:current,files,transport:'packaged Node wrapper -> native stdio MCP',server:initialized.serverInfo,tools,longBriefAccepted:!!prompt.messages,hostDiscovery:'Not inferred. Explicitly read the installed skills in the invoking agent; refresh other hosts after installation.'},null,2));
 process.exitCode=current&&tools.includes('get_design_system')?0:1;
}finally{child.stdin.end();}
