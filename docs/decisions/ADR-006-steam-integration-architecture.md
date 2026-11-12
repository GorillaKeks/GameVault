# ADR-006: Steam Integration Architecture

**Status:** Accepted for local-only topology; read-only Steam client/cache path approved for pursuit by owner
**Date:** 2026-11-12

## Context

The owner explicitly confirmed Windows local-first, Steam-only, Tauri 2 + React/TypeScript + Rust + SQLite, no GameVault backend/server/account/cloud DB, and four mandatory features. Steam player-data APIs require a user Web API key. Steam OpenID is documented as a web flow. No shared key may ship. On 2026-11-12 the owner additionally directed GameVault to pursue SAM's points 1–3 in a strictly read-only form: connect to the logged-in Steam client, match candidate AppIDs, and display achievement definitions/earned state. This is an explicit product direction, not evidence of Valve approval or a support contract.

## Problem

Can v1 safely deliver account-connected library/progress in the intended local-first Tauri desktop architecture?

## Considered alternatives

- Direct desktop plus embedded shared key: rejected.
- User-provided personal key: no confirmed compliant third-party handling model; not recommended.
- Website OpenID + GameVault backend with server-side key: incompatible with the confirmed owner constraint; rejected.
- Individual user key stored locally and sent to Valve: candidate only pending Valve confirmation of desktop distribution, terms, domain/application registration and endpoint host.
- Official public catalog only: can provide catalog metadata where permitted, not user ownership/progress.
- Read-only local VDF/ACF for install detection and localconfig playtime: supports a subset; formats are undocumented and do not provide a complete owned library.
- Read-only direct Steam client interface plus local achievement schema cache, as SAM does: now owner-approved to pursue; uses undocumented/versioned interfaces and binary cache without a Valve compatibility guarantee. Details/limits: `../SAM_READ_ONLY_INTEGRATION.md`.
- SAM-hosted AppID catalog: not adopted; it is a third-party catalog and cannot guarantee a complete owned library.

## Decision

**Accepted owner decisions:** implement GameVault only as a local Windows desktop app using the confirmed stack and without a GameVault backend/server/account. Keep Steam integration behind narrow Rust modules. Pursue a clean, read-only SAM-inspired adapter for Steam-session connection, AppID matching and achievement reading; do not copy SAM code/assets unless separately reviewed and approved. Do not expose or call achievement/stat setters, store or reset functions. This is not a claim of Valve support. The four v1 features remain mandatory: the selected route still does not guarantee a complete owned library or account-wide playtime.

## Consequences

No cloud dependency or service operations are introduced by the selected local route. Its interfaces/cache may break across Steam client updates; missing schema, failed callbacks or incompatible interface versions must become explicit unavailable/error states. The SAM game list is not an owned-library completeness source. Direct Web API user-key access remains Proposed pending Valve clarification. If permitted sources still cannot provide mandatory feature coverage, return for an owner decision; do not remove features.

## Risks

The direct Steam client/cache route is undocumented and may be incompatible with Valve/Steam policy or future clients; the project makes no claim of endorsement. Authentication does not make private data accessible. Steam availability, rate limits, artwork rights and per-game stats remain variable. No SAM code/assets, ASF integration, or SAM write behavior is approved.

## References

- `../FEASIBILITY_REVIEW.md`
- `../STEAM_INTEGRATION.md`
- `../SAM_READ_ONLY_INTEGRATION.md`
- https://partner.steamgames.com/doc/webapi_overview
- https://partner.steamgames.com/doc/webapi_overview/auth
- https://partner.steamgames.com/doc/features/auth
- https://steamcommunity.com/dev/apiterms
