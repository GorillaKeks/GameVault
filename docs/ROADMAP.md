# GameVault Roadmap

**Status date:** 2026-11-15. Local `main` and `origin/main` both point to `216d92d165cfa41e24704f29f79fdc22be246d10`; working-tree changes are uncommitted. The Tauri scaffold and local installed-manifest inspection prototype exist; no Steam client calls or achievement reads have been implemented. Rust and npm lockfiles are present, native tests and frontend production build pass, and the Tauri release executable builds without bundling. Installer packaging and runtime smoke testing remain unverified.

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
2. ADR-007 currently authorizes planning only. Before any read-only SAM-inspired adapter implementation, resolve applicable Valve policy questions and complete the native-boundary threat review, compatibility declaration, fail-closed behavior, and synthetic/mock test plan. See [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md) and ADR-007. The path is unsupported and owner direction is not evidence of Valve approval.
3. Approve the exact project license (recommended Apache-2.0) and confirm copyright holder before adding a root `LICENSE`.
4. Keep the four features mandatory. If permitted sources cannot support them, return to the owner for a product decision; do not remove or downgrade one.
5. `MASTER_PROMPT.md` records the owner's read-only points 1–3 direction for planning only. No SAM code/assets or write capability is approved.

## Phase 1 — Technical foundation (scaffold created; initial local inspection prototype)

Phase 1.7 readiness audit is complete. The owner authorized planning the read-only points 1–3 path; the initial scaffold now exists. Remaining gates and validation are:

1. Keep the Valve user-key inquiry open; the ready-to-send draft is in [`PHASE_1_7_DECISION_PACKAGE.md`](PHASE_1_7_DECISION_PACKAGE.md). Planning the local client path is separate and does not replace evaluating complete account data.
2. Complete the native boundary, Steam client compatibility and local schema parser design in [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md) and ADR-007 before code.
3. Select the project license and copyright holder; do not add a root `LICENSE` until approved.
4. Node.js was not globally upgraded because Windows Installer requested administrator consent; a checksum-verified portable Node `26.13.1` was downloaded to a user-local tools folder. npm is selected for this scaffold.
5. Confirm exact dependency versions/features and review licenses, MSRV, Windows build behavior and maintenance using [`FOUNDATION_READINESS.md`](FOUNDATION_READINESS.md).
6. Treat ADR-007 as planning authorization only; do not expand to writes or represent partial ownership as complete.

The 2026-11-12 environment audit found MSVC Build Tools, Windows SDK and WebView2 Runtime installed. `cargo tauri` is available through Cargo (`tauri-cli 2.12.4`), but a standalone `tauri` command and SQLite CLI are not on PATH. The 2026-11-15 unbundled Tauri release build passed; installer packaging and runtime smoke testing remain.

## Phase 2 — Minimal desktop scaffold (created; unbundled release build verified)

- Created the Tauri 2 + React/TypeScript + Rust structure in `app/`, branded it GameVault, removed the unused opener plugin, and set an initial restrictive CSP.
- Added a bounded read-only detector for conventional Steam roots, `libraryfolders.vdf` and installed `appmanifest` files. It canonicalizes roots, caps file/entry/library counts, rejects manifest paths outside the canonical library, and returns no absolute filesystem paths to the WebView. Schema detection checks file presence only; the scanner does not read schema bytes or connect to Steam.
- Added eight synthetic Rust scanner tests. `cargo fmt --check`, `cargo test --offline`, and `cargo clippy --all-targets --all-features --offline -- -D warnings` pass.
- Installed frontend dependencies with the existing portable Node/npm toolchain; `npm run build` and `cargo tauri build --no-bundle` pass.
- Add frontend formatting/lint/test scripts, review narrow Tauri permissions, and perform a desktop runtime smoke test plus installer packaging check.

## Phase 3 — Local database and installed-game discovery

- Further hardening can add explicit symlink-escape and directory-entry-limit test cases; current path/size/count/DTO protections and synthetic scanner tests are recorded in [`SECURITY.md`](SECURITY.md).
- Add versioned local SQLite migrations and Rust-owned repository.
- Implement a bounded, read-only, configured-root scanner only after approval of the Steam file boundary.
- Discover multiple libraries; handle unavailable/moved drives, partial manifests and scan failures without false “uninstalled” states.
- Keep local app data offline-useful and provenance-labelled.

## Phase 4 — Approved account data: library, playtime and achievements

- Plan the read-only Steam client/schema adapter under ADR-007. Implementation remains gated on Valve policy, native-boundary security, compatibility and synthetic/mock tests; fail closed on missing schema/unsupported interfaces and never call write methods.
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
- The owner has authorized planning read-only points 1–3 only. The current prototype implements only local install/manifest discovery; client connection and achievement reading remain unimplemented. This does not authorize adapter implementation, account/stat writes, SAM source reuse, or unsupported completeness claims.
