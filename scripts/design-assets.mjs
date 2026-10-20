#!/usr/bin/env node
// Asset acquisition with explicit provenance. No API key, scraping or fabricated rights.
import fs from 'node:fs/promises';
import path from 'node:path';
import crypto from 'node:crypto';
const [action,...args]=process.argv.slice(2);
const fail=s=>{throw new Error(s)};
const safe=(root,p)=>{const out=path.resolve(root,p),rel=path.relative(root,out);if(!rel||rel.startsWith('..')||path.isAbsolute(rel))fail('File must be inside the project');return out};
async function inside(root,file){let ancestor=file;while(true){try{await fs.lstat(ancestor);break}catch(e){if(e.code!=='ENOENT')throw e;const parent=path.dirname(ancestor);if(parent===ancestor)throw e;ancestor=parent}}const resolved=await fs.realpath(ancestor),rel=path.relative(root,resolved);if(rel.startsWith('..')||path.isAbsolute(rel))fail('A symlink escapes the project');}
try {
 if(action==='search') {
  const q=args.join(' ').trim();if(!q)fail('search <subject>');
  const u=new URL('https://openaccess-api.clevelandart.org/api/artworks/');u.searchParams.set('q',q);u.searchParams.set('cc0','1');u.searchParams.set('has_image','1');u.searchParams.set('limit','8');
  const res=await fetch(u,{signal:AbortSignal.timeout(20000)});if(!res.ok)fail(`Collection search: ${res.status}`);
  const data=await res.json();console.log(JSON.stringify({provider:'Cleveland Museum of Art primary collection API',query:q,assets:data.data.map(a=>({title:a.title,source:a.url,image:a.images?.web?.url,license:a.share_license_status,creator:a.creators?.map(c=>c.description).join('; ')})),notice:'Inspect each image and its rights. Historical collection objects are not photographs of a client or its products.'},null,2));
 } else if(action==='import') {
  const [rootArg,sourceFile,recordFile]=args;if(!recordFile)fail('import <project> <local-image> <provenance.json>');
  const root=await fs.realpath(rootArg),record=JSON.parse(await fs.readFile(recordFile,'utf8'));
  for(const k of ['path','source','license','credit','role','alt'])if(typeof record[k]!=='string'||!record[k].trim())fail(`Missing ${k}`);
  const dest=safe(root,record.path),manifest=path.join(root,'.layoza/assets.json');await inside(root,dest);await inside(root,manifest);const bytes=await fs.readFile(sourceFile);
  if(!/\.(png|jpe?g|webp|avif|svg)$/i.test(dest))fail('Use an image extension');
  await fs.mkdir(path.dirname(dest),{recursive:true});await fs.writeFile(dest,bytes,{flag:'wx'});
  record.sha256=crypto.createHash('sha256').update(bytes).digest('hex');record.bytes=bytes.length;
  await fs.mkdir(path.dirname(manifest),{recursive:true});
  let records=[];try{records=JSON.parse(await fs.readFile(manifest,'utf8'))}catch(e){if(e.code!=='ENOENT')throw e}
  records=records.filter(a=>a.path!==record.path);records.push(record);await fs.writeFile(manifest,JSON.stringify(records,null,2)+'\n');
  console.log(JSON.stringify(record,null,2));
 } else fail('Usage: design-assets.mjs search <subject> | import <project> <local-image> <provenance.json>');
}catch(e){console.error(e.message);process.exitCode=1}
