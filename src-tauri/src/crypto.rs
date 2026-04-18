use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, Version};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

// pub fn hash_password() -> String {
//     SaltString::generate(&mut OsRng)
// }
