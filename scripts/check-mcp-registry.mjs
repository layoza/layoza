#!/usr/bin/env node
// Copyright (c) 2026 Layoza Contributors. Licensed under the MIT License.
// Release check for the official MCP Registry entry. No dependencies.
//
//   node scripts/check-mcp-registry.mjs                  # check the repository
//   node scripts/check-mcp-registry.mjs --tarball x.tgz  # also check a packed npm archive
//
// It checks server.json against the rules of the 2025-12-11 server schema that this entry uses,
// that every version matches, that package.json carries the exact mcpName, and that the launch
// command the Registry advertises (`npx layoza@<version> mcp`) maps to a real CLI command.
// The Registry still runs its own full validation when the entry is published.

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const SCHEMA = "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json";
const NAME = "io.github.layoza/layoza";

const errors = [];
const check = (ok, msg) => {
  if (!ok) errors.push(msg);
};
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8").replace(/^﻿/, ""));

const server = readJson(path.join(root, "server.json"));
const pkg = readJson(path.join(root, "package.json"));
const cargoVersion = /^\[workspace\.package\][\s\S]*?^version\s*=\s*"([^"]+)"/m.exec(
  fs.readFileSync(path.join(root, "Cargo.toml"), "utf8"),
)?.[1];

// server.json: the schema rules for the fields this entry uses
const allowedTop = new Set(["$schema", "name", "title", "description", "repository", "version", "websiteUrl", "icons", "packages", "remotes", "_meta"]);
for (const k of Object.keys(server)) check(allowedTop.has(k), `server.json: unknown top-level field "${k}"`);
check(server.$schema === SCHEMA, `server.json: $schema must be ${SCHEMA}`);
check(typeof server.name === "string" && /^[a-zA-Z0-9.-]+\/[a-zA-Z0-9._-]+$/.test(server.name) && server.name.length >= 3 && server.name.length <= 200, "server.json: name does not match the schema pattern");
check(server.name === NAME, `server.json: name must be ${NAME}`);
check(typeof server.description === "string" && server.description.length >= 1 && server.description.length <= 100, "server.json: description must be 1-100 characters");
check(server.title === undefined || (typeof server.title === "string" && server.title.length >= 1 && server.title.length <= 100), "server.json: title must be 1-100 characters");
const isPlainVersion = (v) => typeof v === "string" && v.length >= 1 && v.length <= 255 && v !== "latest" && !/[\^~*xX<>=| ]/.test(v);
check(isPlainVersion(server.version), "server.json: version must be an exact version, not a range or `latest`");
check(server.repository?.url === "https://github.com/layoza/layoza" && server.repository?.source === "github", "server.json: repository must be https://github.com/layoza/layoza (source github)");
check(server.remotes === undefined, "server.json: Layoza has no remote endpoint, so `remotes` must not be set");
check(Array.isArray(server.packages) && server.packages.length === 1, "server.json: expected exactly one package");
// Icon: the official app icon, pinned to this release's tag so the URL never changes under the entry
const iconUrl = `https://raw.githubusercontent.com/layoza/layoza/v${pkg.version}/docs/assets/icon-256.png`;
check(Array.isArray(server.icons) && server.icons.length === 1, "server.json: expected one icon");
const icon = server.icons?.[0] ?? {};
check(icon.src === iconUrl, `server.json: icon src must be ${iconUrl}`);
check(icon.mimeType === "image/png" && JSON.stringify(icon.sizes) === '["256x256"]', "server.json: icon must be image/png, sizes [\"256x256\"]");
const iconFile = path.join(root, "docs/assets/icon-256.png");
check(fs.existsSync(iconFile) && fs.readFileSync(iconFile).readUInt32BE(16) === 256 && fs.readFileSync(iconFile).readUInt32BE(20) === 256, "docs/assets/icon-256.png must exist and be 256x256");

const p = server.packages?.[0] ?? {};
check(p.registryType === "npm", "package: registryType must be npm");
check(p.registryBaseUrl === "https://registry.npmjs.org", "package: registryBaseUrl must be https://registry.npmjs.org");
check(p.identifier === pkg.name, `package: identifier must be the npm name "${pkg.name}"`);
check(isPlainVersion(p.version), "package: version must be an exact version");
check(p.runtimeHint === "npx", "package: runtimeHint must be npx");
check(p.transport?.type === "stdio" && Object.keys(p.transport).length === 1, "package: transport must be { type: stdio }");
check(p.environmentVariables === undefined, "package: Layoza needs no environment variables");
for (const a of [...(p.packageArguments ?? []), ...(p.runtimeArguments ?? [])]) {
  check(a.type === "positional" || (a.type === "named" && typeof a.name === "string"), "package: every argument needs type positional, or type named with a name");
  check(a.type !== "positional" || a.value !== undefined || a.valueHint !== undefined, "package: a positional argument needs value or valueHint");
}
// `npx layoza@<v>` runs the bin named like the package (bin/layoza.mjs); `mcp` starts the server
check(JSON.stringify(p.packageArguments) === JSON.stringify([{ type: "positional", value: "mcp" }]), 'package: packageArguments must be [{ "type": "positional", "value": "mcp" }]');
check(p.runtimeArguments === undefined, "package: runtimeArguments are not used");

// Versions and ownership
check(pkg.mcpName === NAME, `package.json: mcpName must be exactly "${NAME}"`);
for (const [label, v] of [["server.json version", server.version], ["server.json package version", p.version], ["Cargo.toml workspace version", cargoVersion]]) {
  check(v === pkg.version, `${label} (${v}) must equal package.json version (${pkg.version})`);
}
check(pkg.bin?.layoza === "bin/layoza.mjs" && pkg.bin?.["layoza-mcp"] === "bin/layoza-mcp.mjs", "package.json: both bin entries (layoza, layoza-mcp) must be kept, without a ./ prefix");
check(/^\s*"mcp"\s*=>/m.test(fs.readFileSync(path.join(root, "crates/layoza-cli/src/main.rs"), "utf8")), "CLI: `layoza mcp` command not found in crates/layoza-cli/src/main.rs");

// Optional: the packed archive must carry the same metadata, since npm is what the Registry reads
const ti = process.argv.indexOf("--tarball");
if (ti > 0) {
  const tgz = path.resolve(process.argv[ti + 1] ?? "");
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "layoza-tgz-"));
  // Relative paths only: GNU tar (Git for Windows) reads "C:\..." as a remote host:path
  fs.copyFileSync(tgz, path.join(dir, "pkg.tgz"));
  const tar = spawnSync("tar", ["-xzf", "pkg.tgz"], { cwd: dir, encoding: "utf8" });
  check(tar.status === 0, `tarball: could not unpack ${tgz}: ${tar.stderr}`);
  if (tar.status === 0) {
    const packed = readJson(path.join(dir, "package", "package.json"));
    check(packed.mcpName === NAME, "tarball: package.json is missing the exact mcpName");
    check(packed.version === server.version, `tarball: version ${packed.version} does not match server.json ${server.version}`);
    check(JSON.stringify(packed.bin) === JSON.stringify(pkg.bin), "tarball: bin entries differ from package.json (npm may have stripped them)");
    for (const f of ["bin/layoza.mjs", "bin/layoza-mcp.mjs", "scripts/browser-qa.mjs", "LICENSE", "README.md"]) {
      check(fs.existsSync(path.join(dir, "package", f)), `tarball: missing ${f}`);
    }
  }
  fs.rmSync(dir, { recursive: true, force: true });
}

if (errors.length) {
  console.error(`MCP Registry check failed (${errors.length}):\n- ${errors.join("\n- ")}`);
  process.exit(1);
}
console.log(`MCP Registry check passed: ${server.name} ${server.version}, launch: npx ${p.identifier}@${p.version} mcp`);
