# Development Environment and Workflow

**Inspection date:** 2026-11-12. No app scaffold exists, so these are environment findings and planned setup rules, not a runnable app guide.

## Local environment observed

| Tool | Observed version/status |
|---|---|
| Windows shell | PowerShell |
| Git | 2.56.0.windows.3 |
| Node.js | v24.22.1 |
| npm | 11.19.1 |
| pnpm | 11.25.0 |
| Rust | 1.99.1 (`x86_64-pc-windows-msvc` target installed) |
| Cargo | 1.99.1 |
| rustup | 1.29.1 |
| .NET SDK | 9.0.319 |
| Tauri CLI | `cargo-tauri.exe` found on PATH; version not checked |
| SQLite CLI | Not found on PATH; app can still use an embedded SQLite library |

These version numbers were reported by installed commands, not independently validated against project compatibility. Tauri currently documents the MSVC C++ Build Tools and WebView2 as Windows development prerequisites, plus Rust MSVC and Node for a JS frontend ([Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)). Presence of the Windows SDK/build tools, WebView2 runtime, Tauri CLI version, and build success has not been checked.

## Before scaffolding

1. Recheck `git status`, `git branch --show-current`, remotes, and recent history.
2. Confirm current stable/LTS runtime support from primary project documentation. Node 24 is installed; verify it is the supported LTS choice at setup time.
3. Check MSVC C++ Build Tools, Windows SDK, WebView2, and the active Rust MSVC toolchain. Install missing items only as needed.
4. Select exactly one JS package manager and commit its lockfile. pnpm is currently installed but no selection is final.
5. Choose Rust SQLite and migration libraries after reviewing licenses, maintenance, and security.
6. Record decisions before adding dependencies.

## Planned quality commands

Once package scripts and Rust project structure exist, configure explicit commands for:

- frontend formatting, lint, TypeScript check, unit/component tests, and production build;
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` as appropriate;
- Tauri build and Windows packaging smoke checks;
- migration, import/export, backup/restore, IPC permissions, and error-path tests.

Do not run or report these commands until the project actually defines them. No tests were run during this documentation-only step.

## Git workflow

- `main` is the default branch and currently tracks `origin/main`.
- Inspect status before and after each task; preserve unrelated changes.
- Stage only files for the completed step; inspect staged names and diff, then run applicable checks.
- The user has requested completed phases to be committed and pushed separately for this preparation. For future work, follow the explicit task authorization and the standing rules in `MASTER_PROMPT.md`.
- Never force-push or commit credentials, local databases, account exports, signing keys, `.env` files, `node_modules`, Rust build output, or distribution artifacts.
- `.gitignore` currently excludes common build output, environment files, SQLite databases, logs and editor settings; review it when the scaffold is added.
