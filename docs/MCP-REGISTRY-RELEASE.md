# Releasing to the official MCP Registry

Layoza is listed in the official MCP Registry as **`io.github.layoza/layoza`**: a local
stdio server installed from the npm package `layoza` and started with

```bash
npx layoza@<version> mcp
```

The Registry stores only metadata (`server.json`). The code is always downloaded from
npm, so the npm release must exist before the Registry entry is published.

## Three separate destinations

Publishing to one of these does not publish to the others.

| Destination | What it is | How Layoza gets there |
| --- | --- | --- |
| Official MCP Registry (`registry.modelcontextprotocol.io`) | The open metadata registry run by the MCP project. Clients and other directories read from it. | This checklist (`mcp-publisher publish`). |
| GitHub MCP server discovery (GitHub's MCP registry pages and Copilot integrations) | GitHub's own interface for finding MCP servers. It is a separate product with its own listing rules. | Follow GitHub's current documentation. Do not assume the official Registry entry appears there. |
| Claude's connector directory | Anthropic's curated directory inside Claude. It has its own submission and review. | Submit separately through Anthropic's directory process. |

## Checklist

Run every step from a clean checkout of `main` at the release commit.

### 1. Test and approve

- [ ] Versions match in `package.json`, `Cargo.toml` (`[workspace.package]`), `Cargo.lock`
      and both `version` fields in `server.json`. Pick a version that is not on npm yet:
      `npm view layoza versions`.
- [ ] `cargo test --locked` passes.
- [ ] `node scripts/check-mcp-registry.mjs` passes.
- [ ] `node scripts/mcp-smoke.mjs -- node bin/layoza.mjs mcp` passes.
- [ ] `CHANGELOG.md` has the release section.
- [ ] CI is green on `main`.
- [ ] The owner approves the release.

### 2. Publish npm through the existing process

- [ ] Push the tag `v<version>`. `.github/workflows/release.yml` builds the five platform
      binaries, creates the GitHub release, checks the packed tarball with
      `check-mcp-registry.mjs --tarball`, runs the MCP handshake against the tarball with
      the exact advertised command, checks npm keeps both bins and publishes.
- [ ] If the workflow's npm publish fails (for example `EOTP` because the account
      requires 2FA for publishing), the owner publishes the same tarball by hand with
      their own 2FA. Never put an npm token or OTP in the repository, a workflow file or a
      log.

The tag also makes the icon URL in `server.json`
(`raw.githubusercontent.com/layoza/layoza/v<version>/docs/assets/icon-256.png`) resolve.

### 3. Verify the public npm artifact

```bash
npm view layoza@<version> mcpName version bin
```

- [ ] `mcpName` is exactly `io.github.layoza/layoza`. The Registry rejects the publish
      if it is missing or different, because that is how it checks npm ownership.
- [ ] Both bins (`layoza`, `layoza-mcp`) are listed.
- [ ] In an empty temp folder, the public command works:
      `node <repo>/scripts/mcp-smoke.mjs -- npx --yes layoza@<version> mcp`.

### 4. Authenticate as a GitHub org Owner

The `io.github.layoza/*` namespace is verified through GitHub. The person publishing
must have the **Owner** role in the `layoza` organization. Membership alone is not
enough.

- [ ] Install `mcp-publisher` from the official releases of
      `github.com/modelcontextprotocol/registry`. On Windows, the release asset is
      `mcp-publisher_windows_amd64.tar.gz`.
- [ ] Run `mcp-publisher login github`. It uses the browser device flow. If you sign in
      with a personal access token instead, it needs `read:org` (classic token) or
      Organization › Members: read-only (fine-grained token). Create it only for this,
      give it a short expiry, and revoke it afterwards.

### 5. Publish `server.json`

```bash
mcp-publisher publish
```

Run it from the repository root. The Registry validates `server.json` against its
schema and checks that `layoza@<version>` on npm carries the matching `mcpName`. A
published version cannot be changed. To fix metadata, publish a new version.

### 6. Verify through the Registry API

```bash
curl "https://registry.modelcontextprotocol.io/v0.1/servers?search=io.github.layoza/layoza"
```

- [ ] The response lists `io.github.layoza/layoza` with the new `version`, the npm
      package `layoza`, transport `stdio` and the `mcp` package argument.
- [ ] Run `mcp-publisher logout`, and revoke any token created for step 4.

## What the entry declares

- Local stdio transport only. No `remotes`, no OAuth and no environment variables.
- One npm package with `runtimeHint: npx` and one positional package argument, `mcp`.
  `npx layoza@<version>` runs the bin named like the package (`bin/layoza.mjs`), and `mcp`
  starts the server. The `layoza-mcp` bin is kept for clients configured with it.
- Tools are discovered at runtime through `tools/list`. `server.json` does not list them.

## Sources

- MCP Registry quickstart: https://modelcontextprotocol.io/registry/quickstart
- Package types (npm `mcpName` verification): https://modelcontextprotocol.io/registry/package-types
- Authentication (GitHub namespaces, Owner role): https://github.com/modelcontextprotocol/registry/blob/main/docs/modelcontextprotocol-io/authentication.mdx
- Schema used by `server.json`: https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json
