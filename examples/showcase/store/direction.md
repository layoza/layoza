PURPOSE: Launch store for a made-up phone shop (plover., Leeds). It sells the Tern 2 phone, a case, Wren earbuds, the Puck 45 charger and a cable. Visitors pick a model, colour and storage and add it to the bag.

PICKED: auto — Direction B (Split Studio). The phone is the hero object, and B's split stage puts one big self-made 3D phone next to tight copy. Its dealt motion (cursor field, view transitions, three) lets the device be turned, taken apart and configured, which is what a phone shop most needs to show.

SHAPE:
- Nav: an ink bar with a "Shopping for someone" mega picker and a live 21:00 cut-off countdown.
- Hero: split, with type on the left and a Three.js phone over a cursor field on the right. The word "apart" spreads with scroll.
- Sections: the Three.js phone explodes into five layers as the scroll passes. Then a split configurator with a drag-to-spin phone, a sideways kit chapter (4 panels, each with its own composition, and "Look closer" opening a view-transition dialog), and a giant 21:00 delivery block with a photo.
- Footer: a signed "Dear new owner," letter.
- Deviation: no pinned or scroll-hijacked stages. Pinning left empty bands in full-page stills, and B's motion does not need it. The explode is scroll-linked as the section passes, and the kit chapter is a native sideways scroller with arrows, drag inertia and a progress bar.

ROUND 1: The QA stills showed the cursor field painting over the hero text, Wren buds text overlapping, the scroll-rise shifting text, and pinned sections leaving huge voids in the desktop still. Fixes: the field is limited to the right 56%, rise only scales images, and pinning is removed.

ROUND 2: The "drag to turn it" hint sat under the canvas, and the nav looked like a stock link row. Fixes: the hint moved below the stage, the nav links moved into the audience mega panel, and the live next-day countdown went into the bar.

ROUND 3: The menu click timed out in headless Chromium because software WebGL ran at 9 to 15 fps. Added adaptive quality: a software GPU gets dpr 0.6 and a 30 fps cap, the explode view renders only on change, and every view pauses off-screen. The click now takes about 0.1 s. The mobile menu panel was stopping halfway down and showing the hero underneath, so it now fills the viewport. Added press states and text-wrap.

REVIEWER: 7/10. The phone is real 3D, live and shoppable, and the page reads as a launch store rather than a spec table. The weakest point is that the kit renders (the SVG buds, charger and case) are flat 2D drawings next to a glossy 3D phone, so the kit chapter drops a full grade in polish.
