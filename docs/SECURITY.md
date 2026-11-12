# Security and Privacy Baseline

**Status:** design requirements; no application or security tests exist yet. **Date:** 2026-11-12.

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
- If local Steam library import is proposed, require explicit user selection, canonicalize paths, read only allowed files, defend against symlink/path traversal, bound file sizes, and document that this is a local-file integration. This is not authorized in the current scope.

## Network and external data

- HTTPS with standard certificate validation, finite timeouts, bounded response sizes, content-type checks, and safe JSON parsing.
- Map errors without logging keys or full sensitive responses. Apply conservative request volume and respect any published limits.
- Treat game names/descriptions and profile strings as untrusted display content; escape/render as text and do not allow active HTML.
- Do not bypass a private profile, reverse-engineer undocumented interfaces, or use process/memory inspection.

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
| Malicious import/local file | No import until separately scoped; path and size validation, read-only access, hostile fixture tests |
| Compromised update | Signed update artifacts, protected key, release verification and recovery plan |

No security tests have been run because no application exists.

## Phase 1.5 additions (2026-11-12)

Steam OpenID proves a Steam identity only after a relying party validates the assertion; it is not API authorization. Any future website flow must use a fixed Steam OP endpoint, a maintained library, exact `return_to` and discovery/signature checks, nonce replay defense, TLS and a one-time state tied to the initiated session. Never trust a callback SteamID before validation. If the desktop receives a GameVault session, keep it short-lived/revocable and do not place it in URLs, logs or crash output.

No Steam Web API key may be embedded in the executable, frontend, source or release artifact. A backend-held key is only a candidate if Valve confirms the registration/application/host design and the owner approves hosted operations. If there is no approved secret boundary, disable player-data calls. Do not ask users for personal keys or parse Steam client files on assumption.

Privacy states must distinguish `private/unavailable`, `unsupported`, `missing`, `stale`, `error`, and numeric zero. Private data, network failure, absent schema and missing installation must not be rendered as empty ownership, zero playtime, all achievements locked or “not installed.” Preserve the last permitted cache with provenance/timestamp and redacted diagnostics.

The detailed threat and degradation assessment is in [`FEASIBILITY_REVIEW.md`](FEASIBILITY_REVIEW.md). These are proposed controls, not tested properties.
