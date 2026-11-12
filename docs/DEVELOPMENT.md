# Development Environment and Workflow

**Inspection date:** 2026-11-12. No app scaffold exists; these are verified environment findings and setup recommendations, not a runnable app guide.

## Local environment verified

| Tool or prerequisite | Observed status |
|---|---|
| Git | 2.56.0.windows.3; local `main` tracks `origin/main` |
| Node.js | v24.22.1; Node 24 is LTS, but v24.22.3 was available as the latest patch during research |
| npm / pnpm | npm 11.19.1 / pnpm 11.25.0; no package manager has been selected for this project |
| Rust / Cargo / rustup | rustc 1.99.1, Cargo 1.99.1, rustup 1.29.1; active `stable-x86_64-pc-windows-msvc`; target `x86_64-pc-windows-msvc` installed |
| Tauri CLI | `cargo tauri --version` reports `tauri-cli 2.12.4` |
| Visual Studio Build Tools | 2022 Build Tools 17.14.37806.7 and 2026 Build Tools 18.11.12324.101 found; both have x64 MSVC compiler binaries |
| Windows SDK | Include directory `10.0.22621.0` found |
| WebView2 | Runtime directory and executable found at `157.0.4341.46` (also older `157.0.4341.42` directory) |
| .NET SDK | 9.0.319; installed for general Windows work, not required by planned Rust/Tauri stack |
| SQLite command line | `sqlite3` not on PATH; not required if an embedded SQLite library is selected |

Tauri's current [Windows prerequisites](https://v2.tauri.app/start/prerequisites/) include MSVC C++ Build Tools and WebView2, plus Rust and Node for a JS frontend. Those components were found locally. Use the Visual Studio 2022 toolchain for the first Tauri build unless current Tauri guidance confirms the newer 2026 toolchain. Node 24 is LTS and suitable as a line, but update it to at least v24.22.3 before scaffolding; alternatively validate the newer Node 26 LTS against the chosen frontend toolchain. No software was installed or system configuration changed.

These checks do **not** prove a working Tauri build: no project exists to build, and no Rust/Node dependency resolution or Windows installer build was run. The `cl.exe` binaries were found in Visual Studio installations but are not exposed through the current shell's PATH; use a Developer PowerShell or configure build discovery as documented by Tauri when a scaffold is authorized.

## Before scaffolding

1. Recheck `git status`, current branch, remote, history, configured local identity, and authenticated push account.
2. Upgrade Node 24 to the current LTS patch and choose npm or pnpm; commit exactly one lockfile.
3. Use Visual Studio 2022 Build Tools initially; verify C++ workload/Windows SDK discovery by building a minimal Tauri scaffold only after the owner approves it.
4. Select Tauri, React, and TypeScript versions from current supported releases.
5. Evaluate SQLite libraries/licenses. `sqlx` is a candidate for Rust-owned async access and embedded migrations; do not add it until MSRV, Tauri runtime compatibility, and Windows build have been checked.
6. Resolve Steam data/auth and privacy model before designing real account connection.

## Planned quality commands

Once package scripts and Rust project structure exist, configure explicit commands for:

- frontend formatting, lint, TypeScript check, unit/component tests, and production build;
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` as appropriate;
- Tauri build and Windows packaging smoke checks;
- migration, import/export, backup/restore, IPC permission, and error-path tests.

Do not run or report these commands until the project defines them. No tests were run in this research phase.

## Git workflow

- `main` is the default branch and tracks `origin/main`.
- Local author/committer identity is `GorillaKeks`; GitHub CLI authentication and connected GitHub profile both report `GorillaKeks`.
- Inspect status before and after each task; preserve unrelated changes.
- Stage only reviewed files; inspect staged names and diff, run relevant checks, then create the explicitly authorized phase commit and push.
- Never force-push or commit credentials, local databases, account exports, signing keys, `.env` files, `node_modules`, Rust build output, or distribution artifacts.
- `.gitignore` excludes common build output, environment files, SQLite databases, logs and editor settings; review it when scaffold files are added.
