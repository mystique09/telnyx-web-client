use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, Responder, web};
use application::usecases::UsecaseError;
use application::usecases::get_conversation_usecase::GetConversationUsecase;
use application::usecases::list_conversations_usecase::ListConversationsUsecase;
use application::usecases::list_messages_by_conversation_usecase::ListMessagesByConversationUsecase;
use application::usecases::list_phone_numbers_usecase::ListPhoneNumbersUsecase;
use domain::repositories::conversation_repository::ConversationRepository;
use domain::repositories::message_repository::MessageRepository;
use domain::repositories::phone_number_repository::PhoneNumberRepository;
use serde::Serialize;
use tracing::error;

use crate::{
    dto::{ConversationProps, FlashProps, MessageProps, PhoneNumberProps},
    flash::extract_flash,
    inertia::Page,
    session::session_user_id,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationPageProps {
    pub flash: Option<FlashProps>,
    pub conversations: Vec<ConversationProps>,
    pub conversation: Option<ConversationProps>,
    pub messages: Vec<MessageProps>,
    pub phone_numbers: Vec<PhoneNumberProps>,
}

pub async fn render_get_conversation(
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
    message_repository: web::Data<Arc<dyn MessageRepository>>,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
) -> impl Responder {
    let conversation_id = path.into_inner();
    let flash = extract_flash(&session);

    let get_conversation_usecase = GetConversationUsecase::builder()
        .conversation_repository(conversation_repository.get_ref().clone())
        .build();
    let list_conversations_usecase = ListConversationsUsecase::builder()
        .conversation_repository(conversation_repository.get_ref().clone())
        .build();
    let list_messages_by_conversation_usecase = ListMessagesByConversationUsecase::builder()
        .message_repository(message_repository.get_ref().clone())
        .build();
    let list_phone_numbers_usecase = ListPhoneNumbersUsecase::builder()
        .phone_number_repository(phone_number_repository.get_ref().clone())
        .build();

    let (conversation, conversations, messages, phone_numbers) = match session_user_id(&session) {
        Some(user_id) => {
            let conversation = match get_conversation_usecase
                .execute(user_id, conversation_id)
                .await
            {
                Ok(item) => Some(ConversationProps::from(&item)),
                Err(UsecaseError::EntityNotFound) => None,
                Err(err) => {
                    error!(
                        "failed to get conversation {} for user {}: {}",
                        conversation_id, user_id, err
                    );
                    None
                }
            };

            let messages = if conversation.is_some() {
                match list_messages_by_conversation_usecase
                    .execute(user_id, conversation_id)
                    .await
                {
                    Ok(items) => items.iter().map(MessageProps::from).collect(),
                    Err(err) => {
                        error!(
                            "failed to list messages for conversation {} and user {}: {}",
                            conversation_id, user_id, err
                        );
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
            };

            let conversations = match list_conversations_usecase.execute(user_id).await {
                Ok(items) => items.iter().map(ConversationProps::from).collect(),
                Err(err) => {
                    error!("failed to list conversations for user {}: {}", user_id, err);
                    Vec::new()
                }
            };

            let phone_numbers = match list_phone_numbers_usecase.execute(user_id).await {
                Ok(items) => items.iter().map(PhoneNumberProps::from).collect(),
                Err(err) => {
                    error!("failed to list phone numbers for user {}: {}", user_id, err);
                    Vec::new()
                }
            };

            (conversation, conversations, messages, phone_numbers)
        }
        None => (None, Vec::new(), Vec::new(), Vec::new()),
    };

    Page::builder()
        .req(req)
        .name("Conversations")
        .props(ConversationPageProps {
            flash,
            conversations,
            conversation,
            messages,
            phone_numbers,
        })
        .build()
        .to_responder()
}
