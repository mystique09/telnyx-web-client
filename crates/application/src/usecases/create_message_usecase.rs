use std::sync::Arc;

use time::OffsetDateTime;

use crate::{
    commands::CreateMessageCommand, responses::CreateMessageResult, usecases::UsecaseError,
};
use domain::{
    models::message::{Message, MessageStatus, MessageType},
    repositories::{
        conversation_repository::ConversationRepository, message_repository::MessageRepository,
        phone_number_repository::PhoneNumberRepository,
    },
    traits::outbound_message_service::{OutboundMessageService, SendMessageRequest},
};

#[derive(bon::Builder)]
pub struct CreateMessageUsecase {
    conversation_repository: Arc<dyn ConversationRepository>,
    message_repository: Arc<dyn MessageRepository>,
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
    outbound_message_service: Arc<dyn OutboundMessageService>,
}

impl CreateMessageUsecase {
    pub async fn execute(
        &self,
        cmd: CreateMessageCommand,
    ) -> Result<CreateMessageResult, UsecaseError> {
        let content = cmd.content.trim();
        if content.is_empty() {
            return Err(garde::Error::new("Message content is required").into());
        }

        let mut conversation = self
            .conversation_repository
            .find_by_id(&cmd.user_id, &cmd.conversation_id)
            .await?;

        let recipient_phone_number = conversation
            .recipient_phone_number
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| garde::Error::new("Conversation recipient phone number is missing"))?
            .to_owned();

        let phone_number = self
            .phone_number_repository
            .find_by_id(&cmd.user_id, &conversation.phone_number_id)
            .await?;

        let provider_response = self
            .outbound_message_service
            .send_text_message(SendMessageRequest {
                from: phone_number.phone.clone(),
                to: recipient_phone_number,
                text: content.to_owned(),
            })
            .await?;

        let now = OffsetDateTime::now_utc();
        let message = Message::builder()
            .id(uuid::Uuid::now_v7())
            .conversation_id(conversation.id)
            .user_id(cmd.user_id)
            .message_type(MessageType::Outbound)
            .status(MessageStatus::Queued)
            .maybe_provider_message_id(Some(provider_response.provider_message_id))
            .maybe_provider_status(Some("queued".to_owned()))
            .maybe_provider_status_updated_at(None)
            .maybe_provider_error_code(None)
            .maybe_provider_error_detail(None)
            .from_number(phone_number.phone)
            .content(content.to_owned())
            .created_at(now)
            .updated_at(now)
            .build();

        let message = self.message_repository.create_message(&message).await?;

        conversation.last_message_at = message.created_at;
        conversation.updated_at = message.updated_at;
        self.conversation_repository
            .update_conversation(&conversation)
            .await?;

        Ok(CreateMessageResult { message })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use domain::{
        models::{
            conversation::Conversation,
            message::{Message, MessageStatus, MessageType},
            phone_number::PhoneNumber,
        },
        repositories::{
            RepositoryError, conversation_repository::ConversationRepository,
            message_repository::MessageRepository, phone_number_repository::PhoneNumberRepository,
        },
        traits::outbound_message_service::{
            OutboundMessageError, OutboundMessageService, SendMessageRequest, SendMessageResponse,
        },
    };
    use time::OffsetDateTime;

    use crate::{commands::CreateMessageCommand, usecases::UsecaseError};

    use super::CreateMessageUsecase;

    struct FakeConversationRepository {
        conversation: Conversation,
        updated_conversation: Mutex<Option<Conversation>>,
    }

    #[async_trait]
    impl ConversationRepository for FakeConversationRepository {
        async fn create_conversation(
            &self,
            _conversation: &Conversation,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn update_conversation(
            &self,
            conversation: &Conversation,
        ) -> Result<(), RepositoryError> {
            self.updated_conversation
                .lock()
                .expect("lock")
                .replace(conversation.clone());
            Ok(())
        }

        async fn find_by_id(
            &self,
            _user_id: &uuid::Uuid,
            _id: &uuid::Uuid,
        ) -> Result<Conversation, RepositoryError> {
            Ok(self.conversation.clone())
        }

        async fn find_by_phone_number_and_recipient(
            &self,
            _user_id: &uuid::Uuid,
            _phone_number_id: &uuid::Uuid,
            _recipient_phone_number: &str,
        ) -> Result<Conversation, RepositoryError> {
            Ok(self.conversation.clone())
        }

        async fn list_by_user_id(
            &self,
            _user_id: &uuid::Uuid,
        ) -> Result<Vec<Conversation>, RepositoryError> {
            Ok(vec![self.conversation.clone()])
        }

        async fn delete_conversation(
            &self,
            _user_id: &uuid::Uuid,
            _id: &uuid::Uuid,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct FakeMessageRepository {
        created_messages: Mutex<Vec<Message>>,
    }

    #[async_trait]
    impl MessageRepository for FakeMessageRepository {
        async fn create_message(&self, message: &Message) -> Result<Message, RepositoryError> {
            self.created_messages
                .lock()
                .expect("lock")
                .push(message.clone());
            Ok(message.clone())
        }

        async fn count_by_user_id(&self, _user_id: &uuid::Uuid) -> Result<u64, RepositoryError> {
            Ok(0)
        }

        async fn find_by_provider_message_id(
            &self,
            _provider_message_id: &str,
        ) -> Result<Message, RepositoryError> {
            Err(RepositoryError::NotFound)
        }

        async fn list_by_conversation_id(
            &self,
            _user_id: &uuid::Uuid,
            _conversation_id: &uuid::Uuid,
        ) -> Result<Vec<Message>, RepositoryError> {
            Ok(Vec::new())
        }

        async fn update_message(&self, message: &Message) -> Result<Message, RepositoryError> {
            Ok(message.clone())
        }
    }

    struct FakePhoneNumberRepository {
        phone_number: PhoneNumber,
    }

    #[async_trait]
    impl PhoneNumberRepository for FakePhoneNumberRepository {
        async fn create_phone_number(
            &self,
            _phone_number: &PhoneNumber,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _user_id: &uuid::Uuid,
            _id: &uuid::Uuid,
        ) -> Result<PhoneNumber, RepositoryError> {
            Ok(self.phone_number.clone())
        }

        async fn find_by_phone(&self, _phone: &str) -> Result<PhoneNumber, RepositoryError> {
            Ok(self.phone_number.clone())
        }

        async fn list_by_user_id(
            &self,
            _user_id: &uuid::Uuid,
        ) -> Result<Vec<PhoneNumber>, RepositoryError> {
            Ok(vec![self.phone_number.clone()])
        }

        async fn delete_phone_number(
            &self,
            _user_id: &uuid::Uuid,
            _id: &uuid::Uuid,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct FakeOutboundMessageService {
        response: Result<SendMessageResponse, OutboundMessageError>,
        requests: Mutex<Vec<SendMessageRequest>>,
    }

    #[async_trait]
    impl OutboundMessageService for FakeOutboundMessageService {
        async fn send_text_message(
            &self,
            request: SendMessageRequest,
        ) -> Result<SendMessageResponse, OutboundMessageError> {
            self.requests.lock().expect("lock").push(request);
            self.response.clone()
        }
    }

    fn build_conversation(
        user_id: uuid::Uuid,
        conversation_id: uuid::Uuid,
        phone_number_id: uuid::Uuid,
        recipient_phone_number: Option<String>,
    ) -> Conversation {
        let now = OffsetDateTime::now_utc();
        Conversation::builder()
            .id(conversation_id)
            .phone_number_id(phone_number_id)
            .user_id(user_id)
            .maybe_recipient_phone_number(recipient_phone_number)
            .last_message_at(now)
            .created_at(now)
            .updated_at(now)
            .build()
    }

    fn build_phone_number(user_id: uuid::Uuid, phone_number_id: uuid::Uuid) -> PhoneNumber {
        let now = OffsetDateTime::now_utc();
        PhoneNumber::builder()
            .id(phone_number_id)
            .user_id(user_id)
            .name("Primary".to_owned())
            .phone("+13125550100".to_owned())
            .created_at(now)
            .updated_at(now)
            .build()
    }

    #[tokio::test]
    async fn creates_queued_outbound_message_and_updates_conversation() {
        let user_id = uuid::Uuid::now_v7();
        let conversation_id = uuid::Uuid::now_v7();
        let phone_number_id = uuid::Uuid::now_v7();
        let conversation_repository = Arc::new(FakeConversationRepository {
            conversation: build_conversation(
                user_id,
                conversation_id,
                phone_number_id,
                Some("+14155551234".to_owned()),
            ),
            updated_conversation: Mutex::new(None),
        });
        let message_repository = Arc::new(FakeMessageRepository {
            created_messages: Mutex::new(Vec::new()),
        });
        let phone_number_repository = Arc::new(FakePhoneNumberRepository {
            phone_number: build_phone_number(user_id, phone_number_id),
        });
        let outbound_message_service = Arc::new(FakeOutboundMessageService {
            response: Ok(SendMessageResponse {
                provider_message_id: "provider-message-id".to_owned(),
            }),
            requests: Mutex::new(Vec::new()),
        });

        let usecase = CreateMessageUsecase::builder()
            .conversation_repository(conversation_repository.clone())
            .message_repository(message_repository.clone())
            .phone_number_repository(phone_number_repository)
            .outbound_message_service(outbound_message_service.clone())
            .build();

        let result = usecase
            .execute(CreateMessageCommand {
                user_id,
                conversation_id,
                content: " Hello ".to_owned(),
            })
            .await
            .expect("message should be created");

        assert_eq!(result.message.conversation_id, conversation_id);
        assert_eq!(result.message.user_id, user_id);
        assert_eq!(result.message.message_type, MessageType::Outbound);
        assert_eq!(result.message.status, MessageStatus::Queued);
        assert_eq!(
            result.message.provider_message_id.as_deref(),
            Some("provider-message-id")
        );
        assert_eq!(result.message.provider_status.as_deref(), Some("queued"));
        assert!(result.message.provider_status_updated_at.is_none());
        assert_eq!(result.message.from_number, "+13125550100");
        assert_eq!(result.message.content, "Hello");

        let requests = outbound_message_service.requests.lock().expect("lock");
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].from, "+13125550100");
        assert_eq!(requests[0].to, "+14155551234");
        assert_eq!(requests[0].text, "Hello");

        let updated_conversation = conversation_repository
            .updated_conversation
            .lock()
            .expect("lock")
            .clone()
            .expect("conversation should be updated");
        assert_eq!(updated_conversation.id, conversation_id);
        assert!(updated_conversation.updated_at >= updated_conversation.created_at);
    }

    #[tokio::test]
    async fn returns_validation_error_when_recipient_is_missing() {
        let user_id = uuid::Uuid::now_v7();
        let conversation_id = uuid::Uuid::now_v7();
        let phone_number_id = uuid::Uuid::now_v7();
        let usecase = CreateMessageUsecase::builder()
            .conversation_repository(Arc::new(FakeConversationRepository {
                conversation: build_conversation(user_id, conversation_id, phone_number_id, None),
                updated_conversation: Mutex::new(None),
            }))
            .message_repository(Arc::new(FakeMessageRepository {
                created_messages: Mutex::new(Vec::new()),
            }))
            .phone_number_repository(Arc::new(FakePhoneNumberRepository {
                phone_number: build_phone_number(user_id, phone_number_id),
            }))
            .outbound_message_service(Arc::new(FakeOutboundMessageService {
                response: Ok(SendMessageResponse {
                    provider_message_id: "provider-message-id".to_owned(),
                }),
                requests: Mutex::new(Vec::new()),
            }))
            .build();

        let err = usecase
            .execute(CreateMessageCommand {
                user_id,
                conversation_id,
                content: "Hello".to_owned(),
            })
            .await
            .expect_err("missing recipient should fail");

        assert!(matches!(err, UsecaseError::Validation(_)));
    }
}
