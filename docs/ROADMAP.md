# GameVault Roadmap

**Status date:** 2026-11-12. Initial Tauri scaffold and local installed-manifest inspection prototype created; no Steam client calls or achievement reads yet.

## Phase 0 — Repository and initial feasibility baseline (complete)

- Established the repository and initial product/engineering brief.
- Added architecture, Steam API, security, license, SAM/ASF and Windows environment research.
- Created the Phase 1.5 feasibility review and six ADRs. Phase 1.5's backend candidate is historical and superseded by the owner's later no-server decision; see [`FEASIBILITY_REVIEW.md`](FEASIBILITY_REVIEW.md).

## Phase 1.6 — Local-first feasibility (research complete; feature gate open)

- Confirmed constraints: no GameVault backend/server/account/cloud database; Windows; Steam only; Tauri 2, React/TypeScript, Rust, SQLite; free and open source.
- Preserved all four mandatory features: complete game library, installed games, playtime and achievements.
- Investigated Windows local Steam structures and conducted a privacy-safe, read-only inventory. See [`LOCAL_STEAM_DATA.md`](LOCAL_STEAM_DATA.md).
- Designed a local architecture and documented the feature-by-feature verdicts. See [`LOCAL_FIRST_ARCHITECTURE.md`](LOCAL_FIRST_ARCHITECTURE.md) and [`V1_FEATURE_FEASIBILITY.md`](V1_FEATURE_FEASIBILITY.md).
- Recommended Apache-2.0 and prepared a clearly non-operative candidate marker. Exact license text/copyright holder still await owner approval. See [`OPEN_SOURCE_LICENSE_RECOMMENDATION.md`](OPEN_SOURCE_LICENSE_RECOMMENDATION.md).
- Updated ADRs. Local Tauri topology and SQLite choice reflect explicit owner decisions; the Steam API/key flow and exact license remain Proposed.

### Remaining evidence gates

1. Obtain Valve's answer about individual user API keys in a distributed desktop app, key/domain/application registration, the confidential/no-sharing obligation and correct public host for player methods.
2. Before implementation of the newly owner-approved read-only SAM-inspired path, complete the native-boundary threat review and define supported Steam client/interface versions and parser failure behavior. See [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md) and ADR-007. The path is unsupported and not evidence of Valve approval.
3. Approve the exact project license (recommended Apache-2.0) and confirm copyright holder before adding a root `LICENSE`.
4. Keep the four features mandatory. If permitted sources cannot support them, return to the owner for a product decision; do not remove or downgrade one.
5. `MASTER_PROMPT.md` now records the owner's explicit read-only points 1–3 direction. No SAM code/assets or write capability is approved.

## Phase 1 — Technical foundation (scaffold created; initial local inspection prototype)

Phase 1.7 readiness audit is complete. The owner authorized the read-only points 1–3 path; the initial scaffold now exists. Remaining gates and validation are:

1. Keep the Valve user-key inquiry open; the ready-to-send draft is in [`PHASE_1_7_DECISION_PACKAGE.md`](PHASE_1_7_DECISION_PACKAGE.md). The local client path is a separate, owner-approved direction, not a replacement for evaluating complete account data.
2. Complete the native boundary, Steam client compatibility and local schema parser design in [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md) and ADR-007 before code.
3. Select the project license and copyright holder; do not add a root `LICENSE` until approved.
4. Node.js was not globally upgraded because Windows Installer requested administrator consent; a checksum-verified portable Node `26.13.1` was downloaded to a user-local tools folder. npm is selected for this scaffold.
5. Confirm exact dependency versions/features and review licenses, MSRV, Windows build behavior and maintenance using [`FOUNDATION_READINESS.md`](FOUNDATION_READINESS.md).
6. Record the owner authorization to pursue read-only points 1–3 in ADR-007; do not expand to writes or represent partial ownership as complete.

The environment audit found MSVC Build Tools, Windows SDK and WebView2 Runtime installed. `cargo tauri` is available through Cargo (`tauri-cli 2.12.4`), but a standalone `tauri` command and SQLite CLI are not on PATH. No build was run; compiler/linker integration remains unverified.

## Phase 2 — Minimal desktop scaffold (created; build not verified)

- Created the Tauri 2 + React/TypeScript + Rust structure in `app/`, branded it GameVault, removed the unused opener plugin, and set an initial restrictive CSP.
- Added a bounded read-only detector for conventional Steam roots, library folders and installed `appmanifest` files. It reports schema-file presence only; it does not read schema bytes or connect to Steam.
- Add formatting/type/lint/Rust quality commands and narrow Tauri permissions.
- Install dependencies, build and smoke-check on Windows; document actual tool versions/results.

## Phase 3 — Local database and installed-game discovery

- Add versioned local SQLite migrations and Rust-owned repository.
- Implement a bounded, read-only, configured-root scanner only after approval of the Steam file boundary.
- Discover multiple libraries; handle unavailable/moved drives, partial manifests and scan failures without false “uninstalled” states.
- Keep local app data offline-useful and provenance-labelled.

## Phase 4 — Approved account data: library, playtime and achievements

- Pursue the owner-approved read-only Steam client/schema adapter under ADR-007; fail closed on missing schema/unsupported interfaces and never call write methods.
- Implement the direct user-key Web API path only if Valve confirms the registration/host/terms pattern.
- Preserve private/unavailable/stale/unsupported states and exact user consent/refresh behavior.
- Add only verified API fields and locally protected key handling; no shared key and no uploads.
- The local SAM-inspired route does not complete the owned library/playtime requirements. If no permitted source supports all mandatory features, return for an owner decision rather than reducing scope.

## Phase 5 — Release readiness

- Accessibility, performance, privacy/security notices, dependency/license inventory, artwork permissions, update/signing approach, user data export/deletion and support/vulnerability process.
- Verify install/update/uninstall, cache/database retention and offline behavior before release.

## Acceptance gates

- Research is separate from implementation; no feature is claimed complete based on a file's presence alone.
- No API, license, security or compatibility result is claimed without evidence.
- Phase 1.6 assessment: **CONDITIONAL GO** for continued planning; account library and achievements remain gated by Valve. All four features remain required.
- The owner has authorized pursuit of read-only points 1–3. The current prototype implements only local install/manifest discovery; client connection and achievement reading remain unimplemented. This does not authorize account/stat writes, SAM source reuse, or unsupported completeness claims.
