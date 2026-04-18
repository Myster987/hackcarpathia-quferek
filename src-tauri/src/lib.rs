pub mod auth;
pub mod crypto;
pub mod state;
pub mod vault;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    simple_logger::init().expect("Couldn't init logger");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_stronghold::Builder::new(|password| {
                use argon2::password_hash::{PasswordHasher, SaltString};
                use argon2::Argon2;
                let salt = SaltString::from_b64("stronghold-init-salt").unwrap();
                let hash = Argon2::default()
                    .hash_password(&password.as_bytes(), &salt)
                    .unwrap()
                    .to_string();
                hash.into_bytes()
            })
            .build(),
        )
        .manage(state::AppState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            vault::register,
            vault::login,
            vault::logout,
            vault::save_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
