import fs from 'node:fs';import path from 'node:path';import {fileURLToPath} from 'node:url';import {connect} from '../design-lab/mcp-client.mjs';
const root=path.dirname(fileURLToPath(import.meta.url)),repo=path.resolve(root,'../..');const client=await connect(path.join(repo,'target/release/layoza.exe'),repo);
const destination=path.resolve(root,process.argv[2]||'arms/layoza/retrieval');
try{for(const b of JSON.parse(fs.readFileSync(path.join(root,'briefs.json')))){const output=await client.call('get_design_system',{query:b.brief,mode:'new_website',brief_context:b.context,detail:'full'});fs.mkdirSync(destination,{recursive:true});fs.writeFileSync(path.join(destination,b.id+'.json'),JSON.stringify(output,null,2));console.log(b.id);}}finally{await client.close();}
