import sys, pathlib, json
from playwright.sync_api import sync_playwright

here = pathlib.Path(__file__).resolve().parent
url = (here.parent / 'index.html').as_uri()
out = here / 'shots'
out.mkdir(exist_ok=True)
report = {}

def scroll_through(page):
    h = page.evaluate('document.documentElement.scrollHeight')
    y = 0
    while y < h:
        page.mouse.wheel(0, 500); page.wait_for_timeout(120); y += 500
        h = page.evaluate('document.documentElement.scrollHeight')
    page.wait_for_timeout(600)
    page.evaluate('window.scrollTo(0,0)'); page.wait_for_timeout(800)

with sync_playwright() as p:
    b = p.chromium.launch()
    for name, w, hgt, rm in [('desktop', 1440, 900, 'no-preference'), ('tablet', 768, 1024, 'no-preference'), ('mobile', 375, 812, 'no-preference'), ('mobile-rm', 375, 812, 'reduce'), ('desktop-rm', 1440, 900, 'reduce')]:
        ctx = b.new_context(viewport={'width': w, 'height': hgt}, reduced_motion=rm)
        page = ctx.new_page()
        errs = []
        page.on('pageerror', lambda e: errs.append(str(e)))
        page.on('console', lambda m: errs.append(m.text) if m.type == 'error' else None)
        page.goto(url, wait_until='networkidle'); page.wait_for_timeout(1800)
        page.screenshot(path=str(out / f'{name}-hero.png'))
        scroll_through(page)
        r = {}
        r['hscroll'] = page.evaluate('document.documentElement.scrollWidth > window.innerWidth')
        r['gsap'] = page.evaluate('!!window.gsap && !!window.ScrollTrigger')
        r['horizontal_days'] = page.evaluate("document.getElementById('days').classList.contains('is-horizontal')")
        # filter
        page.click('.filter button[data-day="sat"]'); page.wait_for_timeout(700)
        r['sat_visible'] = page.evaluate("[...document.querySelectorAll('.act')].filter(e=>!e.hidden).length")
        r['status'] = page.inner_text('#lineup-status')
        # countdown
        a = page.inner_text('#countdown'); page.wait_for_timeout(1300); bb = page.inner_text('#countdown')
        r['countdown_ticks'] = a != bb
        r['countdown_label'] = page.get_attribute('#countdown', 'aria-label')
        # tickets
        page.locator('[data-ticket="weekend"] [data-step="1"]').click()
        page.locator('[data-ticket="weekend"] [data-step="1"]').click()
        page.locator('[data-ticket="camping"] [data-step="1"]').click()
        page.locator('[data-ticket="weekend"] [data-step="-1"]').click()
        page.wait_for_timeout(900)
        r['total'] = page.inner_text('#total')
        page.click('#checkout'); r['checkout_msg'] = page.inner_text('#summary-msg')
        # faq
        page.click('#fq2'); page.wait_for_timeout(500)
        r['faq_expanded'] = page.get_attribute('#fq2', 'aria-expanded')
        r['faq_height'] = page.evaluate("document.getElementById('fa2').getBoundingClientRect().height")
        # newsletter
        page.fill('#email', 'nope'); page.click('#news-form button[type=submit]')
        r['email_error'] = page.inner_text('#email-error')
        page.fill('#email', 'sam@example.com'); page.click('#news-form button[type=submit]'); page.wait_for_timeout(700)
        r['email_success_visible'] = page.is_visible('#news-success')
        page.evaluate('window.scrollTo(0,0)'); page.wait_for_timeout(500)
        page.screenshot(path=str(out / f'{name}-full.png'), full_page=True)
        # sections screenshots
        for sel in ['#days', '#lineup', '.countdown', '#tickets', '#camping', '#info', '.news', '.footer']:
            page.locator(sel).scroll_into_view_if_needed(); page.wait_for_timeout(900)
            page.screenshot(path=str(out / f'{name}-{sel.strip("#.")}.png'))
        if w < 901:
            page.evaluate('window.scrollTo(0,0)'); page.wait_for_timeout(400)
            page.click('.menu-toggle'); page.wait_for_timeout(500)
            r['menu_open'] = page.get_attribute('.menu-toggle', 'aria-expanded')
            page.screenshot(path=str(out / f'{name}-menu.png'))
            page.keyboard.press('Escape'); page.wait_for_timeout(400)
            r['menu_closed_esc'] = page.get_attribute('.menu-toggle', 'aria-expanded')
        r['errors'] = errs
        report[name] = r
        ctx.close()
    b.close()
print(json.dumps(report, indent=1))
(here / 'qa-report.json').write_text(json.dumps(report, indent=1))
