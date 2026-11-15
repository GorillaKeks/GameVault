use serde::Serialize;
use std::{
    collections::BTreeSet,
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

const MAX_MANIFEST_BYTES: u64 = 1_048_576;
const MAX_LIBRARY_FOLDERS_BYTES: u64 = 65_536;
const MAX_LIBRARY_COUNT: usize = 64;
const MAX_DIRECTORY_ENTRIES_PER_SCAN: usize = 65_536;
const MAX_MANIFESTS_PER_SCAN: usize = 4_096;

#[derive(Serialize)]
struct GameCandidate {
    app_id: String,
    name: String,
    schema_available: bool,
}

#[derive(Serialize)]
struct SteamInspection {
    installation_found: bool,
    games: Vec<GameCandidate>,
    client_session: &'static str,
    achievements: &'static str,
    note: &'static str,
    scan_incomplete: bool,
}

struct LibraryDiscovery {
    roots: Vec<PathBuf>,
    incomplete: bool,
}

fn candidate_roots() -> Vec<PathBuf> {
    let mut roots = BTreeSet::new();
    for key in ["PROGRAMFILES(X86)", "PROGRAMFILES", "LOCALAPPDATA"] {
        if let Some(base) = env::var_os(key) {
            let candidate = PathBuf::from(base).join("Steam");
            if let Some(root) = validated_steam_root(&candidate) {
                roots.insert(root);
            }
        }
    }
    roots.into_iter().collect()
}

fn validated_steam_root(candidate: &Path) -> Option<PathBuf> {
    let root = candidate.canonicalize().ok()?;
    if !root.is_dir() {
        return None;
    }
    let executable = root.join("steam.exe").canonicalize().ok()?;
    (executable.starts_with(&root) && executable.is_file()).then_some(root)
}

fn quoted_value(line: &str, key: &str) -> Option<String> {
    let mut parts = line.split('"');
    let found_key = parts.nth(1)?;
    if !found_key.eq_ignore_ascii_case(key) {
        return None;
    }
    parts
        .nth(1)
        .map(|value| value.replace("\\\\", "\\").replace("\\\"", "\""))
}

fn read_bounded(path: &Path, max_bytes: u64) -> io::Result<Vec<u8>> {
    let file = fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input exceeds configured size limit",
        ));
    }
    Ok(bytes)
}

fn read_bounded_text(path: &Path, max_bytes: u64) -> io::Result<String> {
    String::from_utf8(read_bounded(path, max_bytes)?)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "input is not UTF-8"))
}

fn canonical_steamapps(root: &Path) -> Option<PathBuf> {
    let root = root.canonicalize().ok()?;
    let steamapps = root.join("steamapps").canonicalize().ok()?;
    (steamapps.starts_with(&root) && steamapps.is_dir()).then_some(steamapps)
}

fn library_roots(steam_root: &Path) -> LibraryDiscovery {
    let Some(steam_root) = steam_root.canonicalize().ok() else {
        return LibraryDiscovery {
            roots: Vec::new(),
            incomplete: true,
        };
    };
    let Some(main_steamapps) = canonical_steamapps(&steam_root) else {
        return LibraryDiscovery {
            roots: Vec::new(),
            incomplete: true,
        };
    };

    let mut roots = BTreeSet::from([steam_root.clone()]);
    let mut incomplete = false;
    let file = main_steamapps.join("libraryfolders.vdf");
    match read_bounded_text(&file, MAX_LIBRARY_FOLDERS_BYTES) {
        Ok(text) => {
            let mut candidates = 0;
            for line in text.lines() {
                let Some(path) = quoted_value(line, "path") else {
                    continue;
                };
                candidates += 1;
                if candidates >= MAX_LIBRARY_COUNT {
                    incomplete = true;
                    break;
                }

                let path = PathBuf::from(path);
                if !path.is_absolute() {
                    incomplete = true;
                    continue;
                }
                let Ok(canonical) = path.canonicalize() else {
                    incomplete = true;
                    continue;
                };
                if canonical_steamapps(&canonical).is_some() {
                    roots.insert(canonical);
                } else {
                    incomplete = true;
                }
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(_) => incomplete = true,
    }

    let mut roots = roots.into_iter();
    let mut bounded_roots = Vec::new();
    for _ in 0..MAX_LIBRARY_COUNT {
        let Some(root) = roots.next() else {
            break;
        };
        bounded_roots.push(root);
    }
    if roots.next().is_some() {
        incomplete = true;
    }

    LibraryDiscovery {
        roots: bounded_roots,
        incomplete,
    }
}

fn manifest_app_id_from_filename(path: &Path) -> Option<&str> {
    let name = path.file_name()?.to_str()?;
    let app_id = name.strip_prefix("appmanifest_")?.strip_suffix(".acf")?;
    (!app_id.is_empty() && app_id.bytes().all(|byte| byte.is_ascii_digit())).then_some(app_id)
}

fn parse_manifest_fields(text: &str) -> Option<(String, String)> {
    let mut app_id = None;
    let mut name = None;
    for line in text.lines() {
        if app_id.is_none() {
            app_id = quoted_value(line, "appid");
        }
        if name.is_none() {
            name = quoted_value(line, "name");
        }
        if app_id.is_some() && name.is_some() {
            break;
        }
    }
    let id = app_id?;
    if id.is_empty() || !id.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let display_name = name
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| format!("App {id}"));
    Some((id, display_name))
}

fn manifest_fields(
    path: &Path,
    expected_app_id: &str,
    steamapps: &Path,
) -> Option<(String, String)> {
    let steamapps = steamapps.canonicalize().ok()?;
    let manifest = path.canonicalize().ok()?;
    if !manifest.starts_with(&steamapps) || !manifest.is_file() {
        return None;
    }
    let text = read_bounded_text(&manifest, MAX_MANIFEST_BYTES).ok()?;
    let fields = parse_manifest_fields(&text)?;
    (fields.0 == expected_app_id).then_some(fields)
}

fn inspect_roots(roots: Vec<PathBuf>) -> SteamInspection {
    let mut games = Vec::new();
    let mut scan_incomplete = false;
    let mut directory_entries = 0;
    let mut manifest_count = 0;
    let mut limit_reached = false;

    for steam_root in &roots {
        let discovery = library_roots(steam_root);
        scan_incomplete |= discovery.incomplete;
        for library in discovery.roots {
            let Some(steamapps) = canonical_steamapps(&library) else {
                scan_incomplete = true;
                continue;
            };
            let Ok(entries) = fs::read_dir(&steamapps) else {
                scan_incomplete = true;
                continue;
            };
            for entry in entries {
                directory_entries += 1;
                if directory_entries > MAX_DIRECTORY_ENTRIES_PER_SCAN {
                    scan_incomplete = true;
                    limit_reached = true;
                    break;
                }
                let Ok(entry) = entry else {
                    scan_incomplete = true;
                    continue;
                };
                let path = entry.path();
                let Some(app_id) = manifest_app_id_from_filename(&path) else {
                    continue;
                };
                manifest_count += 1;
                if manifest_count > MAX_MANIFESTS_PER_SCAN {
                    scan_incomplete = true;
                    limit_reached = true;
                    break;
                }
                let Some((app_id, name)) = manifest_fields(&path, app_id, &steamapps) else {
                    scan_incomplete = true;
                    continue;
                };
                let schema = steam_root
                    .join("appcache")
                    .join("stats")
                    .join(format!("UserGameStatsSchema_{app_id}.bin"));
                games.push(GameCandidate {
                    app_id,
                    name,
                    schema_available: schema.is_file(),
                });
            }
            if limit_reached {
                break;
            }
        }
        if limit_reached {
            break;
        }
    }

    games.sort_by(|left, right| left.app_id.cmp(&right.app_id));
    games.dedup_by(|left, right| left.app_id == right.app_id);
    games.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| left.app_id.cmp(&right.app_id))
    });

    SteamInspection {
        installation_found: !roots.is_empty(),
        games,
        client_session: "Not connected",
        achievements: "Not read",
        note: "Installed local manifests are only candidates; this is not a complete owned library. No Steam client interface or schema contents are read.",
        scan_incomplete,
    }
}

#[tauri::command]
fn inspect_local_steam() -> SteamInspection {
    inspect_roots(candidate_roots())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![inspect_local_steam])
        .run(tauri::generate_context!())
        .expect("error while running GameVault");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEMP_ID: AtomicUsize = AtomicUsize::new(0);

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!(
                "gamevault-scanner-test-{}-{id}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("create temp test directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn create_library(root: &Path) -> PathBuf {
        let steamapps = root.join("steamapps");
        fs::create_dir_all(&steamapps).expect("create steamapps directory");
        steamapps
    }

    fn write_manifest(steamapps: &Path, filename_id: &str, body_id: &str, name: &str) {
        let content =
            format!("\"AppState\"\n{{\n\t\"appid\"\t\"{body_id}\"\n\t\"name\"\t\"{name}\"\n}}\n");
        fs::write(
            steamapps.join(format!("appmanifest_{filename_id}.acf")),
            content,
        )
        .expect("write synthetic manifest");
    }

    #[test]
    fn parses_expected_manifest_fields_and_case_insensitive_keys() {
        let parsed = parse_manifest_fields(
            "\"AppState\"\n{\n\t\"APPID\" \"42\"\n\t\"Name\" \"Example Game\"\n}",
        );
        assert_eq!(parsed, Some(("42".to_owned(), "Example Game".to_owned())));
    }

    #[test]
    fn rejects_invalid_app_ids_and_uses_name_fallback() {
        assert_eq!(parse_manifest_fields("\"appid\" \"42x\""), None);
        assert_eq!(
            parse_manifest_fields("\"appid\" \"42\"\n\"name\" \"\""),
            Some(("42".to_owned(), "App 42".to_owned()))
        );
    }

    #[test]
    fn bounded_reader_rejects_oversized_input() {
        let dir = TestDir::new();
        let file = dir.path().join("large.vdf");
        fs::write(&file, b"12345").expect("write test file");
        assert_eq!(
            read_bounded(&file, 4).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(read_bounded(&file, 5).unwrap().as_slice(), b"12345");
    }

    #[test]
    fn discovers_only_canonical_existing_libraries_with_a_steamapps_child() {
        let dir = TestDir::new();
        let steam_root = dir.path().join("Steam");
        let main_apps = create_library(&steam_root);
        let external_library = dir.path().join("Library");
        create_library(&external_library);
        let vdf_path = external_library.to_string_lossy().replace('\\', "\\\\");
        fs::write(
            main_apps.join("libraryfolders.vdf"),
            format!("\"libraryfolders\"\n{{\n\t\"1\"\n\t{{\n\t\t\"path\" \"{vdf_path}\"\n\t}}\n}}"),
        )
        .expect("write synthetic library list");

        let discovered = library_roots(&steam_root);
        assert!(!discovered.incomplete);
        assert_eq!(discovered.roots.len(), 2);
        assert!(
            discovered
                .roots
                .iter()
                .any(|path| path == &external_library.canonicalize().unwrap())
        );
    }

    #[test]
    fn oversized_library_metadata_marks_scan_incomplete_and_keeps_main_library() {
        let dir = TestDir::new();
        let steam_root = dir.path().join("Steam");
        let steamapps = create_library(&steam_root);
        fs::write(
            steamapps.join("libraryfolders.vdf"),
            vec![b'x'; MAX_LIBRARY_FOLDERS_BYTES as usize + 1],
        )
        .expect("write oversized synthetic library list");

        let discovered = library_roots(&steam_root);
        assert!(discovered.incomplete);
        assert_eq!(discovered.roots, vec![steam_root.canonicalize().unwrap()]);
    }

    #[test]
    fn library_discovery_caps_candidate_count() {
        let dir = TestDir::new();
        let steam_root = dir.path().join("Steam");
        let main_apps = create_library(&steam_root);
        let mut entries = Vec::new();
        for index in 0..=MAX_LIBRARY_COUNT {
            let library = dir.path().join(format!("Library-{index}"));
            create_library(&library);
            let vdf_path = library.to_string_lossy().replace('\\', "\\\\");
            entries.push(format!("\"{index}\"\n{{\n\t\"path\" \"{vdf_path}\"\n}}"));
        }
        fs::write(
            main_apps.join("libraryfolders.vdf"),
            format!("\"libraryfolders\"\n{{\n{}}}\n", entries.join("\n")),
        )
        .expect("write synthetic library list");

        let discovered = library_roots(&steam_root);
        assert!(discovered.incomplete);
        assert_eq!(discovered.roots.len(), MAX_LIBRARY_COUNT);
        assert!(
            discovered
                .roots
                .contains(&steam_root.canonicalize().unwrap())
        );
    }

    #[test]
    fn scans_synthetic_manifests_without_exposing_local_paths() {
        let dir = TestDir::new();
        let steam_root = dir.path().join("Steam");
        let steamapps = create_library(&steam_root);
        fs::write(steam_root.join("steam.exe"), b"synthetic").expect("create steam marker");
        write_manifest(&steamapps, "42", "42", "Example Game");
        let schema = steam_root
            .join("appcache")
            .join("stats")
            .join("UserGameStatsSchema_42.bin");
        fs::create_dir_all(schema.parent().unwrap()).expect("create schema directory");
        fs::write(&schema, b"synthetic schema marker").expect("create schema marker");

        let result = inspect_roots(vec![steam_root.clone()]);
        assert!(result.installation_found);
        assert!(!result.scan_incomplete);
        assert_eq!(result.games.len(), 1);
        assert_eq!(result.games[0].app_id, "42");
        assert_eq!(result.games[0].name, "Example Game");
        assert!(result.games[0].schema_available);

        let json = serde_json::to_value(result).expect("serialize inspection result");
        assert!(json.get("steam_roots").is_none());
        assert!(json["games"][0].get("manifest_path").is_none());
        assert!(
            !json
                .to_string()
                .contains(&steam_root.to_string_lossy().to_string())
        );
    }

    #[test]
    fn ignores_manifest_when_filename_and_content_ids_do_not_match() {
        let dir = TestDir::new();
        let steam_root = dir.path().join("Steam");
        let steamapps = create_library(&steam_root);
        fs::write(steam_root.join("steam.exe"), b"synthetic").expect("create steam marker");
        write_manifest(&steamapps, "42", "99", "Mismatched Game");

        let result = inspect_roots(vec![steam_root]);
        assert!(result.games.is_empty());
        assert!(result.scan_incomplete);
    }
}
