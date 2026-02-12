use domain::repositories::RepositoryError;

pub mod conversation_repository_impl;
pub mod message_repository_impl;
pub mod phone_number_repository_impl;
pub mod user_repository_impl;

pub trait RbsErrorExt {
    fn to_repository_error(self) -> RepositoryError;
}

impl RbsErrorExt for rbs::Error {
    fn to_repository_error(self) -> RepositoryError {
        match self {
            rbatis::Error::E(e) => {
                if e.contains("23502") || e.contains("23503") {
                    return RepositoryError::DatabaseError(e.to_owned());
                }

                if e.contains("unique_violation") {
                    return RepositoryError::ConstraintViolation(e.to_owned());
                }

                RepositoryError::UnexpectedError(e.to_owned())
            }
        }
    }
}
