use argon2::{
    password_hash::SaltString, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::rngs::OsRng;

pub fn gen_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

pub fn strong_argon2() -> Argon2<'static> {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::new(65536, 3, 4, Some(32)).unwrap(),
    )
}

pub fn hash_password(password: &str, salt: &SaltString) -> String {
    let argon2 = strong_argon2();

    argon2
        .hash_password(password.as_bytes(), salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password_input: &str, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_password).expect("Invalid PHC string");

    strong_argon2()
        .verify_password(password_input.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash() -> anyhow::Result<()> {
        let password = "bad_password_123";
        let salt = gen_salt();

        let hashed_password = hash_password(password, &salt);

        println!("hash: {hashed_password}");

        Ok(())
    }
}
