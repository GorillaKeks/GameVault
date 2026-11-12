# SAM / ArchiSteamFarm (ASF) Audit

**Research date:** 2026-11-12. **Reviewed ASF branch:** `main` at `0ea1a106e4e6a705d004fc127bf66fa5593a2ff7`. This was a read-only source/documentation audit; no code was cloned into GameVault.

## Terminology correction

Steam Achievement Manager (SAM) and ArchiSteamFarm (ASF) are different projects. The prior product discussion proposed SAM-derived achievement functionality. The pasted execution request separately asks to audit ASF. Do not substitute one audit for the other or infer one project's license/security properties from the other.

This document covers ASF as requested and a bounded review of the likely upstream SAM repository. No SAM-derived code or behavior is approved.

## ASF facts observed

- Repository: [JustArchiNET/ArchiSteamFarm](https://github.com/JustArchiNET/ArchiSteamFarm), public, unarchived; default branch `main`.
- Branch reference at research time: [`0ea1a106e4e6a705d004fc127bf66fa5593a2ff7`](https://github.com/JustArchiNET/ArchiSteamFarm/commit/0ea1a106e4e6a705d004fc127bf66fa5593a2ff7).
- Repository README describes ASF as a C# application primarily for farming Steam trading cards from multiple accounts, and lists account automation features such as key/gift redemption, trading, messaging, Steam Guard support, and plugins. It says ASF is powered by SteamKit2 and can operate without the official Steam client. These functions do not match GameVault's read-only library/progress scope.
- GitHub links an Apache-2.0 license; the current `LICENSE.txt` begins with Apache License 2.0. That is a permissive code license but does not make ASF behavior appropriate, Steam interactions permitted, or every dependency/assets reusable without separate review.
- It is an active, substantial C#/.NET codebase with its own architecture and operational assumptions, not a drop-in Tauri/Rust library. No runtime, dependency, security-advisory, or full source audit was performed.

## Relevance and risk assessment

| Area | Finding |
|---|---|
| Product fit | Poor. ASF is primarily an account automation/card farming application; GameVault v1 is a Windows library/progress companion. |
| Architecture | C#/.NET application with SteamKit2; integrating it into Rust/Tauri would add a separate runtime/process or require a substantial bridge. No clear read-only library API was verified. |
| Authentication/security | README indicates advanced multi-account and Steam Guard functionality. That creates credentials/session handling and operational risk outside GameVault's requirements. No credential/security code audit was performed. |
| Steam API/protocol | Project uses SteamKit2 and operates without the official client, so it is not evidence that a method is an official/documented Steam interface. Protocol stability and terms require separate scrutiny. |
| License | ASF repository identifies Apache-2.0. Exact reuse would still need notice/NOTICE/dependency checks and policy/legal review. No ASF code will be copied. |
| Maintenance | Repository is public, unarchived, with recent main commit observed at research time. This is not a security-quality or compatibility endorsement. Recheck releases/advisories before any renewed evaluation. |
| Distribution | Bundling ASF or its automation would introduce unrelated features and obligations; no distribution scenario is approved. |

## Recommendation for ASF

**Use as research reference only. Do not integrate or use as an external runtime dependency.** Its primary function and account automation behavior do not match GameVault's stated scope. It introduces a different stack and broader account-security surface without resolving the central API key/access issue. Apache-2.0 alone is not sufficient reason to reuse it.

## SAM remains an unresolved gate

The upstream matching the prior discussion appears to be [gibbed/SteamAchievementManager](https://github.com/gibbed/SteamAchievementManager). Its `master` branch was observed at `de8b71048a0cee3c3e97cd8535e0f55ca86513e4` on the research date.

Observed from its current README and project/source files:

- README describes a portable SAM tool that requires the Steam client to be running and the user logged in. It says the original closed-source version dates to 2008, with 2011 major and 2013 hotfix releases; the current public source includes maintenance changes and a Fugue Icons attribution.
- `LICENSE.txt` is the zlib License, copyright notice for Rick (gibbed), 2024. It allows reuse subject to preserving the notice, not misrepresenting origin, and marking modified versions. If used, the source/binary distribution and product notices must be reviewed against the full current license and icon asset license.
- `SAM.Game.csproj` is a Windows `WinExe` targeting .NET Framework 4.8, x86, and references Windows Forms plus the `SAM.API` project. This is a different implementation stack from Tauri/Rust.
- The source search shows `Manager.cs` requesting user stats and the `ISteamUserStats` wrapper exposing methods such as `GetAchievement` and `SetAchievement`. This is game/client-facing Steamworks API interaction rather than a documented, generic Steam Web API feed. The source surface makes achievement mutation technically relevant but does not establish that GameVault may safely or permissibly perform it.
- This was a focused repository/source-reference review, not a full line-by-line audit of every file, bundled icon, native dependency, or current Valve policy. No binary was run and no Steam account was used.

Recommendation for SAM: **Do not reuse or integrate for v1.** Its original purpose includes managing achievements/statistics, it relies on a running Steam client and a legacy x86 .NET/Windows Forms architecture, and its API surface includes achievement mutation. It does not solve GameVault's supported account-library access/key problem. Read-only reuse would still require a precise source-level slice, complete dependency/assets and license inventory, a supported API design, and a separate product decision. The user has not approved any such feature.

Before revisiting SAM, re-check:

1. Identify the authoritative upstream repository, exact revision and current license/notice.
2. Complete line-by-line review of the desired files, transitive dependencies, icons/assets, and build provenance.
3. Map how it communicates with Steam/games and whether it writes achievement state, uses undocumented interfaces, or inspects processes/memory.
4. Review current Steam terms, anti-cheat/account risks, and distribution implications.
5. Document a written go/no-go and safer alternatives.
6. Obtain explicit product approval before any SAM-derived code or achievement-writing feature.

Until then, SAM is **not approved for reuse** and achievement unlock/reset/edit actions remain out of scope. Do not imply that ASF's Apache-2.0 license applies to SAM.

## Sources

- ASF repository and README: https://github.com/JustArchiNET/ArchiSteamFarm
- ASF current license text: https://github.com/JustArchiNET/ArchiSteamFarm/blob/main/LICENSE.txt
- ASF audited branch revision: https://github.com/JustArchiNET/ArchiSteamFarm/commit/0ea1a106e4e6a705d004fc127bf66fa5593a2ff7
- SAM repository and README: https://github.com/gibbed/SteamAchievementManager
- SAM license text: https://github.com/gibbed/SteamAchievementManager/blob/master/LICENSE.txt
- SAM audited branch revision: https://github.com/gibbed/SteamAchievementManager/commit/de8b71048a0cee3c3e97cd8535e0f55ca86513e4
- SAM project file: https://github.com/gibbed/SteamAchievementManager/blob/master/SAM.Game/SAM.Game.csproj
- SAM achievement wrapper source: https://github.com/gibbed/SteamAchievementManager/blob/master/SAM.API/Wrappers/SteamUserStats013.cs
- Steam Web API Terms: https://steamcommunity.com/dev/apiterms
