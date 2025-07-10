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

#[tauri::command(async)]
fn get_steam_users_list() -> Result<Vec<SteamUser>, String> {
    let mut steam_users_cache = STEAM_USERS_CACHE.lock().unwrap();
    if steam_users_cache.is_empty() {
        let steam_users = get_steam_users().map_err(|e| format!("Failed fetching users: {}", e))?;
        let mut steam_users_vec = steam_users.to_vec();
        steam_users_vec.sort_by(|a, b| b.time_stamp.cmp(&a.time_stamp));
        *steam_users_cache = steam_users_vec;
    }
    Ok(steam_users_cache.clone())
}

#[tauri::command(async)]
async fn fetch_steam_profile(steam_id: String) -> Result<FetchedProfile, String> {
    // Check cache first
    {
        let cache = PROFILE_CACHE.lock().unwrap();
        if let Some(profile) = cache.get(&steam_id) {
            return Ok(profile.clone());
        }
    }
    // Release lock before await
    let fetched = fetch_profile(&steam_id).await;
    match fetched {
        Ok(profile) => {
            let mut cache = PROFILE_CACHE.lock().unwrap();
            cache.insert(steam_id.to_string(), profile.clone());
            Ok(profile)
        }
        Err(e) => Err(format!("Error fetching profile: {}", e)),
    }
}

#[tauri::command]
fn get_hwid_hash() -> Result<String, String> {
    use machineid_rs::{Encryption, HWIDComponent, IdBuilder};

    let mut builder = IdBuilder::new(Encryption::MD5);
    builder.add_component(HWIDComponent::CPUID);
    builder.add_component(HWIDComponent::CPUCores);
    let hwid = builder
        .build("totalylegit")
        .map_err(|e| format!("Failed to generate HWID: {}", e))?;
    Ok(hwid)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_steam_users_list,
            fetch_steam_profile,
            get_hwid_hash
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
