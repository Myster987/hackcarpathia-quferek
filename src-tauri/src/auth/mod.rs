use std::sync::Mutex;

use tauri::{AppHandle, State};
use thiserror::Error;

use crate::{
    db::{self, create_encrypted, open_encrypted},
    utils::get_user_db_path,
    AppState, User,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] db::Error),
}

impl serde::Serialize for Error {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

#[tauri::command]
pub fn sign_in(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    username: String,
    password: String,
) -> Result<(), Error> {
    let user_db_path = get_user_db_path(app, &username);

    if !user_db_path.exists() {
        return Err(db::Error::UserNotFound.into());
    }

    let conn = open_encrypted(&user_db_path, &password)?;

    let user = User {
        username,
        password,
        db_conn: conn,
    };

    state.lock().unwrap().user = Some(user);

    Ok(())
}

#[tauri::command]
pub fn sign_up(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    username: String,
    password: String,
) -> Result<(), Error> {
    let user_db_path = get_user_db_path(app, &username);

    if user_db_path.exists() {
        return Err(db::Error::UserExists.into());
    }

    let conn = create_encrypted(&user_db_path, &password)?;

    let user = User {
        username,
        password,
        db_conn: conn,
    };

    state.lock().unwrap().user = Some(user);

    Ok(())
}

#[tauri::command]
pub fn sign_out(state: State<'_, Mutex<AppState>>) -> Result<(), Error> {
    state.lock().unwrap().user = None;
    Ok(())
}
