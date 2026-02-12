pub mod conversation_repository;
pub mod user_repository;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Entity not found")]
    NotFound,
    #[error("Database constraint violation: {0}")]
    ConstraintViolation(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
