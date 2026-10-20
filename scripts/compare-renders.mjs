#!/usr/bin/env node
// Actual decoded PNG pixels. No screenshot editing or subjective design verdict.
import fs from 'node:fs';
import zlib from 'node:zlib';
function png(file){
 const b=fs.readFileSync(file);if(!b.subarray(0,8).equals(Buffer.from([137,80,78,71,13,10,26,10])))throw Error('Expected a PNG screenshot');
 let w,h,type,depth,interlace,parts=[];
 for(let i=8;i<b.length;){const len=b.readUInt32BE(i),name=b.toString('ascii',i+4,i+8),d=b.subarray(i+8,i+8+len);if(name==='IHDR'){w=d.readUInt32BE(0);h=d.readUInt32BE(4);depth=d[8];type=d[9];interlace=d[12]}if(name==='IDAT')parts.push(d);i+=len+12}
 if(depth!==8||![2,6].includes(type)||interlace)throw Error('Use a noninterlaced 8-bit RGB/RGBA screenshot');
 if(w*h>32e6)throw Error('PNG is too large');const c=type===6?4:3,stride=w*c,raw=zlib.inflateSync(Buffer.concat(parts),{maxOutputLength:(stride+1)*h}),pixels=Buffer.alloc(w*h*4);let prev=Buffer.alloc(stride),at=0;
 const paeth=(a,b,c)=>{const p=a+b-c,pa=Math.abs(p-a),pb=Math.abs(p-b),pc=Math.abs(p-c);return pa<=pb&&pa<=pc?a:pb<=pc?b:c};
 for(let y=0;y<h;y++){const filter=raw[at++],row=Buffer.from(raw.subarray(at,at+stride));at+=stride;if(filter>4)throw Error('Invalid PNG filter');for(let x=0;x<stride;x++){const a=x>=c?row[x-c]:0,v=prev[x],d=x>=c?prev[x-c]:0;row[x]=(row[x]+[0,a,v,Math.floor((a+v)/2),paeth(a,v,d)][filter])&255}for(let x=0;x<w;x++){const out=(y*w+x)*4;pixels[out]=row[x*c];pixels[out+1]=row[x*c+1];pixels[out+2]=row[x*c+2];pixels[out+3]=c===4?row[x*c+3]:255}prev=row}
 return {w,h,pixels};
}
try{const [a,b]=process.argv.slice(2);if(!b)throw Error('Usage: compare-renders.mjs before.png after.png');const before=png(a),after=png(b);if(before.w!==after.w||before.h!==after.h){console.log(JSON.stringify({comparable:false,before:[before.w,before.h],after:[after.w,after.h],reason:'Dimensions differ; recapture the same viewport and crop.'}));}else{let changed=0,delta=0;for(let i=0;i<before.pixels.length;i+=4){let max=0;for(let c=0;c<4;c++){const d=Math.abs(before.pixels[i+c]-after.pixels[i+c]);max=Math.max(max,d);delta+=d}if(max>16)changed++}console.log(JSON.stringify({comparable:true,width:before.w,height:before.h,changed_pixels:changed,pixel_diff_ratio:changed/(before.w*before.h),mean_channel_difference:delta/before.pixels.length,threshold_per_channel:16,notice:'Pixel change measures regression evidence, not better or worse design. Review the before and after images.'}))}}catch(e){console.error(e.message);process.exitCode=1}
