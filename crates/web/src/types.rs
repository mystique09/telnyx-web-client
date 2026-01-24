use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(serde::Serialize)]
pub(crate) struct DataPage<T>
where
    T: Serialize,
{
    component: String,
    props: T,
    url: String,
}

impl<T> DataPage<T>
where
    T: Serialize,
{
    pub fn new(component: String, props: T, url: String) -> Self {
        Self {
            component,
            props,
            url,
        }
    }
}

/// Web-specific error that wraps UsecaseError for HTTP responses
#[derive(Debug, Error)]
pub enum WebError {
    #[error(transparent)]
    Usecase(#[from] application::usecases::UsecaseError),
}

impl ResponseError for WebError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            WebError::Usecase(e) => match e {
                application::usecases::UsecaseError::Validation(_) => {
                    actix_web::http::StatusCode::BAD_REQUEST
                }
                application::usecases::UsecaseError::EmailAlreadyTaken => {
                    actix_web::http::StatusCode::CONFLICT
                }
                application::usecases::UsecaseError::InvalidCredentials => {
                    actix_web::http::StatusCode::UNAUTHORIZED
                }
                application::usecases::UsecaseError::EntityNotFound => {
                    actix_web::http::StatusCode::NOT_FOUND
                }
                application::usecases::UsecaseError::PasswordHashingFailed(_) => {
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                }
                application::usecases::UsecaseError::TokenGenerationFailed => {
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                }
                application::usecases::UsecaseError::Database(_) => {
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                }
            },
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_msg = match self {
            WebError::Usecase(e) => e.to_http_message(),
        };
        HttpResponse::build(self.status_code()).json(WebErrorResponse { error: error_msg })
    }
}

/// Standard API error response
#[derive(Debug, Serialize)]
pub struct WebErrorResponse {
    pub error: String,
}
