# GameVault — SAM-Inspired Read-Only Steam Client Path

**Review date:** 2026-11-12
**Owner direction:** pursue SAM's connection, game-selection and achievement-reading path (points 1–3 from the discussion), with GameVault strictly read-only. This does not authorize any achievement/stat writes or copying SAM source/assets.
**Status:** Technical approach documented. Prototype currently discovers local installed manifests and reports whether the expected schema file exists; it performs no Steam client calls and reads no schema contents.

## What the requested path means

The user has directed GameVault to pursue the read-only subset of SAM's approach. This changes the earlier product preference to avoid `steamclient.dll` and undocumented cache formats. It does **not** establish Valve endorsement, policy approval or interface stability. The present design must be treated as an experimental, Windows/Steam-version-sensitive local integration with a fallback/error state, not a guaranteed Steam API.

The reviewed SAM source revision is `de8b71048a0cee3c3e97cd8535e0f55ca86513e4`. SAM is a .NET Framework/WinForms x86 program. Its implementation uses custom P/Invoke and vtable wrappers rather than the public Steam Web API. GameVault can pursue an independently written Rust read-only adapter; no SAM source code or artwork is being imported in this phase.

## Point 1 — connect to the logged-in Steam client

SAM reads the install path from the Steam registry key, loads `steamclient.dll`, resolves its `CreateInterface` and callback exports, requests versioned interfaces such as `SteamClient018`, creates a Steam pipe, and connects to the global logged-in user. It then obtains interfaces including `SteamUser012` and `SteamUserStats013`. See SAM's [`Steam.cs`](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Steam.cs) and [`Client.cs`](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Client.cs).

This is not Steam OpenID, a Web API user key, or a general documented API for third-party desktop tools. It does not read a Steam password or cookie in the reviewed code. It also does not inject into a game process in the reviewed path: it loads the Steam client library in the companion process and communicates through Steam's client pipe.

**GameVault read-only boundary:** Rust-only native module; load from a validated Steam installation path; request only the specific read methods needed; no frontend-controlled DLL paths or arbitrary interface names; do not expose interface pointers through Tauri IPC. Never bind/call `SetAchievement`, `ClearAchievement`, `SetStat`, `StoreStats`, or `ResetAllStats`. A failed/unknown interface version must fail closed with “Steam integration unavailable,” not attempt memory scanning, process injection, or guessed vtable layouts.

Version-specific interface identifiers and callback ABI details are an ongoing compatibility risk. Steam client updates can change implementation behavior without a third-party support contract. Do not promise support until tested against a declared set of Steam client versions.

## Point 2 — game catalog and selection

SAM downloads its own `https://gib.me/sam/games.xml` catalog, containing candidate AppIDs and categories. It filters those candidates with `ISteamApps::BIsSubscribedApp` through the Steam client and looks up names using `GetAppData`. See [`GamePicker.cs`](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.Picker/GamePicker.cs), especially its list download and `OwnsGame`/`AddGame` methods.

That is not a complete owned-games enumeration: completeness is bounded by SAM's separately maintained catalog. It also adds a request to a third-party SAM-controlled host. Reusing that feed would create availability, provenance, privacy, attribution and maintenance dependencies. It must not be mistaken for Valve's catalog or treated as a complete Steam account library.

For GameVault, the first prototype derives candidate AppIDs from local installed manifests in conventionally located Steam libraries. `BIsSubscribedApp` has not been implemented. Local candidates support installed-game visibility only and **do not satisfy the mandatory complete owned account library**. Custom Steam install roots outside the standard paths may also be missed until registry-based discovery is added.

## Point 3 — read achievement definitions and earned state

For a selected AppID, SAM:

1. Uses `ISteamUserStats::RequestUserStats` for the currently logged-in SteamID and waits for the user-stats callback.
2. Opens `Steam\appcache\stats\UserGameStatsSchema_<AppID>.bin` and parses it as Valve KeyValues binary to obtain achievement/stat definitions and display metadata.
3. Calls `GetAchievementAndUnlockTime` for each schema achievement and reads stat values through `GetStatValue`.

See SAM's [`Manager.cs`](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.Game/Manager.cs) and [`SteamUserStats013.cs`](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Wrappers/SteamUserStats013.cs). The local schema can be absent or stale; Steam must have fetched it for that AppID, and a schema may not exist for a title with no published achievements. Treat missing/invalid schema and interface errors as “achievement data unavailable,” never as zero achievements or all locked.

The reader should parse only the exact schema file for an explicitly selected/known AppID, with file-size, depth and entry-count limits. It must not recursively search Steam directories, write/delete cache files, or load a schema into a game process. Expose only display metadata and earned state; omit write methods from the GameVault wrapper and UI altogether. Do not infer progress percentages absent a game-defined, meaningful progress stat.

Valve documents Steamworks APIs for games integrated with Steamworks and says Steam caches stats for those APIs to work offline in that game context. That is not documentation of SAM's direct versioned `steamclient.dll` interface plus third-party parsing of a binary schema cache. The distinction remains material; see [Valve's Steamworks API overview](https://partner.steamgames.com/doc/sdk/api) and [stats/achievements documentation](https://partner.steamgames.com/doc/features/achievements).

## Feature result and limits

| Requested point | Technical result | Product limitation |
|---|---|---|
| 1. Connect to current Steam session | SAM demonstrates a working implementation against its supported client versions. | Internal, versioned client interface; compatibility and policy are not guaranteed for GameVault. |
| 2. Find games | Match candidate AppIDs against Steam's current local subscription state. | A candidate catalog is still needed. SAM's third-party `games.xml` is not a complete-library guarantee and is not adopted here. |
| 3. Read achievements | Read schema from local binary cache and earned state through the Steam client interface. | Unsupported local schema format; absent/stale data; no guaranteed cross-game coverage. Read-only display only. |
| Complete Steam owned library | **Not solved** by points 1–3 as SAM implements them. | Keep Phase 1.6 status **Conditional on Steam access**; do not count SAM's game catalog as a complete account source. |
| Playtime | These three points do not establish complete account-wide playtime. | Existing localconfig/API analysis remains unchanged. |

## License and reuse boundary

SAM's repository `LICENSE.txt` is the zlib license. Its conditions include retaining the notice, not misrepresenting origin, and plainly marking altered source versions. The README separately credits Fugue Icons; the repo license does not automatically license those assets. See SAM's [license](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/LICENSE.txt) and [README](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/README.md).

No SAM code or icons are copied. Before adapting an exact file, perform the file/dependency/asset provenance audit and preserve all license obligations. An independently authored adapter still must account for Steam policy, interface compatibility, local-data security and GameVault's eventual license; clean-room code does not convert an undocumented Steam interface into a supported one.

## Phase gate

Owner direction authorizes pursuit of the read-only points 1–3; it does **not** make all mandatory v1 data sources complete, and it does not authorize write behavior. Before implementing the private Steam client/schema calls:

1. Update the standing `MASTER_PROMPT.md` and architecture/ADR records so their former blanket exclusion of Steam client DLL/cache access no longer contradicts this explicit owner direction. Preserve the absolute prohibition on achievement/stat writes.
2. Keep the direct Web API user-key question open; this route is an alternative, not evidence of Valve approval.
3. Add a narrowly scoped native-boundary threat/compatibility review and a plan for synthetic/mock testing. No real account should be used for risky behavior.
4. Keep complete library and complete playtime blocked until their own data sources are validated.
5. Do not use SAM's remote `games.xml` as a product dependency without separate owner approval and source/availability review.

The current Rust command is intentionally narrower than points 1–3: it enumerates local manifest candidates and checks schema-file presence, without reading the schema or connecting to Steam. This is not achievement support and does not establish ownership. It is a local-discovery prototype only.

No application code, calls to the running Steam client, or Steam data parsing have been performed as part of this review.

## Sources reviewed 2026-11-12

- [SAM upstream revision](https://github.com/gibbed/SteamAchievementManager/tree/de8b71048a0cee3c3e97cd8535e0f55ca86513e4)
- [SAM native loader](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Steam.cs)
- [SAM Steam client setup](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Client.cs)
- [SAM game picker](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.Picker/GamePicker.cs)
- [SAM achievement/stat manager](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.Game/Manager.cs)
- [SAM zlib license](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/LICENSE.txt)
- [Valve Steamworks API overview](https://partner.steamgames.com/doc/sdk/api)
- [Valve Steamworks stats and achievements](https://partner.steamgames.com/doc/features/achievements)
- [Steam Web API Terms](https://steamcommunity.com/dev/apiterms)
