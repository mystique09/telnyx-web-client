use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use domain::{
    models::phone_number::PhoneNumber, repositories::RepositoryError,
    repositories::phone_number_repository::PhoneNumberRepository,
};
use time::OffsetDateTime;
use tracing::error;

use crate::{
    dto::{CreatePhoneNumberRequest, CreatePhoneNumberResponse},
    session::get_user_id,
};

pub async fn handle_create_phone_number(
    create_req: web::Json<CreatePhoneNumberRequest>,
    session: Session,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
) -> impl Responder {
    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let now = OffsetDateTime::now_utc();
    let phone_number_id = uuid::Uuid::now_v7();
    let phone_number = PhoneNumber::builder()
        .id(phone_number_id)
        .user_id(user_id)
        .name(create_req.name.clone())
        .phone(create_req.phone.clone())
        .created_at(now)
        .updated_at(now)
        .build();

    match phone_number_repository
        .create_phone_number(&phone_number)
        .await
    {
        Ok(_) => HttpResponse::Created().json(CreatePhoneNumberResponse {
            id: phone_number_id,
        }),
        Err(err) => {
            error!(
                "failed to create phone number for user {} and phone {}: {}",
                user_id, create_req.phone, err
            );

            match err {
                RepositoryError::ConstraintViolation(_) | RepositoryError::DatabaseError(_) => {
                    HttpResponse::BadRequest().finish()
                }
                RepositoryError::NotFound | RepositoryError::UnexpectedError(_) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
