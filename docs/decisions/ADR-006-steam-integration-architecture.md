# ADR-006: Steam Integration Architecture

**Status:** Accepted (owner-confirmed architecture constraint; Steam access path unresolved)
**Date:** 2026-11-12

## Context

The owner explicitly confirmed Windows local-first, Steam-only, Tauri 2 + React/TypeScript + Rust + SQLite, no GameVault backend/server/account/cloud DB, and four mandatory features. Steam player-data APIs require a user Web API key. Steam OpenID is documented as a web flow. No shared key may ship. The Phase 1.6 prompt authorizes research into local file formats but does not by itself approve undocumented cache parsing for production.

## Problem

Can v1 safely deliver account-connected library/progress in the intended local-first Tauri desktop architecture?

## Considered alternatives

- Direct desktop plus embedded shared key: rejected.
- User-provided personal key: no confirmed compliant third-party handling model; not recommended.
- Website OpenID + GameVault backend with server-side key: incompatible with the confirmed owner constraint; rejected.
- Individual user key stored locally and sent to Valve: candidate only pending Valve confirmation of desktop distribution, terms, domain/application registration and endpoint host.
- Official public catalog only: can provide catalog metadata where permitted, not user ownership/progress.
- Read-only local VDF/ACF for install detection and localconfig playtime: supports a subset; formats are undocumented and do not provide a complete owned library.
- Reverse-engineered achievement cache parsing: not approved; no stable Valve format/API guarantee.

## Decision

**Accepted owner decision:** implement GameVault only as a local Windows desktop app using the confirmed stack and without a GameVault backend/server/account. Keep Steam integration behind narrow Rust modules. No application scaffolding is authorized in this phase. The remaining Steam source is not yet selected: local manifests can support installed detection, localconfig can provide best-effort per-device playtime, while complete owned library and reliable achievements require Valve-approved direct user-key access or a separate explicit approval to parse unsupported local caches. All four v1 features remain mandatory.

## Consequences

No cloud dependency or service operations are introduced. Local scanning remains available offline but has incomplete coverage. Account data requires a user key and may still be unavailable for private profiles. If Valve rejects local key use and the owner does not approve unsupported local-cache parsing, the current mandatory feature set is not feasible under the accepted architecture; return for an owner decision, do not remove features.

## Risks

Valve's terms may be revised or may not support the intended usage. Authentication does not make private data accessible. Steam availability, rate limits, artwork rights and per-game stats remain variable. No SAM/ASF integration is approved.

## References

- `../FEASIBILITY_REVIEW.md`
- `../STEAM_INTEGRATION.md`
- https://partner.steamgames.com/doc/webapi_overview
- https://partner.steamgames.com/doc/webapi_overview/auth
- https://partner.steamgames.com/doc/features/auth
- https://steamcommunity.com/dev/apiterms
