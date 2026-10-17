import pathlib
here = pathlib.Path(__file__).resolve().parent
lines = (here/'crowd.out.txt').read_text().split('\n')
data = {lines[i]: lines[i+1] for i in range(0, len(lines)-1, 2)}
t = (here.parent/'src'/'index.template.html').read_text(encoding='utf-8')
for k in ['BACKARMS','BACK','FRONTARMS','FRONT','STARS']:
    t = t.replace('{{'+k+'}}', data[k])
assert '{{' not in t
(here.parent.parent/'index.html').write_text(t, encoding='utf-8')
print('ok', len(t))
