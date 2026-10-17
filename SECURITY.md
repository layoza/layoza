# Security policy

## Reporting a vulnerability

Please do not open a public issue for security problems. Report them privately through
GitHub: **Security → Report a vulnerability** on this repository.

Include what you found, how to reproduce it, and the version (`layoza --version`).
We aim to reply within 7 days and to ship a fix or mitigation within 30 days.

## Supported versions

Only the latest release receives security fixes.

## Scope

Layoza reads project files and web pages that may be untrusted. Issues we especially
want to hear about:

- Prompt injection from inspected pages that reaches the agent unsanitised.
- File reads or writes outside the project root.
- Secrets copied into generated artifacts (`DESIGN.md`, `SITE-DNA.json`, baselines).
- MCP tools that modify files while the server runs with `--read-only`.

How these protections work is described in [docs/SECURITY.md](docs/SECURITY.md).
