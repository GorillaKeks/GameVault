# GameVault v1 Feature Feasibility — Local-First Constraint

**Review date:** 2026-11-12
**Owner constraints:** no GameVault backend/server; Windows local-first; Tauri 2, React/TypeScript, Rust, SQLite; Steam only. The four features below remain mandatory. No feature was removed or weakened.

## Summary matrix

| Mandatory feature | Local source | External API | Offline support | Classification |
|---|---|---|---|---|
| Steam game library (complete account library) | Manifests cover installed apps; `localconfig.vdf` may contain apps seen/played on this device; `appcache` is not a reliable ownership ledger | `IPlayerService/GetOwnedGames` returns owned games when Game Details visibility allows and requires a user Web API key. Key/client-use and endpoint host still need Valve confirmation. | Only locally installed or previously recorded/cache entries; not all owned/uninstalled titles | **Conditional on Steam access** |
| Installed games | Registry-discovered/user-selected roots, `libraryfolders.vdf`, `appmanifest_<appid>.acf`, install directory existence and state flags | Not needed for basic detection | Yes, as long as the library drive is available; last scan can be stale | **Feasible with limitations** |
| Playtime information | Per-device `localconfig.vdf` often has total/recent playtime and last-play timestamps; local stats caches may have game-specific stats | `GetOwnedGames`, `GetRecentlyPlayedGames`, and `GetSingleGamePlaytime` require a user key; AppID association condition applies to the single-game call | Local snapshot may be shown offline with provenance/staleness; full owned-game coverage and remote refresh need API | **Conditional on Steam access** |
| Steam achievements | Some `appcache/stats` schema and per-user binary cache files exist locally; Steamworks SDK caches stats for games using its API, not a documented general third-party desktop interface | `GetPlayerAchievements` and `GetSchemaForGame` use a user key. Deprecated Steam Community XML may expose received achievements for many games but can be unavailable due to profile privacy and is not recommended by Valve. | Local cache only where present and parseable; SDK offline guarantee applies in game-context, not directly to GameVault. Prior approved API snapshot can be shown stale. | **Blocked by unresolved Steam restrictions** |

Classification applies to the whole stated feature, not a partial substitute. A local installed-app list is not the complete owned library. A playtime subset is not full library playtime. A list of received achievements is not necessarily a complete list of locked/unlocked achievements.

## Per-feature conclusions

### 1. Steam game library — conditional

To provide the complete owned account library, including titles never installed or played on this PC, local files are insufficient. Manifests identify only local install/download records. `localconfig.vdf` can retain local app history but is sparse and client-managed. A local app cache is neither a complete ownership ledger nor a reliable catalog. `GetOwnedGames` is the documented account method, requires a user key and only returns the list when the user's owned games/Game Details are visible. Therefore a complete library requires approved direct user-key API access or an owner-approved scope change; the current no-backend constraint makes the key interpretation the blocking external decision.

For installed entries, manifests commonly provide AppID, name, install directory and state flags; the configured library root plus `installdir` yields the expected installation path. Local app metadata may help identify app type/name but is not complete. No game art is packaged; image sources, runtime use and caching/redistribution rights need a separate review.

Installed titles can still appear while offline. Uninstalled owned titles may be absent until an approved account API is configured. Do not represent absent titles as not owned.

### 2. Installed games — feasible with limitations

Multiple libraries can be discovered through the registered root and its library folder metadata; each library contains per-AppID manifests. Verify flags plus directory existence. A configured drive that is missing/unmounted is “unavailable,” not “uninstalled.” A moved path may be stale until Steam updates its mapping or the user selects the new root. Manifest presence and an existing folder do not guarantee a valid, complete install; Steam itself documents cases where installed games appear uninstalled and require library repair/re-recognition.

This supports a useful local installed list without Steam login or network, but relies on internal file formats with no stable Valve schema contract. Keep detection read-only and best effort; do not modify files to make Steam recognize a game.

### 3. Playtime — conditional for complete v1

This machine's local `localconfig.vdf` contained 212 occurrences each of `Playtime` and `LastPlayed`, and eight `Playtime2wks` occurrences; values were not collected. Independent analyses show these are sparse client-local fields. They can persist for titles no longer installed and may lag cloud/account data. They are not guaranteed for every owned game, device or Steam state, and Steam does not publish this file as a third-party API.

`IPlayerService` offers user-key methods for owned/recent games and per-app playtime. To make playtime available across the owned library (including games not represented locally), the direct user-key path must be confirmed. Until then, show only local snapshots with source/time labels; do not fill gaps with zero. The API method's recent-played response/window and timestamp field should be confirmed before relying on those exact fields.

### 4. Achievements — blocked

Valve's Steamworks SDK exposes stats/achievements to the game integrated with it. Valve says Steam caches those stats for the SDK APIs to work offline, but that statement is about the game's API context. GameVault is not the game, and Steam offers no documented generic desktop SDK access to all installed games' player stats. The Web API can return a player's unlocked achievements and a game's schema using a user key, but key use and endpoint host remain unresolved.

If Valve approves the API path and both calls return data, the schema can supply the game's achievement definitions and the player method its unlocked entries; GameVault could derive locked entries by subtracting only when the schema is complete. Display names/descriptions/icons are game-published and can be absent/hidden; unlock timestamps are shown only if the source returns them. Progress bars are available only for games that define a meaningful achievement progress stat; there is no universal progress percentage.

Valve also documents Community Data XML as **deprecated**. It warns privacy settings may suppress data and says that for most games only achievements already received are exposed; this cannot reliably provide a complete set of locked achievements for every game. Local `UserGameStatsSchema_*.bin` and `UserGameStats_*_*.bin` files were present on this PC for eight apps each, but their path/format are undocumented and reverse-engineered. They may be missing, stale, or incompatible, and reading them in production would cross the older master prompt's explicit Steam-file-scraping exclusion. Research presence is not enough to approve this source.

Thus achievement metadata, earned state, icons, unlock times and progress are not guaranteed locally for every title. Per-game progress may only be derived when a game publishes a meaningful progress stat. No achievement writes, unlocks, resets or SAM behavior are in scope.

## Direct key path to investigate (not approved)

The only promising no-backend way to query the documented account APIs is for each user to register and provide their own Web API key, with GameVault sending HTTPS requests directly to Valve from Rust. The app must never contain a developer's shared key. A candidate local protection design is Windows Credential Manager/DPAPI; this protects the key at rest from casual file disclosure but not from a compromised same-user process. Store no key in SQLite, logs, frontend memory longer than necessary, telemetry or exports; allow removal and document revocation.

Technical local protection does **not** answer the legal/product questions. Valve says standard user keys require a domain and acceptance of its terms, and API keys must remain confidential. Valve does not clearly document this third-party app / user-owned-key pattern, who must own the registered application/domain, or whether direct use from a distributed desktop executable is accepted. Valve's method pages show partner-host URLs with user keys, but the overview says all partner-host requests require publisher keys. The direct key route is not cleared until Valve answers the questions below.

## Exact questions for Valve

Ask via the Steam Web API support contact listed in the current API Terms / official developer channel; record Valve's answer and date:

1. May a free, open-source, locally installed Windows desktop app make API requests directly from the user's PC using an individual Steam account holder's own standard user Web API key (no GameVault server and no shared key)?
2. Is that key considered confidential if the user enters it into the app and the app sends it only to `api.steampowered.com` over HTTPS? Does use by this third-party desktop program count as sharing the key with a third party under the Terms?
3. What exact application and domain must each user register for their key? Must the key holder own/operate the application and domain, or can a distributed GameVault app use the user's separately registered key? Can a domain be a static information page with no server?
4. Which hostname should a standard user key use for `GetOwnedGames`, `GetRecentlyPlayedGames`, `GetSingleGamePlaytime`, `GetPlayerAchievements` and `GetSchemaForGame`? Method pages show `partner.steam-api.com`, while the overview says every request to that host requires a publisher key. Confirm whether/how `api.steampowered.com` supports each user-key method.
5. Does Valve permit this app to cache and display returned Steam data locally, and what retention, refresh, per-method limits, attribution and deletion rules apply beyond the API Terms' general daily ceiling?
6. Are deprecated Community Data XML endpoints still permitted for a free, public desktop app, and can they be requested directly from the user's machine? What reliable completeness/visibility behavior should the app expect for achievement pages?
7. Is there a documented client API intended for an independent local desktop companion to read the current user's achievements/schema without injecting into or pretending to be a game?

## User-visible fallback when sources fail

- Account API unavailable/private/not configured: distinguish “account library unavailable” from empty library; retain local installed list and cached snapshots with date.
- Offline: show installed manifests and last locally stored playtime/cache snapshot marked stale; no claim that the account's full owned list is complete.
- Missing/unreachable library drive: mark that library unavailable and show last scan date; do not mark its games uninstalled.
- Incomplete manifest/dir mismatch: “installation status needs Steam verification” or “incomplete/unknown,” not installed/playable.
- Missing achievement schema or data: show “achievement data unavailable for this game”; do not infer all locked, all unlocked, or 0%.
- API schema change: preserve last good data, stop parsing the affected payload and show refresh error; no destructive cache replacement.

## Overall phase assessment

**CONDITIONAL GO — local architecture is viable, but the mandatory account features are not yet cleared.** Installed-game detection and a best-effort local playtime snapshot are technically feasible. A complete account library/playtime and reliable cross-game achievement view require either Valve-approved per-user key use or a separately approved local cache integration. Because the owner forbids any backend and requires all four features, **do not scaffold the Steam integration until Valve answers the key questions**. If no permitted direct key path exists and undocumented cache parsing is not accepted, the project is NO-GO for the current mandatory feature set; return to the owner rather than dropping features.

## Current ADR status

| ADR | Status | Owner decision reflected |
|---|---|---|
| ADR-001 Steam authentication | Proposed | No server; OpenID is not selected; desktop/key pattern remains unresolved |
| ADR-002 Steam API access | Proposed | No shared key/backend; direct individual user key needs Valve confirmation |
| ADR-003 Data storage | Accepted | Owner confirmed local SQLite and Rust native access layer |
| ADR-004 Security/privacy | Proposed | Security baseline and narrow local read boundary await implementation review |
| ADR-005 Open-source license | Proposed | FOSS confirmed; exact Apache-2.0 choice/text awaits approval |
| ADR-006 Integration architecture | Accepted | Owner confirmed no-backend local Tauri stack; account data path remains unresolved |

## Sources (verified 2026-11-12)

- [Valve Web API authentication/key types](https://partner.steamgames.com/doc/webapi_overview/auth)
- [Valve Web API overview and host rules](https://partner.steamgames.com/doc/webapi_overview)
- [Valve API Terms](https://steamcommunity.com/dev/apiterms)
- [IPlayerService](https://partner.steamgames.com/doc/webapi/IPlayerService)
- [ISteamUserStats](https://partner.steamgames.com/doc/webapi/ISteamUserStats)
- [Valve Steamworks stats/achievement API and in-game offline behavior](https://partner.steamgames.com/doc/features/achievements)
- [Deprecated Community Data XML and privacy caveat](https://partner.steamgames.com/documentation/community_data)
- [Steam Support: library folders, moved installs and recovery](https://help.steampowered.com/en/faqs/view/71AB-698D-57EB-178C)
- [Windows Credential Manager/API-key storage guidance](https://learn.microsoft.com/en-us/windows/win32/secbp/handling-passwords)
- For local client-file formats and the local scan details, see [`LOCAL_STEAM_DATA.md`](LOCAL_STEAM_DATA.md).
