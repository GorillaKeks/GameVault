use serde::{Deserialize, Serialize};

const MAX_ACHIEVEMENTS: usize = 1_024;
const MAX_TEXT_BYTES: usize = 2_048;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub achieved: bool,
    pub unlock_time: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AchievementReadResult {
    pub status: String,
    pub achievements: Vec<Achievement>,
    pub message: Option<String>,
}

trait ReadOnlyAchievementSource {
    fn count(&mut self) -> Result<usize, &'static str>;
    fn id_at(&mut self, index: usize) -> Result<String, &'static str>;
    fn state(&mut self, id: &str) -> Result<(bool, u32), &'static str>;
    fn attribute(&mut self, id: &str, key: &str) -> Option<String>;
}

fn collect_achievements(
    source: &mut impl ReadOnlyAchievementSource,
) -> Result<Vec<Achievement>, &'static str> {
    let count = source.count()?;
    if count > MAX_ACHIEVEMENTS {
        return Err("Steam lieferte eine ungültige oder zu große Errungenschaftsliste.");
    }
    let mut items = Vec::with_capacity(count);
    for index in 0..count {
        let id = source.id_at(index)?;
        let (achieved, unlock_time) = source.state(&id)?;
        let name = source.attribute(&id, "name").unwrap_or_else(|| id.clone());
        let description = source.attribute(&id, "desc").unwrap_or_default();
        items.push(Achievement {
            id,
            name,
            description,
            achieved,
            unlock_time: (achieved && unlock_time > 0).then_some(unlock_time),
        });
    }
    Ok(items)
}

impl AchievementReadResult {
    fn unavailable(message: &'static str) -> Self {
        Self {
            status: "unavailable".to_owned(),
            achievements: Vec::new(),
            message: Some(message.to_owned()),
        }
    }
}

pub fn validate_app_id(app_id: &str) -> Option<u32> {
    if app_id.is_empty() || app_id.len() > 10 || !app_id.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let value = app_id.parse::<u32>().ok()?;
    (value != 0).then_some(value)
}

#[cfg(windows)]
mod windows_client {
    use super::{
        Achievement, AchievementReadResult, MAX_TEXT_BYTES, ReadOnlyAchievementSource,
        collect_achievements,
    };
    use std::{
        ffi::{CStr, CString},
        os::windows::ffi::OsStrExt,
        path::Path,
        ptr::null_mut,
        sync::Mutex,
        thread,
        time::{Duration, Instant},
    };

    const STEAM_CLIENT_VERSION: &str = "SteamClient018";
    const STEAM_USER_VERSION: &str = "SteamUser012";
    const STEAM_USER_STATS_VERSION: &str = "STEAMUSERSTATS_INTERFACE_VERSION013";
    const USER_STATS_RECEIVED_CALLBACK: i32 = 1_101;
    const RESULT_OK: i32 = 1;
    const CALLBACK_TIMEOUT: Duration = Duration::from_secs(12);
    const LOAD_WITH_ALTERED_SEARCH_PATH: u32 = 0x0000_0008;

    static STEAM_CLIENT_LOCK: Mutex<()> = Mutex::new(());

    fn trace_stage(stage: &'static str) {
        eprintln!("steam_worker_stage={stage}");
    }

    #[repr(C, packed(1))]
    struct CallbackMessage {
        user: i32,
        callback_id: i32,
        parameter: *mut std::ffi::c_void,
        parameter_size: i32,
    }

    #[repr(C, packed(1))]
    struct UserStatsReceived {
        game_id: u64,
        result: i32,
        user_id: u64,
    }

    #[cfg(target_pointer_width = "64")]
    const _: () = {
        assert!(std::mem::size_of::<CallbackMessage>() == 20);
        assert!(std::mem::size_of::<UserStatsReceived>() == 20);
    };

    type CreateInterface = unsafe extern "system" fn(*const i8, *mut i32) -> *mut std::ffi::c_void;
    type GetCallback = unsafe extern "system" fn(i32, *mut CallbackMessage, *mut i32) -> u8;
    type FreeLastCallback = unsafe extern "system" fn(i32) -> u8;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn LoadLibraryExW(
            file_name: *const u16,
            file: *mut std::ffi::c_void,
            flags: u32,
        ) -> *mut std::ffi::c_void;
        fn GetProcAddress(module: *mut std::ffi::c_void, name: *const i8) -> *mut std::ffi::c_void;
        fn FreeLibrary(module: *mut std::ffi::c_void) -> i32;
    }

    struct ModuleHandle(*mut std::ffi::c_void);

    impl Drop for ModuleHandle {
        fn drop(&mut self) {
            if !self.0.is_null() {
                trace_stage("free_library");
                unsafe {
                    FreeLibrary(self.0);
                }
            }
        }
    }

    struct PipeHandle {
        client: *mut std::ffi::c_void,
        pipe: i32,
        user: i32,
    }

    impl Drop for PipeHandle {
        fn drop(&mut self) {
            unsafe {
                if self.user > 0 {
                    trace_stage("release_user");
                    vtable_method::<unsafe extern "system" fn(*mut std::ffi::c_void, i32, i32)>(
                        self.client,
                        4,
                    )(self.client, self.pipe, self.user);
                }
                if self.pipe > 0 {
                    trace_stage("release_pipe");
                    vtable_method::<unsafe extern "system" fn(*mut std::ffi::c_void, i32) -> u8>(
                        self.client,
                        1,
                    )(self.client, self.pipe);
                }
            }
        }
    }

    struct SteamSession {
        connection: PipeHandle,
        _module: ModuleHandle,
        user_interface: *mut std::ffi::c_void,
        stats: *mut std::ffi::c_void,
        get_callback: GetCallback,
        free_callback: FreeLastCallback,
    }

    impl SteamSession {
        fn connect(root: &Path) -> Result<Self, &'static str> {
            trace_stage("validate_architecture");
            if std::mem::size_of::<usize>() != 8 {
                return Err("Nur 64-Bit-Steam-Clients werden unterstützt.");
            }
            let library = root
                .join("steamclient64.dll")
                .canonicalize()
                .map_err(|_| "Steam-Client-Bibliothek nicht gefunden.")?;
            if !library.starts_with(root) || !library.is_file() {
                return Err("Steam-Client-Bibliothek liegt nicht im geprüften Steam-Ordner.");
            }
            let library_wide: Vec<u16> = library.as_os_str().encode_wide().chain(Some(0)).collect();
            // SAFETY: path is canonical and contained below the verified Steam root; Windows owns
            // the returned module handle until FreeLibrary. No other process is opened or injected.
            let module = unsafe {
                LoadLibraryExW(
                    library_wide.as_ptr(),
                    null_mut(),
                    LOAD_WITH_ALTERED_SEARCH_PATH,
                )
            };
            if module.is_null() {
                return Err("Steam-Client-Bibliothek konnte nicht geladen werden.");
            }
            trace_stage("steam_dll_loaded");
            let module = ModuleHandle(module);
            let create = unsafe { symbol::<CreateInterface>(module.0, b"CreateInterface\0") }
                .ok_or("Steam-Client-Schnittstelle fehlt.")?;
            let get_callback = unsafe { symbol::<GetCallback>(module.0, b"Steam_BGetCallback\0") }
                .ok_or("Steam-Callback-Schnittstelle fehlt.")?;
            let free_callback =
                unsafe { symbol::<FreeLastCallback>(module.0, b"Steam_FreeLastCallback\0") }
                    .ok_or("Steam-Callback-Freigabe fehlt.")?;
            let client = unsafe { create_interface(create, STEAM_CLIENT_VERSION)? };
            trace_stage("steam_client_interface_created");
            let pipe = unsafe {
                vtable_method::<unsafe extern "system" fn(*mut std::ffi::c_void) -> i32>(client, 0)(
                    client,
                )
            };
            if pipe <= 0 {
                return Err("Steam konnte keine Client-Pipe bereitstellen.");
            }
            trace_stage("steam_pipe_created");
            let mut connection = PipeHandle {
                client,
                pipe,
                user: 0,
            };
            let steam_user = unsafe {
                vtable_method::<unsafe extern "system" fn(*mut std::ffi::c_void, i32) -> i32>(
                    client, 2,
                )(client, pipe)
            };
            if steam_user <= 0 {
                return Err("Keine angemeldete Steam-Sitzung gefunden.");
            }
            trace_stage("global_user_connected");
            connection.user = steam_user;
            let user_version = CString::new(STEAM_USER_VERSION).expect("static interface version");
            let stats_version =
                CString::new(STEAM_USER_STATS_VERSION).expect("static interface version");
            let user = unsafe {
                let getter = vtable_method::<
                    unsafe extern "system" fn(
                        *mut std::ffi::c_void,
                        i32,
                        i32,
                        *const i8,
                    ) -> *mut std::ffi::c_void,
                >(client, 5);
                getter(client, steam_user, pipe, user_version.as_ptr())
            };
            trace_stage("steam_user_interface_requested");
            let stats = unsafe {
                let getter = vtable_method::<
                    unsafe extern "system" fn(
                        *mut std::ffi::c_void,
                        i32,
                        i32,
                        *const i8,
                    ) -> *mut std::ffi::c_void,
                >(client, 13);
                getter(client, steam_user, pipe, stats_version.as_ptr())
            };
            trace_stage("steam_user_stats_interface_requested");
            if user.is_null() {
                return Err(
                    "Steam stellt die angeforderte Schnittstelle SteamUser012 nicht bereit.",
                );
            }
            if stats.is_null() {
                return Err(
                    "Steam stellt die angeforderte Schnittstelle SteamUserStats013 nicht bereit.",
                );
            }
            Ok(Self {
                connection,
                _module: module,
                user_interface: user,
                stats,
                get_callback,
                free_callback,
            })
        }

        fn read(&mut self, app_id: u32) -> Result<Vec<Achievement>, &'static str> {
            let mut steam_id = 0u64;
            unsafe {
                vtable_method::<unsafe extern "system" fn(*mut std::ffi::c_void, *mut u64)>(
                    self.user_interface,
                    2,
                )(self.user_interface, &mut steam_id);
            }
            if steam_id == 0 {
                return Err("Steam konnte die angemeldete Steam-ID nicht bereitstellen.");
            }
            trace_stage("steam_id_read");
            let request = unsafe {
                vtable_method::<unsafe extern "system" fn(*mut std::ffi::c_void, u64) -> u64>(
                    self.stats, 15,
                )(self.stats, steam_id)
            };
            if request == 0 {
                return Err("Steam hat die Leseanfrage für Errungenschaften abgelehnt.");
            }
            trace_stage("user_stats_request_sent");
            self.wait_for_stats(app_id)?;
            trace_stage("user_stats_callback_received");
            collect_achievements(self)
        }

        fn wait_for_stats(&mut self, app_id: u32) -> Result<(), &'static str> {
            let started = Instant::now();
            while started.elapsed() < CALLBACK_TIMEOUT {
                let mut message = CallbackMessage {
                    user: 0,
                    callback_id: 0,
                    parameter: null_mut(),
                    parameter_size: 0,
                };
                let mut call = 0i32;
                trace_stage("before_callback_poll");
                let received = unsafe {
                    (self.get_callback)(self.connection.pipe, &mut message, &mut call) != 0
                };
                trace_stage(if received {
                    "callback_poll_received"
                } else {
                    "callback_poll_empty"
                });
                if !received {
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }
                let mut matched = None;
                trace_stage("before_callback_inspect");
                if message.callback_id == USER_STATS_RECEIVED_CALLBACK
                    && !message.parameter.is_null()
                    && message.parameter_size as usize >= std::mem::size_of::<UserStatsReceived>()
                {
                    let payload = unsafe { &*(message.parameter.cast::<UserStatsReceived>()) };
                    let returned_app_id = (payload.game_id & 0x00ff_ffff) as u32;
                    if returned_app_id == app_id {
                        matched = Some(payload.result);
                    }
                }
                trace_stage("before_callback_free");
                unsafe {
                    (self.free_callback)(self.connection.pipe);
                }
                trace_stage("callback_freed");
                if let Some(result) = matched {
                    trace_stage("user_stats_callback_matched");
                    return if result == RESULT_OK {
                        Ok(())
                    } else {
                        Err("Steam konnte Errungenschaften für dieses Spiel nicht laden.")
                    };
                }
            }
            Err("Steam hat die Errungenschaftsanfrage nicht rechtzeitig beantwortet.")
        }
    }

    impl ReadOnlyAchievementSource for SteamSession {
        fn count(&mut self) -> Result<usize, &'static str> {
            let count = unsafe {
                vtable_method::<unsafe extern "system" fn(*mut std::ffi::c_void) -> i32>(
                    self.stats, 13,
                )(self.stats)
            };
            trace_stage("achievement_count_read");
            usize::try_from(count)
                .map_err(|_| "Steam lieferte eine ungültige Errungenschaftsliste.")
        }

        fn id_at(&mut self, index: usize) -> Result<String, &'static str> {
            let index = i32::try_from(index)
                .map_err(|_| "Steam lieferte eine ungültige Errungenschaftsliste.")?;
            let name_ptr = unsafe {
                vtable_method::<unsafe extern "system" fn(*mut std::ffi::c_void, i32) -> *const i8>(
                    self.stats, 14,
                )(self.stats, index)
            };
            trace_stage("achievement_id_read");
            unsafe { read_native_text(name_ptr) }
                .ok_or("Steam lieferte ungültige Errungenschaftsdaten.")
        }

        fn state(&mut self, id: &str) -> Result<(bool, u32), &'static str> {
            let id = CString::new(id).map_err(|_| "Ungültige Errungenschafts-ID.")?;
            let get = unsafe {
                vtable_method::<
                    unsafe extern "system" fn(
                        *mut std::ffi::c_void,
                        *const i8,
                        *mut u8,
                        *mut u32,
                    ) -> u8,
                >(self.stats, 8)
            };
            let mut achieved = 0u8;
            let mut unlock_time = 0u32;
            if unsafe { get(self.stats, id.as_ptr(), &mut achieved, &mut unlock_time) } == 0 {
                return Err("Steam konnte den Status einer Errungenschaft nicht lesen.");
            }
            trace_stage("achievement_state_read");
            Ok((achieved != 0, unlock_time))
        }

        fn attribute(&mut self, id: &str, key: &str) -> Option<String> {
            trace_stage(if key == "name" {
                "achievement_name_read"
            } else {
                "achievement_description_read"
            });
            unsafe { display_attribute(self.stats, id, key) }
        }
    }

    pub(super) fn read(root: &Path, app_id: u32) -> AchievementReadResult {
        if STEAM_CLIENT_LOCK.is_poisoned() {
            return AchievementReadResult::unavailable(
                "Steam-Integration ist nach einem vorherigen Fehler deaktiviert.",
            );
        }
        let Ok(_guard) = STEAM_CLIENT_LOCK.lock() else {
            return AchievementReadResult::unavailable(
                "Steam-Integration ist derzeit nicht verfügbar.",
            );
        };
        let result = SteamSession::connect(root).and_then(|mut session| session.read(app_id));
        match result {
            Ok(achievements) => AchievementReadResult {
                status: "available".to_owned(),
                achievements,
                message: None,
            },
            Err(message) => AchievementReadResult::unavailable(message),
        }
    }

    unsafe fn create_interface(
        create: CreateInterface,
        version: &str,
    ) -> Result<*mut std::ffi::c_void, &'static str> {
        let version =
            CString::new(version).map_err(|_| "Ungültige Steam-Schnittstellenversion.")?;
        let mut return_code = 0;
        let interface = unsafe { create(version.as_ptr(), &mut return_code) };
        if interface.is_null() {
            Err("Diese Steam-Client-Schnittstellenversion wird nicht unterstützt.")
        } else {
            Ok(interface)
        }
    }

    unsafe fn symbol<T: Copy>(module: *mut std::ffi::c_void, name: &'static [u8]) -> Option<T> {
        let address = unsafe { GetProcAddress(module, name.as_ptr().cast()) };
        if address.is_null() {
            None
        } else {
            Some(unsafe { std::mem::transmute_copy(&address) })
        }
    }

    unsafe fn vtable_method<T: Copy>(object: *mut std::ffi::c_void, slot: usize) -> T {
        let table = unsafe { *(object.cast::<*const *const std::ffi::c_void>()) };
        let address = unsafe { *table.add(slot) };
        unsafe { std::mem::transmute_copy(&address) }
    }

    unsafe fn display_attribute(
        stats: *mut std::ffi::c_void,
        id: &str,
        attribute: &str,
    ) -> Option<String> {
        let id = CString::new(id).ok()?;
        let attribute = CString::new(attribute).ok()?;
        let get = unsafe {
            vtable_method::<
                unsafe extern "system" fn(*mut std::ffi::c_void, *const i8, *const i8) -> *const i8,
            >(stats, 11)
        };
        let value = unsafe { get(stats, id.as_ptr(), attribute.as_ptr()) };
        if value.is_null() {
            return None;
        }
        unsafe { read_native_text(value) }
    }

    unsafe fn read_native_text(value: *const i8) -> Option<String> {
        if value.is_null() {
            return None;
        }
        let bytes = unsafe { CStr::from_ptr(value) }.to_bytes();
        if bytes.len() > MAX_TEXT_BYTES {
            return None;
        }
        let text = std::str::from_utf8(bytes).ok()?.trim();
        (!text.is_empty()).then(|| text.to_owned())
    }
}

#[cfg(windows)]
pub fn read_for_app(steam_root: &std::path::Path, app_id: &str) -> AchievementReadResult {
    let Some(app_id) = validate_app_id(app_id) else {
        return AchievementReadResult::unavailable("Ungültige Steam-App-ID.");
    };
    windows_client::read(steam_root, app_id)
}

#[cfg(not(windows))]
pub fn read_for_app(_steam_root: &std::path::Path, _app_id: &str) -> AchievementReadResult {
    AchievementReadResult::unavailable(
        "Die Steam-Errungenschaftsverbindung ist nur unter Windows verfügbar.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAchievementSource {
        entries: Vec<(String, bool, u32, String, String)>,
        fail_state_for: Option<String>,
    }

    impl ReadOnlyAchievementSource for MockAchievementSource {
        fn count(&mut self) -> Result<usize, &'static str> {
            Ok(self.entries.len())
        }
        fn id_at(&mut self, index: usize) -> Result<String, &'static str> {
            self.entries
                .get(index)
                .map(|entry| entry.0.clone())
                .ok_or("missing fixture")
        }
        fn state(&mut self, id: &str) -> Result<(bool, u32), &'static str> {
            if self.fail_state_for.as_deref() == Some(id) {
                return Err("missing state");
            }
            self.entries
                .iter()
                .find(|entry| entry.0 == id)
                .map(|entry| (entry.1, entry.2))
                .ok_or("missing fixture")
        }
        fn attribute(&mut self, id: &str, key: &str) -> Option<String> {
            let entry = self.entries.iter().find(|entry| entry.0 == id)?;
            match key {
                "name" if !entry.3.is_empty() => Some(entry.3.clone()),
                "desc" if !entry.4.is_empty() => Some(entry.4.clone()),
                _ => None,
            }
        }
    }

    #[test]
    fn app_id_validation_rejects_empty_zero_and_path_text() {
        assert_eq!(validate_app_id(""), None);
        assert_eq!(validate_app_id("0"), None);
        assert_eq!(validate_app_id("../42"), None);
        assert_eq!(validate_app_id("42"), Some(42));
        assert_eq!(validate_app_id("4294967296"), None);
    }

    #[test]
    fn unavailable_results_never_claim_zero_or_locked_achievements() {
        let result = AchievementReadResult::unavailable("fixture");
        assert_eq!(result.status, "unavailable");
        assert!(result.achievements.is_empty());
        assert_eq!(result.message.as_deref(), Some("fixture"));
    }

    #[test]
    fn achievement_dto_contains_read_only_fields_only() {
        let achievement = Achievement {
            id: "ACH_ONE".into(),
            name: "First".into(),
            description: "Synthetic fixture".into(),
            achieved: true,
            unlock_time: Some(1_700_000_000),
        };
        let json = serde_json::to_value(achievement).expect("serialize fixture");
        assert_eq!(json["achieved"], true);
        assert!(json.get("set").is_none());
        assert!(json.get("reset").is_none());
        assert!(json.get("write").is_none());
    }

    #[test]
    fn synthetic_read_only_snapshot_distinguishes_earned_from_locked() {
        let mut source = MockAchievementSource {
            entries: vec![
                (
                    "ACH_ONE".into(),
                    true,
                    1_700_000_000,
                    "First".into(),
                    "A description".into(),
                ),
                ("ACH_TWO".into(), false, 0, "Second".into(), "".into()),
            ],
            fail_state_for: None,
        };
        let items = collect_achievements(&mut source).expect("collect synthetic read-only results");
        assert_eq!(items.len(), 2);
        assert!(items[0].achieved);
        assert_eq!(items[0].unlock_time, Some(1_700_000_000));
        assert!(!items[1].achieved);
        assert_eq!(items[1].unlock_time, None);
        assert_eq!(items[0].name, "First");
    }

    #[test]
    fn synthetic_partial_state_fails_closed_instead_of_returning_false_locked() {
        let mut source = MockAchievementSource {
            entries: vec![("ACH_ONE".into(), false, 0, "First".into(), "".into())],
            fail_state_for: Some("ACH_ONE".into()),
        };
        assert_eq!(collect_achievements(&mut source), Err("missing state"));
    }

    #[test]
    fn synthetic_missing_name_uses_id_and_oversized_sets_are_rejected() {
        let mut source = MockAchievementSource {
            entries: vec![("ACH_ONE".into(), false, 0, String::new(), String::new())],
            fail_state_for: None,
        };
        let items = collect_achievements(&mut source).expect("optional title fallback");
        assert_eq!(items[0].name, "ACH_ONE");
        source
            .entries
            .resize(MAX_ACHIEVEMENTS + 1, source.entries[0].clone());
        assert_eq!(
            collect_achievements(&mut source),
            Err("Steam lieferte eine ungültige oder zu große Errungenschaftsliste.")
        );
    }
}
