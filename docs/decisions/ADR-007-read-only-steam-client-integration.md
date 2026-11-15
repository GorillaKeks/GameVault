# ADR-007: Read-Only Steam Client and Achievement Schema Integration

**Status:** Planning only; implementation is gated on policy, security, compatibility, and test review
**Decision date:** 2026-11-12. **Planning-only clarification:** 2026-11-15.

## Context

The owner authorized further planning of SAM discussion points 1–3 for GameVault: connect to the current Steam client session, select/match games, and display achievement definitions and earned state. This authorization does not authorize adapter implementation. The project remains local-only and strictly read-only. SAM's public source uses `steamclient.dll` versioned interfaces and reads a local `UserGameStatsSchema_<AppID>.bin` cache; it also includes write/reset behavior, which is outside this decision.

## Decision

Plan an independently implemented Rust read-only adapter behind the Tauri native boundary. Do not implement or ship it until all implementation gates below are reviewed and pass. If later authorized, it may connect only to the already logged-in local Steam client, operate on explicitly selected/known AppIDs, and read schema/earned state. Do not copy SAM code/assets under this decision. Do not expose setters or call `SetAchievement`, `ClearAchievement`, `SetStat`, `StoreStats`, or `ResetAllStats`. Never inject into a game, inspect process memory, read passwords/cookies/session files, or change Steam/game files.

This decision authorizes planning a technically unsupported path; it does not establish Valve authorization, endorsement, a public compatibility contract, or complete feature coverage. See [`../SAM_READ_ONLY_INTEGRATION.md`](../SAM_READ_ONLY_INTEGRATION.md) for the per-step analysis and failure limits.

## Implementation gates

Before adapter code is written, review and record all of the following:

1. Current Valve policy and applicable terms for this third-party desktop use; owner direction alone is not Valve permission.
2. A native-boundary threat review covering DLL loading, interface and callback ABI, path constraints, parsing limits, failure behavior, and prevention of all write/reset calls.
3. An explicit Steam client compatibility set and fail-closed behavior for unknown versions/interfaces or missing/stale schema data.
4. Deterministic tests using mocks and synthetic schema data, including assertions that no write API is exposed or called. Do not use a real account for risky behavior.
5. Confirmation that the implementation does not claim complete library, playtime, or achievement coverage where the selected source cannot provide it.

Until these gates pass, work is limited to research, threat modeling, interface design, and test planning. If policy remains unanswered, keep the adapter unimplemented and return with the exact question and evidence needed for an owner decision.

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
