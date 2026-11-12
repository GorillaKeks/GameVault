# GameVault Architecture Proposal

**Status:** Proposed; no application code exists. **Research date:** 2026-11-12.

## Goals and constraints

- Windows desktop, Steam only for v1.
- Local-first, read-only Steam integration unless separately approved.
- Do not make account linking or Steam data availability assumptions before the API access gate is cleared.
- Keep Steam integration isolated without introducing a general multi-platform plugin system.
- Keep all frontend-to-native operations explicit and narrowly scoped.

## Proposed component structure

```text
React + TypeScript UI
        │ typed DTOs over Tauri IPC
        ▼
Rust application layer ───── Steam read-only adapter (only approved API calls)
        │
        ├── domain and progress calculations
        ├── repository / migration layer
        ├── bounded network client
        └── diagnostics with redaction
        ▼
SQLite in the per-user application data directory
```

Suggested eventual repository layout (not created yet):

```text
src/                         React UI, views, components, typed IPC client
src-tauri/src/app/            application services and commands
src-tauri/src/domain/         domain types and deterministic calculations
src-tauri/src/steam/          narrow Steam adapter and API DTOs
src-tauri/src/storage/        SQLite repository and versioned migrations
src-tauri/capabilities/       least-privilege Tauri permission declarations
docs/                         decisions, research, security and user docs
```

## Responsibilities and boundaries

- **React UI:** rendering, accessible interaction, view state, and display of explicit unavailable/private/error states. No API keys, direct SQLite access, or raw filesystem authority.
- **Tauri commands:** small typed operations such as reading cached library data or requesting a refresh. Validate every parameter and returned DTO. Avoid generic command dispatch and arbitrary path access.
- **Rust application layer:** orchestration, validation, domain calculations, throttling/timeouts, error mapping, and secrets boundary if an approved flow later requires secrets.
- **Steam adapter:** only documented and approved interfaces, with typed responses and provider-specific errors. It must not inspect processes, memory, or undocumented protocols.
- **Storage:** SQLite behind Rust repository methods. Use schema migrations, parameterized statements, transactions, stable identifiers, and backup/restore verification. Store a minimal cache and distinguish remote facts from local user annotations.
- **Logging:** structured and useful for diagnosis, but redact account identifiers where unnecessary and never log API keys, credentials, tokens, cookies, or full profile payloads.

Tauri capabilities grant permissions to frontend windows/webviews; narrow them to the main window and required operations. Commands registered in Rust and plugin permissions still need deliberate review. Capabilities do not make unsafe Rust code safe. See Tauri's [capability model](https://v2.tauri.app/security/capabilities/).

## Data model sketch (not approved schema)

After Steam access is validated, likely local entities are:

- `steam_account`: SteamID64, display metadata needed for the UI, last successful refresh time; avoid storing an authentication secret here.
- `game`: Steam AppID, display name and any metadata allowed by source terms.
- `library_entry`: account/game relationship, owned/played facts and provenance/last refresh.
- `achievement`: game achievement identifier and display metadata, only where a supported data source supplies it.
- `achievement_state`: earned status and source timestamp, only for supported read-only data.
- `user_game_preference`: favorites and local-only organization.
- `schema_migrations`: ordered migration version and application time.

This is a discussion model, not an implementation decision. Do not treat missing/private data as zero or as a locked achievement. Track source and freshness where useful. Use SQLite's documented embedded/local-storage role as a fit for a single-user desktop app ([SQLite appropriate uses](https://www.sqlite.org/whentouse.html)).

## Integration and auth design

Current research shows Steam's public Web API uses a key, while the API terms require the key be confidential and tie use to the application identified at key signup. Some interfaces are explicitly intended for trusted backend use. A key cannot be embedded in the distributed desktop binary. Do not add a proxy/backend without a separately approved privacy, hosting, cost, and operational design. See [Steam integration research](STEAM_INTEGRATION.md).

Steam OpenID is documented for third-party websites to authenticate a SteamID without collecting a password. A browser-based desktop redirect/callback architecture for this local-only app has not been validated. OpenID identity alone does not guarantee access to private game details or achievements. Do not implement until Valve documentation and the terms are checked against the exact design.

## Database and migration strategy

- Keep SQLite access in Rust, not in arbitrary UI queries.
- Use a maintained library selected after checking license, Windows support, async/runtime fit, and security posture.
- Store the database in Tauri's per-app data directory, not beside the executable or in the repository.
- Use ordered migrations, transactions, integrity checks, and tests from both empty and prior versions.
- Before any destructive migration, create and verify a backup. Never silently discard cached/user-authored data.
- No schema is created in this planning phase.

## Errors, diagnostics, and update model

Map errors into stable user-facing categories: private/unavailable data, offline, throttled, invalid response, local storage failure, and configuration/auth failure. Preserve diagnostic context without exposing secrets. Add timeouts and bounded retries only for retryable operations; honor API throttling.

Automatic updates are not a foundation requirement. If adopted later, use signed artifacts and protect the private signing key; Tauri's updater requires signatures and recommends private-key secrecy ([updater docs](https://v2.tauri.app/plugin/updater/)). Decide release hosting, signing, rollback, and update consent before enabling it.

## Packaging and deployment

Target Windows x64 first. Tauri's current prerequisites include the MSVC C++ Build Tools, WebView2, Rust MSVC toolchain, and Node for a JS frontend; verify the local machine before setup ([Tauri Windows prerequisites](https://v2.tauri.app/start/prerequisites/)). Choose NSIS or MSI only after installer/update requirements are clear. No package format, code-signing certificate, or updater endpoint has been selected.

## Architecture decisions still open

1. Steam data access/authentication strategy and API key ownership.
2. Whether any local Steam-file import is in scope; current master brief excludes scraping and it must not be added by assumption.
3. Rust SQLite library and migration mechanism.
4. Project license, third-party asset strategy, and application name clearance.
5. Installer, update, signing, and support strategy.
