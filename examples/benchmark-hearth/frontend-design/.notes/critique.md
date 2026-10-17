# Critique log
1. Hero h1 wrapped into 5 ragged lines at 1440 -> smaller clamp, one line per sentence (nowrap above 760px).
2. Oven art was a hard rectangle floating in space -> tile face clipped to an arch (repeats the oven-mouth motif); keystone recoloured brick.
3. Bakery "sold out" labels overflowed the bar column (+117px horizontal scroll at 375) -> labels sit inside the fill when the bar passes 45%.
4. Temperature curve had stair-step wobbles (smoothstep per segment) -> cubic Hermite with Catmull-Rom tangents.
5. Mobile nav panel collapsed to a strip: backdrop-filter on the sticky header made it the containing block for the fixed panel -> solid header background.
6. abbr dietary tags showed dotted underlines -> removed.
Verified: no horizontal scroll at 1440/768/375, no JS errors, filter counts, GF empty state for desserts, booking errors + focus + success, accordion, nav Escape.
