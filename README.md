# GameVault

**Your Gaming Progress. One Place.**

GameVault is planned as a Windows desktop companion for understanding and organizing a player's Steam library and progress. Version 1 is Steam-only. No application has been scaffolded and no features are implemented yet.

## Current status

The repository contains the product brief and Phase 1.5/1.6 research. The owner requires a local-only app with four mandatory v1 features: Steam library, installed-game detection, playtime and achievements. Phase 1.6 is **CONDITIONAL GO for planning only**: installed detection is feasible with limitations, while complete account library/playtime and reliable achievements need Valve to confirm a direct per-user key path. A GameVault backend is prohibited. See [Phase 1.6 feature feasibility](docs/V1_FEATURE_FEASIBILITY.md).

## Intended technology direction

- Tauri 2 desktop shell
- React and TypeScript UI
- Rust application services
- SQLite local persistence

Versions and libraries will be selected after the architecture and security review. Windows prerequisite tools are mostly present, but Node.js needs a security update and no scaffold build has been run. See [Architecture](docs/ARCHITECTURE.md), [Development](docs/DEVELOPMENT.md), and [Proposed ADRs](docs/decisions/).

## Scope

The v1 goal is a local-first Windows app for the four mandatory Steam features above. Data availability depends on Steam privacy settings and Valve-approved access. Other storefronts, account automation, game modification, achievement unlock/reset, anti-cheat evasion and all GameVault cloud services are out of scope. Phase 1.6 research covers narrow, read-only local Steam metadata; reverse-engineered achievement-cache parsing is not approved.

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
- [Local Steam data inventory](docs/LOCAL_STEAM_DATA.md)
- [Local-first architecture](docs/LOCAL_FIRST_ARCHITECTURE.md)
- [v1 feature feasibility](docs/V1_FEATURE_FEASIBILITY.md)
- [Security](docs/SECURITY.md)
- [Licensing and attribution](docs/LICENSING.md)
- [Open-source license recommendation](docs/OPEN_SOURCE_LICENSE_RECOMMENDATION.md)
- [ASF / SAM audit](docs/SAM_AUDIT.md)
- [Roadmap](docs/ROADMAP.md)
- [Development setup](docs/DEVELOPMENT.md)
