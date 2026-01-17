use argon2::{
    Argon2, PasswordHash, PasswordHasher as Argon2PasswordHasher, PasswordVerifier,
    password_hash::{Error, SaltString, rand_core::OsRng},
};
use async_trait::async_trait;
use domain::traits::password_hasher::{HashError, HashedPassword, PasswordHasher};

pub struct Argon2Hasher;

impl Argon2Hasher {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PasswordHasher for Argon2Hasher {
    async fn hash(&self, plain_password: &str) -> Result<HashedPassword, HashError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(plain_password.as_bytes(), &salt)
            .map_err(|e| match e {
                Error::Password => HashError::PasswordMismatch,
                _ => HashError::HashingFailed,
            })?;

        let hash = hash.to_string();
        let hashed_password = HashedPassword::new(hash, salt.to_string());

        return Ok(hashed_password);
    }

    async fn verify(&self, plain_password: &str, hashed_password: &str) -> Result<(), HashError> {
        let argon2 = Argon2::default();
        let hash = PasswordHash::new(hashed_password).map_err(|_| HashError::PasswordMismatch)?;

        argon2
            .verify_password(plain_password.as_bytes(), &hash)
            .map_err(|_| HashError::PasswordMismatch)
            .and_then(|_| Ok(()))
    }
}
