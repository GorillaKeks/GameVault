# GameVault

**Your Gaming Progress. One Place.**

GameVault is a Windows desktop companion for understanding and organizing a player's Steam library and progress. Version 1 is Steam-only. A Tauri 2 + React/TypeScript + Rust scaffold now lives in `app/`; its first read-only prototype detects local Steam installs and installed-game manifests.

## Current status

The repository contains the product brief, Phase 1.5–1.7 research and an initial local detection prototype. The owner requires a local-only app with four mandatory v1 features: Steam library, installed-game detection, playtime and achievements. Current status remains **CONDITIONAL GO**: the prototype only sees local installed manifests; it does not yet connect to Steam or read achievements. The private Steam client/cache route is unsupported and does not provide a complete owned library or account-wide playtime. Valve Web API key use remains unresolved. A GameVault backend and all achievement/stat writes are prohibited. See [Phase 1.6 feature feasibility](docs/V1_FEATURE_FEASIBILITY.md), the [Phase 1.7 decision package](docs/PHASE_1_7_DECISION_PACKAGE.md), and the [SAM read-only integration review](docs/SAM_READ_ONLY_INTEGRATION.md).

## Intended technology direction

- Tauri 2 desktop shell
- React and TypeScript UI
- Rust application services
- SQLite local persistence

The initial scaffold uses the candidate stack recorded in [Foundation readiness](docs/FOUNDATION_READINESS.md). Dependencies have not been installed and no scaffold build has been run. See [Architecture](docs/ARCHITECTURE.md), [Development](docs/DEVELOPMENT.md), and [ADRs](docs/decisions/).

## Scope

The v1 goal is a local-first Windows app for the four mandatory Steam features above. Data availability depends on Steam privacy settings and the limitations of each data source. Other storefronts, account automation, game modification, achievement unlock/reset, anti-cheat evasion and all GameVault cloud services are out of scope. The owner has authorized pursuit of the narrow read-only Steam client/schema route in [ADR-007](docs/decisions/ADR-007-read-only-steam-client-integration.md); this is not a claim of Valve support or complete library coverage.

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
- [Phase 1.7 decision package](docs/PHASE_1_7_DECISION_PACKAGE.md)
- [Foundation readiness](docs/FOUNDATION_READINESS.md)
- [Phase 1.7 execution prompt](docs/prompts/PHASE_1_7_DECISION_GATE_AND_FOUNDATION_READINESS.md)
- [SAM read-only integration review](docs/SAM_READ_ONLY_INTEGRATION.md)
- [Security](docs/SECURITY.md)
- [Licensing and attribution](docs/LICENSING.md)
- [Open-source license recommendation](docs/OPEN_SOURCE_LICENSE_RECOMMENDATION.md)
- [ASF / SAM audit](docs/SAM_AUDIT.md)
- [Roadmap](docs/ROADMAP.md)
- [Development setup](docs/DEVELOPMENT.md)
