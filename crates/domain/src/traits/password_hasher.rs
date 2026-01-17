use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("Failed to hash password")]
    HashingFailed,
    #[error("Password did not match")]
    PasswordMismatch,
}

#[derive(Debug)]
pub struct HashedPassword {
    pub hash: String,
    pub salt: String,
}

impl HashedPassword {
    pub fn new(hash: String, salt: String) -> Self {
        Self { hash, salt }
    }
}

#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, plain_password: &str) -> Result<HashedPassword, HashError>;
    async fn verify(&self, plain_password: &str, hashed_password: &str) -> Result<(), HashError>;
}
