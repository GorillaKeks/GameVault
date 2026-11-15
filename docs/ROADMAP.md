# GameVault Roadmap

**Status date:** 2026-11-15. Local `main` and `origin/main` point to `01e6feaf697e54df92ce5f0d81e075abd9a5347f`; working-tree changes are uncommitted. The Tauri scaffold includes an experimental per-game read-only Steam-client achievement prototype. It has no live ABI/account validation; Rust tests, frontend production build and unbundled Tauri debug build pass. Installer packaging and runtime smoke testing remain unverified.

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
2. ADR-007 records the owner's explicit risk acceptance and prototype authorization. Valve-policy and Steam-client compatibility remain unresolved; do not claim Valve approval. Keep the path opt-in/read-only, expand synthetic callback/native-seam tests, and require a separate live smoke-test plan before any real-account validation. See [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md) and ADR-007.
3. Approve the exact project license (recommended Apache-2.0) and confirm copyright holder before adding a root `LICENSE`.
4. Keep the four features mandatory. If permitted sources cannot support them, return to the owner for a product decision; do not remove or downgrade one.
5. `MASTER_PROMPT.md` records the experimental read-only implementation authorization. No SAM code/assets or write capability is approved.

## Phase 1 — Technical foundation (scaffold created; initial local inspection prototype)

Phase 1.7 readiness audit is complete. The owner now authorizes an experimental read-only points 1–3 prototype under accepted policy/compatibility risk; the initial implementation exists. Remaining gates and validation are:

1. Keep the Valve user-key inquiry open; the ready-to-send draft is in [`PHASE_1_7_DECISION_PACKAGE.md`](PHASE_1_7_DECISION_PACKAGE.md). Planning the local client path is separate and does not replace evaluating complete account data.
2. Complete mock callback/ABI tests and review the actual DLL loader and interface boundary in [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md) and ADR-007; do not test with a real account without a separately reviewed plan.
3. Select the project license and copyright holder; do not add a root `LICENSE` until approved.
4. Node.js was not globally upgraded because Windows Installer requested administrator consent; a checksum-verified portable Node `26.13.1` was downloaded to a user-local tools folder. npm is selected for this scaffold.
5. Confirm exact dependency versions/features and review licenses, MSRV, Windows build behavior and maintenance using [`FOUNDATION_READINESS.md`](FOUNDATION_READINESS.md).
6. Treat the achievement reader as experimental and unsupported; do not expand to writes or represent partial ownership as complete.

The 2026-11-12 environment audit found MSVC Build Tools, Windows SDK and WebView2 Runtime installed. `cargo tauri` is available through Cargo (`tauri-cli 2.12.4`), but a standalone `tauri` command and SQLite CLI are not on PATH. The 2026-11-15 unbundled Tauri release build passed; installer packaging and runtime smoke testing remain.

## Phase 2 — Minimal desktop scaffold (created; unbundled release build verified)

- Created the Tauri 2 + React/TypeScript + Rust structure in `app/`, branded it GameVault, removed the unused opener plugin, and set an initial restrictive CSP.
- Added a bounded read-only detector for conventional Steam roots, `libraryfolders.vdf` and installed `appmanifest` files. It canonicalizes roots, caps file/entry/library counts, rejects manifest paths outside the canonical library, and returns no absolute filesystem paths to the WebView. Schema detection checks file presence only; the scanner does not read schema bytes or connect to Steam.
- Added a bounded local Steam icon-cache lookup for each detected AppID. It reads only one small, unambiguous JPEG from that game's canonical `appcache/librarycache/<appid>` directory, returns bytes without paths, makes no network request, and uses initials if no valid icon is available.
- Added eleven synthetic Rust scanner/icon tests. `cargo fmt --check`, `cargo test --offline`, and `cargo clippy --all-targets --all-features --offline -- -D warnings` pass.
- Installed frontend dependencies with the existing portable Node/npm toolchain; `npm run build` and `cargo tauri build --no-bundle` pass.
- Add frontend formatting/lint/test scripts and review narrow Tauri permissions. A manual runtime scan/icon check passed in the Windows debug build; installer packaging remains unchecked.
- The icon source is the user's local Steam cache. Remote fetching and packaged artwork remain gated on a documented source and verified usage conditions. See [`STEAM_INTEGRATION.md`](STEAM_INTEGRATION.md).

## Phase 3 — Local database and installed-game discovery

- Further hardening can add explicit symlink-escape and directory-entry-limit test cases; current path/size/count/DTO protections and synthetic scanner tests are recorded in [`SECURITY.md`](SECURITY.md).
- Add versioned local SQLite migrations and Rust-owned repository.
- Implement a bounded, read-only, configured-root scanner only after approval of the Steam file boundary.
- Discover multiple libraries; handle unavailable/moved drives, partial manifests and scan failures without false “uninstalled” states.
- Keep local app data offline-useful and provenance-labelled.

## Phase 4 — Approved account data: library, playtime and achievements

- Continue the experimental read-only per-game Steam-client achievement view under ADR-007. Fail closed on missing interface, timeout or incomplete state; never call write methods. Live client compatibility and Valve policy remain unresolved.
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
- The owner has explicitly accepted the residual private-interface risk and authorized a read-only achievement prototype. It has not been live-tested and does not claim Valve support, complete library/playtime coverage, account/stat writes or SAM source reuse.
