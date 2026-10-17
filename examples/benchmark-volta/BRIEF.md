# Brief: Volta One (fictional) — style: 3D

Product landing page for **Volta One**, a lightweight electric city bike from a small Bristol company. 16 kg, 80 km range, belt drive, hidden battery in the frame, 3 colours (Graphite, Moss, Clay). Price £2,190.

**Style direction from the client (must be followed): a real-time 3D experience. The bike (or its key parts) is shown in interactive 3D the visitor can rotate, with the 3D tied to the story as they scroll. Built from procedural geometry — no external 3D model files. Must degrade gracefully without WebGL and with reduced motion.**

Required content:
1. Header: logo, nav (Ride, Specs, Colours, Test ride), "Order" button, mobile navigation.
2. Hero with product name, one-line promise, price, primary + secondary CTA, and the 3D bike.
3. Three feature stories (battery in frame, belt drive, weight) tied to the 3D view.
4. Colour picker (3 colours) that updates the 3D bike and the order summary.
5. Specs table (at least 10 rows).
6. Test ride booking form: name, email, city select (Bristol, Bath, Cardiff), preferred date; inline validation and success message.
7. 3 rider quotes (fictional).
8. FAQ accordion (5 questions).
9. Footer with address, social links.

Constraints:
- One `index.html` with inline CSS and JS. Google Fonts allowed. Pinned-version JavaScript libraries from cdn.jsdelivr.net are allowed (e.g. three.js). No external images or model files: use code-generated geometry, CSS, SVG or canvas.
- Must work and look intentional at 1440x900, 768x1024, and 375x812.
- Real, usable interactions (3D rotate, colour picker, form validation, accordion, mobile navigation).
- No invented awards or real-world claims.
