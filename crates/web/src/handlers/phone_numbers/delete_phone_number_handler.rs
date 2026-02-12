use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use application::usecases::UsecaseError;
use application::usecases::delete_phone_number_usecase::DeletePhoneNumberUsecase;
use domain::repositories::phone_number_repository::PhoneNumberRepository;
use tracing::error;

use crate::session::get_user_id;

pub async fn handle_delete_phone_number(
    path: web::Path<uuid::Uuid>,
    session: Session,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
) -> impl Responder {
    let phone_number_id = path.into_inner();

    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let delete_phone_number_usecase = DeletePhoneNumberUsecase::builder()
        .phone_number_repository(phone_number_repository.get_ref().clone())
        .build();

    match delete_phone_number_usecase
        .execute(user_id, phone_number_id)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(UsecaseError::EntityNotFound) => HttpResponse::NotFound().finish(),
        Err(err) => {
            error!(
                "failed to delete phone number {} for user {}: {}",
                phone_number_id, user_id, err
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
