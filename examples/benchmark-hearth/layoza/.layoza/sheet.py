import glob, sys
from PIL import Image
pre, start, n, out = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
fs = [f"shots/{pre}-{i}.png" for i in range(start, start + n)]
ims = [Image.open(f) for f in fs if __import__('os').path.exists(f)]
w, h = ims[0].size
sheet = Image.new("RGB", (w * len(ims) + 10 * (len(ims) - 1), h), "white")
for i, im in enumerate(ims): sheet.paste(im, (i * (w + 10), 0))
sheet.save(out)
