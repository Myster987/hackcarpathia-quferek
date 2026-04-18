use tauri::{AppHandle, Manager, State};

use crate::{
    auth::{self, AuthError},
    state::{AppState, Session},
};

#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("{0}")]
    Auth(String),
    #[error("Not authenticated")]
    NotAuthenticated,
    #[error("Stronghold error: {0}")]
    Stronghold(String),
}

impl From<auth::AuthError> for VaultError {
    fn from(value: auth::AuthError) -> Self {
        VaultError::Auth(value.to_string())
    }
}

impl serde::Serialize for VaultError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub async fn register(
    app: AppHandle,
    username: String,
    password: String,
) -> Result<(), VaultError> {
    log::trace!("Register user data: username - {username}, password - {password}");

    let app_data_dir = app.path().app_data_dir().unwrap();
    auth::register_user(&app_data_dir, &username, &password)?;

    log::trace!("User {username} was registerd");

    Ok(())
}

#[tauri::command]
pub async fn login(
    app: AppHandle,
    state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<(), VaultError> {
    log::trace!("Login user data: username - {username}, password - {password}");

    let app_data_dir = app.path().app_data_dir().unwrap();

    let vault_key = auth::derive_vault_key(&app_data_dir, &username, &password)?;

    let stronghold = app.state::<tauri_plugin_stronghold::stronghold::Stronghold>();
    let vault_path = app_data_dir.join(format!("{}.vault", username));

    let Ok(client) = stronghold.load_client(vault_path.to_string_lossy().to_string()) else {
        return Err(VaultError::Auth(AuthError::UserNotFound.to_string()));
    };

    log::trace!("User {username} signed in!");

    *state.session.lock().unwrap() = Some(Session {
        username,
        vault_key,
        client,
    });

    Ok(())
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<(), VaultError> {
    *state.session.lock().unwrap() = None;
    Ok(())
}

#[tauri::command]
pub async fn save_entry(
    state: State<'_, AppState>,
    entry_name: String,
    secret: String,
) -> Result<(), VaultError> {
    let session = state.session.lock().unwrap();
    let session = session.as_ref().ok_or(VaultError::NotAuthenticated)?;

    session
        .client
        .store()
        .insert(entry_name.into(), secret.into(), None)
        .map_err(|e| VaultError::Stronghold(e.to_string()))?;

    Ok(())
}
