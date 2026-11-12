# ADR-002: Steam Web API Access

**Status:** Proposed
**Date:** 2026-11-12

## Context

Steam player methods relevant to library, playtime and achievement display list a user Web API key. Standard user keys require a domain and acceptance of Valve API Terms. Publisher keys require a publisher group and Valve says they must stay on secure servers. Terms require key confidentiality, data retrieval at user request, disclosure of nonpublic data and storage countries, and name an application in key registration.

## Problem

How can distributed GameVault clients retrieve account data without exposing a shared secret or assuming a permission Valve has not documented?

## Considered alternatives

- Shared GameVault key in desktop executable/source: rejected because it cannot remain confidential.
- Personal end-user key collected by GameVault: not recommended; third-party collection/storage and app/domain use not confirmed.
- Server-side GameVault registered user key: candidate only; requires Valve confirmation of application/user-data model and backend approval.
- Public-only endpoints: safe for data those endpoints actually return, but not a substitute for owned games/playtime/achievements.
- No account API use: available if v1 is explicitly narrowed.

## Decision

**Recommendation:** make no player-data API calls until the exact registration, key type, public-host endpoint and terms are confirmed. Never ship a shared key. A backend-held key is the leading investigation candidate, not an approved architecture. Do not use the partner-only host with a user key: Valve's overview states that every request to that host requires a publisher key, while method references display partner-host examples; ask Valve to resolve this discrepancy and use no uncertain mapping.

## Consequences

Steam-connected features remain blocked. Any approved API implementation must use only end-user-requested calls, HTTPS, bounded explicit refresh, conservative volume, typed parsing and distinct private/unsupported/error states. TOS call ceiling is not a target; endpoint quotas were not found in reviewed docs.

## Risks

Terms last show July 2010 and may change. Key and host registration semantics, commercial desktop distribution and visibility cases need confirmation. API can change or terminate without a stable service guarantee.

## References

- https://partner.steamgames.com/doc/webapi_overview
- https://partner.steamgames.com/doc/webapi_overview/auth
- https://partner.steamgames.com/doc/webapi/IPlayerService
- https://partner.steamgames.com/doc/webapi/ISteamUserStats
- https://steamcommunity.com/dev/apiterms
- `../FEASIBILITY_REVIEW.md`
