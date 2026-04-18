use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::crypto::{self, gen_salt, verify_password};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Hash error: {0}")]
    Hash(String),
    #[error("Użytkownik już istnieje")]
    UserExists,
    #[error("Nie znaleziono użytkownika")]
    UserNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize)]
struct UserRecord {
    username: String,
    password_hash: String,
    salt_b64: String,
}

fn user_record_path(app_data_dir: &Path, username: &str) -> PathBuf {
    app_data_dir.join(format!("{}.user.json", username))
}

pub fn register_user(app_data_dir: &Path, username: &str, password: &str) -> Result<(), AuthError> {
    let path = user_record_path(app_data_dir, username);
    if path.exists() {
        return Err(AuthError::UserExists);
    }

    let salt = gen_salt();
    let password_hash = crypto::hash_password(password, &salt);

    let record = UserRecord {
        username: username.to_string(),
        password_hash,
        salt_b64: salt.to_string(),
    };

    std::fs::create_dir_all(app_data_dir)?;
    std::fs::write(&path, serde_json::to_string_pretty(&record)?)?;
    Ok(())
}

pub fn derive_vault_key(
    app_data_dir: &Path,
    username: &str,
    password: &str,
) -> Result<Zeroizing<Vec<u8>>, AuthError> {
    let path = user_record_path(app_data_dir, username);
    if !path.exists() {
        return Err(AuthError::UserNotFound);
    }

    let record: UserRecord = serde_json::from_str(&std::fs::read_to_string(&path)?)?;

    verify_password(password, &record.password_hash);

    let salt = argon2::password_hash::SaltString::from_b64(&record.salt_b64)
        .map_err(|e| AuthError::Hash(e.to_string()))?;

    let mut key = Zeroizing::new(vec![0u8; 32]);
    crypto::strong_argon2()
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut key)
        .map_err(|e| AuthError::Hash(e.to_string()))?;

    Ok(key)
}
