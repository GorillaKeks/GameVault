use serde::Serialize;
use std::{collections::BTreeSet, env, fs, path::{Path, PathBuf}};

const MAX_MANIFEST_BYTES: u64 = 1_048_576;

#[derive(Serialize)]
struct GameCandidate {
    app_id: String,
    name: String,
    manifest_path: String,
    schema_available: bool,
}

#[derive(Serialize)]
struct SteamInspection {
    installation_found: bool,
    steam_roots: Vec<String>,
    games: Vec<GameCandidate>,
    client_session: &'static str,
    achievements: &'static str,
    note: &'static str,
}

fn candidate_roots() -> Vec<PathBuf> {
    let mut roots = BTreeSet::new();
    for key in ["PROGRAMFILES(X86)", "PROGRAMFILES", "LOCALAPPDATA"] {
        if let Some(base) = env::var_os(key) {
            roots.insert(PathBuf::from(base).join("Steam"));
        }
    }
    roots.into_iter().filter(|p| p.join("steam.exe").is_file()).collect()
}

fn quoted_value(line: &str, key: &str) -> Option<String> {
    let mut parts = line.split('"');
    let found_key = parts.nth(1)?;
    if !found_key.eq_ignore_ascii_case(key) { return None; }
    parts.nth(1).map(|value| value.replace("\\\\", "\\").replace("\\\"", "\""))
}

fn library_roots(steam_root: &Path) -> Vec<PathBuf> {
    let mut libraries = BTreeSet::from([steam_root.to_path_buf()]);
    let file = steam_root.join("steamapps").join("libraryfolders.vdf");
    if let Ok(text) = fs::read_to_string(file) {
        for line in text.lines() {
            if let Some(path) = quoted_value(line, "path") {
                let path = PathBuf::from(path);
                if path.is_absolute() && path.join("steamapps").is_dir() { libraries.insert(path); }
            }
        }
    }
    libraries.into_iter().collect()
}

fn manifest_fields(path: &Path) -> Option<(String, String)> {
    let metadata = fs::metadata(path).ok()?;
    if metadata.len() > MAX_MANIFEST_BYTES { return None; }
    let text = fs::read_to_string(path).ok()?;
    let mut app_id = None;
    let mut name = None;
    for line in text.lines() {
        if app_id.is_none() { app_id = quoted_value(line, "appid"); }
        if name.is_none() { name = quoted_value(line, "name"); }
        if app_id.is_some() && name.is_some() { break; }
    }
    let id = app_id?;
    if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) { return None; }
    Some((id, name.filter(|n| !n.is_empty()).unwrap_or_else(|| format!("App {id}"))))
}

#[tauri::command]
fn inspect_local_steam() -> SteamInspection {
    let roots = candidate_roots();
    let mut games = Vec::new();
    for steam_root in &roots {
        for library in library_roots(steam_root) {
            let apps = library.join("steamapps");
            let Ok(entries) = fs::read_dir(&apps) else { continue; };
            for entry in entries.flatten() {
                let path = entry.path();
                let Some(file) = path.file_name().and_then(|s| s.to_str()) else { continue; };
                if !file.starts_with("appmanifest_") || !file.ends_with(".acf") { continue; }
                let Some((app_id, name)) = manifest_fields(&path) else { continue; };
                let schema = steam_root.join("appcache").join("stats")
                    .join(format!("UserGameStatsSchema_{app_id}.bin"));
                games.push(GameCandidate {
                    app_id,
                    name,
                    manifest_path: path.to_string_lossy().into_owned(),
                    schema_available: schema.is_file(),
                });
            }
        }
    }
    games.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    games.dedup_by(|a, b| a.app_id == b.app_id);
    SteamInspection {
        installation_found: !roots.is_empty(),
        steam_roots: roots.iter().map(|p| p.to_string_lossy().into_owned()).collect(),
        games,
        client_session: "Not connected",
        achievements: "Not read",
        note: "Installed local manifests are only candidates; this is not a complete owned library. No Steam client interface or schema contents are read.",
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![inspect_local_steam])
        .run(tauri::generate_context!())
        .expect("error while running GameVault");
}
