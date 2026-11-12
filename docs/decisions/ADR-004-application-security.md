# ADR-004: Application Security and Privacy Baseline

**Status:** Proposed
**Date:** 2026-11-12

## Context

The future app combines a web UI, native Rust commands and local database; it may make direct user-approved Steam requests, but must not depend on a GameVault service. No app code or data collection exists yet.

## Problem

What security boundaries are required before a desktop shell or Steam connection is implemented?

## Considered alternatives

- Broad Tauri filesystem/shell/network permissions for frontend convenience: rejected; unnecessary authority increases impact.
- Narrow capabilities and typed domain commands: recommended least-privilege baseline.
- Embed an API key in binary: rejected; binary secrets are extractable and Valve requires confidentiality.
- User-provided key in local plaintext settings: rejected; unnecessary persistent exposure.
- User-provided key in Windows Credential Manager/DPAPI: candidate only if Valve confirms that app/user-key use is permitted; same-user malware or app compromise can still read it while in use.

## Decision

**Recommendation:** least-privilege Tauri capabilities; narrow typed Rust commands; no arbitrary SQL, shell, process or frontend filesystem access. Rust may read only configured Steam roots and the narrowly scoped metadata files described by `../LOCAL_STEAM_DATA.md`; all file reads remain read-only and bounded. Validate IPC, paths, VDF and network data. HTTPS, finite timeouts, response limits and safe parsing. No telemetry, uploads or elevated privileges. A user Web API key is not collected until Valve approves the flow; if approved, store via Windows-protected credentials, never SQLite/logs/frontend, and send only direct to Steam.

## Consequences

Frontend compromise has fewer native capabilities. File parsing increases local privacy and parser attack surface; scanning must be limited to known Steam metadata, with no raw file logging and clear opt-in root selection. A direct API key flow adds local key theft and revocation risk.

## Risks

No implementation or tests mean controls are requirements, not verified properties. Profile privacy and API behavior must be tested with mocks and synthetic accounts only after authorization. Signing/updater keys require a separate release design.

## References

- https://v2.tauri.app/security/capabilities/
- https://openid.net/specs/openid-authentication-2_0.html
- https://steamcommunity.com/dev/apiterms
- `../SECURITY.md`
