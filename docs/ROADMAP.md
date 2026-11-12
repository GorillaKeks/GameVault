# GameVault Roadmap

**Status date:** 2026-11-12. No application features or scaffold exist.

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

### Open approval and evidence gates

1. Obtain Valve's answer about individual user API keys in a distributed desktop app, key/domain/application registration, the confidential/no-sharing obligation and correct public host for player methods.
2. Decide whether unsupported local binary achievement cache parsing may be considered if Valve disallows the direct per-user key path. Current recommendation is not to rely on it as the primary source.
3. Approve the exact project license (recommended Apache-2.0) and confirm copyright holder before adding a root `LICENSE`.
4. Keep the four features mandatory. If permitted sources cannot support them, return to the owner for a product decision; do not remove or downgrade one.
5. Resolve the conflict with the older `MASTER_PROMPT.md`, which still excludes Steam file scraping and allows no change without authorization. Phase 1.6 authorizes research and design of narrow read-only VDF/ACF scanning; the master file was intentionally left unchanged.

## Phase 1 — Technical foundation (not started)

1. Resolve Valve/key and achievement-source gates above.
2. Update Node.js 24.22.1 to a current supported LTS security patch; choose npm or pnpm.
3. Select exact Tauri/React/TypeScript/Rust/SQLite versions and libraries; review licenses, MSRV, Windows build behavior and maintenance.
4. Confirm MSVC/Windows SDK detection and run a minimal Tauri shell build after scaffold authorization. The current environment was diagnosed but no build was performed.
5. Approve implementation-ready architecture/ADR status.

## Phase 2 — Minimal desktop scaffold (await explicit authorization)

- Create empty Tauri 2 + React/TypeScript + Rust structure, no Steam data functionality.
- Add formatting/type/lint/Rust quality commands and narrow Tauri permissions.
- Build and smoke-check on Windows; document actual tool versions/results.

## Phase 3 — Local database and installed-game discovery

- Add versioned local SQLite migrations and Rust-owned repository.
- Implement a bounded, read-only, configured-root scanner only after approval of the Steam file boundary.
- Discover multiple libraries; handle unavailable/moved drives, partial manifests and scan failures without false “uninstalled” states.
- Keep local app data offline-useful and provenance-labelled.

## Phase 4 — Approved account data: library, playtime and achievements

- Implement only a Valve-confirmed no-backend direct user-key approach, if approved.
- Preserve private/unavailable/stale/unsupported states and exact user consent/refresh behavior.
- Add only verified API fields and locally protected key handling; no shared key and no uploads.
- If key use is rejected, stop and request an owner decision rather than silently relying on a reverse-engineered cache or reducing required scope.

## Phase 5 — Release readiness

- Accessibility, performance, privacy/security notices, dependency/license inventory, artwork permissions, update/signing approach, user data export/deletion and support/vulnerability process.
- Verify install/update/uninstall, cache/database retention and offline behavior before release.

## Acceptance gates

- Research is separate from implementation; no feature is claimed complete based on a file's presence alone.
- No API, license, security or compatibility result is claimed without evidence.
- Phase 1.6 assessment: **CONDITIONAL GO** for continued planning; account library and achievements remain gated by Valve. All four features remain required.
- Do not start Phase 2 automatically. Wait for explicit owner authorization.
