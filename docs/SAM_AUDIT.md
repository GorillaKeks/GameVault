# SAM / ArchiSteamFarm (ASF) Audit

**Research date:** 2026-11-12. **Reviewed ASF branch:** `main` at `0ea1a106e4e6a705d004fc127bf66fa5593a2ff7`. This was a read-only source/documentation audit; no code was cloned into GameVault.

## Terminology correction

Steam Achievement Manager (SAM) and ArchiSteamFarm (ASF) are different projects. The prior product discussion proposed SAM-derived achievement functionality. The pasted execution request separately asks to audit ASF. Do not substitute one audit for the other or infer one project's license/security properties from the other.

This document covers ASF as requested and records that the SAM source, exact revision, license, dependency set, and behavior have **not** yet been audited. No SAM-derived code or behavior is approved.

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

For Steam Achievement Manager (SAM), before any implementation:

1. Identify the authoritative upstream repository, exact revision and current license/notice.
2. Read its source and dependencies; map how it communicates with Steam/games and whether it writes achievement state, uses undocumented interfaces, or inspects processes/memory.
3. Review current Steam terms, anti-cheat/account risks, and distribution implications.
4. Document a written go/no-go and safer alternatives.
5. Obtain explicit product approval before any SAM-derived code or achievement-writing feature.

Until then, SAM is **not approved for reuse** and achievement unlock/reset/edit actions remain out of scope. Do not imply that ASF's Apache-2.0 license applies to SAM.

## Sources

- ASF repository and README: https://github.com/JustArchiNET/ArchiSteamFarm
- ASF current license text: https://github.com/JustArchiNET/ArchiSteamFarm/blob/main/LICENSE.txt
- ASF audited branch revision: https://github.com/JustArchiNET/ArchiSteamFarm/commit/0ea1a106e4e6a705d004fc127bf66fa5593a2ff7
- Steam Web API Terms: https://steamcommunity.com/dev/apiterms
