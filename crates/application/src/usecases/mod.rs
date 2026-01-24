pub mod create_user_usecase;
pub mod login_usecase;

use domain::repositories::RepositoryError;
use garde::Report;

#[derive(Debug, thiserror::Error)]
pub enum UsecaseError {
    #[error("Validation failed: {0}")]
    Validation(Report),

    #[error("Email already taken")]
    EmailAlreadyTaken,

    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Entity not found")]
    EntityNotFound,

    #[error("Password hashing failed: {0}")]
    PasswordHashingFailed(String),

    #[error("Token generation failed")]
    TokenGenerationFailed,

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

impl UsecaseError {
    /// Returns a user-friendly error message for HTTP responses
    pub fn to_http_message(&self) -> String {
        match self {
            UsecaseError::Validation(report) => {
                let mut errors = Vec::new();
                for (path, err) in report.iter() {
                    errors.push(format!("{}: {}", path, err.message()));
                }
                errors.join(", ")
            }
            UsecaseError::EmailAlreadyTaken => {
                "An account with this email already exists".to_string()
            }
            UsecaseError::InvalidCredentials => "Invalid email or password".to_string(),
            UsecaseError::EntityNotFound => "Required resource not found".to_string(),
            UsecaseError::PasswordHashingFailed(_) => {
                "An error occurred while processing your request".to_string()
            }
            UsecaseError::TokenGenerationFailed => {
                "An error occurred while generating authentication token".to_string()
            }
            UsecaseError::Database(_) => "An error occurred while saving your account".to_string(),
        }
    }
}
