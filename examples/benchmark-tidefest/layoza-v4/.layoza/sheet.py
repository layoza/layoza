import sys, glob
from PIL import Image
files = sorted(glob.glob(sys.argv[1])); cols = int(sys.argv[3]); sc = float(sys.argv[4])
ims = [Image.open(f) for f in files]; w, h = int(ims[0].width*sc), int(ims[0].height*sc)
rows = (len(ims)+cols-1)//cols
S = Image.new('RGB', (cols*w+(cols-1)*6, rows*h+(rows-1)*6), 'white')
for i, im in enumerate(ims): S.paste(im.resize((w, h)), ((i%cols)*(w+6), (i//cols)*(h+6)))
S.save(sys.argv[2])
