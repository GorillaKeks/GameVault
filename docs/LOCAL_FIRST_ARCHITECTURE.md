# Local-First Windows Architecture

**Status:** proposed design constrained by owner-approved product decisions. **Date:** 2026-11-12. No application code exists.

## Confirmed boundaries

- Windows desktop, Steam only, local-first.
- No GameVault service, server, account, central database, subscription or always-running external process.
- UI: React + TypeScript; desktop/native boundary: Tauri 2; native services: Rust; local database: SQLite.
- Internet may be used only for permitted Steam requests, metadata/artwork or updates. Network availability is optional for locally available information.
- No Steam passwords, session cookies, shared API key, hidden telemetry, automatic user-data upload, elevation, SAM or ASF integration, or achievement writes.
- All four v1 product requirements remain mandatory: library, installed games, playtime and achievements. A feasibility blocker must be returned to the owner, not silently removed.

## Proposed component flow

```text
React + TypeScript
  Dashboard / library / installed / details / progress / settings
       │ typed, narrow Tauri commands; no direct filesystem or SQL access
       ▼
Rust application layer
  ├─ Steam root discovery (registry + optional user-selected roots)
  ├─ read-only VDF/ACF scanner (bounded file set; no recursion through game data)
  ├─ Steam Web API adapter (only if personal-key use and endpoint are confirmed)
  ├─ metadata/artwork client (optional; source and use rights verified first)
  ├─ provenance/state mapper (installed / history / remote / unavailable / stale)
  └─ repository and migration layer
       ▼
SQLite in per-user app data (cache/preferences only; no Steam secrets)
```

No process is required beyond GameVault. Do not launch or control `steam.exe`; do not load `steamclient.dll`; do not attach to game processes. Direct reads of Steam files do not require Steam to be running, but a running client may update them concurrently. Refresh should tolerate file locks/partial reads and use bounded retries without changing Steam state.

## Discovery and local scan

1. Read the registered Steam root from the current user's Steam registry entry. Do not scan every disk or inspect process memory. Offer an explicit folder picker for additional/unregistered Steam roots; persist only roots the user chose.
2. For each root, read `steamapps/libraryfolders.vdf` using a parser that supports observed old and new KeyValues forms. Include the root's own library and each configured library path. Treat missing, malformed and inaccessible paths as distinct conditions.
3. Enumerate only `steamapps/appmanifest_*.acf` in configured libraries. Parse AppID, name, `installdir` and relevant state flags case-insensitively; bound file sizes/depth and ignore unknown fields. Confirm that the expected `steamapps/common/<installdir>` directory exists. Manifest present + folder present is evidence, not a guarantee of a complete/playable game.
4. Surface library-level states: `available`, `drive unavailable`, `path moved`, `scan error`, and `no manifest`. Never translate an unreachable external drive into “uninstalled.” A moved library can be reselected or Steam can repair/update its own mapping.
5. Refresh on startup/focus and explicit user request. An optional filesystem watcher can later trigger a bounded rescan of configured folders; do not keep polling every drive. A watcher cannot guarantee detection when Steam is closed, a drive is offline, or filesystem notifications overflow; always provide manual refresh.
6. Detect non-game Steam apps carefully. Local manifests can also represent tools, DLC, demos or soundtracks. If no verified app-type metadata exists, label them as Steam apps or leave type unknown instead of inventing “game.”

## Data/storage ownership

Steam remains authoritative for Steam-owned state. The local database stores an app's observed source and timestamp, not an independently editable copy of Steam state. Keep local organization separate from imported facts. Proposed conceptual entities (not a schema decision):

- `steam_installation`: normalized root selected/discovered, last scan, reachability.
- `installed_app`: AppID, manifest-derived name/relative install directory, state, observed source and scan time.
- `local_playtime_snapshot`: AppID, source (`localconfig` or verified API), value/unit, source timestamp and freshness; absent stays `unknown`.
- `achievement_snapshot`: AppID, achievement key, state/metadata/source and fetch time only where a supported source actually returns it.
- `user_preference`: local-only display preferences/favorites if retained in approved product scope.
- versioned migrations and database integrity metadata.

Do not store the user's Steam API key in SQLite, logs, export files or frontend state. If Valve confirms an individual user-key flow, hold the key in Windows Credential Manager/DPAPI and use it only in the Rust process over HTTPS. Windows protected storage limits casual disclosure but does not protect against same-user malware or a compromised GameVault process; allow revoke/delete and never send the key to any server. This design is conditional, not authorized yet.

Cache invalidation: rescan manifests when library state changes; replace a snapshot atomically after a successful parse; preserve last good cache on I/O/parse error but mark stale. Do not delete local preferences when a game is uninstalled. Explicit user deletion should remove cached account/app data while retaining only user-selected configuration if requested.

## Offline behavior

- Installed status and metadata already cached from local manifests can be shown without internet while the corresponding drive is available.
- Local `localconfig.vdf` playtime snapshots may be available offline for apps Steam recorded on this machine; label them as local and potentially stale. Missing value is “not available,” never zero.
- Owned-but-uninstalled library data, verified remote playtime and complete achievement/schema info need a working, approved Steam source; do not promise them offline unless a prior permitted snapshot exists.
- When a source cannot be reached, retain a timestamped last successful snapshot and show stale status. Do not delete or overwrite it on empty/invalid responses.
- No local cache, background service or cloud upload should imply synchronization with Steam. Only Steam/game integration can change Steam-owned state, and GameVault stays read-only.

## Security and permission boundaries

- Only Rust code reads the narrow Steam metadata files. The WebView gets typed DTOs through explicit commands, never arbitrary file paths, VDF text, SQL, HTTP URLs or shell execution.
- Request no administrator rights. Canonicalize configured roots, limit reads to known filenames and configured library roots, reject path escapes, apply size/depth/entry-count limits, and treat VDF values as untrusted display text.
- Never read `loginusers.vdf`, browser cookies, authentication files, broad log folders, arbitrary game saves or process memory for v1. Parse only the needed keys from `localconfig.vdf`; avoid logging the raw file or values.
- No telemetry or automatic upload. Network requests, API key use, artwork sources, updater checks and user-visible consent must be documented before enabling them.
- SQLite only through Rust repository methods, parameterized SQL, migrations and integrity tests. Back up before destructive migration; no production database yet.

## Blocking architectural choice

The mandatory owned-library and achievement features do not follow from local manifests. The backendless remote option is a per-user Web API key sent directly from GameVault to Steam; Valve confirmation is needed for application/domain registration, client-side use, confidentiality, public host and endpoint rules. Local `appcache/stats` reads would rely on undocumented binary formats and remain best-effort. See [`V1_FEATURE_FEASIBILITY.md`](V1_FEATURE_FEASIBILITY.md). No route is accepted for those features yet.

Current ADR statuses: ADR-001 **Proposed**, ADR-002 **Proposed**, ADR-003 **Accepted** (owner-confirmed SQLite), ADR-004 **Proposed**, ADR-005 **Proposed** (FOSS confirmed, exact license pending), ADR-006 **Accepted** (owner-confirmed local-only topology).

## References

- [Steam Support on Storage/library folders](https://help.steampowered.com/en/faqs/view/71AB-698D-57EB-178C)
- [Steam Support on moved/appearing uninstalled games](https://help.steampowered.com/en/faqs/view/4578-18A7-C819-8620)
- [Valve Web API key rules](https://partner.steamgames.com/doc/webapi_overview/auth)
- [Windows Credential Manager / secret handling](https://learn.microsoft.com/en-us/windows/win32/secbp/handling-passwords)
- [Tauri capabilities](https://v2.tauri.app/security/capabilities/)
- [`LOCAL_STEAM_DATA.md`](LOCAL_STEAM_DATA.md)
