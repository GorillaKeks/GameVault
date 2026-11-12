# GameVault — Foundation Readiness (Phase 1.7)

**Checked:** 2026-11-12
**Purpose:** Record the accessible Windows development prerequisites and a non-binding candidate stack. This is an audit, not authorization to install tools, scaffold or build the application.

## Repository state

- Local directory: `C:\Projekte\GameVault`.
- Branch `main`, local HEAD `762590859e7f5b2aa7d09b85f2e9cca266783d96`.
- `origin/main` resolved to the same commit during a read-only ref query.
- The pre-existing untracked Phase 1.7 prompt was left alone. No other tracked/untracked user work was changed.
- Git executable: `2.56.0.windows.1`; Git identity is configured as `GorillaKeks <286478775+GorillaKeks@users.noreply.github.com>`.
- GitHub metadata: public, unarchived, default branch `main`, size 69 KB.

## Machine prerequisites observed

| Component | Observed state | Assessment |
|---|---|---|
| Windows | Windows 11 Home, 25H2, build `10.0.26200.9711`, 64-bit (WMI and registry). | Current OS edition/build recorded. |
| Node.js | `v24.22.1` | Usable by current Vite engines, but behind the Node `24.22.3` LTS security release published 2026-11-11. Recommend updating to `24.22.3` before scaffold setup. Node 24 is Maintenance LTS through 2028-04-30; Node 26 is Active LTS, but no need to migrate major lines just for this project. |
| npm | `11.19.1` | Available with Node. Recommend npm and a committed `package-lock.json` for the simplest reproducible Windows setup. |
| pnpm | `11.25.0` | Available, but supplied from a Windows Apps-managed path. No project convention exists; avoid adding a second package-manager dependency. |
| Yarn | Not found on PATH | Not needed if npm is selected. |
| Rust | `rustc 1.99.1`, stable `x86_64-pc-windows-msvc` | Installed target is appropriate for Windows desktop. `rustup check` reports the compiler current; rustup itself has a minor update available (`1.29.1` to `1.29.3`) but no project requirement depends on updating it now. |
| Tauri CLI | `cargo tauri --version` reports `tauri-cli 2.12.4`; standalone `tauri` command is not on PATH. | Present as a Cargo-installed subcommand. For the eventual project, prefer a project-local npm CLI dev dependency for reproducibility; do not add another global install. |
| Tauri Rust crate | `tauri 2.12.5` is current stable from crates.io; `3.0.0-alpha.6` also exists and must not be selected for a Tauri 2 project. | Candidate stable line only; pin/lock after authorized scaffold and compatibility review. Tauri 2.12.5 declares Rust MSRV 1.90. |
| Visual C++ Build Tools | Visual Studio Build Tools 2026 at `18.11.2`; `vswhere` found the VC x86/x64 tools component. `cl.exe` version path indicates MSVC `14.51.36231`. A 2022 Build Tools installation is also present, but the queried C++ component was found for VS 2026. | Required compiler family is present. A real Tauri build was not run, so linker/runtime integration is not yet validated. |
| Windows SDK | Windows Kits 10 SDK `10.0.22621.0` registered and its Include folder exists. | Present. |
| WebView2 | Evergreen Runtime `157.0.4341.46` registered. | Present. Tauri uses the system WebView2 runtime on Windows. |
| SQLite CLI | `sqlite3` not found on PATH. | Not a blocker for an embedded Rust SQLite library. Do not install CLI just for the app. |

No environment variables, registry values or build prerequisites were modified. `tauri`, `cl` and `sqlite3` were not globally available as shell commands. The compiler was detected by Visual Studio installation metadata and its installed file path; it was not invoked.

## Non-binding technology proposal

The versions below are registry/release observations made on 2026-11-12, not approved pins. Select compatible patches and commit lockfiles only when the owner authorizes scaffolding.

| Layer | Candidate | Reason / review notes |
|---|---|---|
| Runtime | Node.js `24.22.3` latest Node 24 patch; keep to supported LTS | Installed `24.22.1` is behind the security release published 2026-11-11. Vite `8.3.8` declares `^20.19.0 || >=22.12.0`, so Node 24 satisfies its engine range. Node 26 Active LTS is an alternative after compatibility review. |
| Package manager | npm `11.x` | Already installed with Node; simple default and lockfile support. Use only npm; do not mix npm/pnpm lockfiles. |
| Tauri | Tauri 2 stable `2.12.x`; crates.io `tauri 2.12.5`, npm CLI/API `2.12.4` observed | Keep Rust and JS packages on Tauri 2, reject the 3.0 alpha. Verify coordinated release compatibility before pinning. License shown for Rust crate: `Apache-2.0 OR MIT`; inspect transitive/plugin licenses later. |
| Frontend | React `19.3.1`, TypeScript `7.0.2` observed | React license metadata: MIT; TypeScript metadata: Apache-2.0. Confirm current stable release and ecosystem support at scaffold time. Keep `strict` TypeScript enabled. |
| Frontend tooling | Vite `8.3.8`, `@vitejs/plugin-react 6.3.0` observed | Tauri recommends Vite for SPA frameworks such as React. Vite license MIT; its engine range includes Node 24. Review plugin compatibility before pinning. |
| Rust toolchain | stable MSVC `1.99.1` currently installed; Tauri 2.12.5 MSRV 1.90 | Choose the supported stable toolchain at scaffold time and record it in `rust-toolchain.toml` if a pinned development toolchain is desired. CI must use the same host target. |
| SQLite access | Rust-owned repository; `rusqlite 0.42.0` is a candidate (MIT, MSRV 1.89) | Avoid frontend SQL and broad Tauri SQL plugin access. Compare `bundled` vs system SQLite build and exact feature/transitive license tree before selection. Embed ordered SQL migrations and apply transactionally; test migration/backup behavior once code exists. SQLite core itself is public domain, but wrappers/build inputs still need inventory. |
| Persistence boundary | Narrow Rust data layer and typed domain commands | No raw SQL, arbitrary paths or broad filesystem authority in the WebView. User data in per-user application data, subject to approved retention policy. |

### Proposed quality commands for a future scaffold

These are conventions to configure, not commands already present or run:

- Frontend: `npm run format:check` (Prettier), `npm run lint` (ESLint), `npm run typecheck` (`tsc --noEmit`), and `npm run test` (Vitest for meaningful UI/domain behavior).
- Rust: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`.
- Desktop: authorized Windows `tauri dev` smoke check and `tauri build` only after Phase 2 authorization.
- No test or build commands were run in Phase 1.7 because no application scaffold exists and this is a readiness audit.

### Tauri capability and IPC baseline

- Start with the smallest Tauri 2 capability for the one main window and core event/window behavior actually needed.
- Do not enable broad frontend filesystem, shell, process, or HTTP permissions by default. The local Steam scanner, if separately approved, should be a Rust service constrained to configured Steam library roots and known filenames.
- Expose narrow typed commands. Never expose arbitrary filesystem paths, SQL, URL fetching or command execution from the WebView.
- Add individual plugins and permissions only after documenting the feature need and permission scope. Review generated capability configuration rather than trusting scaffold defaults.
- No direct Steam HTTP, credential storage, scanner implementation or database schema is approved in this phase.

## Prerequisite status for Phase 2

**Update 2026-11-12:** Phase 2 scaffold has since been created under `app/`. Node 26.13.1 is available as a portable user-local toolchain with npm 11.20.0; the system Node was not changed. No project dependencies were installed and no build or tests were run. See [`DEVELOPMENT.md`](DEVELOPMENT.md) for current status.

| Prerequisite | State |
|---|---|
| Node patch update | Recommended, not performed (`24.22.1` observed). |
| C++ Build Tools / MSVC and Windows SDK | Detected; actual Tauri link/build not validated. |
| WebView2 Runtime | Detected. |
| Tauri CLI | Cargo-installed `tauri-cli 2.12.4` is present; recommend project-local npm CLI after authorization. |
| Package manager | npm recommended, not formally owner-approved. |
| Tauri/Rust/React/TypeScript versions | Candidate only, not pinned/approved. |
| SQLite crate and exact feature set | Open; `rusqlite` candidate requires feature/license/build review. |
| Steam API access and file boundary | Unresolved owner/Valve gates. |
| Project license and copyright holder | Unresolved owner decision. |
| Scaffold | Created in `app/`; no dependency installation/build verification yet. |

## References checked 2026-11-12

- [Tauri v2 Windows prerequisites](https://v2.tauri.app/start/prerequisites/)
- [Tauri project creation/templates](https://v2.tauri.app/start/create-project/)
- [Tauri frontend configuration (Vite for SPA frameworks)](https://v2.tauri.app/start/frontend/)
- [Tauri WebView2 versions/distribution](https://v2.tauri.app/reference/webview-versions/)
- [Node.js release schedule](https://github.com/nodejs/Release)
- [Node.js supported releases](https://nodejs.org/en/about/previous-releases)
- [Node.js 24.22.3 LTS security release (2026-11-11)](https://nodejs.org/en/blog/release/v24.22.3)
- [Tauri Rust crate and MSRV](https://crates.io/crates/tauri)
- [Tauri CLI package](https://www.npmjs.com/package/@tauri-apps/cli)
- [React package metadata](https://www.npmjs.com/package/react)
- [TypeScript package metadata](https://www.npmjs.com/package/typescript)
- [Vite package metadata and engine range](https://www.npmjs.com/package/vite)
- [rusqlite crate, MSRV and license](https://crates.io/crates/rusqlite)
- [rusqlite upstream feature/library licensing notes](https://github.com/rusqlite/rusqlite)
- [SQLite copyright and public-domain statement](https://www.sqlite.org/copyright.html)
- [Tauri capabilities](https://v2.tauri.app/security/capabilities/)
