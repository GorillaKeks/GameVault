# Steam Integration Feasibility

**Research date:** 2026-11-12. Findings reflect the primary Valve/Steam material linked below as inspected on this date. Recheck before implementation and release. This is a technical and product assessment, not legal advice or confirmation of access for an individual account.

## Executive assessment

**Conditional; not implementation-ready for Steam account data.** Valve documents APIs for owned games, playtime, profile summaries, and achievements, but the relevant player-data calls require a Web API user key. Valve documents that user keys are registered to a domain and tied to the registrant's application; it requires key confidentiality. Publisher keys have stronger access, require a Steamworks publisher group, and must be kept server-side. Steam OpenID is documented as a browser-based website flow that returns a SteamID; it does not grant private profile or library access, and the reviewed documentation does not define a native desktop redirect/loopback flow.

No GameVault domain, Web API registration, backend, privacy policy, or key model exists. Do not ship a shared key in the desktop app or start Steam account integration until the owner chooses a permitted deployment model and the exact key/redirect arrangements are confirmed against current Valve requirements.

## Key and API access distinctions

Valve says some Web API methods expose public data without authorization; others require a user key; protected methods require a publisher key and special permissions. User keys are available to Steam account holders but registration requires an associated domain and agreement to the API Terms. Publisher keys are connected to a Steamworks publisher account/group and are intended for requests from secure publisher servers; Valve says not to distribute them with a game client.

The API Terms identify use with the application named in the key signup, require the key to remain confidential, say Steam Data must be retrieved as requested by an end user, require a privacy policy and disclosure of storage/countries for nonpublic user data, and currently state a limit of 100,000 calls per day. The Terms page shows a July 2010 update date, so all terms must be checked again before launch. The documentation reviewed did not state per-endpoint quotas.

**Implications:**

- A shared key compiled into GameVault, a frontend bundle, public source, or distributable binary is not acceptable.
- A publisher key is not a client credential and would require an eligible publisher relationship plus a secure server. No such access is assumed.
- A user-provided personal user key might be technically possible, but Valve's reviewed docs do not explain a third-party desktop app collecting/using an end user's key, and the domain/application registration requirement remains. Do not treat this as approved or user-friendly without confirmation.
- A GameVault-operated backend could keep a GameVault Web API key confidential, but adds hosting, privacy-policy, storage-country, security, deletion, availability, and operating-cost obligations. The owner has not approved a backend.
- OpenID verifies identity; it does not exchange for a documented general-purpose API token or override Steam privacy settings.

## Data-by-data assessment

| Desired data | Documented method/source | Requirements and limitations | Assessment for the proposed standalone app |
|---|---|---|---|
| SteamID identity | Steam OpenID 2.0 through `https://steamcommunity.com/openid/` | Valve's docs describe a user beginning on a third-party website, signing in on Steam Community, and being redirected to the website's `return` URL with OpenID data. No password is given to GameVault. No native desktop callback/loopback scheme is documented in the material reviewed. | **Conditionally viable** via an approved website/backend design; not verified for a pure desktop app. |
| Public profile summary | `ISteamUser/GetPlayerSummaries` | Requires a user Web API key and SteamID. Valve says this call should be used when a website needs to display user information; it cautions game clients to use client APIs for their own user context. Returned fields reflect profile visibility. | **Conditionally viable** only after confirming the GameVault key and desktop/web deployment model. Do not imply private data access. |
| Owned games | `IPlayerService/GetOwnedGames` | Requires a user Web API key and SteamID. Valve says games are returned only if the player's owned games/game details are visible. Optional parameters include game info and played free games. | **Conditionally viable** with visible Game Details and a compliant key path; current local-only distribution does not have one. |
| Recently played and playtime | `IPlayerService/GetRecentlyPlayedGames`, `GetSingleGamePlaytime` | Both list a user key. The single-game call says the Web API key must be associated with the AppID for results. Availability and privacy behavior must be verified per endpoint. | **Conditionally viable** after key/access review. Treat absent results as unknown, not zero. |
| Earned achievement state | `ISteamUserStats/GetPlayerAchievements` | Requires a user key, SteamID, AppID. It returns unlocked achievements. The method page does not completely specify all profile visibility/failure cases. | **Conditionally viable** for read-only display after account-data access and per-game behavior are confirmed. |
| Achievement definitions/schema | `ISteamUserStats/GetSchemaForGame` | Requires a user Web API key and AppID; describes that game's published stats/achievement schema. No universal availability guarantee is documented. | **Conditionally viable**; only show schema/data actually returned. |
| Public store catalog names/basic data | `IStoreService/GetAppList` | Valve describes this service as public store information; its method page lists “any Web API key,” requires pagination for the full catalog, and exposes timestamps. The older `ISteamApps/GetAppList` is marked deprecated. Host/key pairing and the intended use for this desktop product still need confirmation. | **Partly viable** for store catalog lookup after confirming registration/terms. It does not establish ownership or installed state. |
| Full store descriptions, artwork, prices, other metadata | Store-facing methods and client/community surfaces vary | The reviewed docs do not establish one complete, unrestricted desktop API for all desired detail or grant blanket redistribution/caching rights for game artwork. | **Unresolved**. Use no packaged Steam/game art; verify each field and asset right before display/cache. |
| Installed games and Steam library folders | Local Steam client configuration/manifests or client behavior | No documented Web API method for a user's installed-game list or library paths was found in the reviewed Valve reference. Steam-local files are implementation details and may be stale, changed, or incomplete; their compatibility/terms were not established. | **Out of scope** under `MASTER_PROMPT.md`. Do not read these files or reverse engineer the client in this project task. A future explicit opt-in import requires a separate policy, threat, and supportability decision. |

### Public endpoints vs. keys

Some public Web API methods may be callable without a key, according to Valve's authentication documentation. That does not include the player-specific methods above that explicitly require a user key. Public app catalog data is not a substitute for account ownership, playtime, or achievement state. Do not infer access from a browser page being publicly visible.

## Authentication decision

Steam's OpenID docs explicitly describe a browser-based website relying party. They do not document `localhost`, a custom URI scheme, or a native desktop application's return/callback registration. A web callback hosted on a GameVault-controlled domain may be a design to investigate, but it would create an online service, and identity alone does not make private player data available. Never collect a Steam password, cookie, Steam Guard code, or session token.

**No supported authentication flow for a standalone, local-only GameVault has been verified.** Before implementation, obtain current Valve confirmation or authoritative documentation for one of:

1. A GameVault-operated website/backend that registers the application/domain, validates OpenID, keeps an API key server-side, and serves user-requested data under a published privacy and retention policy; or
2. A user-owned key flow, if Valve confirms that the app may accept/store the key and how each user registers it to the required domain/application; or
3. A narrowed product that drops account-connected data and redefines v1 (requires owner approval and new scope).

Do not add a backend or ask users for API keys on assumption alone.

## Privacy, rate limits, caching, and data integrity

- The Web API Terms currently cap use at 100,000 calls/day per the Terms page; no endpoint-specific limits were found. Use a much smaller budget, explicit user refresh, bounded concurrency, timeout, and retry/backoff only after access is approved. Never evade throttling.
- The Terms require a privacy policy for nonpublic end-user data, retrieval at the user's request, disclosure of data stored and countries where it is stored, and “as is” treatment. A desktop app storing locally still needs a reviewed policy/terms interpretation before it queries nonpublic user data.
- The policy page currently shows a July 2010 revision date; verify current text with Valve before relying on it.
- Persist `fetched_at` and source/provenance separately from game event timestamps. Represent private, not returned, unsupported, and stale values distinctly from zero/locked.
- Make cache duration and artwork handling dependent on verified API/asset terms; no durations or redistribution rights are assumed here.
- Validate response shape and bounds. Handle private profiles, missing game schemas, key errors, invalid SteamIDs, timeouts, offline operation, HTTP errors, and changed fields without dropping local annotations.

## Steamworks SDK and client integration

Valve describes Steamworks SDK/API as a set of game/app integration features. The documented C++ `ISteamUserStats` methods operate in a game's Steam context; they are not a generic remote catalog of a user's entire account library. The SDK API is distinct from Web API methods and from direct access to Steam client internals.

The proposed React/Tauri companion is not itself a Steam game. Do not assume that using the SDK grants library access or that Steam partner/publisher access exists. Do not directly load `steamclient.dll`, inspect process memory, use undocumented client interfaces, scrape Community pages, or bypass a profile's privacy choice.

## Recommendations

- **SteamID sign-in:** conditional; website flow documented, native desktop callback not verified.
- **Public profile, owned games, playtime, achievements:** conditional on a documented key strategy and Steam privacy settings. Not ready in current local-only design.
- **Public store catalog:** potentially usable for basic catalog lookup, but verify registered key and endpoint host against current docs/terms before using.
- **Installed game/library paths:** out of scope; no verified documented remote source and local-file parsing is excluded by current project scope.
- **Game images and detailed metadata:** unresolved until source rights, endpoint and cache/distribution permissions are verified.
- **Achievement mutation:** out of scope. No unlock, clear, reset, stat edit, automation, or SAM behavior.

## Primary sources reviewed

- Steam Web API authentication/key types: https://partner.steamgames.com/doc/webapi_overview/auth
- Steam Web API overview (public vs protected methods; HTTPS; host notes): https://partner.steamgames.com/doc/webapi_overview
- Steam Web API Terms (confidentiality, privacy, user request, storage-country, call cap): https://steamcommunity.com/dev/apiterms
- Steam Community API and OpenID developer page: https://steamcommunity.com/dev
- Steam OpenID / authentication and ownership (website redirect flow): https://partner.steamgames.com/doc/features/auth
- `IPlayerService`: https://partner.steamgames.com/doc/webapi/IPlayerService
- `ISteamUser`: https://partner.steamgames.com/doc/webapi/ISteamUser
- `ISteamUserStats`: https://partner.steamgames.com/doc/webapi/ISteamUserStats
- `IStoreService`: https://partner.steamgames.com/doc/webapi/IStoreService
- `ISteamApps` (deprecated `GetAppList`): https://partner.steamgames.com/doc/webapi/ISteamApps
- Steamworks SDK/API overview: https://partner.steamgames.com/doc/sdk and https://partner.steamgames.com/doc/sdk/api
- Steam branding guidance: https://partner.steamgames.com/doc/marketing/branding
