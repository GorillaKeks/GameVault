# GameVault Phase 1.5 — Architecture Decision & Feasibility Review

**Review date:** 2026-11-12
**Repository revision reviewed:** `a2093b460bd24eea684c46eb382fb504f0dbdc51` (`main`)
**Scope:** research, architecture, policy, licensing and read-only local diagnostics. No application implementation, dependency installation, account use or service deployment occurred. This is not legal advice.

## Executive assessment

**CONDITIONAL GO — planning only; not ready to scaffold.** The Windows toolchain is substantially present, but no Tauri build has been verified. Valve documents Steam OpenID as a browser-based website sign-in returning a SteamID; it is not authorization to private profile data. Relevant player Web API methods require a user Web API key, and Valve's terms require confidentiality, application identification, user-requested retrieval and privacy disclosures. A distributed shared key is not suitable. The endpoint reference examples also show `partner.steam-api.com` while Valve's overview says that partner host requires a publisher key on every request; the intended host/key pairing for user-key calls must be confirmed. No integration can be considered ready until this is resolved.

**Recommendation:** retain the Windows/Tauri 2 + React/TypeScript + Rust + SQLite direction as a proposed technical baseline. For account-connected v1, investigate a GameVault-owned website callback and backend that keeps its registered user key server-side, only after written/authoritative confirmation that the exact application, user-requested data and host/key model comply with current Valve requirements. This introduces a hosted service and conflicts with the local-first/no-cloud expectation, so it requires explicit owner approval. If the owner rejects hosting or Valve does not confirm the model, defer Steam account-connected features and re-scope v1 before scaffolding. Do not substitute a shared key in the desktop binary, a user key collected from players, undocumented local Steam-file parsing, or SAM/ASF behavior.

No owner-dependent ADR is Accepted. All six ADRs in `docs/decisions/` are **Proposed**. No license file has been added, no backend authorized, and no code/features have been implemented.

## Repository and review boundaries

- `main` tracks `origin/main`; the working tree was clean before review. The remote `main` and local `HEAD` were both `a2093b460bd24eea684c46eb382fb504f0dbdc51`.
- GitHub reports public, unarchived `GorillaKeks/GameVault`, default branch `main`; the contents API lists the existing brief and planning docs. Local project is `C:\Projekte\GameVault`.
- Existing `MASTER_PROMPT.md` was read and left unchanged. It continues to prohibit implementation until approval gates are resolved. No `sources/` reference files were edited.
- SAM and ASF were reviewed from official repositories at the revisions listed in `SAM_AUDIT.md`; no code or assets were copied, and neither program was run.
- Evidence below is a dated snapshot. API documentation and Steam terms can change; re-check before implementation and release.

## Phase A — Authentication and API architecture

### Verified Steam facts

1. Steam's OpenID 2.0 flow is documented for a third-party **website**: the browser is redirected to Steam Community and returned to the site's `return` URL. The validated Claimed ID contains the SteamID. The docs do not define a native Tauri desktop callback, loopback receiver, custom URI handler, or desktop registration convention. OpenID authenticates an identity; it does not itself grant access to private profile, library or achievement data.
2. OpenID 2.0 requires verification of the assertion signature and required fields, exact `return_to` match, discovery consistency and replay-resistant nonce handling. A relying party must not trust a SteamID merely because it arrived in a callback URL. Use a maintained OpenID library, fixed Steam OP endpoint, HTTPS, one-time state bound to the initiated login, strict return URL validation, nonce/replay tracking, short-lived app sessions, and safe unlink/session revocation. Never collect Steam credentials, cookies or Steam Guard codes.
3. Valve says some Web API methods are public, others require an API key, and protected methods require publisher permissions. Standard **user keys** require a Steam account, associated domain and acceptance of API Terms. Publisher keys require an eligible Steamworks group; Valve says they are for secure publisher servers and must not ship in a client.
4. Steam Web API Terms (page says last updated July 2010) identify the application in the key signup, require key confidentiality, retrieve Steam data only as requested by an end user, disclose nonpublic data storage and storage countries in a privacy policy, present Steam Data on “as is” terms, prohibit implied Valve affiliation, and state a 100,000 calls/day ceiling. Valve may change or terminate the API. The terms page does not expressly resolve every commercial desktop distribution or key-proxy pattern. Do not present this review as legal clearance.
5. Player methods reviewed use user Web API keys: `GetOwnedGames`, `GetRecentlyPlayedGames`, `GetSingleGamePlaytime`, `GetPlayerSummaries`, `GetPlayerAchievements`, and `GetSchemaForGame`. `GetOwnedGames` is returned only where the user has access/visibility to owned-game details. Exact privacy and error behavior for all endpoints is not fully specified. `GetSingleGamePlaytime` says the key must be associated with that AppID.
6. A host mismatch needs resolution: the method references display partner-host URLs, but the overview says the public API host is `api.steampowered.com`, while `partner.steam-api.com` is for secure publisher servers and **every request** to that host requires a valid publisher key. A user-key design must use only a host/method combination Valve confirms. Do not infer that an endpoint example overrides the host overview.
7. Steamworks SDK is a game/app integration surface, not a general third-party desktop API for a player's entire account library. Do not claim Steamworks partner access or SDK access.

### Architecture alternatives

| Approach | Security | Privacy / policy / legal feasibility | Cost / scalability / maintenance | UX / reliability / distribution | Review outcome |
|---|---|---|---|---|---|
| Direct desktop calls with a GameVault shared key | Key can be extracted from source, memory or binary; cannot be kept confidential | Conflicts with Valve's confidentiality condition; app/key registration relationship still applies | Lowest service cost, but key abuse affects all users; rotation and incident handling become urgent. Scaling calls amplifies abuse/quota risk. | Smoothest UX; API/key outage affects all users. Unsuitable for distributing a binary containing the key. | **Reject** |
| Direct desktop calls with each user's own key | Key can be protected locally but app must collect/use a secret; local compromise can expose it | Domain registration and application-specific terms remain; no authoritative confirmation that a third-party GameVault may collect/store/relay end-user keys as proposed | No central hosting, but setup/support is per-user; users manage registration and revocation; fragmented maintenance as keys/settings change | Poor setup UX and more failure modes; API still depends on Valve. Desktop distribution not established today. | **Do not recommend; blocked pending Valve confirmation** |
| GameVault website + backend keeping a GameVault key | Secret stays server-side; requires hardened service, access controls, logging redaction, abuse limits, incident/rotation process | Leading candidate under current docs, but application/domain registration, host/key pairing, commercial desktop use and per-user data handling require confirmation and legal review. Privacy policy must state stored data and countries. | Recurring hosting, monitoring, support, privacy, retention/deletion and availability costs. Scale needs quotas and abuse controls; backend is an ongoing dependency. | Website OpenID can provide a documented identity flow; desktop needs short-lived session handoff. Online availability gates refresh; client can ship without the key. Changes local-first expectation. | **Conditional candidate, not approved** |
| Hybrid of official API plus local Steam client files | API key issue remains; local parsing expands filesystem/path attack surface and can break when Steam changes formats | Paths/config can expose personal data; no stable schema/permission guarantee established. `MASTER_PROMPT.md` excludes Steam-file scraping. | No hosted service, but OS-specific discovery, Steam variants and support burden raise maintenance costs; scale is per device. | May supplement install status but cannot replace cloud ownership/progress. Requires local Steam data, can be stale/unavailable and is limited to installed-client users. | **Reject for current v1; reconsider only with separate owner/policy review** |
### Recommendation and exact limits

The recommended **investigation path**, not a final approved architecture, is a website-based OpenID relying party plus a minimal GameVault service that keeps a registered key server-side and returns only requested Steam data to the authenticated user. It could conditionally support identity, visible owned games, player summaries, playtime and per-game achievement/schema reads if Valve confirms the domain/application/key/host arrangement and the account exposes the data. It cannot grant private-profile visibility, guarantee schemas or artwork rights, detect installed games, provide arbitrary local configuration, or safely ship a shared key. It adds an online service, operational cost and privacy duties. The app's session design and backend are not authorized.

An alternative product decision is a reduced offline/local-organizer v1 with no account-connected Steam data. That is viable only after the owner explicitly accepts a changed product scope; the current v1 vision includes library/progress, so this review does not silently make that change.

## Phase B — Data capability and privacy matrix

“Local” here means data available on this PC. Reading Steam client files is currently outside scope. **API/private** means data might be returned only through an approved API design and applicable profile visibility. A missing result is never evidence of zero or “not owned.”

| Category | Candidate source and auth | Privacy / local availability | Reliability and v1 feasibility |
|---|---|---|---|
| Account identity (SteamID) | Steam OpenID website flow; no API key is needed for identity itself | User initiates browser sign-in; app receives ID only after server validates assertion. No documented native callback | **Conditional** on website/backend/session handoff and approval; identity does not authorize other data |
| Profile information | `ISteamUser/GetPlayerSummaries`, user key | Returned public fields respect visibility; not all fields exist for private profiles | **Conditional**. Label omitted/private fields; do not claim private details |
| Owned games | `IPlayerService/GetOwnedGames`, user key + SteamID | Requires owned games/Game Details visibility per method docs; no supported local source in scope | **Conditional** on key/host resolution and profile setting; empty/error is not unowned |
| Names and basic game metadata | `GetOwnedGames` `include_appinfo`; public `IStoreService/GetAppList` requires “any Web API key” and pagination | Store catalog is not an ownership list; detailed store metadata and cache/display rights vary | **Partly conditional**; use only approved endpoint/fields, validate key host and asset rights |
| Installed games | No documented general Web API found in reviewed Valve sources; Steam client manifests would be local files | No local detection authorized; a machine may not have Steam or multiple library folders | **Out of scope**. Never infer installation from ownership or playtime |
| Steam library locations | Local Steam client configuration only, not a verified general Web API field | Paths can expose personal filesystem layout; reading is excluded in `MASTER_PROMPT.md` | **Out of scope**; no path access or discovery in v1 without new review |
| Playtime | `GetOwnedGames`, `GetRecentlyPlayedGames`, `GetSingleGamePlaytime`, user key | API/privacy access rules apply; single-game method documents an AppID/key association condition | **Conditional**; report source/fetch time, distinguish missing from zero |
| Achievements earned/locked | `ISteamUserStats/GetPlayerAchievements`, user key + SteamID + AppID | Method returns unlocked achievements; exact profile visibility/error behavior incompletely documented | **Conditional read-only** per game; no returned data => unavailable/unknown, not all locked |
| Achievement progress | `GetUserStatsForGame` exposes published user stats, not a universal achievement percent/progress contract | Many games publish no meaningful progress stats; mappings are game-specific | **No universal progress feature established**. Defer per-game progress calculations unless data semantics are verified |
| Last played timestamp | `GetRecentlyPlayedGames`, user key; method is documented to return recently played games, but the field-level timestamp contract was not explicit in the method description reviewed | Recent-list coverage/window, timestamp field and privacy behavior must be confirmed against the actual official response contract before relying on it | **Conditional/unresolved field**; missing timestamp stays unavailable, not “never played” |
| Game artwork | Endpoint-specific returned icon/avatar or store image URL | Steam/game art is third-party content; API terms do not grant blanket packaged redistribution or caching rights | **Unresolved**. No artwork bundled; identify each exact source and terms before display/cache |
| Game categories/collections | No reviewed general Web API method; Steam client-local category metadata is not in scope | User-created categories are local/private preference data | **Not available from the approved v1 source set**. GameVault favorites/categories may be new local organization features but require product design, not Steam import |
| Local game configuration | Local client/game files; not an API capability | Can contain paths, launch options and potentially sensitive details; no scanning authorized | **Out of scope**. No scanning, process inspection, config parsing or modification |

### Graceful degradation contract

- **Private profile / hidden Game Details:** show the account identity and only fields that the API explicitly returns; clearly state that library/progress is unavailable due to visibility or access when the service can distinguish it. Do not repeatedly retry or infer zero/unowned.
- **User offline / no network:** retain any permitted cache with an “as of” timestamp and stale marker; disable refresh; local preferences remain usable. Never fabricate a fresh response.
- **Steam API unavailable / throttled:** bounded timeout and conservative retry only where safe; preserve cache and local annotations; surface a retry-later status. Do not rotate hosts/keys to evade limits.
- **No achievement schema or no game stats:** show “achievement details unavailable for this game”; do not count every achievement as locked or claim a progress percentage.
- **Installation cannot be detected:** because install detection is out of scope, show no installation state. Do not infer it from owned games, playtime, or recent activity.
- **API schema changes/malformed responses:** validate bounded payloads; reject invalid fields without destroying existing cache; record redacted diagnostic category and fetch time; pause refresh for the affected method if parsing is unsafe.
- **Missing field, zero, private, unsupported, stale and error:** represent as distinct domain states. Persist provenance and fetch timestamp separately from game timestamps.

## Phase C — ASF and SAM

| Project | Verified review | Decision for GameVault |
|---|---|---|
| Steam Achievement Manager (`gibbed/SteamAchievementManager`) | `de8b71048a0cee3c3e97cd8535e0f55ca86513e4`; zlib license. WinForms/.NET Framework 4.8 x86. Inspected code loads Steam client DLL exports and local stats schema; exposes achievement/stat writes. README credits Fugue Icons, whose asset license was not audited. Project/dependency/assets and policy review remains incomplete. | **Research reference only; excluded from v1.** No copying, linking, dependency or behavior reuse. Read-only product goal and undocumented client coupling/write functions are incompatible. Not legal clearance. |
| ArchiSteamFarm (`JustArchiNET/ArchiSteamFarm`) | `9d34daacea8ab785ca22af4685daa76e29fdf9c9`; Apache-2.0. C# service/console with SteamKit2, plugins, multi-account/Steam Guard and broad automation; project dependency set includes web/API/auth/logging packages. Its maintained localhost IPC is an administrative API; `IPCPassword` defaults to null unless configured and the API can issue commands and modify bot configuration. Not a small Rust library or a Steam user-data API. No full transitive dependency/license/security audit; no binaries or account behavior run. | **Research reference only; no external runtime dependency; excluded from v1.** Account automation, protocol and service complexity do not fit read-only companion. The administrative API creates a high-privilege boundary. Apache-2.0 does not establish Steam policy permission or product fit. |

See [`SAM_AUDIT.md`](SAM_AUDIT.md) for exact file/revision references and limitations. A reuse decision would require exact-file provenance, complete dependency/asset inventory, policy review, security review and explicit owner approval. No definitive legal conclusion is made.

## Phase D — Project license

| Candidate | Main trade-off for distributed GameVault |
|---|---|
| MIT | Simple permissive reuse, including proprietary derivatives, while retaining copyright/permission notices; no express contributor patent grant in the standard short text. |
| Apache-2.0 | Permissive commercial reuse and explicit contributor patent license; preserve license/copyright, applicable NOTICE and mark modified files; no trademark license. |
| GPL-3.0-only | Strong reciprocal terms for covered distributed derivatives/combined works and corresponding source obligations; exact scope depends on how works are combined and warrants legal review. |
| MPL-2.0 | File-level copyleft for covered files, with larger works allowed under other terms; requires notices and source availability for covered modifications. |
| Proprietary | Owner retains control and can restrict redistribution, but users receive no open-source reuse permission; requires owner-controlled legal text, contribution rights and distribution terms. |

**Recommendation, not decision:** If the project is intended to be openly licensed and permissively reusable, Apache-2.0 is a reasonable default; MIT is simpler. If closed-source distribution is the goal, proprietary is a separate owner choice. Since user intent and contribution model are not yet confirmed, **no license is selected** and no `LICENSE` file is created. Audit the exact dependency versions, bundled SQLite implementation and assets at scaffold time. SQLite's core deliverable is public domain; wrappers, extensions, tools and optional bundled components retain their own terms. Tauri code is MIT and/or Apache-2.0 by component; React is MIT. Those facts do not decide GameVault's license.

For commercial distribution, MIT and Apache-2.0 permit downstream commercial/proprietary reuse subject to their conditions; GPL-3.0-only favors reciprocal distribution of covered derivatives; MPL-2.0 permits a larger application with other licenses while keeping covered files' obligations; proprietary terms reserve reuse rights to the owner. Permissive choices usually make community contributions and a future plugin ecosystem simpler, but a plugin API needs its own security and license boundary. A GPL or MPL choice requires clearer separation/notice/source processes. The exact Rust/SQLite wrappers and UI/native dependencies are not yet known, so compatibility cannot be signed off from framework names alone.

## Phase E — Windows environment

| Requirement | Observed | Assessment |
|---|---|---|
| OS | Windows 11 Home, 10.0.26200, 64-bit | Supported platform class for Tauri Windows development |
| Node/npm/pnpm | Node `v24.22.1`, npm `11.19.1`, pnpm `11.25.0` | Node 24 LTS line present, but Node `24.22.3` security release was published 2026-11-11. Update before scaffold; do not install in this review. Package manager undecided. |
| Rust/Cargo/rustup | `rustc 1.99.1`, Cargo `1.99.1`, rustup `1.29.1`; `stable-x86_64-pc-windows-msvc`; MSVC target installed | Present |
| Tauri CLI | `tauri-cli 2.12.4` via `cargo tauri --version` | Tauri 2 CLI present; no app build tested |
| MSVC | VS 2022 Build Tools `17.14.37806.7` and VS 18 Build Tools `18.11.12324.101`; `cl.exe` found under both installations | Compiler files present; not on current shell PATH. Use Developer PowerShell/build discovery. Verify required C++ workload/components on scaffold build. |
| Windows SDK | Include version `10.0.22621.0` | Headers found; exact SDK component/workload completeness not build-tested |
| WebView2 | Runtime files `157.0.4341.42` and `157.0.4341.46` found | Present; Tauri uses system WebView2 on Windows 11 |
| SQLite | `sqlite3` CLI not found; no app-selected SQLite crate or native dev package exists | CLI is not required for Rust embedded/bundled SQLite. Exact dependency and native build path remain unselected; verify at scaffold time. |
| Git / identity | Git `2.56.0.windows.3`; identity `GorillaKeks <286478775+GorillaKeks@users.noreply.github.com>`; GitHub CLI user `GorillaKeks` | Ready for authorized repo workflow |

**Readiness:** prerequisite executables/components are present enough to proceed to a later minimal scaffold after gates, but “ready to compile” is **not proven**. No scaffold exists, no dependencies were resolved, no build or tests were run, and current Node is behind a security release. Tauri's official Windows prerequisite list includes Microsoft C++ Build Tools and WebView2; a JS frontend additionally needs Node. Before scaffold: update Node to current supported LTS patch, select npm/pnpm, confirm VS C++ workload/SDK detection, choose SQLite crate/bundling strategy, then perform a minimal build only after implementation authorization.

**Node update procedure (not performed):** use the official [Node.js 24.22.3 release page](https://nodejs.org/en/blog/release/v24.22.3) or official [Windows downloads](https://nodejs.org/en/download/), select the x64 LTS Windows Installer, install the security patch, open a fresh terminal, then verify `node --version` and `npm --version`. Do not install a second package manager until the project chooses npm or pnpm. No Windows SDK/MSVC/WebView2 installation was identified as missing in the read-only checks; compilation still needs confirmation with the later scaffold.

## Owner decisions and blockers

1. Is a hosted GameVault website/backend acceptable despite the local-first/no-cloud product direction and its recurring operational/privacy burden?
2. Obtain Valve confirmation (or authoritative documentation) on GameVault application/domain registration, the use of its registered user key for end-user-requested player data, and the correct public-host/user-key endpoint mapping.
3. If hosting is not acceptable or the API model cannot be confirmed, approve a reduced v1 that removes account-connected library/progress features, or pause the project.
4. Choose the GameVault license and whether closed-source distribution is intended. No license may be published before approval.
5. Decide whether any local Steam file import is desired; current scope excludes it and this review recommends keeping it excluded.

## Phase F — ADR status

| ADR | Recommendation | Status |
|---|---|---|
| [ADR-001 Steam authentication](decisions/ADR-001-steam-authentication.md) | Website OpenID, only with validated callback and owner-approved backend session | Proposed |
| [ADR-002 Steam API access](decisions/ADR-002-steam-api-access.md) | No desktop shared key; resolve host/key pairing and Valve terms before any player-data call | Proposed |
| [ADR-003 Data storage](decisions/ADR-003-data-storage.md) | Local SQLite behind Rust repository; minimal, sourced cache | Proposed |
| [ADR-004 Application security](decisions/ADR-004-application-security.md) | Least-privilege Tauri; no secret in desktop; narrow IPC and privacy-safe degradation | Proposed |
| [ADR-005 Application licensing](decisions/ADR-005-application-licensing.md) | Apache-2.0 if openly/permissively licensed; otherwise owner chooses; add no license now | Proposed |
| [ADR-006 Steam integration architecture](decisions/ADR-006-steam-integration-architecture.md) | Conditional website/backend candidate; reduced no-account mode as fallback after owner decision | Proposed |

## Sources and verification date

Steam/Valve official sources reviewed 2026-11-12:

- [Web API overview and host distinctions](https://partner.steamgames.com/doc/webapi_overview)
- [Web API key authentication and key types](https://partner.steamgames.com/doc/webapi_overview/auth)
- [Web API Terms](https://steamcommunity.com/dev/apiterms)
- [Steam Community developer/OpenID page](https://steamcommunity.com/dev)
- [OpenID user authentication and ownership](https://partner.steamgames.com/doc/features/auth)
- [IPlayerService](https://partner.steamgames.com/doc/webapi/IPlayerService)
- [ISteamUser](https://partner.steamgames.com/doc/webapi/ISteamUser)
- [ISteamUserStats](https://partner.steamgames.com/doc/webapi/ISteamUserStats)
- [IStoreService](https://partner.steamgames.com/doc/webapi/IStoreService)
- [Steamworks SDK overview](https://partner.steamgames.com/doc/sdk) and [API](https://partner.steamgames.com/doc/sdk/api)
- [OpenID 2.0 verification requirements](https://openid.net/specs/openid-authentication-2_0.html)
- [Steam branding guidance](https://partner.steamgames.com/doc/marketing/branding)

Other primary sources:

- [Tauri Windows prerequisites](https://v2.tauri.app/start/prerequisites/) and [WebView2 versions](https://v2.tauri.app/reference/webview-versions/)
- [Node 24.22.3 LTS security release (2026-11-11)](https://nodejs.org/en/blog/release/v24.22.3)
- [SQLite copyright/public-domain status](https://www.sqlite.org/copyright.html)
- [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt), [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.html), [MPL 2.0](https://www.mozilla.org/en-US/MPL/2.0/), [MIT license information](https://opensource.org/license/mit/)
- [Tauri repository and license](https://github.com/tauri-apps/tauri), [React repository and license](https://github.com/facebook/react)
- ASF and SAM upstream revisions and files are listed in [`SAM_AUDIT.md`](SAM_AUDIT.md), including ASF's documented IPC API/authentication defaults.

## Final assessment

**CONDITIONAL GO — specific owner decisions and technical confirmation remain.** Suitable to continue research, product decisions and planning. Not ready for Tauri scaffolding or account-connected feature work because the Steam key/host/use model, backend acceptability, license and final v1 data scope remain unresolved. No next phase should start automatically.
