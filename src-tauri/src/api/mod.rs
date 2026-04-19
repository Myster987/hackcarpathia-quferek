use std::sync::Mutex;
use tauri::State;

use crate::{
    db::{self, Login},
    AppState,
};

#[tauri::command]
pub fn get_all_logins(state: State<'_, Mutex<AppState>>) -> Result<Vec<String>, db::Error> {
    let lock = state.lock().unwrap();
    let Some(user) = lock.user.as_ref() else {
        return Err(db::Error::LoginNotFound);
    };

    db::get_all_logins(&user.db_conn)
}

#[tauri::command]
pub fn get_login(state: State<'_, Mutex<AppState>>, name: &str) -> Result<Login, db::Error> {
    let lock = state.lock().unwrap();
    let Some(user) = lock.user.as_ref() else {
        return Err(db::Error::LoginNotFound);
    };

    db::get_login(&user.db_conn, name)
}

#[tauri::command]
pub fn insert_new_login(
    state: State<'_, Mutex<AppState>>,
    name: &str,
    password: &str,
) -> Result<(), db::Error> {
    let lock = state.lock().unwrap();
    let Some(user) = lock.user.as_ref() else {
        return Err(db::Error::LoginNotFound);
    };

    let login = Login {
        name: name.to_string(),
        password: password.to_string(),
    };

    db::insert_new_login(&user.db_conn, login)
}

#[tauri::command]
pub fn delete_login(state: State<'_, Mutex<AppState>>, name: &str) -> Result<(), db::Error> {
    let lock = state.lock().unwrap();
    let Some(user) = lock.user.as_ref() else {
        return Err(db::Error::LoginNotFound);
    };

    db::delete_login(&user.db_conn, name)
}
