pub mod create_user_usecase;

use domain::repositories::RepositoryError;
use garde::Report;

#[derive(Debug, thiserror::Error)]
pub enum UsecaseError {
    #[error("Validation failed: {0}")]
    Validation(Report),

    #[error("Email already taken")]
    EmailAlreadyTaken,

    #[error("Entity not found")]
    EntityNotFound,

    #[error("Password hashing failed: {0}")]
    PasswordHashingFailed(String),

    #[error("Database error: {0}")]
    Database(String),
}

impl From<Report> for UsecaseError {
    fn from(err: Report) -> Self {
        Self::Validation(err)
    }
}

impl From<garde::Error> for UsecaseError {
    fn from(err: garde::Error) -> Self {
        let mut report = garde::Report::new();
        report.append(garde::Path::empty(), err);
        Self::Validation(report)
    }
}

impl From<RepositoryError> for UsecaseError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::ConstraintViolation(e) => {
                if e.contains("email") {
                    return UsecaseError::EmailAlreadyTaken;
                }

                return UsecaseError::Database(e);
            }
            RepositoryError::NotFound => UsecaseError::EntityNotFound,
            RepositoryError::UnexpectedError(e) => UsecaseError::Database(e),
            RepositoryError::DatabaseError(e) => UsecaseError::Database(e),
        }
    }
}

impl From<domain::traits::password_hasher::HashError> for UsecaseError {
    fn from(err: domain::traits::password_hasher::HashError) -> Self {
        Self::PasswordHashingFailed(err.to_string())
    }
}
