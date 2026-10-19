#!/usr/bin/env node
// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Minimal MCP stdio client used to check a release before it is published. No dependencies.
//
//   node scripts/mcp-smoke.mjs [--cwd <dir>] -- <command> [args...]
//   node scripts/mcp-smoke.mjs --cwd /tmp/empty -- npx --yes layoza@0.1.4 mcp
//
// It runs initialize, notifications/initialized, tools/list, one read-only tools/call and ping,
// checks that every stdout line is a JSON-RPC message, then closes stdin and waits for the
// server to exit on its own. Exits non-zero on any failure.

import { spawn } from "node:child_process";

const argv = process.argv.slice(2);
const sep = argv.indexOf("--");
if (sep < 0 || sep === argv.length - 1) {
  console.error("Usage: node scripts/mcp-smoke.mjs [--cwd <dir>] -- <command> [args...]");
  process.exit(2);
}
const opts = argv.slice(0, sep);
const [cmd, ...args] = argv.slice(sep + 1);
const cwd = opts.includes("--cwd") ? opts[opts.indexOf("--cwd") + 1] : process.cwd();
const TIMEOUT_MS = Number(process.env.LAYOZA_SMOKE_TIMEOUT_MS || 180000);

// Windows needs a shell to run npx.cmd; pass one quoted command line. Arguments come from the developer.
const quote = (a) => (/[\s"]/.test(a) ? `"${a.replace(/"/g, '\\"')}"` : a);
const child =
  process.platform === "win32"
    ? spawn([cmd, ...args].map(quote).join(" "), { cwd, stdio: ["pipe", "pipe", "pipe"], shell: true })
    : spawn(cmd, args, { cwd, stdio: ["pipe", "pipe", "pipe"] });
let stdoutBuf = "";
let stderr = "";
const nonJsonLines = [];
const pending = new Map();
const failures = [];
const fail = (m) => failures.push(m);

child.stderr.on("data", (d) => (stderr += d));
child.stdout.on("data", (d) => {
  stdoutBuf += d;
  let i;
  while ((i = stdoutBuf.indexOf("\n")) >= 0) {
    const line = stdoutBuf.slice(0, i).trim();
    stdoutBuf = stdoutBuf.slice(i + 1);
    if (!line) continue;
    let msg;
    try {
      msg = JSON.parse(line);
    } catch {
      nonJsonLines.push(line);
      continue;
    }
    if (msg.jsonrpc !== "2.0") nonJsonLines.push(line);
    const waiter = pending.get(msg.id);
    if (waiter) {
      pending.delete(msg.id);
      waiter(msg);
    }
  }
});
const exited = new Promise((resolve) => child.on("exit", (code, signal) => resolve({ code, signal })));

let nextId = 1;
const request = (method, params) =>
  new Promise((resolve, reject) => {
    const id = nextId++;
    const timer = setTimeout(() => reject(new Error(`${method} timed out after ${TIMEOUT_MS} ms`)), TIMEOUT_MS);
    pending.set(id, (msg) => {
      clearTimeout(timer);
      resolve(msg);
    });
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
  });
const notify = (method, params) => child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method, params }) + "\n");

const started = Date.now();
try {
  const init = await request("initialize", {
    protocolVersion: "2025-06-18",
    capabilities: {},
    clientInfo: { name: "layoza-release-smoke", version: "1" },
  });
  const r = init.result;
  if (init.error || !r) fail(`initialize returned an error: ${JSON.stringify(init.error)}`);
  else {
    if (!/^\d{4}-\d{2}-\d{2}$/.test(r.protocolVersion ?? "")) fail(`initialize: bad protocolVersion ${r.protocolVersion}`);
    if (!r.serverInfo?.name || !r.serverInfo?.version) fail("initialize: serverInfo.name/version missing");
    if (!r.capabilities?.tools) fail("initialize: tools capability missing");
    if (!r.capabilities?.prompts) fail("initialize: prompts capability missing");
    console.log(`initialize      ok  protocol ${r.protocolVersion}, server ${r.serverInfo?.name} ${r.serverInfo?.version} (${Date.now() - started} ms)`);
  }

  notify("notifications/initialized");

  const list = await request("tools/list", {});
  const tools = list.result?.tools;
  if (!Array.isArray(tools) || tools.length === 0) fail(`tools/list returned no tools: ${JSON.stringify(list.error)}`);
  else {
    for (const t of tools) {
      if (typeof t.name !== "string" || !/^[a-zA-Z0-9_-]{1,64}$/.test(t.name)) fail(`tool has an invalid name: ${t.name}`);
      if (typeof t.description !== "string" || !t.description) fail(`tool ${t.name}: description missing`);
      if (t.inputSchema?.type !== "object") fail(`tool ${t.name}: inputSchema.type must be "object"`);
      for (const req of t.inputSchema?.required ?? []) {
        if (!(req in (t.inputSchema.properties ?? {}))) fail(`tool ${t.name}: required "${req}" is not in properties`);
      }
    }
    console.log(`tools/list      ok  ${tools.length} tools: ${tools.map((t) => t.name).join(", ")}`);
  }

  const prompts = await request("prompts/list", {});
  const pl = prompts.result?.prompts;
  if (!Array.isArray(pl) || pl.length !== 7) fail(`prompts/list: expected 7 prompts, got ${JSON.stringify(prompts.error ?? pl?.length)}`);
  else console.log(`prompts/list    ok  ${pl.map((p) => p.name).join(", ")}`);
  const pg = await request("prompts/get", { name: "art-direct", arguments: { brief: "funeral home, Cafe – Zürich" } });
  const ptext = pg.result?.messages?.[0]?.content?.text ?? "";
  if (!ptext.includes("Zürich")) fail(`prompts/get: UTF-8 brief did not round-trip: ${JSON.stringify(pg.error ?? ptext.slice(0, 80))}`);
  else console.log(`prompts/get     ok  art-direct, ${ptext.split(/\s+/).length} words, UTF-8 intact`);

  // Read-only call: Layoza's mechanics catalogue, no files touched
  const call = await request("tools/call", { name: "deal_mechanics", arguments: { brief: "seafood restaurant", count: 2, ledger: false } });
  const text = call.result?.content?.find((c) => c.type === "text")?.text;
  if (call.error || call.result?.isError || !text) fail(`tools/call deal_mechanics failed: ${JSON.stringify(call.error ?? call.result)}`);
  else console.log(`tools/call      ok  deal_mechanics returned ${text.length} chars`);

  // Bad input must come back as an actionable tool error, not a crash
  const range = await request("tools/call", { name: "deal_mechanics", arguments: { brief: "x", count: 99 } });
  const rangeText = range.result?.content?.[0]?.text ?? "";
  if (!range.result?.isError || !/count/.test(rangeText)) fail(`out-of-range count was not reported clearly: ${JSON.stringify(range)}`);
  else console.log(`bad argument    ok  ${rangeText}`);

  const bad = await request("tools/call", { name: "no_such_tool", arguments: {} });
  if (!bad.error && !bad.result?.isError) fail("tools/call with an unknown tool did not report an error");
  else console.log(`unknown tool    ok  ${(bad.error?.message ?? bad.result?.content?.[0]?.text ?? "").slice(0, 80)}`);

  const ping = await request("ping");
  if (ping.error) fail("ping failed");
  else console.log("ping            ok");
} catch (e) {
  fail(e.message);
}

// Shutdown per the stdio transport: close stdin, then the server should exit by itself
child.stdin.end();
const outcome = await Promise.race([exited, new Promise((r) => setTimeout(() => r(null), 10000))]);
if (!outcome) {
  fail("server did not exit within 10 s of stdin closing");
  child.kill();
} else {
  if (outcome.code !== 0) fail(`server exited with code ${outcome.code} after stdin closed`);
  console.log(`shutdown        ok  exited with code ${outcome.code} after stdin closed`);
}

if (nonJsonLines.length) fail(`stdout carried ${nonJsonLines.length} non-JSON-RPC line(s), first: ${nonJsonLines[0].slice(0, 120)}`);
else console.log("stdout          ok  only JSON-RPC messages");
console.log(`stderr          ${stderr.trim() ? stderr.trim().split("\n").length + " line(s): " + stderr.trim().split("\n")[0].slice(0, 120) : "empty"}`);

if (failures.length) {
  console.error(`\nMCP smoke test FAILED:\n- ${failures.join("\n- ")}`);
  process.exit(1);
}
console.log("\nMCP smoke test passed");
