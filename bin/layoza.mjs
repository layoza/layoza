#!/usr/bin/env node
// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Lightweight Node wrapper for `npx layoza <command>` that delegates to the native Rust binary or builds it on first run.

import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, "..");
const isWin = process.platform === "win32";
const binName = isWin ? "layoza.exe" : "layoza";

// Prebuilt binary shipped in the npm package (built by .github/workflows/release.yml).
const vendorBin = path.join(rootDir, "vendor", `${process.platform}-${process.arch}`, binName);
const candidates = [
  path.join(rootDir, "target", "release", binName),
  path.join(rootDir, "target", "debug", binName),
  vendorBin,
];

// Prefer the most recently built binary, so a stale release build never shadows a newer debug build.
const mtime = (p) => {
  try {
    return fs.statSync(p).mtimeMs;
  } catch {
    return -1;
  }
};
let resolvedBin = candidates
  .filter((p) => mtime(p) >= 0)
  .sort((a, b) => mtime(b) - mtime(a))[0];

// npm drops the exec bit on packaged files; restore it, or run a temp copy when the install dir is read-only.
if (resolvedBin === vendorBin && !isWin) {
  try {
    fs.accessSync(vendorBin, fs.constants.X_OK);
  } catch {
    try {
      fs.chmodSync(vendorBin, 0o755);
    } catch {
      const dir = path.join(os.tmpdir(), "layoza-bin");
      fs.mkdirSync(dir, { recursive: true });
      const copy = path.join(dir, `layoza-${fs.statSync(vendorBin).size}`);
      if (!fs.existsSync(copy)) fs.copyFileSync(vendorBin, copy);
      fs.chmodSync(copy, 0o755);
      resolvedBin = copy;
    }
  }
}

if (!resolvedBin) {
  // No prebuilt binary for this platform: compile the native binary. Build output goes to stderr so it never corrupts MCP stdout.
  console.error("Layoza: building the native binary (first run only, needs the Rust toolchain)...");
  const build = spawnSync("cargo", ["build", "--release", "--quiet", "--bin", "layoza"], {
    cwd: rootDir,
    stdio: ["ignore", process.stderr, "inherit"],
  });
  if (build.error && build.error.code === "ENOENT") {
    console.error(
      `Error: no prebuilt Layoza binary for ${process.platform}-${process.arch}, and \`cargo\` was not found to build one.\n` +
        "Install Rust from https://rustup.rs, then run the command again.",
    );
    process.exit(1);
  }
  if (build.status === 0 && fs.existsSync(candidates[0])) {
    resolvedBin = candidates[0];
  }
}

if (!resolvedBin) {
  console.error(`Error: Layoza binary not found and the build failed. Run \`cargo build --release\` in ${rootDir}.`);
  process.exit(1);
}

// A long-lived `mcp` server would lock the binary on Windows and break the next `cargo build`.
// Run the server from a content-addressed temp copy instead, so rebuilding never collides with it.
if (process.argv[2] === "mcp") {
  try {
    const stat = fs.statSync(resolvedBin);
    const dir = path.join(os.tmpdir(), "layoza-mcp");
    fs.mkdirSync(dir, { recursive: true });
    const copy = path.join(dir, `layoza-${stat.size}-${Math.floor(stat.mtimeMs)}${isWin ? ".exe" : ""}`);
    if (!fs.existsSync(copy)) {
      fs.copyFileSync(resolvedBin, copy);
      if (!isWin) fs.chmodSync(copy, 0o755);
    }
    resolvedBin = copy;
  } catch {
    // Fall back to running the build output directly
  }
}

const res = spawnSync(resolvedBin, process.argv.slice(2), {
  stdio: "inherit",
});
if (res.error) {
  console.error(`Error: failed to run ${resolvedBin}: ${res.error.message}`);
  process.exit(1);
}
if (res.status === null) {
  console.error(`Error: layoza was terminated by signal ${res.signal ?? "unknown"}.`);
  process.exit(1);
}
process.exit(res.status);
