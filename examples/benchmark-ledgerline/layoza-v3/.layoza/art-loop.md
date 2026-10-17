# Art loop (dev notes)

ROUND 1 (first build)
Seen in one second: a big condensed "Bank feed in. VAT return out." over a red-ruled form.
Subject named from the picture alone: yes (bank lines + numbered tax boxes), but read as a UI table.
Prompt style words: brief names none beyond "intentional at 3 widths" -> n/a.
Layout words: hero=sheet / how=sequence / product=list / estimate=split / pricing=table / security=split / customers=quote / faq=list / trial=split form / footer=close
Scores: design 7 · usability 6.5 · creativity 7.5 · content 8 -> 7.1 (no QA errors; usability hit by the trial success panel showing before submit)
Top 3:
1. Trial success panel visible under the form ([hidden] overridden by display:grid) -> global [hidden]{display:none!important}.
2. Hero: no visible link between feed and boxes, sheet cut by the fold, empty left column -> 88px routing channel with drawn "ledger lines" from each row's box chips to its box numbers; 6th feed row (wages, outside VAT scope); tighter rows; sheet fits 1440x900.
3. Security map clip-art (looked like a tree) -> "Country where data is stored" as form character cells; footer wordmark as 10 form cells.

ROUND 2
Seen in one second: the return sheet with red routing lines fanning into boxes 1-9.
Subject named from the picture alone: yes.
Layout words: hero=sheet / how=sequence / product=grid list / estimate=statement (huge figure) / pricing=table / security=split / customers=quote / faq=list / trial=form / footer=cells close
Scores: design 8.3 · usability 8.6 · creativity 8.6 · content 8.6 -> 8.5
Top 3:
1. Boxes 3 and 5 inconsistent mid-animation (3 != 1+2) -> derived each step.
2. Status counts disagreed with "6 of 214" -> 213 matched / 214 of 214.
3. Sub-13px text: box chips 12px, SVG label 11px -> chips 13px, SVG label replaced by a rule.
Ceiling pass: 1 idea to last pixel ok (form cells in security, trial, footer); 2 dominant per screen ok; 3 hero h1 ~7vw ok; 4 sizes outside body: d1, d2, h3 (+ estimator figure as d1-class display) ok; 5 usability ok; 6 voids: section padding 88-144px, nothing under footer; 7 signature = routing lines + request receipt; 8 no layout word twice in a row; 9 ornament = red box numbers (box numbers, chips, cells: 3 types); 10 objects drawn as working notation; 11 n/a; 12 designed close ok.
Open: product (six capabilities) is the plainest section; h2-left/p-right head repeats (how, pricing, footer close).
