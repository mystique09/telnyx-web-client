use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use application::usecases::list_phone_numbers_usecase::ListPhoneNumbersUsecase;
use domain::repositories::phone_number_repository::PhoneNumberRepository;
use tracing::error;

use crate::{dto::PhoneNumberProps, session::get_user_id};

pub async fn handle_list_phone_numbers(
    session: Session,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
) -> impl Responder {
    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let list_phone_numbers_usecase = ListPhoneNumbersUsecase::builder()
        .phone_number_repository(phone_number_repository.get_ref().clone())
        .build();

    match list_phone_numbers_usecase.execute(user_id).await {
        Ok(phone_numbers) => HttpResponse::Ok().json(
            phone_numbers
                .iter()
                .map(PhoneNumberProps::from)
                .collect::<Vec<_>>(),
        ),
        Err(err) => {
            error!("failed to list phone numbers for user {}: {}", user_id, err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
