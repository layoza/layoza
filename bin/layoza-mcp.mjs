#!/usr/bin/env node
// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Lightweight Node wrapper for `npx layoza-mcp` stdio MCP server.

import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const cliWrapper = path.join(__dirname, "layoza.mjs");

const res = spawnSync(process.execPath, [cliWrapper, "mcp", ...process.argv.slice(2)], {
  stdio: "inherit",
});
if (res.error) {
  console.error(`Error: failed to run ${cliWrapper}: ${res.error.message}`);
  process.exit(1);
}
if (res.status === null) {
  console.error(`Error: layoza-mcp was terminated by signal ${res.signal ?? "unknown"}.`);
  process.exit(1);
}
process.exit(res.status);
