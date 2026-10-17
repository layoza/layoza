(function () {
  // ---- mobile navigation ----
  var btn = document.querySelector('.menu-btn');
  var nav = document.getElementById('site-nav');
  function setMenu(open) {
    btn.setAttribute('aria-expanded', open ? 'true' : 'false');
    btn.textContent = open ? 'Close' : 'Menu';
    nav.classList.toggle('open', open);
  }
  btn.addEventListener('click', function () { setMenu(btn.getAttribute('aria-expanded') !== 'true'); });
  nav.addEventListener('click', function (e) { if (e.target.closest('a')) setMenu(false); });
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape' && btn.getAttribute('aria-expanded') === 'true') { setMenu(false); btn.focus(); }
  });
  document.addEventListener('click', function (e) {
    if (btn.getAttribute('aria-expanded') === 'true' && !e.target.closest('.bar')) setMenu(false);
  });

  // ---- project filter ----
  var words = ['No projects', 'One project', 'Two projects', 'Three projects', 'Four projects', 'Five projects', 'Six projects'];
  var names = { all: '', housing: ' in housing', civic: ' in civic work', extensions: ' in extensions' };
  var buttons = Array.prototype.slice.call(document.querySelectorAll('[data-filter]'));
  var projects = Array.prototype.slice.call(document.querySelectorAll('.proj'));
  var count = document.querySelector('.count');
  function applyFilter(type) {
    var shown = 0;
    projects.forEach(function (p) {
      var on = type === 'all' || p.getAttribute('data-type') === type;
      p.hidden = !on;
      if (on) shown++;
    });
    buttons.forEach(function (b) { b.setAttribute('aria-pressed', b.getAttribute('data-filter') === type ? 'true' : 'false'); });
    count.textContent = words[shown] + names[type];
  }
  buttons.forEach(function (b) {
    b.addEventListener('click', function () { applyFilter(b.getAttribute('data-filter')); });
  });
  Array.prototype.forEach.call(document.querySelectorAll('[data-jump]'), function (a) {
    a.addEventListener('click', function () { applyFilter(a.getAttribute('data-jump')); });
  });

  // ---- notch position for the folded card on phones ----
  var card = document.querySelector('.card');
  var faceA = document.querySelector('.face.a');
  function placeFold() { card.style.setProperty('--fold', (faceA.offsetHeight - 14) + 'px'); }
  placeFold();
  window.addEventListener('resize', placeFold);

  // ---- hero sheet: drop frames that fall wholly outside the cropped sheet ----
  var sheet = document.querySelector('.sheet');
  var heroFrames = Array.prototype.slice.call(sheet.querySelectorAll('.fr'));
  function pruneSheet() {
    heroFrames.forEach(function (fr) { fr.style.display = ''; fr.style.visibility = ''; fr.lastElementChild.style.visibility = ''; });
    heroFrames.forEach(function (fr) {
      var box = fr.parentElement.closest('.pair') || sheet;
      var b = box.getBoundingClientRect(), r = fr.getBoundingClientRect();
      if (r.top >= b.bottom || r.bottom <= b.top) { fr.style.display = 'none'; return; }
      // a caption strip cut by the crop is unreadable: leave the drawing, drop its strip
      var cut = r.bottom > b.bottom + 1;
      fr.lastElementChild.style.visibility = cut ? 'hidden' : '';
      // a frame showing less than half of its drawing reads as an empty box: leave ground instead
      fr.style.visibility = (cut && (b.bottom - r.top) < r.height * 0.5) ? 'hidden' : '';
    });
  }
  pruneSheet();
  window.addEventListener('resize', pruneSheet);
  if (document.fonts && document.fonts.ready) document.fonts.ready.then(pruneSheet);

  // ---- enquiry form ----
  var form = document.getElementById('enquiry');
  var done = document.getElementById('done');
  var rules = {
    name: function (v) { return v.trim().length >= 2 ? '' : 'Please tell us your name.'; },
    email: function (v) {
      if (!v.trim()) return 'Please add an email address so we can reply.';
      return /^[^\s@]+@[^\s@]+\.[^\s@]{2,}$/.test(v.trim()) ? '' : 'That email address looks incomplete. Check for a missing @ or domain.';
    },
    type: function (v) { return v ? '' : 'Choose the kind of project.'; },
    budget: function (v) { return v ? '' : 'Choose a budget range, or “Not sure yet”.'; },
    message: function (v) { return v.trim().length >= 20 ? '' : 'A sentence or two about the building helps us reply properly (20 characters or more).'; }
  };
  function check(field) {
    var msg = rules[field.name](field.value);
    var err = document.getElementById(field.getAttribute('aria-describedby'));
    err.textContent = msg;
    if (msg) field.setAttribute('aria-invalid', 'true'); else field.removeAttribute('aria-invalid');
    return !msg;
  }
  Array.prototype.forEach.call(form.elements, function (el) {
    if (!rules[el.name]) return;
    el.addEventListener('blur', function () { if (el.value || el.hasAttribute('aria-invalid')) check(el); });
    el.addEventListener('input', function () { if (el.hasAttribute('aria-invalid')) check(el); });
    el.addEventListener('change', function () { if (el.tagName === 'SELECT') check(el); });
  });
  form.addEventListener('submit', function (e) {
    e.preventDefault();
    var first = null;
    Array.prototype.forEach.call(form.elements, function (el) {
      if (rules[el.name] && !check(el) && !first) first = el;
    });
    if (first) { first.focus(); return; }
    var name = form.elements.name.value.trim().split(/\s+/)[0];
    var type = form.elements.type.value.toLowerCase(); if (type === 'something else') type = 'new project';
    var email = form.elements.email.value.trim();
    done.innerHTML = '';
    var p1 = document.createElement('p');
    p1.textContent = 'Thank you, ' + name + '. Your enquiry about a ' + type + ' is with us.';
    var p2 = document.createElement('p');
    p2.textContent = 'One of the six of us will reply to ' + email + ' within five working days.';
    done.appendChild(p1); done.appendChild(p2);
    done.setAttribute('role', 'status');
    form.hidden = true;
    done.hidden = false;
    done.focus();
  });
})();
