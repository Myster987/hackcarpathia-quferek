use iota_stronghold::Client;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri_plugin_stronghold::stronghold::Stronghold;

/// Sesja użytkownika.
pub struct VaultSession {
    pub stronghold: Stronghold,
    pub client: Client,
}

/// Globalny rejestr sesji użytkoników.
pub struct AppState {
    pub sessions: Mutex<HashMap<String, VaultSession>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}
