import sys
def L(h):
    h=h.lstrip('#'); c=[int(h[i:i+2],16)/255 for i in (0,2,4)]
    c=[x/12.92 if x<=0.03928 else ((x+0.055)/1.055)**2.4 for x in c]
    return 0.2126*c[0]+0.7152*c[1]+0.0722*c[2]
def cr(a,b):
    x,y=sorted([L(a),L(b)],reverse=True); return (x+0.05)/(y+0.05)
pairs=[("#2A1D15","#F5F0E1"),("#6B5544","#F5F0E1"),("#FFF8EC","#9A4A1C"),("#9A4A1C","#F5F0E1"),("#55622E","#F5F0E1"),("#F5F0E1","#1C1410"),("#BFAE96","#1C1410"),("#E8925A","#1C1410"),("#6B5544","#EBE1CC"),("#2A1D15","#EBE1CC"),("#9A4A1C","#EBE1CC"),("#A3341F","#F5F0E1"),("#1C1410","#E8925A"),("#55622E","#FFFBF2"),("#6B5544","#FFFBF2")]
for a,b in pairs: print(a,b,round(cr(a,b),2))
