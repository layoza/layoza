C="/c/Program Files/Google/Chrome/Application/chrome.exe"
P='~\AppData\Local\Temp\claude\C--Users-user-Desktop-sharethrone\725a4bdd-89bc-4f8f-8d81-ab622d19f68b\scratchpad\chr'
O='examples\benchmark-halden-eyecare\impeccable\.notes\.impeccable\review'
U="examples/benchmark-halden-eyecare/impeccable/index.html"
shot(){ timeout 60 "$C" --headless=new --disable-gpu --user-data-dir="$P-$1" --no-first-run --window-size=$2 --screenshot="$O\$1" "$U" >/dev/null 2>&1; }
shot desktop-fold.png 1440,900
shot desktop.png 1440,7400
shot tablet-fold.png 768,1024
shot tablet.png 768,10400
shot mobile-fold.png 375,812
shot mobile.png 375,13500
