# Local Steam Data on Windows

**Research date:** 2026-11-12
**Scope:** read-only inspection and public-format research. No Steam/game files were modified, no game names, account IDs, library paths, playtime values or profile contents are included here.
**Important:** Valve does not publish a supported schema or stability contract for most Steam client files below. VDF/ACF observations are common community reverse engineering and implementation practice, not a guarantee from Valve. Revalidate on Steam client updates.

## Findings from this Windows machine

The registered Steam installation was present and the Steam client was running during inspection. A read-only metadata scan found:

- Two configured library roots, both reachable.
- Fourteen `appmanifest_*.acf` files across those roots; all fourteen had a present install directory and the installed state bit set in the manifest at scan time.
- Sixteen files in `appcache/stats/`: eight schema-named files and eight per-user stats-named files. Their contents and identifiers were not inspected.
- One Steam user-data profile and one `localconfig.vdf` (113,539 bytes). An in-memory key-name count found 212 `Playtime`, 8 `Playtime2wks`, and 212 `LastPlayed` occurrences. Values were not collected or printed.

This is a single, running client snapshot, not a controlled truth set. Steam could have changed a file during the scan; counts show local availability only, not accuracy, completeness or format support. No credential, username, app ID, game title, path, time value or binary content is disclosed.

## Relevant files and data

| Location relative to registered Steam root | Common structure / data | Source status and reliability | Steam running / compatibility considerations |
|---|---|---|---|
| Windows registry `HKCU\\Software\\Valve\\Steam` (`SteamPath` / `InstallPath`) | Registered primary client root; suitable discovery starting point | Widely used Windows client registration. May identify only the registered/last installation; not a documented multi-install enumeration API. | Registry read does not require Steam running. User-selected roots are needed for unregistered secondary installs. |
| `steamapps/libraryfolders.vdf` | Text KeyValues/VDF tree; newer shape maps numbered library entries to `path`, optional `apps` and other fields. Older clients used a different shape, and older configuration may use `BaseInstallFolder_N`. | Format is observed in Steam and parsed by independent tools; Valve documents multiple library folders in Storage support docs but does not publish a file-schema contract. | File exists offline. Can be stale while a drive is unplugged, a folder is moved, or Steam is writing settings. Handle old/new shape, missing keys, casing and parse errors. |
| `<library>\\steamapps\\appmanifest_<appid>.acf` | Text KeyValues/ACF under each library. Common `AppState` values include app ID, display name, install directory, state flags, build/update metadata, and possibly `UserConfig`/installed depots. | Practical local installation record; Valve Support confirms games are moved among Steam Library folders and can appear uninstalled when Steam no longer recognizes their files. Manifest field details are community-derived, not a stable public API. | Steam process is not needed to read an existing file. A manifest can be partial/stale or remain during download/repair; verify state flags and referenced directory. Do not treat file existence alone as proof a playable install is complete. |
| `<library>\\steamapps\\common\\<installdir>` | Installed game files below the manifest's install directory | Files are game/publisher supplied and arbitrary; presence does not prove integrity, launcher readiness, or game type. | Directory can be moved, deleted, partially populated, permission-blocked or on a missing external drive. Never execute or inspect binaries for discovery. |
| `config/config.vdf` | Text KeyValues with Steam client settings; older library-location entries may appear as `BaseInstallFolder_N` | Internal configuration; not a supported public data contract. Do not parse unrelated settings. | Can change as client settings change. No need to read it unless supporting a verified legacy fallback. |
| `userdata/<account-id>/config/localconfig.vdf` | Large text KeyValues tree, commonly rooted at `UserLocalConfigStore`; per-app entries may include `Playtime`, `Playtime2wks`, `LastPlayed`, cloud/configuration values and other user preferences. Launch options and other personal Steam state may also be present. | Independent tools/forensic research and this machine's key-name scan confirm playtime/timestamp fields can be present. Valve has not documented this as a third-party app data API; it is sparse/client-managed and can be stale or account/device-specific. Do not treat absent keys as zero or values as server-authoritative. | Steam can update it; running-client reads can be stale or race with writes. A read-only parser can use a bounded snapshot and retry changed/locked files. Do not read unrelated content or emit values in logs. |
| `appcache/appinfo.vdf` and other `appcache` metadata | Cached application metadata in internal VDF/binary forms, depending on cache/file | Cache only, not an account ownership list or stable product catalog. Coverage and schema are undocumented. | May be refreshed/removed by Steam; offline data may be incomplete. Use only as an optional name/type hint, never a source of ownership. |
| `appcache/stats/UserGameStatsSchema_<appid>.bin` | Observed binary per-app stat/achievement schema cache; reverse-engineered readers treat it as a binary KeyValues-like structure containing achievement/stat definitions and localized display metadata when present. | Valve documents achievement schemas and the Steamworks game API, but not this file path or binary schema. SAM and independent tools read it. Treat as unsupported and version-sensitive. |
| `appcache/stats/UserGameStats_<steamid3>_<appid>.bin` | Observed per-user/per-app stats cache file name; third-party parsers report achievement bits, timestamps and stat values in encoded groups. | Exact contents/semantics are reverse-engineered and not guaranteed by Valve. Existence does not prove a complete or current state. No content was opened in this review. Not approved as an app integration source yet. |
| `userdata/<account-id>/<appid>/remote/` and game-owned save folders | Steam Cloud local copies or game-specific saves/settings may be stored here; exact location and format are game-specific. | No universal Steam achievement or playtime schema. Cloud files are not a general account-library API. | May be absent, disabled, stale, encrypted or unrelated to achievements. Do not recursively scan game/user data. |
| `userdata/<account-id>/config/shortcuts.vdf` | Binary KeyValues for non-Steam shortcuts; may include app name, executable/path, launch options, tags and shortcut IDs. | Useful only for optional non-Steam shortcut display; not proof of Steam ownership and out of v1 scope. Format is community-derived. | Requires account-folder discovery; values expose user paths and launch parameters. Do not import in Steam-only v1. |

### Distinguish the data classes

- **Installed:** local app manifest plus an existing install directory is evidence of a local Steam-managed install, but may still be incomplete or damaged. An app ID can be a tool, demo, soundtrack or DLC, not necessarily a game.
- **Previously installed/played:** old `localconfig.vdf` entries or a retained cache may remain after uninstall. They are history, not proof of a current install or current ownership.
- **Owned:** manifests and client caches do not enumerate the complete account license library. Uninstalled, never-played owned titles cannot be recovered reliably from these local files. `GetOwnedGames` is an account Web API call requiring a user key and visible Game Details.
- **Public profile:** Steam Community/OpenID/API public profile data is a separate remote source. It does not establish local installation and does not override profile privacy.

## Local source conclusion

Local Steam files can support a read-only installed-app scanner and may supply per-device playtime for apps represented in `localconfig.vdf`. The files do **not** provide a complete, reliable owned-library list. The local achievement cache is present on this one system for some apps, but its schema is undocumented and not a general third-party SDK contract. Using that cache in GameVault would need a separate owner decision because `MASTER_PROMPT.md` excludes Steam client file scraping; the Phase 1.6 request authorizes this research only.

## Sources

Official Valve sources (2026-11-12):

- [Steam Support: managing library folders, moving installs and update/install failure cases](https://help.steampowered.com/en/faqs/view/71AB-698D-57EB-178C)
- [Steam Support: moving Steam installation/games](https://help.steampowered.com/en/faqs/view/4BD4-4528-6B2E-8327)
- [Steam Support: installed games appearing uninstalled](https://help.steampowered.com/en/faqs/view/4578-18A7-C819-8620)
- [Steamworks stats and achievements, including game-context offline cache behavior](https://partner.steamgames.com/doc/features/achievements)
- [Steamworks Community Data XML (explicitly deprecated; privacy may suppress data)](https://partner.steamgames.com/documentation/community_data)
- [Steam Web API `GetOwnedGames`](https://partner.steamgames.com/doc/webapi/IPlayerService)

Independent format evidence, not Valve guarantees:

- [KeyValues/VDF and client-file observations](https://reverseengineering.wiki/index.php/Format%3AKeyValues1_%28VDF%2C_ACF%29)
- [ValvePython VDF parser](https://github.com/ValvePython/vdf)
- [Steam achievement schema/cache reader in SAM, pinned source revision](https://github.com/gibbed/SteamAchievementManager/blob/de8b71048a0cee3c3e97cd8535e0f55ca86513e4/SAM.Game/Manager.cs)
- [Community analysis of localconfig playtime fields](https://github.com/grimm00/steam-config-analyzer/blob/main/docs/localconfig-schema-reference.md)
