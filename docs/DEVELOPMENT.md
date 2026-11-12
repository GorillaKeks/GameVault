# Development Environment and Workflow

**Inspection date:** 2026-11-12. An initial app scaffold exists in `../app`; dependencies are not installed and build/linker integration is not yet verified.

Phase 1.7 recheck on 2026-11-12 found Windows 11 Home 25H2 `10.0.26200.9711` (64-bit). See [`FOUNDATION_READINESS.md`](FOUNDATION_READINESS.md) for the current toolchain assessment; [`FEASIBILITY_REVIEW.md`](FEASIBILITY_REVIEW.md) retains the earlier baseline.

## Local environment verified

| Tool or prerequisite | Observed status |
|---|---|
| Git | 2.56.0.windows.3; local `main` tracks `origin/main` |
| Node.js | System v24.22.1; portable user-local v26.13.1 downloaded and checksum-verified for scaffold work |
| npm / pnpm | System npm 11.19.1 / pnpm 11.25.0; project selects npm; portable Node includes npm 11.20.0 |
| Rust / Cargo / rustup | rustc 1.99.1, Cargo 1.99.1, rustup 1.29.1; active `stable-x86_64-pc-windows-msvc`; target `x86_64-pc-windows-msvc` installed |
| Tauri CLI | `cargo tauri --version` reports `tauri-cli 2.12.4`; standalone `tauri` is not on PATH |
| Visual Studio Build Tools | VS 2022 Build Tools 17.14.37806.7 and VS 18 Build Tools 18.11.12324.101 found; `vswhere` confirms the C++ x86/x64 component in VS 18; MSVC `14.51.36231` compiler found, not on current shell PATH |
| Windows SDK | Include directory `10.0.22621.0` found |
| WebView2 | Evergreen Runtime registered at `157.0.4341.46` |
| .NET SDK | 9.0.319; installed for general Windows work, not required by planned Rust/Tauri stack |
| SQLite command line | `sqlite3` not on PATH; not required if an embedded SQLite library is selected; no Rust crate/native dependency is selected |

Tauri's current [Windows prerequisites](https://v2.tauri.app/start/prerequisites/) include MSVC C++ Build Tools and WebView2, plus Rust and Node for a JS frontend. Those components were found locally. The C++ component is confirmed in Visual Studio Build Tools 2026, but a real Tauri build has not verified linker/runtime integration. Node 24 is LTS and suitable as a line, but update it to v24.22.3 before scaffolding; alternatively validate the newer Node 26 LTS against the chosen frontend toolchain. No software was installed or system configuration changed. See the [Node 24.22.3 security release](https://nodejs.org/en/blog/release/v24.22.3).

These checks do **not** prove a working Tauri build: no Rust/Node dependency resolution or Windows installer build was run. Compiler binaries and the Visual Studio C++ component were found; linker/runtime integration is unverified and `cl.exe` is not exposed through the current shell's PATH. `cargo tauri --version` works and reports `tauri-cli 2.12.4`. A global Node update requested Administrator consent and was not completed; project work uses a separate portable Node toolchain.

## Before the first build

1. Recheck `git status`, current branch, remote, history, configured local identity, and authenticated push account.
2. Decide whether to retain portable Node 26 or retry a supported system LTS update; project package manager is npm; generate exactly one lockfile.
3. Verify C++ workload/Windows SDK discovery by building the scaffold.
4. Review scaffold-selected Tauri, React, Vite, and TypeScript versions before dependency installation.
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
