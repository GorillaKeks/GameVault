# GameVault — SAM-Inspired Read-Only Steam Client Path

**Research baseline:** 2026-11-12. **Implementation update:** 2026-11-15.
**Owner direction:** experimental read-only Steam-client achievement reads authorized after explicit acceptance of the unresolved policy and compatibility risks. Achievement/stat writes and copying SAM source/assets remain prohibited.
**Status:** A per-game opt-in prototype requests `SteamClient018`, `SteamUser012` and `SteamUserStats013`, reads the signed-in user's stats, and displays definitions and earned state. Earlier helper attempts crashed; a wrong `GetSteamID` signature was corrected to use its output pointer. The owner-provided live result confirms one successful read for Dune: Awakening (AppID 1172710): 74 achievements, 8 earned. Steam build and ABI compatibility beyond this one local observation remain unverified. See ADR-007 for gate-by-gate results.

## What the requested path means

The owner has explicitly accepted the read-only interface risk and authorized a prototype. This changes the earlier planning-only direction, but does **not** establish Valve endorsement, policy approval or interface stability. Treat the integration as experimental, Windows/Steam-version-sensitive and fail-closed, not as a guaranteed Steam API.

The reviewed SAM source revision is `de8b71048a0cee3c3e97cd8535e0f55ca86513e4`. SAM is a .NET Framework/WinForms x86 program. Its implementation uses custom P/Invoke and vtable wrappers rather than the public Steam Web API. GameVault now contains an independently authored experimental Rust read-only adapter; no SAM source code or artwork is imported.

## Point 1 — connect to the logged-in Steam client

SAM reads the install path from the Steam registry key, loads `steamclient.dll`, resolves its `CreateInterface` and callback exports, requests versioned interfaces such as `SteamClient018`, creates a Steam pipe, and connects to the global logged-in user. It then obtains interfaces including `SteamUser012` and `SteamUserStats013`. See SAM's [`Steam.cs`](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Steam.cs) and [`Client.cs`](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Client.cs).

This is not Steam OpenID, a Web API user key, or a general documented API for third-party desktop tools. The GameVault prototype does not read a Steam password or cookie, and does not inject into a game process: it loads the Steam client library in its own process and communicates through Steam's client pipe.

**GameVault read-only boundary:** Rust-only native module; load the architecture-matched client library from a validated Steam installation path; request only the specific read methods needed; no frontend-controlled DLL paths or arbitrary interface names; do not expose interface pointers through Tauri IPC. Never bind/call `SetAchievement`, `ClearAchievement`, `SetStat`, `StoreStats`, or `ResetAllStats`. A failed/unknown interface version must fail closed with “Steam integration unavailable,” not attempt memory scanning, process injection, or guessed vtable layouts.

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

## Security and compatibility review

The design review informed the current prototype but does not make the private Steam client interface or binary schema supported. No GameVault-supported Steam client build range can be declared from the reviewed sources.

| Boundary / failure | Required design before implementation | Fail-closed result |
|---|---|---|
| Steam DLL discovery and loading | Select and review a Steam-root discovery source; canonicalize it; load only the expected DLL from inside that root. Never accept a DLL path or interface name from the WebView, environment override, or imported file. | Missing, ambiguous, or out-of-root DLL: integration unavailable; do not search other directories or fall back to guessed paths. |
| Native interface and callback ABI | Prototype pins exact interface IDs and uses fixed read-only vtable slots in one Rust module. Keep raw pointers and callback structures private. Callback wait is bounded and handles/pipes are released on error. | One live read succeeded after correcting the `GetSteamID` output-pointer signature. Missing interface ID, malformed/irrelevant callback, timeout, or partial setup returns unavailable; no vtable scanning or guessed fallback. One successful game/client observation does not establish broad ABI compatibility. |
| Read-only guarantee | Define only the read operations needed for session identity, selected AppID and earned state. Do not define setter/reset methods or generic vtable dispatch in the app-facing interface. Review the exact native calls and teardown path before every implementation change. | Any operation outside the read allowlist: reject it; no writes, resets, game launch, injection, process/memory access, or Steam file mutation. |
| Achievement definitions and state | Prototype reads the definition count, IDs, display attributes and earned state from the selected app's Steam client stats interface. It does not parse local schema bytes. Returned count and text length are limited and a single missing state fails the whole response. | Missing definitions/status, unsupported app or interface, timeout or malformed response returns unavailable; never infer all locked or zero achievements. |
| Tauri boundary and diagnostics | Expose a typed result containing only display data, source/freshness and explicit unavailable/error states. Keep pointers, paths, raw schema bytes and raw callbacks inside Rust; redact identifiers and parser/input details from logs. | Invalid IPC inputs are rejected. No arbitrary path, SQL, HTTP, shell, native handle, or raw parser error reaches the WebView. |
| Policy and product claims | Valve terms and first-party documentation reviewed; the exact third-party desktop behavior remains unanswered. Owner accepted this residual risk for an experimental prototype only. Keep library and account-wide playtime marked unresolved. | No Valve endorsement or release claim; keep the prototype opt-in and experimental. Revisit if policy guidance changes. |

### Compatibility declaration

There is no supportable Steam client build range yet. The prototype pins SteamClient018, SteamUser012 and SteamUserStats013, targets 64-bit Windows, and has synthetic mapping tests; callback lifecycle and native ABI remain untested. Missing exact interfaces fail closed. A compatibility claim requires a separately reviewed, controlled smoke test for every declared build; success on one machine or SAM's compatibility is not evidence for other builds. Do not auto-select a nearby interface version.

### Synthetic and mock test plan

The synthetic/mock plan remains the target for native-boundary validation. Current tests cover read-only mapping and bounds only; they do not simulate the DLL, vtable or callback ABI.

| Test group | Synthetic cases | Required assertion |
|---|---|---|
| Interface negotiation | Exact supported ID; missing ID; unknown newer/older ID; wrong architecture; loader error | Only exact declared IDs proceed; all other cases return unavailable without fallback or process injection. |
| Callback lifecycle | Success; failure result; unrelated AppID; duplicate/out-of-order callback; timeout; shutdown during wait | Bounded completion, correct AppID correlation, and resource cleanup on every exit. |
| Achievement response mapping | Complete definitions and status; no achievements; missing definition/status; duplicate IDs; excessive counts/text | Deterministic read-only DTO for complete data; fail closed for partial or oversized data; no fabricated defaults. |
| Earned-state mapping | Earned/locked entries; missing state; unknown achievement ID; absent schema | Preserve unknown/unavailable separately from locked; never compute a completeness or progress percentage from missing data. |
| Read-only invariant | Mock call recorder with the allowed read surface; invalid operation request | No setter/reset operation exists in the adapter interface, and the recorder observes only the approved read allowlist. |
| Path and input validation | Invalid AppID; path traversal text; non-canonical/out-of-root path; missing file; file replaced during open | Inputs cannot escape the canonical expected directory; invalid input produces unavailable without reading unrelated files. |
| Resource / parser robustness | Property-based or fuzz-generated arbitrary schema bytes within and above limits | Parsing remains bounded and never invokes Steam or writes files. |

Use synthetic achievement fixtures and mock interface/callback providers for unit tests. Current mock tests cover state mapping and bounds but not interface negotiation or callback lifecycle. No credentials or real account data belong in unit tests. Any later live compatibility smoke test requires its own reviewed plan and must remain read-only.

## Phase gate

Owner direction now authorizes the experimental read-only prototype despite unresolved Valve-policy/compatibility uncertainty. It does **not** make all mandatory v1 data sources complete or authorize write behavior. Before expanding or releasing this integration:

1. Keep `MASTER_PROMPT.md` and architecture/ADR records aligned with the owner-accepted experimental status. Preserve the absolute prohibition on achievement/stat writes.
2. Keep the direct Web API user-key question open; this route is an alternative, not evidence of Valve approval.
3. Add native callback/interface mock tests and separately review any live compatibility smoke test. No real account is used in automated tests.
4. Keep complete library and complete playtime blocked until their own data sources are validated.
5. Do not use SAM's remote `games.xml` as a product dependency without separate owner approval and source/availability review.

The local scanner still enumerates installed manifest candidates and checks schema-file presence; that check is informational only and does not establish ownership or achievement availability. The new per-game command connects only after explicit user action.

## Achievement display contract (2026-11-15)

The UI must not imply that a game's achievements are locked, absent, or complete based only on schema-file presence. The current prototype uses this display contract:

- Show each achievement's source-provided name, description and icon only when present; missing optional metadata stays missing rather than being invented.
- Distinguish **earned**, **not earned** and **unknown/unavailable**. “Not earned” is valid only when the source returned the achievement's state and the definition set is known complete for that game.
- Show an `earned / total` count only when both the earned-state result and complete definition list apply to the same AppID and refresh snapshot. Do not calculate a completion percentage from partial, stale or absent data.
- Represent private profile, unsupported game, missing/stale schema, unsupported client version, offline/unavailable session and parser/interface failure as unavailable states, with no inferred locked achievements.
- Keep the whole feature read-only: no unlock, reset, stat update, or other Steam/game-state action in UI or IPC.

Valve's currently published `ISteamUserStats` reference lists `GetPlayerAchievements` as returning a user's unlocked achievements and requires a user authentication key; `GetSchemaForGame` returns the complete game definition list and also requires a user key. These documented Web API methods provide a useful data contract, but the pages do not answer whether embedding each end user's key in a distributed local desktop application is permitted under the project's constraints. Valve's API Terms (last updated July 2010 on the reviewed page) identify an application via the API-key sign-up form, require the key to remain confidential and say Steam data is retrieved only as requested by the end user. This is insufficient to resolve that direct-client use question. The SDK stats documentation describes offline cache behavior in the integrated game's API context; it does not document a generic third-party desktop client interface.

The direct Web API user-key question remains open. The private client route is implemented experimentally under owner risk acceptance, is not Valve-approved, and has only one successful live observation on the owner's machine/game. Do not infer Valve support or broad Steam-version compatibility from this result or from successful operation in another project.

## Sources reviewed 2026-11-12

- [SAM upstream revision](https://github.com/gibbed/SteamAchievementManager/tree/de8b71048a0cee3c3e97cd8535e0f55ca86513e4)
- [SAM native loader](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Steam.cs)
- [SAM Steam client setup](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.API/Client.cs)
- [SAM game picker](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.Picker/GamePicker.cs)
- [SAM achievement/stat manager](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.Game/Manager.cs)
- [SAM zlib license](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/LICENSE.txt)
- [Valve Steamworks API overview](https://partner.steamgames.com/doc/sdk/api)
- [Valve Steamworks stats and achievements](https://partner.steamgames.com/doc/features/achievements)
- [Valve ISteamUserStats Web API reference](https://partner.steamgames.com/doc/webapi/ISteamUserStats)
- [Steam Web API Terms](https://steamcommunity.com/dev/apiterms)
