# ADR-002: Steam Web API Access

**Status:** Proposed
**Date:** 2026-11-12

## Context

The owner prohibits any GameVault backend/server. Steam player methods relevant to library, playtime and achievement display list a user Web API key. Standard user keys require a domain and acceptance of Valve API Terms. Publisher keys require a publisher group and Valve says they must stay on secure servers. Terms require key confidentiality, data retrieval at user request, disclosure of nonpublic data and storage countries, and name an application in key registration.

## Problem

How can distributed GameVault clients retrieve account data without exposing a shared secret or assuming a permission Valve has not documented?

## Considered alternatives

- Shared GameVault key in desktop executable/source: rejected because it cannot remain confidential.
- Individual user key entered locally and sent directly to Steam: potentially avoids a shared secret/server; Valve has not confirmed desktop collection, app/domain registration and third-party handling.
- Server-side GameVault registered key: rejected by owner; no backend is allowed.
- Public-only endpoints: safe for data those endpoints actually return, but not a substitute for owned games/playtime/achievements.
- No account API use: available if v1 is explicitly narrowed.

## Decision

**Recommendation:** no shared key and no backend. Investigate only an individual user Web API key stored locally in Windows Credential Manager/DPAPI and sent directly from Rust to Valve over HTTPS. This remains Proposed and must not be implemented until Valve answers the exact questions in `../V1_FEATURE_FEASIBILITY.md`. Standard user-key registration requires a domain; do not invent the registration/application relationship. Resolve the `api.steampowered.com` versus `partner.steam-api.com` documentation discrepancy before choosing a host. No Community XML, endpoint scraping or local binary cache parser is approved as a substitute.

## Consequences

Account-data features remain blocked pending Valve confirmation. If approved, API calls must be direct to Steam from the user's device, explicitly user-requested, HTTPS, bounded and rate-limited conservatively; no data is routed through GameVault infrastructure. A Windows vault limits casual disclosure but cannot protect a key from same-user malware or a compromised app. TOS call ceiling is not a target; endpoint quotas were not found in reviewed docs.

## Risks

Terms last show July 2010 and may change. User-key ownership, domain/application registration, confidentiality, commercial desktop distribution and host pairing need confirmation. API can change or terminate without a stable service guarantee. Users may need to create/revoke their own key, which adds friction and potential support burden.

## References

- https://partner.steamgames.com/doc/webapi_overview
- https://partner.steamgames.com/doc/webapi_overview/auth
- https://partner.steamgames.com/doc/webapi/IPlayerService
- https://partner.steamgames.com/doc/webapi/ISteamUserStats
- https://steamcommunity.com/dev/apiterms
- `../FEASIBILITY_REVIEW.md`
