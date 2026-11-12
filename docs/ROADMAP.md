# GameVault Roadmap

**Status date:** 2026-11-12. No application features are implemented.

## Phase 0 — Repository and feasibility baseline

**Done for this preparation:**

- Rechecked local directory and GitHub repository. The remote was empty before initialization; local contained `MASTER_PROMPT.md` only.
- Initialized local Git on `main`, added `origin`, created `.gitignore`, committed the prompt, and pushed. Commit: `f34f0ea3f628967afad8bcfaeaba3d062405bd13`.
- Added and expanded architecture, Steam access, security, licensing, ASF/SAM, roadmap, and development research docs.
- Rechecked Steam endpoint/key distinctions, the likely desktop auth gap, SAM mutation behavior, ASF fit, and local Windows prerequisites against primary sources/current repository files.
- Completed Phase 1.5 feasibility review, architecture alternatives, expanded data/privacy matrix, six Proposed ADRs, bounded ASF/SAM decision, and read-only Windows toolchain diagnostics. See [`FEASIBILITY_REVIEW.md`](FEASIBILITY_REVIEW.md).

**Still open before implementation:**

- Owner decision: approve a GameVault website/backend that keeps a registered Web API key confidential, confirm a Valve-compliant per-user key model, or narrow v1 to remove account-connected data.
- Resolve the native desktop authentication flow; Steam OpenID documentation reviewed describes a web-site redirect, not a validated native callback.
- Obtain authoritative Valve clarification on user-key use for GameVault, app/domain registration, commercial desktop use and the discrepancy between player-method partner-host examples and the overview's publisher-key-only partner host.
- Owner decision on whether a hosted GameVault website/backend is acceptable. If not, explicitly re-scope v1 to remove account-connected data before app scaffolding.
- Do not parse local Steam client files. Reopen that scope only if the owner explicitly approves a separately reviewed import.
- SAM/ASF bounded feasibility decision is complete: both research reference only and excluded from v1. Full exact-file/dependency/asset/policy audit remains necessary before any proposed reuse; no reuse is approved.
- Select GameVault's own license and check product-name/brand rights before distribution.
- Check actual Tauri build/runtime compatibility after the owner authorizes a scaffold; the installed tools have not been tested in an app build.

## Phase 1 — Product and technical foundation

1. Decide product requirements and acceptable data sources based on the documented Steam API gate.
2. Windows prerequisites are mostly present; update Node 24.22.1 to 24.22.3 or newer supported LTS (24.22.3 was a security release on 2026-11-11) and verify a Tauri build after scaffold authorization.
3. Select and record specific Tauri/React/TypeScript/Rust/SQLite versions and dependencies with license and maintenance checks.
4. Approve Proposed ADRs, data inventory, threat model, license, and packaging direction.
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
- The Phase 1.5 assessment is **CONDITIONAL GO for planning only**. Account-connected Steam features and SAM/ASF behavior are not ready for implementation. All six ADRs are Proposed.
- Do not start implementation automatically after a GO assessment; follow the authorization rule in `MASTER_PROMPT.md`.
