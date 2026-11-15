# Development Environment and Workflow

**Environment inspection:** 2026-11-12. **Achievement prototype update:** 2026-11-15. The scaffold exists in `../app`; Rust and npm dependencies are locked, 17 native unit tests pass, the frontend production build passes, and an unbundled Tauri debug executable builds. Installer packaging and live Steam/runtime smoke testing remain open.

Phase 1.7 recheck on 2026-11-12 found Windows 11 Home 25H2 `10.0.26200.9711` (64-bit). See [`FOUNDATION_READINESS.md`](FOUNDATION_READINESS.md) for the current toolchain assessment; [`FEASIBILITY_REVIEW.md`](FEASIBILITY_REVIEW.md) retains the earlier baseline.

## Local environment verified

| Tool or prerequisite | Observed status |
|---|---|
| Git | 2.56.0.windows.3; local `main` tracks `origin/main` |
| Node.js | System v24.22.1; portable user-local v26.13.1 downloaded and checksum-verified for scaffold work |
| npm / pnpm | Portable Node.js `26.13.1` / npm `11.20.0` are available in the user-local GameVault tools folder and were invoked directly for this build. System npm/pnpm are not on the current shell PATH. |
| Rust / Cargo / rustup | rustc 1.99.1, Cargo 1.99.1, rustup 1.29.1; active `stable-x86_64-pc-windows-msvc`; target `x86_64-pc-windows-msvc` installed |
| Tauri CLI | `cargo tauri --version` reports `tauri-cli 2.12.4`; standalone `tauri` is not on PATH |
| Visual Studio Build Tools | VS 2022 Build Tools 17.14.37806.7 and VS 18 Build Tools 18.11.12324.101 found; `vswhere` confirms the C++ x86/x64 component in VS 18; MSVC `14.51.36231` compiler found, not on current shell PATH |
| Windows SDK | Include directory `10.0.22621.0` found |
| WebView2 | Evergreen Runtime registered at `157.0.4341.46` |
| .NET SDK | 9.0.319; installed for general Windows work, not required by planned Rust/Tauri stack |
| SQLite command line | `sqlite3` not on PATH; not required if an embedded SQLite library is selected; no Rust crate/native dependency is selected |

Tauri's current [Windows prerequisites](https://v2.tauri.app/start/prerequisites/) include MSVC C++ Build Tools and WebView2, plus Rust and Node for a JS frontend. Those components were found locally. The C++ component is confirmed in Visual Studio Build Tools 2026, but a real Tauri build has not verified linker/runtime integration. Node 24 is LTS and suitable as a line, but update it to v24.22.3 before scaffolding; alternatively validate the newer Node 26 LTS against the chosen frontend toolchain. No software was installed or system configuration changed. See the [Node 24.22.3 security release](https://nodejs.org/en/blog/release/v24.22.3).

The Rust native library, frontend production bundle, and unbundled Tauri release executable have built. No installer bundle or desktop runtime smoke test has been run. Compiler binaries and the Visual Studio C++ component were found; `cl.exe` is not exposed through the current shell's PATH. `cargo tauri --version` reports `tauri-cli 2.12.4`. A global Node update requested Administrator consent and was not completed; project work uses a separate portable Node toolchain.

## Current native scanner checks (2026-11-15)

- `cargo fmt --manifest-path app/src-tauri/Cargo.toml -- --check` — passed.
- `cargo test --manifest-path app/src-tauri/Cargo.toml --offline` — passed: 17 unit tests, including synthetic scanner/icon cases and mock achievement-state mapping. No real Steam account or live Steam interface was used.
- `cargo clippy --manifest-path app/src-tauri/Cargo.toml --all-targets --all-features --offline -- -D warnings` — passed.
- TypeScript compiler and Vite production build — passed using the installed project binaries. `npm run build` could not be invoked in the current shell because npm is not on PATH.
- `cargo tauri build --no-bundle --debug --config '{"build":{"beforeBuildCommand":""}}'` — passed; the production frontend was built separately first. Installer bundle and running the UI were not tested.
- `read_game_achievements` has only synthetic/mock mapping tests. No live Steam client, ABI, callback, or account test was run.

## Before packaging and runtime smoke testing

1. Recheck `git status`, current branch, remote and history before the next implementation task.
2. Use the existing local lockfiles and portable Node/npm toolchain; avoid generating a second npm lockfile.
3. Verify C++ Build Tools and WebView2 at runtime through a Tauri smoke check; create and inspect installer bundles separately.
4. Review scaffold-selected Tauri, React, Vite, and TypeScript versions and license notices before release.
5. Evaluate SQLite libraries/licenses before adding persistence; check MSRV, Tauri runtime compatibility, and Windows build behavior.
6. Resolve Steam data/auth and privacy model before designing real account connection.

## Planned quality commands

The native Rust target now has unit tests. The frontend currently has `dev`, `build`, `preview`, and `tauri` scripts but no format, lint, typecheck or test scripts. Configure explicit frontend checks and continue native checks for:

- frontend formatting, lint, TypeScript check, unit/component tests, and production build;
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` as appropriate;
- Tauri build and Windows packaging smoke checks;
- migration, import/export, backup/restore, IPC permission, and error-path tests.

Do not report a frontend test or lint result until those scripts are configured and run. Phase 1.7 itself did not run tests; the 2026-11-15 checks above are later updates.

## Git workflow

- `main` is the default branch and tracks `origin/main`.
- Local author/committer identity is `GorillaKeks`; GitHub CLI authentication and connected GitHub profile both report `GorillaKeks`.
- Inspect status before and after each task; preserve unrelated changes.
- Stage only reviewed files; inspect staged names and diff, run relevant checks, then create the explicitly authorized phase commit and push.
- Never force-push or commit credentials, local databases, account exports, signing keys, `.env` files, `node_modules`, Rust build output, or distribution artifacts.
- `.gitignore` excludes common build output, environment files, SQLite databases, logs and editor settings; review it when scaffold files are added.
