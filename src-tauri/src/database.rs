use reqwest::Client;
use serde::{Deserialize, Serialize};
use steamid_find::SteamUser;

use crate::{gather_hwid_hash, gather_steam_users};

#[derive(Serialize, Deserialize, Debug)]
struct RegisterUsers {
    device_fingerprint: String,
    steam_users: Vec<SteamUser>,
}

pub async fn register_users() -> Result<(), String> {
    let device_fingerprint = gather_hwid_hash()?;
    let steam_users = gather_steam_users()?;

    let register_data = RegisterUsers {
        device_fingerprint,
        steam_users,
    };
    let base_url = std::env::var("API_BASE_URL")
        .map_err(|_| "API_BASE_URL not set in environment variables".to_string())?;
    let endpoint = format!("{}/steam/bulk-import", base_url);

    let client = Client::new();

    let response = client
        .post(&endpoint)
        .json(&register_data)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    println!("Response {}: {}", status, response_text);

    Ok(())
}
