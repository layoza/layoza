import fs from 'node:fs';import path from 'node:path';import assert from 'node:assert/strict';import {connect} from './mcp-client.mjs';
const lab=path.dirname(new URL(import.meta.url).pathname.replace(/^\/([A-Za-z]:)/,'$1'));
const repo=path.resolve(lab,'../..'),exe=path.resolve(process.argv[2]||path.join(repo,'target/release/layoza.exe')),cases=JSON.parse(fs.readFileSync(path.join(lab,'cases.json'))),sources=JSON.parse(fs.readFileSync(path.join(lab,'asset-sources.json')));
const results=[];
for(const c of cases){const root=path.join(lab,c.id),client=await connect(exe,root);try{
 const list=await client.request('tools/list',{});assert.equal(list.tools.length,24);
 const deal=await client.call('get_design_system',{query:c.brief,mode:'new_website',brief_context:c.context,detail:'full',ledger:false});
 fs.writeFileSync(path.join(root,'.layoza/mcp-deal.json'),JSON.stringify(deal,null,2));
 assert.equal(deal.understood_brief.subject,c.context.subject);assert.equal(deal.directions.length,3);
 if(c.id==='intervals'){for(const d of deal.directions){assert.equal(d.typography.display_font,'Helvetica');assert.equal(d.typography.body_font,'Helvetica');assert.equal(d.dials.motion_intensity,0)}}
 if(c.id==='noor')for(const d of deal.directions)assert.equal(d.dials.motion_intensity,0);
 if(c.id==='willow')assert(!JSON.stringify(deal.directions.map(d=>d.content_strategy)).includes('install command'));
 for(const file of c.assets){const a=sources.find(a=>a.file===file);await client.call('design_assets',{action:'register',asset:{path:`assets/${file}`,source:a.source_page,license:a.license,credit:a.creator,role:a.usage_note,alt:a.title},write_approved:true})}
 const inventory=await client.call('design_assets',{action:'inventory'});assert.equal(inventory.assets.length,c.assets.length);
 results.push({brief:c.id,subject:deal.understood_brief.subject,directions:3,pinned_choices:'passed',registered_assets:inventory.assets.length,tools:list.tools.length});
 }finally{await client.close()}}
fs.writeFileSync(path.join(lab,'engine-results.json'),JSON.stringify(results,null,2));console.log(JSON.stringify(results,null,2));
