use serde::{Deserialize, Serialize};

// Database Models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbUser {
    pub id: Option<String>,
    pub device_fingerprint: String,
    pub created_at: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbSteamAccount {
    pub id: Option<String>,
    pub steam_id: String,
    pub nickname: Option<String>,
    pub name_history: Vec<String>,
    pub sources: Vec<String>,
    pub most_recent: Option<bool>,
    pub time_stamp: Option<String>,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbAccountConnection {
    pub id: Option<String>,
    pub primary_steam_id: String,
    pub linked_steam_id: String,
    pub confidence: f64,
    pub reason: String,
    pub created_at: Option<String>,
}
