use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, http::header::LOCATION, web};
use domain::{
    models::phone_number::PhoneNumber, repositories::RepositoryError,
    repositories::phone_number_repository::PhoneNumberRepository,
};
use time::OffsetDateTime;
use tracing::error;

use crate::{
    dto::{CreatePhoneNumberRequest, CreatePhoneNumberResponse, FlashProps},
    flash::set_flash,
    session::get_user_id,
};

pub async fn handle_create_phone_number(
    req: HttpRequest,
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
        Ok(_) => {
            if req.headers().contains_key("x-inertia") {
                set_flash(&session, FlashProps::success("Phone number added."));
                return HttpResponse::Found()
                    .append_header((LOCATION, "/"))
                    .finish();
            }

            HttpResponse::Created().json(CreatePhoneNumberResponse {
                id: phone_number_id,
            })
        }
        Err(err) => {
            error!(
                "failed to create phone number for user {} and phone {}: {}",
                user_id, create_req.phone, err
            );

            match err {
                RepositoryError::ConstraintViolation(_) | RepositoryError::DatabaseError(_) => {
                    if req.headers().contains_key("x-inertia") {
                        set_flash(
                            &session,
                            FlashProps::error(
                                "Unable to add phone number. Check for duplicates and try again.",
                            ),
                        );
                        return HttpResponse::Found()
                            .append_header((LOCATION, "/"))
                            .finish();
                    }

                    HttpResponse::BadRequest().finish()
                }
                RepositoryError::NotFound | RepositoryError::UnexpectedError(_) => {
                    if req.headers().contains_key("x-inertia") {
                        set_flash(
                            &session,
                            FlashProps::error("Unable to add phone number right now."),
                        );
                        return HttpResponse::Found()
                            .append_header((LOCATION, "/"))
                            .finish();
                    }

                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
