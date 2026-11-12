# GameVault — Phase 1.7: Decision Gate & Technical Foundation Readiness

## Mission

Continue the GameVault project from the completed Phase 1.6 local-first feasibility review. Produce a decision-ready, evidence-based package that identifies what can proceed, what requires an owner decision, and what remains blocked before the minimal desktop scaffold. This is a planning and readiness phase only.

Do not implement application features, create the app scaffold, install or upgrade dependencies, contact Valve, submit forms, choose a license on the owner's behalf, commit, or push. Do not ask the user to repeat decisions already recorded in the current repository. Prepare concrete questions and recommendations for decisions that remain open.

## Repository and project

- Local project: `C:\Projekte\GameVault`
- GitHub: `https://github.com/GorillaKeks/GameVault.git`
- Expected committer/pusher when a future task explicitly authorizes it: **GorillaKeks**
- Product: Windows desktop app; Steam only for v1; free and open source.
- Owner-confirmed architecture constraints: no GameVault backend/server/account/cloud database; Tauri 2, React, TypeScript, Rust and SQLite.
- Four mandatory v1 capabilities remain: complete Steam account library, installed-game detection, playtime and achievements. Do not remove, weaken or relabel a partial substitute as a complete feature.

## Required first steps

1. Read `MASTER_PROMPT.md`, `README.md`, `docs/ROADMAP.md`, `docs/V1_FEATURE_FEASIBILITY.md`, `docs/LOCAL_FIRST_ARCHITECTURE.md`, `docs/LOCAL_STEAM_DATA.md`, `docs/STEAM_INTEGRATION.md`, `docs/SECURITY.md`, `docs/LICENSING.md`, `docs/OPEN_SOURCE_LICENSE_RECOMMENDATION.md`, and all six ADRs.
2. Inspect Git status, branch, remotes, recent history, and the GitHub repository's current metadata/files. Preserve unrelated changes. Do not use destructive Git commands.
3. Reconcile the live state against the docs. Treat the live repository as authoritative for what files and commits exist; treat explicit owner decisions in this brief and prior project records as authoritative product constraints.
4. Note the known conflict: the older `MASTER_PROMPT.md` excludes Steam client-file scraping, while Phase 1.6 authorized research/design of narrow, read-only VDF/ACF scanning. Do not edit the master prompt silently. Recommend a precise amendment or ask the owner to resolve the contradiction before any scanner implementation.

## Workstream A — Steam access decision readiness

Recheck current primary Valve documentation and API terms because they may change. Use dated citations and quote no more than needed. Reassess only the unresolved no-backend questions:

- Can a distributed, free, open-source Windows desktop app use an individual end user's own standard Steam Web API key directly from that user's computer?
- What application/domain registration and key confidentiality rules apply? Is a user entering their own key into GameVault considered disclosure to a third party?
- Which host and exact methods are valid with a standard user key: `GetOwnedGames`, `GetRecentlyPlayedGames`, `GetSingleGamePlaytime`, `GetPlayerAchievements`, and `GetSchemaForGame`?
- What privacy visibility, caching, retention, user-request, attribution, rate, and deletion requirements apply?
- Is there a documented local/client API for an independent desktop companion to read achievements without impersonating a game or accessing process memory?

Do not infer approval from endpoint examples or from the fact that a method accepts a key. Do not make API calls, request/handle keys, or contact Valve. Prepare a ready-to-send, concise Valve support inquiry and identify the official submission route only if it is documented. Mark all unanswered points as unanswered.

Preserve the Phase 1.6 classification unless new primary-source evidence justifies a documented correction:

- Complete Steam account library: **Conditional on Steam access**.
- Installed games: **Feasible with limitations**.
- Complete account-wide playtime: **Conditional on Steam access**.
- Reliable cross-game achievements: **Blocked by unresolved Steam restrictions**.

If direct key use is not confirmed, do not substitute unsupported binary-cache parsing, deprecated Community XML, a backend, scraping, Steam-client DLL access, process memory, credentials, cookies, or session tokens. State that the mandatory-feature project gate remains unresolved and request an owner decision; do not quietly reduce v1 scope.

## Workstream B — License and ownership decision package

Recheck primary license texts and reliable project guidance as needed. Compare at least Apache-2.0 and MIT, and explain practical differences relevant to this Rust/TypeScript desktop project, including patent grant/retaliation, attribution/notices, warranty disclaimer, and compatibility with dependencies. Verify any claims against the actual license texts.

Do not add a root `LICENSE` or choose a copyright holder without owner approval. Keep `docs/license-candidate/LICENSE-APACHE-2.0.proposed.txt` clearly non-operative until approval. Present a short recommendation and the exact owner decisions needed:

1. Select the project license (recommend Apache-2.0, MIT, or another named option).
2. Confirm the copyright holder name to place in the file.
3. Confirm whether the owner wants the exact canonical license text added in a later authorized task.

Do not present the recommendation as a legal opinion.

## Workstream C — Phase 1 foundation readiness (audit only)

Inspect the currently accessible Windows development environment without changing it. Record actual versions/availability for Node.js, npm/pnpm/yarn, Rust/Cargo/rustup targets, Tauri CLI (if present), MSVC compiler, Windows SDK, WebView2 runtime, Git, and SQLite tooling. Check current official Tauri 2 and Microsoft prerequisites; do not install, update, configure, or build anything.

Propose a small, maintainable version/dependency selection matrix for the eventual scaffold. Verify candidate versions, support status, licenses, Windows compatibility, maintenance and Rust MSRV from primary or authoritative sources. Do not pin versions as approved decisions. Include:

- package manager choice and rationale;
- Tauri 2 / React / TypeScript / Rust versions or selection policy;
- SQLite access approach owned by Rust, with migrations;
- essential formatting, lint, type-check, Rust-check and test commands;
- minimum Tauri capabilities and IPC boundary;
- known Windows prerequisites and missing items.

No package installation, project generation, feature code, database, real Steam data access, or build is permitted in this phase.

## Deliverables

Create or update only planning documents needed for this phase, preserving existing work:

1. `docs/PHASE_1_7_DECISION_PACKAGE.md` — concise executive status, current evidence, decision table, risks, owner questions, and a ready-to-send Valve inquiry.
2. `docs/FOUNDATION_READINESS.md` — observed machine prerequisites, a non-binding technology/dependency proposal, primary references, and exact missing prerequisites.
3. Update `docs/ROADMAP.md` to show Phase 1.7 status and preserve the Phase 2 scaffold as explicitly awaiting owner authorization and resolution of prerequisite gates.
4. Update relevant ADRs only where new evidence changes the decision record. Do not mark an ADR Accepted for a decision the owner has not made.
5. Update README links/status only if needed to keep the project overview accurate.

Do not edit application code, `MASTER_PROMPT.md`, the proposed license marker, or synced/reference files unless a verified factual correction is essential. If an essential change conflicts with this restriction, document the proposed change instead of applying it.

## Quality and Git workflow

- Keep verified facts, owner decisions, recommendations and open assumptions distinct.
- Cite and date current policy, API and tooling research. Prefer Valve, Tauri, Microsoft, Rust and package-maintainer primary sources.
- Do not claim support, permission, completeness, test success, or build success without direct evidence.
- Check local Markdown links and `git diff --check`. Do not run application tests or builds because no application exists and this is a documentation/readiness phase.
- Review the final diff and ensure no secrets, Steam identifiers, personal paths, account exports or unrelated files were added.
- Do not commit or push. Report exact files changed, evidence sources, checks performed, unresolved blockers, and the smallest next step.

## Exit criteria

Phase 1.7 is complete when:

- the current Steam key/host/policy questions are either answered by authoritative evidence or clearly listed as pending Valve confirmation;
- the owner has a concise, ready-to-send Valve inquiry and a clear choice list for unresolved product gates;
- license options and the exact owner approval needed are documented without making the choice for them;
- the real Windows toolchain state and scaffold prerequisites are recorded without modifying the environment;
- the plan for Phase 2 is explicit, minimal and still gated on owner authorization;
- the four mandatory v1 capabilities and no-backend constraint remain unchanged.

If an answer from Valve or an owner decision is required to proceed, finish the independent readiness work and leave a concrete decision request. Do not invent an answer or begin Phase 2.
