use iota_stronghold::Client;
use std::sync::Mutex;
use zeroize::Zeroizing;

/// Sesja użytkownika.
pub struct Session {
    pub username: String,
    pub vault_key: Zeroizing<Vec<u8>>,
    pub client: Client,
}

/// Globalny rejestr sesji użytkoników.
pub struct AppState {
    pub session: Mutex<Option<Session>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(None),
        }
    }
}
