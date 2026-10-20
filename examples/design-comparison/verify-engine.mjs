import fs from 'node:fs';import path from 'node:path';import assert from 'node:assert/strict';import {fileURLToPath} from 'node:url';
const root=path.dirname(fileURLToPath(import.meta.url)),checks=[];
for(const id of ['intervals','willow','noor','hearth']){const x=JSON.parse(fs.readFileSync(path.join(root,'engine-after',id+'.json')));const joined=x.directions.map(d=>d.layout_composition).join('\n');
if(['intervals','noor'].includes(id)){assert.equal(x.active_dials.motion_intensity,0);assert(!x.directions.some(d=>d.hero_material==='video'));assert(x.craft_floor.some(s=>s.startsWith('Motion: none.')));assert(!x.craft_floor.some(s=>s.includes('one signature moment')));}
if(id==='intervals'){assert(!joined.includes('plinth'));assert(!joined.includes('timed ticket'));assert(x.craft_floor.some(s=>s.includes('preserve')||s.includes('preserve'.replace('p','P'))));}
if(id==='willow'){assert(!joined.includes('Booking-first'));assert(joined.includes('search the catalogue'));assert(joined.includes('books'));}
if(id==='noor'){assert(!joined.includes('one real quote'));assert(!joined.includes('week grid'));}
if(id==='hearth'){assert(!joined.includes('generated code'));assert(x.craft_floor.some(s=>s.includes('working interface')));}
checks.push({id,pass:true});}
fs.writeFileSync(path.join(root,'engine-fix-checks.json'),JSON.stringify({checks,scope:'Actual updated MCP get_design_system outputs, same four frozen brief contexts; not a visual quality score.'},null,2));console.log('Updated MCP: client task, assets/evidence constraints and contextual craft checks passed');
