# GameVault

**Your Gaming Progress. One Place.**

GameVault is an experimental, local-first Windows desktop companion for viewing Steam games detected on this PC and, on request, reading per-game achievement data from the signed-in Steam client. It is built with Tauri 2, React, TypeScript, and Rust.

> **Prototype status:** GameVault is not ready for general release. The current app detects locally installed Steam games and can display their locally cached icons. It also has an experimental, read-only achievement reader. A live read has succeeded for one game on one user's machine; this does not establish general compatibility or Valve support. The complete Steam account library and account-wide playtime are not implemented. See [current limitations](#current-limitations) and [ADR-007](docs/decisions/ADR-007-read-only-steam-client-integration.md).

## What works today

- Detects a conventional local Steam installation and installed-game manifests across discovered Steam libraries.
- Shows each detected game's name and AppID.
- Looks for a small JPEG icon in that game's local Steam icon cache; shows the game's initial if no suitable cached icon is available.
- Lets the user explicitly request achievements for an installed game through the already-running, signed-in Steam client.
- Displays achievement names, descriptions, earned or locked state, and unlock date when supplied by Steam.
- Runs the experimental Steam achievement read in an isolated helper process with time and output limits, so a helper failure is reported without taking down the main app.
- Reports unavailable or incomplete local data instead of treating it as a complete account library.

Achievement access uses private, version-sensitive Steam client interfaces. It is labeled experimental in the app and fails closed when required interfaces or data are unavailable. The one recorded live observation was Dune: Awakening (AppID 1172710), with 74 achievements returned and 8 earned. The exact Steam client build was not captured, and no compatibility range has been established.

## Product boundaries

GameVault is designed to remain local-first and Windows-focused, with Steam as the initial and only storefront. The app has no GameVault backend, server, hosted account, or cloud sync. It does not need the user's Steam password and must not read or store passwords, cookies, session files, or authentication tokens. There is no shared Web API key or telemetry upload.

The achievement prototype is read-only. GameVault must not unlock, clear, reset, or modify achievements or stats; modify games; inject into game processes; or inspect process memory. Steam Achievement Manager (SAM) and its enhanced variants are research comparators only: GameVault does not depend on them at runtime and does not copy their code or assets. No unreviewed workaround or privacy bypass is in scope.

GameVault is an independent project and is not affiliated with, endorsed by, or sponsored by Valve or Steam.

## Current limitations

- **Owned library:** The app currently discovers local install manifests, not every game owned by the Steam account. Uninstalled games are not listed.
- **Playtime:** Account-wide playtime is not implemented.
- **Achievements:** The direct client route is experimental, undocumented by Valve for third-party desktop companions, and sensitive to Steam client versions and interface availability. Results have only been observed on one machine for one game. Missing or private data may be unavailable.
- **Icons:** Icons come only from the local Steam cache. A game may have no usable cached icon. GameVault does not download artwork.
- **Persistence and organization:** A SQLite store, favorites, history, data export, backup, and deletion controls are not implemented.
- **Packaging and release:** A distributable installer, signing, update strategy, and release readiness have not been verified. The project license and copyright holder have not yet been selected; the repository does not grant permission to redistribute the project.

The four required v1 outcomes remain: Steam library, installed-game detection, playtime, and achievements. Current gaps are not a change to that product scope. Steam privacy settings, local data availability, and unresolved interface/policy questions may limit what can ultimately be supported. If permitted data sources cannot provide the required features, the product decision must return to the owner; the app must not imply broader coverage than it has.

## Run from source

### Requirements

- Windows 10 or later, x64
- Node.js and npm
- Rust with the MSVC Windows target/toolchain
- Microsoft C++ Build Tools and the Windows SDK
- Microsoft Edge WebView2 Runtime

See the current [Tauri Windows prerequisites](https://v2.tauri.app/start/prerequisites/) for installation details. GameVault does not require Steam credentials. The achievement reader requires a running Steam client with the user already signed in.

### Start the desktop app

```powershell
cd C:\Projekte\GameVault\app
npm ci
npm run tauri -- dev
```

The first install uses the committed `package-lock.json`. To run only the frontend in a browser for UI work, use `npm run dev`; native Steam scanning and Tauri commands require the desktop app.

### Build

```powershell
cd C:\Projekte\GameVault\app
npm ci
npm run build
npm run tauri -- build
```

The Tauri build creates platform bundles according to the local Tauri configuration. A successful local build does not by itself mean that an installer, signing, or release has been reviewed.

## Development checks

Run Rust checks from the repository root:

```powershell
cargo fmt --manifest-path app/src-tauri/Cargo.toml -- --check
cargo test --manifest-path app/src-tauri/Cargo.toml --offline
cargo clippy --manifest-path app/src-tauri/Cargo.toml --all-targets --all-features --offline -- -D warnings
```

Build the frontend from `app/` with `npm run build`. The project currently has no configured frontend lint or test script. Rust tests use synthetic data and mocks; they do not connect to a Steam account or validate the real Steam DLL ABI and callbacks. Do not use a real account for automated or risky tests.

## Repository layout

```text
app/                  Tauri desktop app: React/TypeScript UI and Rust core
app/src/              User interface
app/src-tauri/src/    Local Steam scanner and isolated achievement reader
docs/                 Architecture, security, feasibility, roadmap, and decisions
MASTER_PROMPT.md      Standing product and engineering brief
```

## Key project documents

- [Architecture](docs/ARCHITECTURE.md) — system boundaries and technical direction
- [Roadmap](docs/ROADMAP.md) — milestones, remaining work, and acceptance gates
- [Development setup and checks](docs/DEVELOPMENT.md) — environment notes and verified commands
- [Security and privacy](docs/SECURITY.md) — data boundaries and security baseline
- [Steam integration research](docs/STEAM_INTEGRATION.md) — reviewed sources and unresolved access questions
- [Local Steam data inventory](docs/LOCAL_STEAM_DATA.md) — inspected local files and limits
- [v1 feature feasibility](docs/V1_FEATURE_FEASIBILITY.md) — status of the four mandatory features
- [ADR-007: read-only Steam client integration](docs/decisions/ADR-007-read-only-steam-client-integration.md) — authorization, scope, risks, and implementation gates
- [Read-only integration review](docs/SAM_READ_ONLY_INTEGRATION.md) — technical comparator and safety boundaries
- [Licensing notes](docs/LICENSING.md) and [license recommendation](docs/OPEN_SOURCE_LICENSE_RECOMMENDATION.md) — licensing remains unresolved
- [Master prompt](MASTER_PROMPT.md) — standing project brief and workflow

## Contributing and licensing

Contributions should preserve the local-only and read-only boundaries above and follow the [Master Prompt](MASTER_PROMPT.md), [security baseline](docs/SECURITY.md), and applicable ADRs. Keep Steam access explicit, bounded, and fail-closed. Do not add a backend, credential collection, telemetry, write/reset functionality, SAM/ASF runtime dependency, or an undocumented workaround without a separate owner decision and the required evidence review.

No project license has been selected or added yet. Until a license is approved and included in the repository, do not assume that the source or assets may be reused or redistributed. Third-party packages and artwork also retain their own license and usage terms.
