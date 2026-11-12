# ADR-006: Steam Integration Architecture

**Status:** Proposed
**Date:** 2026-11-12

## Context

GameVault is a Windows desktop app intended to present Steam library/progress. Player-data APIs require a user Web API key. Steam OpenID is documented as web flow. No shared key may be shipped, and local Steam-file reading is excluded by the product brief.

## Problem

Can v1 safely deliver account-connected library/progress in the intended local-first Tauri desktop architecture?

## Considered alternatives

- Direct desktop plus embedded shared key: rejected.
- User-provided personal key: no confirmed compliant third-party handling model; not recommended.
- Website OpenID + GameVault backend with server-side key: strongest candidate, pending Valve confirmation and owner acceptance of cloud/operational responsibilities.
- Official public catalog only: can provide catalog metadata where permitted, not user ownership/progress.
- Local file hybrid: excluded from current scope; not a supported substitute for API data.
- Offline-only organizer: viable fallback only if owner explicitly narrows product scope.

## Decision

**Recommendation:** preserve Tauri 2 + React/TypeScript + Rust + SQLite as a proposed technology baseline, but do not scaffold yet. Treat the website/backend path as a conditional research candidate, not an approved architecture. Resolve API key registration, host/key mapping, terms and privacy obligations, then obtain explicit owner decision. If not approved, redefine v1 as offline-only before any scaffold.

## Consequences

Account features are blocked. Hosting changes “local-first” expectations, adds availability and privacy duties, and may create cost. The offline-only route would remove core account features and needs product acceptance. Neither route is chosen by this ADR.

## Risks

Valve's terms may be revised or may not support the intended usage. Authentication does not make private data accessible. Steam availability, rate limits, artwork rights and per-game stats remain variable. No SAM/ASF integration is approved.

## References

- `../FEASIBILITY_REVIEW.md`
- `../STEAM_INTEGRATION.md`
- https://partner.steamgames.com/doc/webapi_overview
- https://partner.steamgames.com/doc/webapi_overview/auth
- https://partner.steamgames.com/doc/features/auth
- https://steamcommunity.com/dev/apiterms
