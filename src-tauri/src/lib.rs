use std::sync::Mutex;

use rusqlite::Connection;
use tauri::Manager;

pub mod auth;
pub mod db;
pub mod utils;

pub struct User {
    pub username: String,
    pub password: String,
    pub db_conn: Connection,
}

pub struct AppState {
    user: Option<User>,
}

impl AppState {
    fn new() -> Self {
        Self { user: None }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    simple_logger::init().expect("Couldn't init logger");

    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::new()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            auth::sign_in,
            auth::sign_up,
            auth::sign_out
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
