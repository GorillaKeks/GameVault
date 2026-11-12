# ADR-007: Read-Only Steam Client and Achievement Schema Integration

**Status:** Accepted owner direction to pursue; implementation remains gated on boundary review
**Date:** 2026-11-12

## Context

The owner asked to pursue SAM discussion points 1–3 for GameVault: connect to the current Steam client session, select/match games, and display achievement definitions and earned state. The project remains local-only and strictly read-only. SAM's public source uses `steamclient.dll` versioned interfaces and reads a local `UserGameStatsSchema_<AppID>.bin` cache; it also includes write/reset behavior, which is outside this decision.

## Decision

Pursue an independently implemented Rust read-only adapter behind the Tauri native boundary. It may connect only to the already logged-in local Steam client, operate on explicitly selected/known AppIDs, and read schema/earned state. Do not copy SAM code/assets under this decision. Do not expose setters or call `SetAchievement`, `ClearAchievement`, `SetStat`, `StoreStats`, or `ResetAllStats`. Never inject into a game, inspect process memory, read passwords/cookies/session files, or change Steam/game files.

This decision accepts pursuit of a technically unsupported path; it does not establish Valve authorization, endorsement, a public compatibility contract, or complete feature coverage. See [`../SAM_READ_ONLY_INTEGRATION.md`](../SAM_READ_ONLY_INTEGRATION.md) for the per-step analysis and failure limits.

## Consequences

- Interface versions and binary cache structure may change without notice. Fail closed on an unknown interface/schema and report achievement data unavailable.
- No schema means no achievement definitions. Never infer 0%, all locked, or complete coverage.
- A game-ID candidate list must come from a separate source. SAM's external XML catalog is not adopted and cannot guarantee all owned games.
- Complete owned library and complete account-wide playtime remain unresolved and mandatory.
- A separate Valve Web API user-key investigation remains open; this client route does not settle that question.
- Implementation requires native-boundary security review, strict read-only tests with mocks/synthetic data, current Steam policy review, and an explicit declared Steam client compatibility set.

## References

- [`../SAM_READ_ONLY_INTEGRATION.md`](../SAM_READ_ONLY_INTEGRATION.md)
- [SAM Steam client connection source](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Client.cs)
- [SAM achievement manager source](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.Game/Manager.cs)
- [Valve Steamworks SDK API overview](https://partner.steamgames.com/doc/sdk/api)
- [Valve stats/achievements documentation](https://partner.steamgames.com/doc/features/achievements)
