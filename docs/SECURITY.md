# Security and Privacy Baseline

**Status:** security requirements with an experimental read-only Steam-client prototype; no dedicated security suite or live ABI review exists yet. **Updated:** 2026-11-15.

## Data and trust boundaries

Expected data, if future gates permit it: SteamID and public profile fields, library/app identifiers, playtime and achievement snapshots, and local favorites/settings. Treat all API responses, imported files, URLs, paths, and UI values as untrusted. The UI/WebView is a less-trusted boundary than the Rust core. Tauri's capability system can constrain frontend access, but cannot correct unsafe Rust code or loose permission scopes ([Tauri security boundaries](https://v2.tauri.app/security/capabilities/)).

No telemetry, analytics, account service, or network data sharing is approved by default. Before connecting Steam, document each request, purpose, identifier, retention period, error behavior, and privacy notice.

## Credentials and secrets

- Never collect a Steam password, session cookie, or authenticator code.
- Never place a shared Web API key or private signing key in frontend code, a public binary, Git, logs, crash output, or test fixtures.
- Do not implement authentication until an approved flow and secret boundary exist. An OS credential store does not solve the distribution of a shared API key.
- If a future approved design needs a user token, use the operating system's protected credential facility through Rust, minimize scope, provide revoke/delete behavior, and test that secrets are redacted.
- Keep release signing keys offline or in a controlled secret manager with recovery and rotation planning.

## Tauri and OS access

- Start with no broad filesystem, shell, process, or HTTP plugin capabilities.
- Define explicit capability files and allow only commands required by the main window. Review command registration and plugin permissions together.
- Validate all IPC input and enforce bounds/allow-lists in Rust.
- Do not expose arbitrary filesystem paths or spawn a shell from a UI command.
- Phase 1.6 authorizes research for a narrow read-only Steam library scanner, not broad Steam-file import. If later implemented, require configured/user-selected roots, canonicalize paths, read only known manifests/library metadata, defend against path traversal, bound file sizes/depth, and document that Steam provides no third-party schema contract. Do not recursively scan saves or arbitrary game/user data.

### Current local scanner review (2026-11-15)

The prototype remains read-only. The scanner now caps `libraryfolders.vdf` at 64 KiB, limits candidate libraries to 64, limits total directory entries to 65,536 and manifest files to 4,096 per scan, and caps each manifest at 1 MiB. Steam and library roots, `steamapps`, and manifest files are canonicalized; linked `steamapps` or manifest paths that resolve outside their expected parent are rejected. Absolute Steam-root and manifest paths were removed from the IPC DTO, and the UI reports when a scan is incomplete.

Seventeen Rust tests now cover manifest/icon scanning and synthetic read-only achievement mapping (earned/locked, partial-state failure, app-ID validation and count limits). `cargo test --offline`, `cargo fmt --check` and Clippy pass. The achievement tests do not validate the native Steam DLL ABI or callbacks; no real account/client was used. Add explicit symlink-escape and directory-entry-limit scanner cases before increasing scanner scope.

## Network and external data

- HTTPS with standard certificate validation, finite timeouts, bounded response sizes, content-type checks, and safe JSON parsing.
- Map errors without logging keys or full sensitive responses. Apply conservative request volume and respect any published limits.
- Treat game names/descriptions and profile strings as untrusted display content; escape/render as text and do not allow active HTML.
- Do not bypass a private profile or use process/memory inspection. The owner explicitly accepted the risks for the narrow experimental read-only SAM-inspired Steam-client path documented in [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md); treat all native results as untrusted/unstable, fail closed, and never call write/reset functions. Achievement reads are user-triggered and sent through the signed-in Steam client; no GameVault backend or telemetry endpoint is used.

## Persistence and user controls

- Keep the database under the per-user app data directory with normal user-only filesystem permissions.
- Use parameterized SQL, transactions, versioned migrations, and atomic backup/restore procedures.
- Keep only data needed by an agreed feature; retain source and last-refresh metadata where relevant.
- Before data collection ships, provide controls to refresh, export, disconnect, and delete as applicable. Define what disconnecting deletes and what local preferences remain.
- Back up before destructive migrations and test restoration; do not silently overwrite or discard data.

## Logging, dependencies, release

- Redact tokens, keys, cookies, sensitive payloads, and unnecessary account identifiers.
- Review direct and transitive dependency licenses, advisories, maintenance, and permissions before selection and release.
- Produce reproducible/reviewable release artifacts where practical; sign Windows releases where feasible and explain unsigned builds honestly.
- If enabling the Tauri updater, require signed artifacts and protect its private signing key ([Tauri updater security](https://v2.tauri.app/plugin/updater/)).
- Establish a vulnerability reporting contact before public release; do not promise a certification or security guarantee.

## Threat scenarios and planned checks

| Threat | Minimum mitigation/check |
|---|---|
| Malicious web content or frontend compromise | Tight CSP where compatible, minimal Tauri capabilities, narrow commands, no arbitrary path/shell access |
| Leaked Steam API key | Do not distribute a shared key; gate integration on compliant key architecture |
| Malformed/oversized API response | Bound response size, typed parsing, fuzz/negative tests, graceful errors |
| SQL injection or corrupt migration | Parameterized queries, migration tests, transactions, backup/restore test |
| Sensitive diagnostics | Redaction tests for keys/tokens/IDs and review structured logs |
| Malformed local Steam metadata | Read only known files below configured roots, canonicalize paths, bound size/depth, never log raw content; parser negative tests |
| Compromised update | Signed update artifacts, protected key, release verification and recovery plan |

No dedicated security test suite has been run. The current unit tests are deterministic and synthetic; they do not exercise the native DLL/callback boundary.

## Phase 1.5 additions (2026-11-12)

Steam OpenID proves a Steam identity only after a relying party validates the assertion; it is not API authorization. A website flow is not selected because the owner prohibits a GameVault server. Any future web research must still validate exact `return_to`, discovery/signature and nonce replay protection; never trust a callback SteamID before validation.

No Steam Web API key may be embedded in the executable, frontend, source or release artifact. A GameVault backend is prohibited. An individual user key stored in Windows Credential Manager/DPAPI and used directly by Rust remains a candidate pending Valve confirmation. Do not request or persist user keys before approval. The read-only Steam-client achievement prototype is owner-authorized despite unresolved policy and compatibility questions; its release remains blocked pending native ABI/callback review and a live compatibility plan. See [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md). Never bind or call achievement/stat mutation or reset functions; never inject into a game, inspect process memory, or read credentials/session files.

Privacy states must distinguish `private/unavailable`, `unsupported`, `missing`, `stale`, `error`, and numeric zero. Private data, network failure, absent schema and missing installation must not be rendered as empty ownership, zero playtime, all achievements locked or “not installed.” Preserve the last permitted cache with provenance/timestamp and redacted diagnostics.

The current threat and degradation assessment is in [`LOCAL_FIRST_ARCHITECTURE.md`](LOCAL_FIRST_ARCHITECTURE.md) and [`V1_FEATURE_FEASIBILITY.md`](V1_FEATURE_FEASIBILITY.md). These are proposed controls, not tested properties.
