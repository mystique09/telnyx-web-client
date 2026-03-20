use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use application::commands::CreateMessageCommand;
use application::usecases::UsecaseError;
use application::usecases::create_message_usecase::CreateMessageUsecase;
use domain::{
    repositories::{
        conversation_repository::ConversationRepository, message_repository::MessageRepository,
        phone_number_repository::PhoneNumberRepository,
    },
    traits::outbound_message_service::OutboundMessageService,
};
use serde::Serialize;
use tracing::error;

use crate::{
    dto::{CreateMessageRequest, CreateMessageResponse, MessageProps},
    session::session_user_id,
};

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn handle_create_message(
    path: web::Path<uuid::Uuid>,
    create_req: web::Json<CreateMessageRequest>,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
    message_repository: web::Data<Arc<dyn MessageRepository>>,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
    outbound_message_service: web::Data<Arc<dyn OutboundMessageService>>,
) -> impl Responder {
    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let create_message_usecase = CreateMessageUsecase::builder()
        .conversation_repository(conversation_repository.get_ref().clone())
        .message_repository(message_repository.get_ref().clone())
        .phone_number_repository(phone_number_repository.get_ref().clone())
        .outbound_message_service(outbound_message_service.get_ref().clone())
        .build();

    let conversation_id = path.into_inner();
    let cmd = CreateMessageCommand {
        user_id,
        conversation_id,
        content: create_req.content.clone(),
    };

    match create_message_usecase.execute(cmd).await {
        Ok(result) => HttpResponse::Created().json(CreateMessageResponse {
            message: MessageProps::from(&result.message),
        }),
        Err(err) => {
            error!(
                "failed to create message for user {} and conversation {}: {}",
                user_id, conversation_id, err
            );

            match err {
                UsecaseError::Validation(_)
                | UsecaseError::MessageRejected(_)
                | UsecaseError::EntityNotFound => {
                    HttpResponse::UnprocessableEntity().json(ErrorResponse {
                        error: err.to_http_message(),
                    })
                }
                UsecaseError::ExternalService(_) => {
                    HttpResponse::BadGateway().json(ErrorResponse {
                        error: err.to_http_message(),
                    })
                }
                _ => HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Unable to create message right now.".to_owned(),
                }),
            }
        }
    }
}
