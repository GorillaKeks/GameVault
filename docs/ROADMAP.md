# GameVault Roadmap

**Status date:** 2026-11-12. No application features are implemented.

## Phase 0 — Repository and feasibility baseline

**Done for this preparation:**

- Rechecked local directory and GitHub repository. The remote was empty before initialization; local contained `MASTER_PROMPT.md` only.
- Initialized local Git on `main`, added `origin`, created `.gitignore`, committed the prompt, and pushed. Commit: `f34f0ea3f628967afad8bcfaeaba3d062405bd13`.
- Added the initial architecture, Steam access, security, licensing, ASF/SAM, roadmap, and development research docs.

**Still open:**

- Resolve the Steam API key/authentication/privacy model for a distributed desktop app.
- Complete a true SAM source/license/behavior audit (ASF is not SAM).
- Select GameVault's own license and check product-name/brand rights before distribution.

## Phase 1 — Product and technical foundation

1. Decide product requirements and acceptable data sources based on the documented Steam API gate.
2. Verify local Windows build prerequisites and supported versions (MSVC C++ tools, WebView2, Rust MSVC, Node LTS).
3. Select and record specific Tauri/React/TypeScript/Rust/SQLite versions and dependencies with license and maintenance checks.
4. Approve architecture decisions, data inventory, threat model, license, and packaging direction.
5. Only then scaffold an empty app shell with quality checks; no feature implementation before gates pass.

## Phase 2 — Local shell and persistence

- Accessible desktop shell and settings.
- Versioned SQLite schema and migration behavior.
- Clearly separated demo data.
- User-controlled backup, export, and deletion with verified behavior.

## Phase 3 — Approved read-only Steam connection

- Implement only a verified and permitted authentication/API path.
- Handle private profiles, missing fields, offline, throttling, invalid/revoked access, and schema changes honestly.
- No Steam file scraping or account automation.

## Phase 4 — Library and progress presentation

- Library, playtime, and achievements only where the approved source supplies them.
- Explicit provenance/freshness and correct distinction between unknown and zero.
- No achievement mutation unless separately reviewed and explicitly approved; current product direction excludes it.

## Phase 5 — Release readiness

- Accessibility review, performance, localization foundation, privacy/licensing notices, Windows packaging, signing/update decision, support and vulnerability response.
- Verify clean install/update/uninstall and database retention/restore before release.

## Acceptance gates

- No phase is complete until its decisions and evidence are documented.
- No tests or compatibility results may be claimed without running them.
- The current assessment is **CONDITIONAL GO for planning/foundation work**, not for Steam API integration or SAM-derived behavior.
- Do not start implementation automatically after a GO assessment; follow the authorization rule in `MASTER_PROMPT.md`.
