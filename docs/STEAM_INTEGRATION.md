# Steam Integration Feasibility

**Research date:** 2026-11-12. Findings reflect the primary Valve/Steam material linked below as inspected on this date. Recheck before implementation and release. This is a technical and product assessment, not legal advice or confirmation of access for an individual account.

**Current owner constraint (Phase 1.6):** no GameVault backend, server, cloud account or central database. The hosted-backend path discussed in earlier Phase 1.5 text is superseded. Current data-source conclusions are in [`V1_FEATURE_FEASIBILITY.md`](V1_FEATURE_FEASIBILITY.md).

## Executive assessment

**Conditional; not implementation-ready for complete account data.** Valve documents APIs for owned games, playtime, profile summaries, and achievements, but relevant player-data calls require a Web API user key. Valve documents domain association and key confidentiality. Publisher keys require a Steamworks publisher group and must stay server-side, so they cannot be used in a local app. Steam OpenID is a website flow returning a SteamID, not authorization; no native desktop callback is documented.

The owner has ruled out a GameVault server/backend. Do not ship a shared key or start player-data API calls until Valve confirms whether an individual user's own key may be used directly by a distributed desktop app, what domain/application each user must register, and which hostname is correct for each user-key endpoint. No GameVault login or API-key flow is implemented.

## Key and API access distinctions

Valve says some Web API methods expose public data without authorization; others require a user key; protected methods require a publisher key and special permissions. User keys are available to Steam account holders but registration requires an associated domain and agreement to the API Terms. Publisher keys are connected to a Steamworks publisher account/group and are intended for requests from secure publisher servers; Valve says not to distribute them with a game client.

The API Terms identify use with the application named in the key signup, require the key to remain confidential, say Steam Data must be retrieved as requested by an end user, require a privacy policy and disclosure of storage/countries for nonpublic user data, and currently state a limit of 100,000 calls per day. The Terms page shows a July 2010 update date, so all terms must be checked again before launch. The documentation reviewed did not state per-endpoint quotas.

**Implications:**

- A shared key compiled into GameVault, a frontend bundle, public source, or distributable binary is not acceptable.
- A publisher key is not a client credential and would require an eligible publisher relationship plus a secure server. No such access is assumed.
- A user-provided personal user key might be technically possible, but Valve's reviewed docs do not explain a third-party desktop app collecting/using an end user's key, and the domain/application registration requirement remains. Do not treat this as approved or user-friendly without confirmation.
- A GameVault-operated backend could keep a project key confidential, but it is explicitly prohibited by the current owner decision and is not an option in this phase.
- OpenID verifies identity; it does not exchange for a documented general-purpose API token or override Steam privacy settings.

## Data-by-data assessment

| Desired data | Documented method/source | Requirements and limitations | Assessment for the proposed standalone app |
|---|---|---|---|
| SteamID identity | Steam OpenID 2.0 through `https://steamcommunity.com/openid/` | Valve's docs describe a user beginning on a third-party website, signing in on Steam Community, and being redirected to the website's `return` URL with OpenID data. No password is given to GameVault. No native desktop callback/loopback scheme is documented. | **Not selected** under no-server constraint; manual ID entry is not authentication. |
| Public profile summary | `ISteamUser/GetPlayerSummaries` | Requires a user Web API key and SteamID. Valve says this call should be used when a website needs to display user information; it cautions game clients to use client APIs for their own user context. Returned fields reflect profile visibility. | **Conditionally viable** only after confirming the GameVault key and desktop/web deployment model. Do not imply private data access. |
| Owned games | `IPlayerService/GetOwnedGames` | Requires a user Web API key and SteamID. Valve says games are returned only if the player's owned games/game details are visible. Optional parameters include game info and played free games. | **Conditionally viable** with visible Game Details and a compliant key path; current local-only distribution does not have one. |
| Recently played and playtime | `IPlayerService/GetRecentlyPlayedGames`, `GetSingleGamePlaytime` | Both list a user key. The single-game call says the Web API key must be associated with the AppID for results. Availability and privacy behavior must be verified per endpoint. | **Conditionally viable** after key/access review. Treat absent results as unknown, not zero. |
| Earned achievement state | `ISteamUserStats/GetPlayerAchievements`; experimental read-only local Steam-client interface | Web API requires a user key, SteamID, AppID and has unresolved host/third-party use. Local interface is undocumented for a companion and Steam-version-sensitive. | **Experimental prototype authorized under explicit owner risk acceptance; no live ABI validation, completeness or Valve support claim.** |
| Achievement definitions/schema | `ISteamUserStats/GetSchemaForGame` | Requires a user Web API key and AppID; describes that game's published stats/achievement schema. No universal availability guarantee is documented. | **Conditionally viable**; only show schema/data actually returned. |
| Public store catalog names/basic data | `IStoreService/GetAppList` | Valve describes this service as public store information; its method page lists “any Web API key,” requires pagination for the full catalog, and exposes timestamps. The older `ISteamApps/GetAppList` is marked deprecated. Host/key pairing and the intended use for this desktop product still need confirmation. | **Partly viable** for store catalog lookup after confirming registration/terms. It does not establish ownership or installed state. |
| Full store descriptions, artwork, prices, other metadata | Store-facing methods and client/community surfaces vary | The reviewed docs do not establish one complete, unrestricted desktop API for all desired detail or grant blanket redistribution/caching rights for game artwork. | **Unresolved**. Use no packaged Steam/game art; verify each field and asset right before display/cache. |
| Installed games and Steam library folders | Read-only `libraryfolders.vdf` and `appmanifest_<appid>.acf` scan (research authorized in Phase 1.6) | No documented Web API method for installed games/paths was found. The local formats are client-managed and may be stale, changed or incomplete; no third-party schema/stability contract was found. | **Feasible with limitations** for local detection. Narrow scanning is proposed; arbitrary Steam-file scraping remains out of scope. See [`LOCAL_STEAM_DATA.md`](LOCAL_STEAM_DATA.md). |

### Public endpoints vs. keys

Some public Web API methods may be callable without a key, according to Valve's authentication documentation. That does not include the player-specific methods above that explicitly require a user key. Public app catalog data is not a substitute for account ownership, playtime, or achievement state. Do not infer access from a browser page being publicly visible.

## Authentication decision

Steam's OpenID docs explicitly describe a browser-based website relying party. They do not document `localhost`, a custom URI scheme, or a native desktop application's return/callback registration. A website callback would require a server and is incompatible with the owner constraint. Identity alone does not make private player data available. Never collect a Steam password, cookie, Steam Guard code, or session token.

**No supported account-authorization flow for the standalone local-only GameVault has been verified.** For public Web API account access, obtain current Valve confirmation or authoritative documentation before implementation of that route:

1. May a free, open-source desktop app use an individual user's own standard key, entered and held locally, to send direct HTTPS requests to Steam?
2. What domain/application must each user register, and does the Terms' confidentiality/no-sharing condition allow use by this third-party desktop app?
3. Which public host is correct for the player methods that list a user key, given the method URL examples and overview host restrictions?
4. What precise privacy, visibility, caching, retention and rate-limit requirements apply to these calls?

Do not add a backend (owner-prohibited) or ask users for API keys on assumption alone.

## Privacy, rate limits, caching, and data integrity

- The Web API Terms currently cap use at 100,000 calls/day per the Terms page; no endpoint-specific limits were found. Use a much smaller budget, explicit user refresh, bounded concurrency, timeout, and retry/backoff only after access is approved. Never evade throttling.
- The Terms require a privacy policy for nonpublic end-user data, retrieval at the user's request, disclosure of data stored and countries where it is stored, and “as is” treatment. A desktop app storing locally still needs a reviewed policy/terms interpretation before it queries nonpublic user data.
- The policy page currently shows a July 2010 revision date; verify current text with Valve before relying on it.
- Persist `fetched_at` and source/provenance separately from game event timestamps. Represent private, not returned, unsupported, and stale values distinctly from zero/locked.
- Make cache duration and artwork handling dependent on verified API/asset terms; no durations or redistribution rights are assumed here.
- Validate response shape and bounds. Handle private profiles, missing game schemas, key errors, invalid SteamIDs, timeouts, offline operation, HTTP errors, and changed fields without dropping local annotations.

## Steamworks SDK and client integration

Valve describes Steamworks SDK/API as a set of game/app integration features. The documented C++ `ISteamUserStats` methods operate in a game's Steam context; they are not a generic remote catalog of a user's entire account library. The SDK API is distinct from Web API methods and from direct access to Steam client internals.

The proposed React/Tauri companion is not itself a Steam game. Do not assume that using the SDK grants library access or that Steam partner/publisher access exists. **Owner direction (2026-11-15) explicitly accepts the unresolved risks and authorizes a narrow read-only SAM-inspired prototype** that loads `steamclient.dll`, requests fixed versioned interfaces and reads definitions/earned state through Steam's client pipe. This remains a private, undocumented, version-sensitive interface, not a documented Steamworks SDK/API guarantee. See [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md) and ADR-007. It must not inspect process memory, inject into games, read credentials/session files, write achievements/stats, scrape Community pages, or bypass profile privacy. A read-only client integration cannot claim to enumerate a complete owned library by itself.

## Recommendations

- **SteamID sign-in:** not selected; website flow conflicts with no-server constraint and a native desktop callback is undocumented. A manually entered SteamID is not authenticated.
- **Public profile, owned games, playtime:** conditional on a Valve-approved local per-user key flow and Steam privacy settings. Not ready for implementation.
- **Achievements:** experimental per-game read-only Steam-client prototype exists under owner-accepted risk; no Valve support or client-build compatibility claim. No writes are permitted.
- **Public store catalog:** potentially usable for basic catalog lookup, but verify registered key and endpoint host against current docs/terms before using.
- **Installed games:** local read-only manifest/library scanning can provide best-effort detection; local formats carry no third-party stability contract.
- **Library paths/local configuration:** narrow, read-only access is under research; full owned library and playtime do not follow from manifests alone.
- **Game images and detailed metadata:** unresolved until source rights, endpoint and cache/distribution permissions are verified. A focused artwork follow-up is recorded below.
- **Achievement mutation:** out of scope. No unlock, clear, reset, stat edit, automation, or SAM write behavior. The read-only SAM-inspired definitions/state prototype is authorized, experimental and not live-validated. See [`SAM_READ_ONLY_INTEGRATION.md`](SAM_READ_ONLY_INTEGRATION.md).

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

## Game artwork follow-up (2026-11-15)

Valve's current Steamworks graphical-assets documentation confirms that games have an App Icon (184×184 JPG) for compact Community/Client presentation and separate Library assets such as the 600×900 Library Capsule. It does **not** document a public third-party desktop retrieval endpoint or grant GameVault permission to hotlink, cache, redistribute, or package those images. The Steam Web API Terms page does not specifically settle these artwork questions and still identifies itself as last updated July 2010.

**Local cache feasibility was verified on 2026-11-15.** A recursive, read-only inspection of this machine's `appcache/librarycache` found 1,588 image files and matching 32×32 hashed JPEG icons for all seven games currently detected by GameVault. The cache is client-managed and undocumented, so availability and layout can change across Steam versions or machines.

**Implemented local-only behavior:** for each detected AppID, GameVault checks only `appcache/librarycache/<appid>` under the canonical Steam root. It accepts one unambiguous 40-hex-name JPEG icon, validates it as a small square image, caps the file at 16 KiB and the directory at 128 entries, and returns image bytes without returning a filesystem path. The UI renders the bytes as a data URL and falls back to the game's initial if the image is absent or invalid. This makes no network request, sends no AppIDs to an image host, and does not modify or copy Steam files into the repository. This is best-effort local display of artwork already cached by Steam, not permission to bundle or redistribute game art. External icon retrieval, packaged art and export of the images remain out of scope until their source and usage conditions are verified.

**Next work:** run the updated desktop build and verify the local icons appear, the initials fallback works for a missing cache entry, and the behavior remains offline. Revisit remote sources only if local coverage proves insufficient and a documented source/usage basis is found.

### Support access attempt and ready-to-submit inquiry

On 2026-11-15 the official Steamworks Support link was opened after the user completed Steam sign-in. `HelpWithPublishing` redirected to the general Steam Support home rather than presenting a developer ticket form. The official Steamworks Development group says its discussions are restricted to accounts with Steamworks access; its main page offered a “send join request” action. The owner explicitly approved sending that request, but Steam rejected it because the group had reached its maximum number of pending join requests and directed the user to contact a moderator. The selected group moderator's profile exposes a friend-request action but no direct-message action. The owner later explicitly approved a public group comment, but no comment editor was available to the signed-in non-member account; Steam's group page also states the group is restricted to Steamworks-access accounts and only announcements are public. No ticket, join request, friend request, moderator message, or public post was submitted. Do not contact a moderator by friend request or use another public channel under this identity without the owner's explicit authorization for that specific action and audience.

If Steamworks developer-support access becomes available, submit this inquiry without account credentials, SteamIDs, installed-game IDs, logs, or screenshots:

> **Subject:** Guidance on displaying Steam game app icons in a local-only desktop app
>
> Hello,
>
> We are developing GameVault, a free and open-source Windows desktop application for organizing a player's Steam games. It has no backend, hosted account, shared API key, or telemetry. The app can detect locally installed games and we would like to show the matching Steam app icon beside each game.
>
> Which documented interface or asset URL should a third-party desktop application use to retrieve the 184×184 app icon for a known AppID? Is direct display/hotlinking permitted for this use, or must the image be downloaded? If local caching is permitted, what retention/refresh limits and attribution requirements apply? Does retrieval require a registered Steamworks application or API key, and are there rate limits or other terms we should follow?
>
> We intend to request artwork only when the user asks GameVault to refresh its local library, disclose any external requests, and avoid sending account credentials or other Steam session data. Please point us to the applicable current documentation or terms.
>
> Thank you.

This draft is not a claim that the described use is already permitted. Keep icon retrieval unimplemented until Valve provides a documented route and usage conditions, or the owner chooses an explicitly reviewed alternative.

Primary references checked 2026-11-15: [Valve Graphical Assets - Overview](https://partner.steamgames.com/doc/store/assets?l=english), [Valve Library Assets](https://partner.steamgames.com/doc/store/assets/libraryassets?language=english), and [Steam Web API Terms of Use](https://steamcommunity.com/dev/apiterms). These references document asset types and general API terms, not the missing third-party icon-fetch/caching grant.

## Phase 1.5 revalidation addendum (2026-11-12)

The current Valve documentation reconfirms the conditional assessment. The Steam Web API overview adds a material host/key ambiguity: it describes `api.steampowered.com` as the public host and says every call to `partner.steam-api.com` must contain a valid publisher key; the endpoint method references nevertheless show `partner.steam-api.com` URLs while documenting user keys for player methods. Do not copy a method URL into a client without resolving this inconsistency. Any approved user-key client path must use the host and method Valve confirms for that key type.

The terms page still says **Last updated July 2010**, names an app in the key sign-up, says the key must stay confidential, requires retrieval at an end user's request, and requires a privacy policy naming stored Steam data and storage countries. This page permits the use of Steam Web API/data in an application under its terms, but does not settle every question about commercial desktop distribution, a backend proxy, end-user key collection, or domain registration on behalf of GameVault. These are questions for Valve and qualified counsel, not conclusions from this review.

The requested data scope is evaluated in [`V1_FEATURE_FEASIBILITY.md`](V1_FEATURE_FEASIBILITY.md), and local file structures in [`LOCAL_STEAM_DATA.md`](LOCAL_STEAM_DATA.md). Achievement progress is not a universal data contract; app-published stats do not guarantee a comparable percentage. Artwork terms and retention remain unresolved.

**OpenID implementation security note:** use a fixed Steam OP endpoint and maintained OpenID 2.0 relying-party library. Verify signature, required signed fields, discovery, exact `return_to`, and one-time nonce/replay protection before accepting the SteamID. Bind a one-time state value to the initiated user action, use TLS and short-lived/revocable GameVault sessions, redact callback parameters, and never accept a SteamID from an unverified callback. Valve's OpenID flow does not issue a general API token or grant access to private information.

**Current conclusion:** no backend is permitted. Valve confirmation for per-user Web API keys remains unresolved. Separately, the owner has accepted the risk for an experimental read-only SAM-inspired local client path; it uses undocumented client interfaces and carries policy/compatibility uncertainty. It does not provide complete library enumeration or account-wide playtime. The four owner-mandated features remain mandatory; label missing data honestly and return for an owner decision if no permitted/reliable source supports a feature.

The official Web API overview, key authentication page, OpenID and ownership page, endpoint references, API terms and OpenID 2.0 validation spec were re-opened on 2026-11-12. See the full source list in [`FEASIBILITY_REVIEW.md`](FEASIBILITY_REVIEW.md).
