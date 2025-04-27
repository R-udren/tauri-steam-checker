// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use lazy_static::lazy_static;
use std::sync::Mutex;
use steamid_find::{FetchedProfile, SteamUser, fetch_profile, get_steam_users};

lazy_static! {
    static ref STEAM_USERS_CACHE: Mutex<Vec<SteamUser>> = Mutex::new(Vec::new());
}

lazy_static! {
    static ref PROFILE_CACHE: Mutex<std::collections::HashMap<String, FetchedProfile>> =
        Mutex::new(std::collections::HashMap::new());
}

#[tauri::command]
fn get_steam_users_list() -> Vec<SteamUser> {
    let mut steam_users_cache = STEAM_USERS_CACHE.lock().unwrap();
    if steam_users_cache.is_empty() {
        let steam_users = get_steam_users().unwrap_or_default();
        let mut steam_users_vec = steam_users.to_vec();
        steam_users_vec.sort_by(|a, b| b.time_stamp.cmp(&a.time_stamp));
        *steam_users_cache = steam_users_vec;
    }
    steam_users_cache.clone()
}

#[tauri::command]
fn fetch_steam_profile(steam_id: &str) -> Result<FetchedProfile, String> {
    let mut cache = PROFILE_CACHE.lock().unwrap();
    if let Some(profile) = cache.get(steam_id) {
        return Ok(profile.clone());
    }
    let fetched = fetch_profile(steam_id);
    match fetched {
        Ok(profile) => {
            cache.insert(steam_id.to_string(), profile.clone());
            Ok(profile)
        }
        Err(e) => Err(format!("Error fetching profile: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_steam_users_list,
            fetch_steam_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
