# GameVault

**Your Gaming Progress. One Place.**

GameVault is planned as a Windows desktop companion for understanding and organizing a player's Steam library and progress. Version 1 is Steam-only. No application has been scaffolded and no features are implemented yet.

## Current status

The repository contains the product brief, Phase 0 research and Phase 1.5 feasibility review. The current recommendation is **CONDITIONAL GO for planning only**. Steam account connection is blocked on confirmation of API-key registration/host use, authentication and privacy model; no backend has been approved. SAM/ASF are research references only and excluded from v1. All six architecture decisions remain Proposed; achievement-writing behavior is out of scope, and the project license remains an owner decision. See [Phase 1.5 review](docs/FEASIBILITY_REVIEW.md).

## Intended technology direction

- Tauri 2 desktop shell
- React and TypeScript UI
- Rust application services
- SQLite local persistence

Versions and libraries will be selected after the architecture and security review. Windows prerequisite tools are mostly present, but Node.js needs a security update and no scaffold build has been run. See [Architecture](docs/ARCHITECTURE.md), [Development](docs/DEVELOPMENT.md), and [Proposed ADRs](docs/decisions/).

## Scope

The initial goal is a local-first Windows app for read-only Steam library and progress presentation, plus local organization. Data availability depends on Steam privacy settings and API access. Other storefronts, account automation, game modification, achievement unlock/reset, anti-cheat evasion, cloud accounts, and unsupported Steam client scraping are out of scope.

## Repository

```text
C:\Projekte\GameVault
https://github.com/GorillaKeks/GameVault.git
```

See [MASTER_PROMPT.md](MASTER_PROMPT.md) for the standing product and engineering brief.

## Notices

GameVault is an independent project and is not affiliated with, endorsed by, or sponsored by Valve or Steam. The project license has not yet been selected. Do not distribute builds until licensing, attribution, branding, Steam API access, and privacy requirements have been reviewed.

## Planning documents

- [Architecture](docs/ARCHITECTURE.md)
- [Steam integration research](docs/STEAM_INTEGRATION.md)
- [Security](docs/SECURITY.md)
- [Licensing and attribution](docs/LICENSING.md)
- [ASF / SAM audit](docs/SAM_AUDIT.md)
- [Roadmap](docs/ROADMAP.md)
- [Development setup](docs/DEVELOPMENT.md)
