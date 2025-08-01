// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use steamid_find::{FetchedProfile, SteamUser};

mod database;
mod gathering;

pub use gathering::{gather_hwid_hash, gather_steam_users, web_fetch_steam_profile};

// Tauri command wrappers
#[tauri::command(async)]
fn get_steam_users_list() -> Result<Vec<SteamUser>, String> {
    gather_steam_users()
}

#[tauri::command(async)]
async fn fetch_steam_profile(steam_id: String) -> Result<FetchedProfile, String> {
    web_fetch_steam_profile(steam_id).await
}

#[tauri::command]
fn get_hwid_hash() -> Result<String, String> {
    gather_hwid_hash()
}

#[tauri::command]
async fn register_users() -> Result<(), String> {
    database::register_users().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_steam_users_list,
            fetch_steam_profile,
            get_hwid_hash,
            register_users
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
