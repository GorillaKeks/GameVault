# Steam Achievement Manager (SAM) and ArchiSteamFarm (ASF) Audit

**Research date:** 2026-11-12. Repository reviews were read-only. Neither application was run, no code/assets were copied, and no Steam account was used. This is a focused source/dependency and license review, not a legal opinion or complete security audit.

## Keep the projects distinct

Steam Achievement Manager (SAM) and ArchiSteamFarm (ASF) are separate products. Prior GameVault discussion considered possible SAM-derived achievement functionality; the execution brief separately requested an ASF audit. Do not conflate their behavior or licenses.

## SAM — `gibbed/SteamAchievementManager`

**Reviewed branch:** `master` at `de8b71048a0cee3c3e97cd8535e0f55ca86513e4` (still the branch head when rechecked on 2026-11-12).

### License and assets

- The repository's `LICENSE.txt` is the zlib License, copyright Rick (gibbed), 2024. It permits use, modification, and redistribution subject to retaining the notice, not misrepresenting origin, and clearly marking altered versions.
- The README credits most/all icons to the Fugue Icons set. The exact asset inventory and applicable Fugue asset license were not reviewed, so the project-level zlib license must not be assumed to cover those icons.
- The project files inspected show no explicit `PackageReference` in `SAM.API` or `SAM.Game`, but this was not a complete solution-wide inventory of generated/runtime/native assets or licenses.

### Architecture and behavior observed

- README says SAM requires the Steam client running, a logged-in Steam account, and network access.
- `SAM.Game.csproj` is a Windows Forms `WinExe`, targets .NET Framework 4.8 and x86, and references `SAM.API`; `SAM.API.csproj` also targets .NET Framework 4.8/x86 and enables unsafe blocks.
- `SAM.API/Steam.cs` reads the Steam installation path from the Windows registry, loads `steamclient.dll`, resolves `CreateInterface`, `Steam_BGetCallback`, and `Steam_FreeLastCallback`, and obtains Steam client interface versions. `SAM.API/Client.cs` creates a Steam pipe and connects to the global user.
- `SAM.Game/Manager.cs` reads `appcache/stats/UserGameStatsSchema_<AppID>.bin` for achievement/stat schema, requests user stats, queries earned states, and downloads achievement icons from a Steam CDN URL.
- The achievement wrapper exposes `SetAchievement` and `ClearAchievement`; the UI can edit stats and calls `StoreStats()` to commit them. This is account/game state mutation, outside GameVault's approved read-only scope.
- The observed route depends on Steam client DLL exports/interface versions and local cache format. Valve's public Web API documentation does not document this as a general-purpose third-party desktop library API. The source was not exhaustively searched for process/memory inspection, so none is asserted either way.

### Recommendation for SAM

**Do not reuse or integrate for v1; research reference only.** The existing implementation is x86 Windows Forms/.NET Framework, directly loads Steam client internals, reads local Steam cache data, and supports achievement/stat writes. This does not solve GameVault's documented Web API key/authentication blocker and conflicts with the read-only product direction. The zlib license is permissive subject to its notice/modified-source conditions, but that alone does not clear the native interface, asset, Steam policy, security, or supportability risks.

If reconsidered, first inventory every proposed source file, transitive dependency, generated/native asset, and icon license; review current Valve policy; and obtain owner approval for the exact read/write feature. Do not copy code or assets in the meantime.

## ASF — `JustArchiNET/ArchiSteamFarm`

**Reviewed branch:** `main` at `9d34daacea8ab785ca22af4685daa76e29fdf9c9` (rechecked 2026-11-12).

### Purpose and architecture

- ASF's README describes a C# application primarily for farming Steam trading cards across multiple accounts. It also advertises Steam authentication/Steam Guard support, account actions, plugins, and operation without the official Steam client; it cites SteamKit2 as a central dependency.
- Current project files show a console executable with a web API/UI integration and a plugin architecture. The main project has dependencies including SteamKit2, ASP.NET OpenAPI, JSON Web Tokens, NLog, Scalar, cryptography and composition packages. `Directory.Packages.props` at review time pins SteamKit2 `4.0.0-beta.2`; this version is only a source snapshot, not a dependency recommendation.
- ASF documents an officially maintained HTTP/JSON IPC API for a running ASF process, including status reads and powerful operations such as sending commands and editing/adding/deleting bot configuration. It binds to localhost by default; the documented default `IPCPassword` is null, meaning API calls need no password unless the operator configures one. Remote binding requires explicit network/security configuration. This API is for administering ASF itself; it is not a supported substitute for Steam's user Web API or a read-only GameVault data source.
- Consequently, embedding/launching ASF or connecting to its IPC would bring a second long-running service, account/bot configuration, a powerful local control API, and a separate auth/patch boundary into the desktop app. Restricting it to localhost alone would not make the administrative API fit GameVault's least-privilege product boundary.
- ASF is a large C#/.NET operational application with multi-account, service and plugin concerns. It is not a small read-only library suitable for linking into GameVault's Rust/Tauri process.
- Its SteamKit2 protocol/library usage is not evidence that the corresponding operations are officially supported for a third-party Steam companion. No SteamKit2 or ASF behavior was executed or tested.

### License and maintenance snapshot

- GitHub links `LICENSE.txt`, which contains Apache License 2.0. Reuse would require following the full license, preserving copyright/license and any applicable NOTICE, marking modifications where required, and separately reviewing dependency/assets licenses. Apache-2.0 does not grant rights to ASF trademarks or make Steam account automation acceptable for GameVault.
- Repository was public and unarchived, and the branch head was active at the commit above. This is not a security, quality, or compatibility endorsement.

### Recommendation for ASF

**Use as research reference only; do not integrate or use as an external runtime dependency.** ASF's primary account automation and card-farming purpose, credentials/security surface, SteamKit2 dependence, service architecture, and extra runtime/dependency burden do not fit the Steam-only, read-only desktop companion. Apache-2.0 by itself is not a reason to reuse it.

## Overall gate and remaining audit limits

- No SAM-derived code or behavior is approved. Achievement unlock, clear, reset, stat editing, account automation, and anti-cheat-sensitive actions remain out of scope.
- No code or assets from ASF or SAM are present in GameVault.
- Remaining before any reuse: full dependency/asset inventory for the exact files proposed, a fuller source audit, current Steam policy review, license compatibility review, and explicit owner approval.

## Phase 1.5 completion decision (bounded review)

The specific question for this phase is whether SAM or ASF should become a GameVault dependency or supply v1 behavior. The recommendation for **both** is **research reference only; excluded from v1**. Neither is a dependency, and no files/assets were copied. SAM's direct Steam client DLL/interface coupling, local schema dependency and achievement/stat mutation conflict with GameVault's read-only documented-API boundary. ASF's multi-account automation, SteamKit2 protocol surface, credentials/Steam Guard, service/plugin architecture and broad dependency graph do not fit a read-only Rust/Tauri companion.

This does not certify either project as legally or technically safe to reuse. The review was a bounded reading of the listed files/repository metadata, not an exhaustive transitive dependency, binary, security or policy audit. SAM's icon credits need a separate asset license review. ASF's full dependency and bundled asset notices need review if an exact component is ever proposed. No definitive claim is made about Valve's policy treatment of all SAM/ASF use cases.

Reviewed upstream heads rechecked 2026-11-12: SAM `de8b71048a0cee3c3e97cd8535e0f55ca86513e4`; ASF `9d34daacea8ab785ca22af4685daa76e29fdf9c9`. Any future review must pin exact commit/file provenance rather than rely on moving branch names. No implementation is authorized by this audit.

## Sources

- SAM repository: https://github.com/gibbed/SteamAchievementManager
- SAM license: https://github.com/gibbed/SteamAchievementManager/blob/master/LICENSE.txt
- SAM reviewed revision: https://github.com/gibbed/SteamAchievementManager/commit/de8b71048a0cee3c3e97cd8535e0f55ca86513e4
- SAM client integration: https://github.com/gibbed/SteamAchievementManager/blob/master/SAM.API/Steam.cs
- SAM Steam client wrapper: https://github.com/gibbed/SteamAchievementManager/blob/master/SAM.API/Client.cs
- SAM stats UI: https://github.com/gibbed/SteamAchievementManager/blob/master/SAM.Game/Manager.cs
- SAM achievement API wrapper: https://github.com/gibbed/SteamAchievementManager/blob/master/SAM.API/Wrappers/SteamUserStats013.cs
- SAM WinForms project: https://github.com/gibbed/SteamAchievementManager/blob/master/SAM.Game/SAM.Game.csproj
- ASF repository and README: https://github.com/JustArchiNET/ArchiSteamFarm
- ASF license: https://github.com/JustArchiNET/ArchiSteamFarm/blob/main/LICENSE.txt
- ASF reviewed revision: https://github.com/JustArchiNET/ArchiSteamFarm/commit/9d34daacea8ab785ca22af4685daa76e29fdf9c9
- ASF project dependencies: https://github.com/JustArchiNET/ArchiSteamFarm/blob/main/ArchiSteamFarm/ArchiSteamFarm.csproj and https://github.com/JustArchiNET/ArchiSteamFarm/blob/main/Directory.Packages.props
- ASF IPC API and authentication defaults: https://github.com/JustArchiNET/ArchiSteamFarm/wiki/IPC
- Steam Web API Terms: https://steamcommunity.com/dev/apiterms
