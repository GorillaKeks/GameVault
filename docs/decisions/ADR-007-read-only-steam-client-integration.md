# ADR-007: Read-Only Steam Client and Achievement Schema Integration

**Status:** Experimental read-only prototype authorized by owner on 2026-11-15 despite unresolved Valve-policy and client-compatibility risks. Not a Valve approval or release-ready integration.
**Decision date:** 2026-11-12. **Owner risk acceptance and implementation review:** 2026-11-15.

## Context

The owner authorized an experimental implementation of SAM discussion points 1–3 for GameVault: connect to the current Steam client session, select/match games, and display achievement definitions and earned state. The authorization accepts unresolved compatibility and policy risks for a prototype only. The project remains local-only and strictly read-only. SAM's public source uses `steamclient.dll` versioned interfaces and reads a local `UserGameStatsSchema_<AppID>.bin` cache; it also includes write/reset behavior, which is outside this decision.

## Decision

Maintain the implemented Rust adapter as an experimental read-only prototype behind the Tauri native boundary. Do not expand or release it until the gates below are reviewed and pass. It may connect only to the already logged-in local Steam client, operate on explicitly selected/known AppIDs, and read definitions/earned state. Do not copy SAM code/assets under this decision. Do not expose setters or call `SetAchievement`, `ClearAchievement`, `SetStat`, `StoreStats`, or `ResetAllStats`. Never inject into a game, inspect process memory, read passwords/cookies/session files, or change Steam/game files.

This decision records owner-accepted experimentation only; it does not establish Valve authorization, endorsement, a public compatibility contract, or complete feature coverage. See [`../SAM_READ_ONLY_INTEGRATION.md`](../SAM_READ_ONLY_INTEGRATION.md) for the per-step analysis and failure limits.

## Implementation gates

Before expanding or releasing the adapter, review and record all of the following:

1. Current Valve policy and applicable terms for this third-party desktop use; owner direction alone is not Valve permission.
2. A native-boundary threat review covering DLL loading, interface and callback ABI, path constraints, parsing limits, failure behavior, and prevention of all write/reset calls.
3. An explicit Steam client compatibility set and fail-closed behavior for unknown versions/interfaces or missing/stale schema data.
4. Deterministic tests using mocks and synthetic schema data, including assertions that no write API is exposed or called. Do not use a real account for risky behavior.
5. Confirmation that the implementation does not claim complete library, playtime, or achievement coverage where the selected source cannot provide it.

Until these gates pass, keep the adapter experimental and limit claims to the observed behavior. Unanswered policy questions remain documented; do not infer Valve approval or broader support.

## Owner risk acceptance and gate review (2026-11-15)

After the remaining limitations were explained, the owner explicitly accepted the documented unsupported-interface and compatibility risks and authorized a strictly read-only prototype. This records owner acceptance only; it does not change Valve's policy, establish support, or make the prototype ready for release.

| Gate | Review result | Evidence / remaining requirement |
|---|---|---|
| 1. Valve policy for a third-party local desktop client | **OWNER ACCEPTED RESIDUAL RISK; NOT CLEARED BY VALVE** | Current Valve docs describe Steamworks stats/achievements in the game-integrated SDK context and document Web API alternatives. They do not establish permission or support for GameVault directly loading private/versioned client interfaces. No written Valve answer for this use is recorded. This remains an evidence gap, not a claim that Valve has expressly prohibited it. |
| 2. Native-boundary threat review | **ONE LIVE READ SUCCEEDED; RELEASE REVIEW OPEN** | Earlier helper attempts exited with `0xc0000374` during teardown and `0xc0000005` during callback polling. The incorrect `ISteamUser::GetSteamID` signature was corrected to use an output pointer; the helper is isolated and callback steps remain traced. The owner-provided result now confirms a live read for Dune: Awakening (AppID 1172710): 74 achievements returned, 8 earned. This is one successful machine/game observation, not release validation. |
| 3. Steam client compatibility set | **ONE LOCAL BUILD OBSERVED; NO COMPATIBILITY RANGE** | The prototype requests x64 Windows interfaces `SteamClient018`, `SteamUser012` and `SteamUserStats013`; the native stats lookup uses `STEAMUSERSTATS_INTERFACE_VERSION013`. One live read succeeded on the owner's installed client, but the exact Steam client build was not captured. Missing IDs fail closed. No client build range or ABI guarantee is established. |
| 4. Deterministic adapter tests | **MOCK MAPPING TESTS PASS; NATIVE ABI TESTS OPEN** | `cargo test --offline` passes 17 Rust tests, including synthetic read-only achievement fixtures for earned/locked mapping, partial-state failure and oversized lists. They do not validate the real DLL ABI or callback behavior. Separately, one user-triggered live read succeeded for Dune: Awakening; this does not replace automated native-boundary testing. |
| 5. Coverage and product claims | **PASS — LIMITED CLAIMS** | UI requests achievements per locally installed game, displays unavailable explicitly, and does not claim complete account library or playtime. Per-game counts are shown only after a complete response. Recheck these claims against future changes. |

### Policy and interface evidence reviewed

- Valve's [Steamworks API overview](https://partner.steamgames.com/doc/sdk/api) describes the API as enabling a game to use Steam systems; the [stats/achievements guide](https://partner.steamgames.com/doc/features/achievements) describes use from within the integrated game. The overview explains that Steamworks initializes against the running client and that interfaces are versioned, but it does not provide a third-party desktop companion contract for direct `steamclient.dll` use.
- Valve's [ISteamUserStats Web API reference](https://partner.steamgames.com/doc/webapi/ISteamUserStats) lists `GetPlayerAchievements` (unlocked achievements) and `GetSchemaForGame` (the complete definition list); both require a user authentication key. This is a separate route and does not clear the private client adapter or settle direct user-key use in a distributed local application.
- Valve's [Steam Web API Terms](https://steamcommunity.com/dev/apiterms), whose page says it was last updated July 2010, require keeping the API key confidential, identify the application through the API key sign-up form, and require user-requested retrieval. The text does not specifically decide the private client-interface use proposed here.

### Third-party technical comparator: HxB SAM Enhanced

The owner-provided [HxB SAM Enhanced repository](https://github.com/Hegxib/SteamAchievementManager-enhanced) provides concrete evidence that this technical path is feasible in a working Windows tool. Its [`Client.cs`](https://github.com/Hegxib/SteamAchievementManager-enhanced/blob/main/SAM.API/Client.cs) requests versioned interfaces including `SteamClient018`, connects a global Steam user, and obtains `SteamUserStats013`. Its [`Steam.cs`](https://github.com/Hegxib/SteamAchievementManager-enhanced/blob/main/SAM.API/Steam.cs) loads `steamclient.dll` and resolves native exports. Its [`Manager.cs`](https://github.com/Hegxib/SteamAchievementManager-enhanced/blob/main/SAM.Game/Manager.cs) calls `RequestUserStats` and `GetAchievementAndUnlockTime` to display state.

This materially strengthens the technical-feasibility evidence; the adapter is no longer only a planning hypothesis inferred from the original SAM. It does not close the policy or compatibility gates: successful operation in another tool is not Valve approval, a public third-party API contract, or a GameVault-supported client-version matrix. The HxB README also describes unlock/commit and bulk-reset features, its source includes `SetAchievement` and `SetStat` calls, and it warns users that the tool modifies achievement data and may violate Steam's Terms of Service. Those capabilities and risks are outside GameVault's read-only boundary. Do not copy or adapt its code; its README describes a mix of original SAM components and separately copyrighted enhancements, so any future reuse would need an exact file-level license/provenance review.

The implementation is a first experimental prototype under the owner's explicit risk acceptance. Keep it opt-in per game, visibly label it experimental, do not make Valve-support or broad compatibility claims, and do not use a real Steam account for automated tests. A written Valve response remains the best way to close the policy uncertainty; the unresolved native ABI/build compatibility remains the principal technical risk.

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
- [Valve ISteamUserStats Web API reference](https://partner.steamgames.com/doc/webapi/ISteamUserStats)
- [Steam Web API Terms of Use](https://steamcommunity.com/dev/apiterms)
