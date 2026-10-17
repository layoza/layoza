import sys, pathlib
from PIL import Image
d=pathlib.Path(sys.argv[1]); cols=int(sys.argv[2]) if len(sys.argv)>2 else 2; sc=float(sys.argv[3]) if len(sys.argv)>3 else .5
fs=sorted(d.glob("f*.png")); per=cols*2
for k in range(0,len(fs),per):
    ims=[Image.open(f) for f in fs[k:k+per]]; w,h=int(ims[0].width*sc),int(ims[0].height*sc)
    S=Image.new("RGB",(w*cols+(cols-1)*6,h*2+6),"red")
    for i,im in enumerate(ims): S.paste(im.resize((w,h)),((i%cols)*(w+6),(i//cols)*(h+6)))
    S.save(d/f"sheet_{k//per}.png"); print(d/f"sheet_{k//per}.png")
