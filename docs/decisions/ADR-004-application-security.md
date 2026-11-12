# ADR-004: Application Security and Privacy Baseline

**Status:** Proposed
**Date:** 2026-11-12

## Context

The future app would combine a web UI, native Rust commands, local database, and possibly remote Steam/account services. No app code or data collection exists yet.

## Problem

What security boundaries are required before a desktop shell or Steam connection is implemented?

## Considered alternatives

- Broad Tauri filesystem/shell/network permissions for frontend convenience: rejected; unnecessary authority increases impact.
- Narrow capabilities and typed domain commands: recommended least-privilege baseline.
- Embed an API key in binary: rejected; binary secrets are extractable and Valve requires confidentiality.
- Hosted secret boundary: candidate only after backend/Valve approval; introduces a new trust boundary and operations.

## Decision

**Recommendation:** least-privilege Tauri capabilities; only narrow typed Rust commands; no arbitrary SQL, shell, path, process or filesystem access. Validate and bound IPC and network data. HTTPS, finite timeouts, response size limits and safe JSON parsing. Redact identifiers/secrets. No telemetry or data sharing by default. No credential is collected absent an approved flow. This security baseline is proposed for adoption before scaffold.

## Consequences

Frontend compromise has fewer native capabilities. Validation and explicit error-state work are required. If a backend is selected, add a formal threat model for session handoff, key storage, abuse limits, logging, retention, incident response and deletion.

## Risks

No implementation or tests mean controls are requirements, not verified properties. OpenID, profile privacy and API behavior must be tested with mocks and synthetic accounts only after authorization. Signing/updater keys require a separate release design.

## References

- https://v2.tauri.app/security/capabilities/
- https://openid.net/specs/openid-authentication-2_0.html
- https://steamcommunity.com/dev/apiterms
- `../SECURITY.md`
