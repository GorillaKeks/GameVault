# Phase 1.7 — Decision Package

**Review date:** 2026-11-12
**Status:** Readiness review complete; owner and Valve decisions remain open.
**Scope:** Documentation and read-only environment/repository inspection only. No application scaffold or features were created; no packages were installed into the project and no Valve contact or API request was made.

## Executive status

**CONDITIONAL GO for planning. Phase 2 is not yet ready to start.** The local Windows build prerequisites are present, and a candidate Tauri 2 / React / TypeScript / Rust / SQLite stack is technically plausible. The account-data route is still not cleared by Valve documentation: player endpoint pages label their key as a user key but show the partner host, while Valve's overview says every request to the partner-only host requires a publisher key. Valve's current documentation also says a standard user key is associated with a domain and the API Terms require the key owner to keep it confidential and not share it. That does not establish that an end user's key can be entered into a third-party distributed desktop app.

The four mandatory product capabilities stay unchanged:

| Feature | Phase 1.6 classification | Current conclusion |
|---|---|---|
| Complete Steam account library | Conditional on Steam access | No change; local files do not establish all owned titles. |
| Installed-game detection | Feasible with limitations | No change; narrow read-only VDF/ACF scan design only. |
| Complete account-wide playtime | Conditional on Steam access | No change; local per-device data is incomplete. |
| Reliable cross-game achievements | Blocked by unresolved Steam restrictions | No change; no general documented third-party local API established. |

## Repository and remote snapshot

- Local branch: `main`; HEAD: `762590859e7f5b2aa7d09b85f2e9cca266783d96`.
- `origin` points to `https://github.com/GorillaKeks/GameVault.git`; a read-only remote ref check found `main` at the same commit.
- GitHub metadata reports a public, unarchived repository named `GorillaKeks/GameVault`, default branch `main`, repository size 69 KB.
- The only pre-existing working-tree item at the start of this phase was the untracked Phase 1.7 prompt at `docs/prompts/PHASE_1_7_DECISION_GATE_AND_FOUNDATION_READINESS.md`; it was preserved.
- Commit identity is configured as GorillaKeks. No GitHub or Git mutation was made.

## Steam access findings

Valve currently describes `api.steampowered.com` as the public Web API host. It describes `partner.steam-api.com` as a publisher-only host for secure publisher servers and says every request to that host must use a publisher key. However, the `IPlayerService` and `ISteamUserStats` method pages show `partner.steam-api.com` example URLs for calls whose `key` parameter is described as a user authentication key. The docs do not clearly map a standard user key to the actual supported host for this local desktop use case. Do not try a key against either host to discover behavior; Valve says requests with the wrong key to the partner host can receive 403 and strict rate limits.

Valve's user-key page says registration requires a Steam account, an associated domain and acceptance of the API Terms. The Terms identify the API application as one owned and operated by the key registrant, require user-requested retrieval and a privacy policy for nonpublic data, and say the key must remain confidential and not be shared with a third party. A locally stored key is still available to the application while in use, so OS-protected storage alone does not resolve whether that usage is permitted.

**No direct user-key flow is approved or rejected by this review.** The exact questions and a ready-to-send inquiry are below. Valve was not contacted.

## Decisions needed from the owner

| Decision | Recommendation | What is still needed |
|---|---|---|
| Steam user-key pattern | Keep it blocked until Valve answers in writing. | Owner to submit the prepared question to Valve support and preserve the reply/date. |
| Fallback if Valve declines | Do not rely on undocumented local binary achievement caches by default. | If needed, owner must explicitly decide whether to stop/re-scope the product or authorize a separate risk review of unsupported local data. The four features remain mandatory until then. |
| Scope of local-file access | Permit only a clearly listed read-only file boundary for installed-game detection and best-effort local playtime, if desired. | Resolve the conflict with `MASTER_PROMPT.md`, which excludes Steam client-file scraping. Phase 1.6 authorized research/design, not implementation. |
| Project license | Apache-2.0 remains the recommendation; MIT is the simpler permissive alternative. | Owner selects the license and confirms the copyright holder name. No root `LICENSE` is added before that. |
| Phase 2 | Do not start the scaffold yet. | Explicit owner authorization after the product/source gates and master-prompt conflict are resolved. |

The exact license comparison is in [`OPEN_SOURCE_LICENSE_RECOMMENDATION.md`](OPEN_SOURCE_LICENSE_RECOMMENDATION.md). The machine-specific foundation state and candidate selection policy are in [`FOUNDATION_READINESS.md`](FOUNDATION_READINESS.md).

## Draft inquiry for Valve (not sent)

**Subject:** Web API user key use by a local Windows desktop application

Hello Steam Web API Support,

I am evaluating a free, open-source Windows desktop application called GameVault. It runs entirely on the user's PC, has no application-operated backend or account service, and would make any Steam Web API request directly from that PC over HTTPS. It would only read the user's Steam library, playtime and achievements for display in the local app. It would not collect Steam passwords, cookies or session tokens, and would not write achievements or game state.

Could you please clarify the following before any implementation?

1. May a distributed third-party desktop app make these requests using the end user's own standard Steam Web API key, entered and stored locally by that user, rather than a key owned by GameVault?
2. Would the app's access to that key, even if it remains on the user's PC and is sent only to Valve over HTTPS, count as sharing/disclosing the key to a third party under the Steam Web API Terms? Who must own and operate the application named during registration?
3. What domain must be associated with the user's key? Can it be a static project information/privacy page, and may the user associate a domain they do not personally operate with the app?
4. Which hostname supports standard user keys for `IPlayerService/GetOwnedGames`, `GetRecentlyPlayedGames`, `GetSingleGamePlaytime`, `ISteamUserStats/GetPlayerAchievements`, and `GetSchemaForGame`? The method pages show `partner.steam-api.com` examples and user-key descriptions, while the overview says every request to that partner-only host requires a publisher key. Should a user key use `api.steampowered.com` for these methods?
5. May the app cache and display the returned data locally? What retention, refresh, attribution, deletion, user-request, privacy-policy, or method-specific rate limits must it implement?
6. Is there a documented client/local API intended for a general third-party desktop companion to read the current user's achievement schema and progress without running as a game, injecting into a game, reading process memory, or accessing credentials/session files?

The app will not ship a shared key or make API calls until these questions are resolved. Please point me to any controlling documentation or terms applicable to this exact deployment model.

Thank you,
GameVault maintainer

The Steamworks SDK API page has a “Contact Support” link, but it currently redirects to Steam Help's publishing support sign-in; it does not establish that this is the right route for a general Web API policy question. The API Terms list `webapi@valvesoftware.com` only for reporting a user's material breach, not as a general inquiry address. Therefore this message is ready in content but the correct submission channel remains unverified. Before sending, use Valve's support flow to identify an appropriate API/Steamworks contact route; do not send to the breach-report address unless Valve directs you to use it. Do not attach keys, account identifiers or personal Steam data.

## Open gate and exit

The path to Phase 2 is:

1. Owner submits the inquiry and evaluates Valve's response; update ADR-001/002 only with the response and date.
2. Owner resolves the permitted local file boundary against `MASTER_PROMPT.md` before scanner code is authorized.
3. Owner selects the project license/copyright holder before a root `LICENSE` is added.
4. Owner explicitly authorizes the minimal Tauri scaffold. The actual machine prerequisites below are already present, apart from a recommended Node LTS patch update and optional convenience tools; those updates are not made here.

If Valve does not permit a no-backend route for account data, stop and present the feasibility consequence to the owner. Do not add a backend, embed a shared key, fall back silently to undocumented binary parsing, or remove any of the four required capabilities.

## Sources checked on 2026-11-12

- [Valve: Authentication using Web API keys](https://partner.steamgames.com/doc/webapi_overview/auth)
- [Valve: Web API overview and host rules](https://partner.steamgames.com/doc/webapi_overview)
- [Valve: IPlayerService methods](https://partner.steamgames.com/doc/webapi/IPlayerService)
- [Valve: ISteamUserStats methods](https://partner.steamgames.com/doc/webapi/ISteamUserStats)
- [Valve: Steam Web API Terms of Use](https://steamcommunity.com/dev/apiterms) (page says last updated July 2010)
- [Valve: Steamworks SDK support/contact guidance](https://partner.steamgames.com/doc/sdk/api#contact)
- [Phase 1.6 feature feasibility](V1_FEATURE_FEASIBILITY.md)
- [Phase 1.6 local Steam file inventory](LOCAL_STEAM_DATA.md)
