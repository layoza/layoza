// Layoza's own minimal Chrome DevTools Protocol client for the review scripts.
// Needs Node 22+ (global WebSocket) and a local Chrome, Edge or Playwright Chromium.
// Set LAYOZA_CHROME to a browser executable to skip the search.
import {spawn} from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

export const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

function playwrightChromes() {
  const roots = [
    process.env.PLAYWRIGHT_BROWSERS_PATH,
    process.env.LOCALAPPDATA && path.join(process.env.LOCALAPPDATA, 'ms-playwright'),
    path.join(os.homedir(), '.cache', 'ms-playwright'),
    path.join(os.homedir(), 'Library', 'Caches', 'ms-playwright'),
  ].filter(Boolean);
  const tails = [
    ['chrome-win', 'chrome.exe'], ['chrome-win64', 'chrome.exe'],
    ['chrome-linux', 'chrome'], ['chrome-linux64', 'chrome'],
    ['chrome-mac', 'Chromium.app', 'Contents', 'MacOS', 'Chromium'],
  ];
  const found = [];
  for (const root of roots) {
    let dirs = [];
    try { dirs = fs.readdirSync(root).filter(d => /^chromium-\d+$/.test(d)).sort().reverse(); } catch { continue; }
    for (const dir of dirs) for (const tail of tails) found.push(path.join(root, dir, ...tail));
  }
  return found;
}

export function findChrome() {
  const pf = [process.env.PROGRAMFILES, process.env['PROGRAMFILES(X86)'], process.env.LOCALAPPDATA].filter(Boolean);
  const candidates = [
    process.env.LAYOZA_CHROME,
    ...pf.flatMap(p => [
      path.join(p, 'Google', 'Chrome', 'Application', 'chrome.exe'),
      path.join(p, 'Microsoft', 'Edge', 'Application', 'msedge.exe'),
    ]),
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
    '/Applications/Chromium.app/Contents/MacOS/Chromium',
    '/usr/bin/google-chrome', '/usr/bin/google-chrome-stable', '/usr/bin/chromium', '/usr/bin/chromium-browser',
    ...playwrightChromes(),
  ].filter(Boolean);
  const hit = candidates.find(file => { try { return fs.statSync(file).isFile(); } catch { return false; } });
  if (!hit) throw new Error('No Chrome, Edge or Playwright Chromium found. Set LAYOZA_CHROME to a browser executable.');
  return hit;
}

class Connection {
  constructor(socket) {
    this.socket = socket;
    this.nextId = 1;
    this.pending = new Map();
    this.listeners = new Set();
    socket.addEventListener('message', event => {
      const msg = JSON.parse(event.data);
      if (msg.id && this.pending.has(msg.id)) {
        const {resolve, reject, method} = this.pending.get(msg.id);
        this.pending.delete(msg.id);
        if (msg.error) reject(new Error(`${method}: ${msg.error.message}`)); else resolve(msg.result);
      } else if (msg.method) {
        for (const listener of this.listeners) listener(msg);
      }
    });
    socket.addEventListener('close', () => {
      for (const {reject, method} of this.pending.values()) reject(new Error(`${method}: browser connection closed`));
      this.pending.clear();
    });
  }
  send(method, params = {}, sessionId) {
    const id = this.nextId++;
    return new Promise((resolve, reject) => {
      this.pending.set(id, {resolve, reject, method});
      this.socket.send(JSON.stringify({id, method, params, ...(sessionId ? {sessionId} : {})}));
    });
  }
  waitFor(method, sessionId, timeoutMs) {
    return new Promise(resolve => {
      const done = value => { clearTimeout(timer); this.listeners.delete(listener); resolve(value); };
      const listener = msg => { if (msg.method === method && msg.sessionId === sessionId) done(true); };
      const timer = setTimeout(() => done(false), timeoutMs);
      this.listeners.add(listener);
    });
  }
}

class Page {
  constructor(conn, targetId, sessionId) { Object.assign(this, {conn, targetId, sessionId}); }
  send(method, params) { return this.conn.send(method, params, this.sessionId); }
  async goto(url, {settleMs = 500, timeoutMs = 45000} = {}) {
    const loaded = this.conn.waitFor('Page.loadEventFired', this.sessionId, timeoutMs);
    const result = await this.send('Page.navigate', {url});
    if (result.errorText) throw new Error(`Could not open ${url}: ${result.errorText}`);
    if (!await loaded) throw new Error(`Timed out loading ${url}`);
    await sleep(settleMs);
  }
  // Runs a function (or an expression string) in the page and returns its JSON value.
  async eval(fn, arg) {
    const expression = typeof fn === 'function' ? `(${fn})(${arg === undefined ? '' : JSON.stringify(arg)})` : fn;
    const {result, exceptionDetails} = await this.send('Runtime.evaluate', {expression, awaitPromise: true, returnByValue: true});
    if (exceptionDetails) throw new Error(exceptionDetails.exception?.description || exceptionDetails.text);
    return result.value;
  }
  close() { return this.conn.send('Target.closeTarget', {targetId: this.targetId}).catch(() => {}); }
}

export class Browser {
  static async launch({executable = findChrome(), timeoutMs = 20000} = {}) {
    if (typeof WebSocket !== 'function') throw new Error('Review scripts need Node 22+ (global WebSocket).');
    const profile = fs.mkdtempSync(path.join(os.tmpdir(), 'layoza-cdp-'));
    const child = spawn(executable, [
      '--headless=new', '--remote-debugging-port=0', `--user-data-dir=${profile}`,
      '--no-first-run', '--no-default-browser-check', '--hide-scrollbars', '--mute-audio',
      '--allow-file-access-from-files', 'about:blank',
    ], {stdio: ['ignore', 'ignore', 'pipe']});
    // A browser that never prints its DevTools URL, or whose socket will not open, is killed with its profile.
    const fail = e => { if (child.exitCode === null) child.kill(); try { fs.rmSync(profile, {recursive: true, force: true, maxRetries: 3, retryDelay: 200}); } catch {} throw e; };
    const wsUrl = await new Promise((resolve, reject) => {
      let log = '';
      const timer = setTimeout(() => reject(new Error(`Browser did not start: ${log.slice(-400)}`)), timeoutMs);
      child.stderr.on('data', chunk => {
        log += chunk;
        const match = log.match(/DevTools listening on (ws:\/\/\S+)/);
        if (match) { clearTimeout(timer); resolve(match[1]); }
      });
      child.once('exit', code => { clearTimeout(timer); reject(new Error(`Browser exited (${code}): ${log.slice(-400)}`)); });
    }).catch(fail);
    const socket = new WebSocket(wsUrl);
    await new Promise((resolve, reject) => { socket.addEventListener('open', resolve, {once: true}); socket.addEventListener('error', () => reject(new Error(`DevTools socket did not open: ${wsUrl}`)), {once: true}); }).catch(fail);
    return new Browser(child, new Connection(socket), profile);
  }
  constructor(child, conn, profile) { Object.assign(this, {child, conn, profile}); }
  // Phone widths (< 600px) emulate a touch phone by default, so (hover: none) and (pointer: coarse) match.
  async newPage({width = 1440, height = 900, reducedMotion = false, mobile = width < 600, touch = mobile} = {}) {
    const {targetId} = await this.conn.send('Target.createTarget', {url: 'about:blank'});
    const {sessionId} = await this.conn.send('Target.attachToTarget', {targetId, flatten: true});
    const page = new Page(this.conn, targetId, sessionId);
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setDeviceMetricsOverride', {width, height, deviceScaleFactor: 1, mobile});
    if (touch) await page.send('Emulation.setTouchEmulationEnabled', {enabled: true, maxTouchPoints: 5});
    await page.send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: reducedMotion ? 'reduce' : 'no-preference'}]});
    return page;
  }
  async close() {
    await this.conn.send('Browser.close').catch(() => {});
    await Promise.race([new Promise(resolve => this.child.once('exit', resolve)), sleep(3000)]);
    if (this.child.exitCode === null) this.child.kill();
    try { fs.rmSync(this.profile, {recursive: true, force: true, maxRetries: 3, retryDelay: 200}); } catch {}
  }
}
