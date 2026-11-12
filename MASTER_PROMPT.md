# GameVault — Master Prompt for ChatGPT Work / Codex

> **Repository update (2026-11-12):** An initial Tauri scaffold is present in `app/`, with a read-only local Steam install/manifest detector. Steam client connection and achievement reads remain unimplemented; see `docs/SAM_READ_ONLY_INTEGRATION.md`. Earlier milestone-specific “no implementation” statements below describe those historical phases.

## How to use this document

Treat this file as the standing product and engineering brief for work in the GameVault project. At the start of each task:

1. Read this document and the repository's current `AGENTS.md`, README, architecture notes, and relevant source files.
2. Inspect the current local Git state, branch, remotes, and recent history before changing anything. Also inspect the GitHub repository when connected and relevant.
3. Preserve existing work. Do not overwrite, delete, rename, or move existing files without a clear task-specific reason. Never touch synced project reference files under `sources/`.
4. Work only on the task requested. Do not implement unrelated features or silently expand scope.
5. Report what changed, what you checked, any uncertainty or blocker, and the exact next step. Do not commit or push unless the user explicitly asks.

If repository state differs from the baseline below, trust the live repository for facts about files and Git state, and this document for the product direction unless the user updates it.

## Project identity

- **Product:** GameVault
- **Working tagline:** “Your Gaming Progress. One Place.”
- **Repository:** `https://github.com/GorillaKeks/GameVault.git`
- **Local project directory:** `C:\Projekte\GameVault`
- **Initial platform scope:** Steam only for v1. Do not build generic multi-platform abstractions or integrations now. Keep boundaries clean enough to revisit other providers later, only when requested.

The project name and tagline are working product choices, not a claim of trademark clearance. Before public release, investigate name, trademark, domain, and related-project conflicts and document the findings.

## Verified starting baseline (2026-11-12)

The initial inspection found:

- GitHub repository `GorillaKeks/GameVault` exists, is public and unarchived, has `main` as its default branch, and reports size 0. GitHub's contents API returned “This repository is empty.”
- `C:\Projekte\GameVault` exists and was empty at inspection time; it did not contain `.git` or project files. Git commands reported that it is not a repository.
- Available commands reported: Git 2.56.0, Node.js 24.22.1, npm 11.19.1, pnpm 11.25.0, Rust 1.99.1, Cargo 1.99.1, rustup 1.29.1, and .NET 9.0.319. `sqlite3` command-line utility was not available.
- No app implementation, dependencies, schema, tests, or repository conventions were present to infer from existing code.

Recheck these facts before setup because local and remote state can change. Do not assume a Tauri CLI, Windows build dependencies, Steam credentials, partner access, or SDK access are available just because the versions above were found.

## Product vision

Build a polished Windows desktop companion that helps a player understand and organize their Steam library and gaming progress in one place. Prioritize reliable, transparent data handling, a clear interface, and a safe relationship with Steam accounts and game state.

Potential v1 outcomes, subject to API, policy, and feasibility research:

- Connect or identify a Steam account through a documented, supported method.
- Show the player's library and useful game details.
- Display achievements and earned/locked status where data is actually available.
- Summarize playtime and progress, with calculations and missing/private data represented honestly.
- Allow local organization such as favorites and a recent-history view.
- Store only the local data needed for the product, with user-controlled refresh, export, backup, and deletion.

These are product goals, not promises that Steam exposes every requested field or supports every operation for every account/game. Mark unsupported or inaccessible data clearly. Never fabricate, scrape around, or imply official endorsement.

## v1 scope and exclusions

**In scope:** Windows desktop app; Steam only; library and progress presentation; local preferences and cache; basic local organization; documented import/export/backup paths; accessibility and responsive desktop layouts; privacy and security appropriate for a local account-connected application.

**Out of scope unless explicitly approved later:** other storefronts/platforms; cloud sync or hosted accounts; social network features; game launching/modification; automated gameplay or achievement manipulation; anti-cheat evasion; memory inspection, process injection or Steam credential/session-file access; broad Steam client file scraping; any achievement unlock/reset/alteration feature; and claims of Valve/Steam affiliation.

**Owner direction recorded 2026-11-12:** pursue only a read-only, SAM-inspired route for (1) connecting to the already logged-in local Steam client, (2) matching candidate AppIDs against that client, and (3) displaying achievement definitions/earned state. This is a research/implementation direction, not a claim that Valve supports or endorses the private interface, and not approval to copy SAM code/assets. Keep achievement/stat writes, resets, launch/modification, process injection, memory inspection, credentials, cookies and session files prohibited. The route, source limitations and remaining gates are documented in `docs/SAM_READ_ONLY_INTEGRATION.md`.

The read-only route still requires the SAM provenance/license/architecture audit and a Steam policy, security and compatibility review. Use a clean implementation unless exact-source reuse separately passes provenance and license review. Do not silently claim the SAM-hosted catalog is a complete owned library: complete account library and playtime remain subject to their own source gate. Do not start Steam integration code until the narrow read-only boundary and failure behavior are reflected in an ADR and reviewed against the security baseline.

## Architecture direction

Preferred starting stack, to validate against current supported versions and Windows build requirements before scaffolding:

- **Desktop shell and native boundary:** Tauri 2.
- **UI:** React + TypeScript, with strict type checking and accessible components.
- **Native application services:** Rust, exposed through narrow Tauri commands and well-defined interfaces.
- **Local persistence:** SQLite, accessed through a small Rust data-access layer and versioned migrations.
- **Steam connectivity:** a provider module that wraps only verified, permitted Steam interfaces. Prefer documented APIs and clear capability boundaries. Do not assume Steamworks SDK is a general-purpose client SDK or that a Steamworks partner account, publisher key, or privileged interface is available.

Keep UI, application/domain logic, Steam integration, persistence, and operating-system integration separate. Validate inputs at process boundaries. Minimize Tauri capabilities and permissions; expose narrowly scoped commands rather than broad filesystem or shell access. Avoid premature plugin frameworks and multi-platform abstractions.

Before selecting exact libraries, versions, API endpoints, authentication flow, or database plugin, check current primary documentation, support status, license, Windows compatibility, maintenance health, and security posture. Record decisions and alternatives in `docs/architecture/` or a concise ADR. Pin and update dependencies deliberately.

## Steam integration and authentication gate

Before coding account connection or data import:

1. Investigate current official Valve/Steam documentation and policies using primary sources. Distinguish Steamworks partner/developer APIs and SDKs from public Web APIs, community endpoints, client protocols, and local Steam client data.
2. For each desired field/action (profile identity, owned games, playtime, achievements, descriptions, icons, etc.), record the exact interface, access requirements, visibility/privacy constraints, key requirements, rate limits, terms/policy, and expected failure behavior.
3. Verify whether the interface is intended and permitted for this third-party desktop use. Do not claim SDK, endpoint, authentication, or account access that has not been confirmed.
4. Design account linking around a supported flow. Never ask for or store a Steam password. Never embed a private Web API key or other secret in the frontend, a public binary, or source control. If an official supported flow requires a server or secret that this local-first product does not have, state the limitation and propose options before implementation.
5. Treat account IDs and API responses as untrusted input. Handle private profiles, revoked access, network errors, missing metadata, throttling, and schema changes without data loss or misleading UI.
6. Keep refresh explicit and bounded; cache responsibly and respect published policy. Do not scrape pages, reverse engineer private protocols, bypass privacy settings, or evade rate limits.

Do not describe an interface as “officially supported” unless current primary documentation supports that exact claim. Cite and date research in project docs because policies and API availability can change.

## SAM-derived capability and license audit gate

SAM means Steam Achievement Manager or any code derived from it. Before copying, adapting, linking, or distributing any SAM code or behavior:

- Identify the exact upstream project, source revision, files, dependencies, and license. Read the actual license and notices; do not infer terms from a repository badge or third-party summary.
- Review contribution, attribution, trademark, redistribution, modification, and dependency obligations with a qualified reviewer when needed. Record provenance and required notices.
- Audit the architecture and behavior for process inspection, undocumented interfaces, writes to Steam/game state, anti-cheat or account risk, and compatibility with the product's safety boundaries and Steam policies.
- Assess whether the intended use is legal, permitted, supportable, and appropriate for a general consumer app. Keep this as an explicit unresolved question until evidence is gathered.
- Present a written go/no-go recommendation, alternatives (including a clean-room implementation of the explicitly owner-approved read-only path), and risks. Do not import SAM-derived code/assets before exact provenance/license review. Never implement SAM's write/reset behavior under the current read-only approval.

Never claim that an achievement action is safe, undetectable, endorsed, or policy-compliant without strong current evidence. Do not implement anti-cheat bypass or stealth behavior.

## Security, privacy, and data integrity

- Follow least privilege in Tauri; minimize permissions, plugins, IPC commands, and filesystem access.
- Keep secrets out of source control, logs, telemetry, crash reports, frontend bundles, and distributable binaries. Prefer OS-protected storage for any credential/token only if the chosen supported flow requires one; document lifecycle, scope, revocation, and deletion.
- Do not collect analytics or transmit local data by default. Clearly document any network calls and data sent. Obtain user consent where needed and provide meaningful controls.
- Use parameterized SQL, safe migrations, transaction boundaries, and constraints. Back up before destructive migrations and never silently discard user data.
- Validate and bound external data, file imports, paths, and user input. Avoid unsafe deserialization and shell execution.
- Use HTTPS for network calls, validate server identity through standard platform libraries, apply timeouts, and handle throttling and errors safely.
- Provide clear actions to refresh, export, back up, disconnect, and delete local data as features mature. Include a retention model.
- Do not log passwords, tokens, API keys, full sensitive payloads, or unnecessary identifiers.
- Maintain dependency review, vulnerability response, and secure release practices. Do not invent a security certification.

## Licensing, attribution, and branding

- Choose and document GameVault's own project license before distributing code; do not assume a license. Ensure all contributors and assets are compatible with it.
- Track third-party code, packages, fonts, icons, artwork, and other assets with exact source, version, license, required notices, and modifications. Preserve notices and provide attribution where required.
- Do not copy Steam or game assets into the repository unless their use and redistribution are permitted. Prefer user-facing assets from documented endpoints only where allowed, and cache them according to applicable terms.
- Use Steam/Valve names and marks descriptively and accurately. Add a clear non-affiliation statement if appropriate; do not suggest endorsement.
- Audit SAM and each dependency separately; the project license does not override upstream obligations.

## Development milestones

Complete milestones in order. Keep each one reviewable and avoid feature work ahead of its prerequisites.

### 0 — Repository and feasibility baseline

- Reconfirm local and remote state, permissions, and intended branch/workflow.
- Decide how to connect the empty local directory to the empty GitHub repository without overwriting anything.
- Research Steam interfaces/authentication/policy; complete SAM provenance/license/architecture audit; identify project license and branding questions.
- Produce a short decision log with sources, open questions, and go/no-go items. No application feature implementation during this milestone.

### 1 — Product and technical foundation

- Agree v1 requirements and non-goals.
- Validate Tauri 2 + React + TypeScript + Rust + SQLite compatibility, Windows prerequisites, packaging approach, and dependency choices.
- Scaffold only after the foundation choices are recorded. Add formatting, linting, type checks, Rust checks, migration strategy, and minimal app-shell smoke coverage.

### 2 — Local-first shell and persistence

- Establish navigation, visual system, settings, database lifecycle, migrations, and sample/demo data clearly separated from real Steam data.
- Add backup/export and deletion behavior with documented guarantees.

### 3 — Read-only Steam connection and library

- Implement only the approved, documented integration path. Handle privacy restrictions and unavailable fields gracefully. Verify offline and error states.

### 4 — Achievements and progress presentation

- Add achievement metadata/progress only where an approved interface provides it. Keep all features read-only unless a separate explicit review approves more.

### 5 — Polish and release readiness

- Accessibility, localization foundation, performance, Windows packaging/signing options, update strategy, privacy/licensing notices, support docs, and release checklist.

Milestones may be reordered only with a documented reason. Do not imply completion when a gate or acceptance criterion is still open.

## Testing and quality expectations

For each code change, choose checks that match its risk and report exact commands and results. Do not claim tests were run if they were not.

- **Frontend:** formatting, lint, TypeScript checks, component tests for meaningful behavior, and accessibility review.
- **Rust:** formatting, clippy, unit tests, and boundary/error-path tests.
- **Database:** migration tests from empty and prior schemas, transaction and rollback behavior, import/export round trips, and backup/restore verification.
- **Integration:** mock Steam responses for success, private profile, missing fields, throttling, offline, revoked/invalid credentials, and malformed data. Keep tests deterministic and do not use real account credentials in CI.
- **Desktop:** Tauri command permission checks, Windows smoke tests, clean install/update/uninstall and data-retention checks before release.
- **Security:** review IPC capabilities, secret handling, SQL, file paths, dependencies, and release artifacts. Add automated scanners only with reviewed configuration and actionable handling.

Never use a real Steam account to test risky behavior. Do not test achievement writes, resets, or anti-cheat-sensitive behavior.

## Documentation expectations

Keep docs concise and synchronized with implementation. As appropriate, maintain:

- `README.md`: product summary, status, prerequisites, safe setup/run instructions, scope, and known limitations.
- `docs/architecture/`: component/data-flow overview and architecture decisions.
- `docs/research/steam-integration.md`: dated source links, endpoint-by-endpoint access/policy matrix, authentication design, and unresolved questions.
- `docs/research/sam-audit.md`: upstream identity/revision, license and provenance findings, architecture/safety findings, and explicit decision.
- `docs/security/`: threat model, data inventory, secrets and retention, vulnerability reporting, and release security notes.
- `THIRD_PARTY_NOTICES.md` or equivalent: code and asset attributions and license notices.
- `CHANGELOG.md`: user-visible changes when release work begins.

Do not publish unsupported claims or leave dead links. Date policy/API research and revisit it before release.

## Git and collaboration workflow

- Inspect `git status`, current branch, remotes, and recent commits before every task that may write files.
- Keep changes small, focused, and reviewable. Preserve unrelated user changes. Do not use destructive reset/clean/checkout operations to “fix” a dirty tree.
- Do not commit, push, create releases, or change GitHub settings unless the user explicitly asks for that action. A request to edit files is not a request to commit them.
- Before a commit the user requested, summarize the intended files and confirm no unrelated changes are included; use a clear, scoped message.
- Never place credentials, private keys, tokens, personal account exports, or local database files in Git. Maintain an appropriate `.gitignore` once project setup begins.
- When reconciling local and remote work, inspect both histories and contents before merging or copying. Never replace one side wholesale without explicit authorization.

## Working style and response format

Be direct, careful, and transparent. Separate verified facts from assumptions and recommendations. Ask only when a decision blocks safe progress; otherwise state a reasonable assumption and continue. For tasks involving changing Steam, account data, licensing, or destructive data operations, stop at the relevant review gate and explain the evidence and decision needed.

For each task, finish with:

1. **Findings:** verified state and relevant assumptions.
2. **Changes:** files or behavior changed.
3. **Checks:** what was checked and the actual outcome.
4. **Blockers/risks:** unresolved items that affect the next step.
5. **Next steps:** the smallest concrete actions to continue.

For the initial project task, create planning and research artifacts only. Do not implement application features until the user explicitly moves the project into implementation and prerequisite gates are resolved.
