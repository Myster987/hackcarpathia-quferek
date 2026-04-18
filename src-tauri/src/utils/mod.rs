use std::path::PathBuf;

use tauri::Manager;

fn get_app_data_dir(app: tauri::AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap()
}

pub fn get_user_db_path(app: tauri::AppHandle, username: &str) -> PathBuf {
    get_app_data_dir(app).join(format!("db-\"{username}\""))
}
