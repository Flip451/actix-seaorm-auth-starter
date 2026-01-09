use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};
use domain::user::PasswordHasher;
use domain::user::PasswordHashingError;
use domain::user::{HashedPassword, RawPassword};

pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, PasswordHashingError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(raw.as_bytes(), &salt)
            .map_err(|_| PasswordHashingError::HashingFailed)?
            .to_string();
        Ok(HashedPassword::from_raw_str(&hash))
    }

    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(hashed.as_str()) else {
            return false;
        };
        Argon2::default()
            .verify_password(raw.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
