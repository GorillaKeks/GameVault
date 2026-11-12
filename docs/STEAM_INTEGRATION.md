# Steam Integration Research

**Research date:** 2026-11-12. Valve API and policy details can change; re-check before implementation and release. Findings below are technical research, not legal advice or confirmation of individual access.

## Executive assessment

**Conditional / not implementation-ready.** Steam documents a Web API and Steam OpenID, but the library and achievement methods relevant here require an API key. Valve's API terms state that the key is confidential, personal to the registrant and specific to the application named in the signup form. A public desktop binary cannot keep a shared key secret. Several Steamworks methods are explicitly intended for trusted/secure servers. No approved backend, publisher relationship, or supported local-only key distribution model has been established.

Do not begin account linking or call these APIs from a shipped client until the exact API key and authentication model is confirmed with current Valve documentation and terms. Never request a Steam password or persist cookies/session credentials.

## Access categories

| Method | What it can provide | Access / limitations | GameVault position |
|---|---|---|---|
| Public Steam Web API | Documented HTTP interfaces, but each method has its own parameters and restrictions | API key required by the Web API documentation generally; key and terms bind use to a registered application; privacy and method restrictions apply | Preferred source only if a compliant key/auth model is approved |
| Steam OpenID | Verifies a SteamID through Steam's OpenID 2.0 provider without asking for a Steam password | Documented primarily for websites; a desktop callback design is not validated here; identity does not grant access to private owned games or achievements | Research further; no implementation yet |
| Steamworks SDK | Game-facing integration (e.g. the current user's SteamID and game-specific functionality) | SDK/API is designed for apps/games integrating Steamworks. Steam documentation describes SDK distribution/use in a developer context; it is not established as a generic third-party library inventory SDK | Do not assume it gives GameVault a player's entire library |
| Steam Community pages/interfaces | Some publicly viewable community information | A page being publicly reachable does not make its interface a supported API; scraping is fragile and policy-sensitive | Do not scrape |
| Local Steam installation, library folders, app manifests | Potential clues about installed local games, not authoritative ownership of a full account library | Local implementation detail; can be stale, incomplete, per-machine and altered. No support/stability or terms review completed. Reading user files requires explicit scope and careful path/permission handling | Excluded by current master prompt; reconsider only through a separate documented decision |
| Third-party libraries/services | May wrap Steam protocols or expose convenience methods | Must audit provenance, protocol stability, license, credential handling, account/security risk, and terms | No speculative dependency or integration |

## Relevant documented interfaces

### Owned games and playtime

Valve documents `IPlayerService/GetOwnedGames`. It requires a user Web API key and SteamID, with flags such as `include_appinfo` and `include_played_free_games`. The method returns owned games only if the player's owned games/game details are visible. `GetRecentlyPlayedGames` and `GetSingleGamePlaytime` are also documented; the latter notes the key must be associated with the AppID for results. These are real documented interfaces, but that does not establish that embedding a shared key in GameVault's client is permitted or safe.

Primary reference: [IPlayerService](https://partner.steamgames.com/doc/webapi/IPlayerService).

### Profile information

`ISteamUser/GetPlayerSummaries` accepts SteamIDs and a user Web API key. Valve says it should be used when a website needs to display user information, and recommends client APIs if a game client needs that information. Do not interpret this as blanket approval for a third-party desktop application; verify terms and the app's exact use.

Primary reference: [ISteamUser](https://partner.steamgames.com/doc/webapi/ISteamUser).

### Achievements

`ISteamUserStats/GetPlayerAchievements` documents retrieval of a user's unlocked achievements by SteamID and AppID using a user Web API key. `GetSchemaForGame` documents game achievement/stat schema lookup, also using a key. Data can be absent or unavailable for private profiles or games without usable published schema; test those cases only with controlled mock data. The docs do not provide a universal, keyless, local desktop achievement feed.

Primary reference: [ISteamUserStats](https://partner.steamgames.com/doc/webapi/ISteamUserStats).

### Steamworks SDK distinction

Valve describes the Steamworks API as providing features to an application/game integrating Steam, including achievements, stats, overlay, and user identity. The API overview says the SDK officially supports C++ and describes initialization through the Steam client context. This is distinct from a generic account-library API. Do not claim broad library access through the SDK or ship its redistributables until the actual license, distribution rules, use case, and eligibility are reviewed.

Primary references: [Steamworks SDK](https://partner.steamgames.com/doc/sdk) and [Steamworks API Overview](https://partner.steamgames.com/doc/sdk/api).

## Authentication and secret constraints

- Steam Web API docs say API calls require a key and identify Steam OpenID as a way to authenticate a SteamID without the user entering a Steam password into a third-party site: [Steam Community developer docs](https://steamcommunity.com/dev).
- Steam Web API Terms state that the API is for retrieving data to present in the registrant's application, requires a privacy policy for nonpublic user data, says data is retrieved as requested by the end user, requires disclosure of stored data/countries, and requires keeping the API key confidential. The terms currently show a July 2010 update date and a 100,000 calls/day cap; validate current terms before reliance: [Steam Web API Terms](https://steamcommunity.com/dev/apiterms).
- Valve's Web API overview distinguishes public methods from protected methods intended for trusted backend applications and says many methods have key restrictions: [Web API Overview](https://partner.steamgames.com/doc/webapi_overview).
- Valve's authentication/ownership guide documents OpenID for linking a SteamID and describes some ownership checks as secure-server calls: [User Authentication and Ownership](https://partner.steamgames.com/doc/features/auth).

**Design consequence:** do not ship a shared private key in a desktop binary, frontend bundle, source repository, or example config. A server-side proxy would create new data-controller, privacy, hosting, security, and operating cost obligations and needs explicit product approval. A user-supplied personal API key is also not approved as a solution: its terms, onboarding burden, client exposure, and per-user registration implications need review.

## Data-by-data feasibility snapshot

| Desired data | Documented route | Current status |
|---|---|---|
| SteamID identity | Steam OpenID 2.0; Web API lookup methods also exist | Website flow documented; desktop redirect/callback remains unverified |
| Owned games | `IPlayerService/GetOwnedGames` | Documented; needs key and visible game details; key distribution blocks local-only client pending a compliant strategy |
| Game name/icon in response | `include_appinfo` option on GetOwnedGames | Documented fields; licensing/caching/asset-use terms need separate review |
| Recently played / playtime | `GetRecentlyPlayedGames`, `GetSingleGamePlaytime` | Documented; privacy/key constraints; one method notes app-associated key; evaluate exact access |
| Achievement state | `GetPlayerAchievements` | Documented read endpoint with key; availability per profile/game needs empirical mock/authorized verification |
| Achievement definitions | `GetSchemaForGame` | Documented with key; availability depends on app's published schema |
| Installed state/library location | Local files or client behavior | Not established by the cited Web API; excluded unless a separate local import review is approved |
| Unlock/reset achievements | Publisher/game functions may write stats; not required for read-only goal | Explicitly out of scope; no writes or SAM behavior |

## Rate limits, privacy, resilience

Terms list a global API call cap but not a per-method operational budget. Design conservative user-triggered refresh, bounded concurrency, caching, timeouts, and backoff only if and when access is approved. Do not evade throttling or privacy restrictions. A private profile, hidden game details, absent achievement schema, revoked access, invalid key, or missing API response must be represented as unavailable, never as zero games or zero achievements.

## Open questions before implementation

1. Can a GameVault standalone desktop application obtain a key tied to its registration, and what exact distribution model does Valve permit?
2. Is there a documented/current authorization method fit for a pure desktop app, beyond the website-oriented OpenID description?
3. Is a backend acceptable, and who operates it, publishes its privacy policy, stores what data, and handles deletion/security requests?
4. What Steam data and game imagery may be cached locally and for how long?
5. Are any local Steam files necessary for the product? If yes, define a separately opt-in, read-only scope and verify applicable terms.
6. Confirm current rates, profile privacy behavior, and terms immediately before implementation.
