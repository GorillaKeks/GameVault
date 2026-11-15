// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if gamevault_lib::run_steam_worker_mode() {
        return;
    }
    gamevault_lib::run()
}
