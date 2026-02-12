use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, http::header::LOCATION, web};
use application::commands::CreateConversationCommand;
use application::usecases::UsecaseError;
use application::usecases::create_conversation_usecase::CreateConversationUsecase;
use domain::repositories::conversation_repository::ConversationRepository;
use tracing::error;

use crate::{
    dto::{CreateConversationRequest, CreateConversationResponse, FlashProps},
    flash::set_flash,
    session::get_user_id,
};

pub async fn handle_create_conversation(
    req: HttpRequest,
    create_req: web::Json<CreateConversationRequest>,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
) -> impl Responder {
    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Found()
            .append_header((LOCATION, "/auth/login"))
            .finish();
    };

    let cmd = CreateConversationCommand {
        user_id,
        phone_number_id: create_req.phone_number_id,
    };

    let create_conversation_usecase = CreateConversationUsecase::builder()
        .conversation_repository(conversation_repository.get_ref().clone())
        .build();

    match create_conversation_usecase.execute(cmd).await {
        Ok(result) => {
            if req.headers().contains_key("x-inertia") {
                set_flash(&session, FlashProps::success("Conversation created."));
                return HttpResponse::Found()
                    .append_header((LOCATION, format!("/conversations/{}", result.id)))
                    .finish();
            }

            HttpResponse::Created().json(CreateConversationResponse { id: result.id })
        }
        Err(err) => {
            error!(
                "failed to create conversation for user {} and phone_number {}: {}",
                user_id, create_req.phone_number_id, err
            );

            match err {
                UsecaseError::Database(_) => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
