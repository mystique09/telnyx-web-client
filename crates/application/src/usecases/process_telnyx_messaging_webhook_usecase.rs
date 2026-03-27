use std::{cmp::max, sync::Arc};

use time::OffsetDateTime;

use crate::{
    commands::{
        ProcessTelnyxWebhookCommand, TelnyxWebhookMessageError, TelnyxWebhookMessageParticipant,
    },
    usecases::UsecaseError,
};
use domain::{
    models::{
        conversation::Conversation,
        message::{Message, MessageStatus, MessageType},
        processed_webhook_event::ProcessedWebhookEvent,
    },
    repositories::{
        RepositoryError, conversation_repository::ConversationRepository,
        message_repository::MessageRepository, phone_number_repository::PhoneNumberRepository,
        processed_webhook_event_repository::ProcessedWebhookEventRepository,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessagingWebhookNotificationKind {
    MessageCreated,
    MessageUpdated,
}

#[derive(Debug, Clone)]
pub struct MessagingWebhookNotification {
    pub user_id: uuid::Uuid,
    pub kind: MessagingWebhookNotificationKind,
    pub message: Message,
    pub conversation: Conversation,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessTelnyxMessagingWebhookResult {
    pub notification: Option<MessagingWebhookNotification>,
}

#[derive(bon::Builder)]
pub struct ProcessTelnyxMessagingWebhookUsecase {
    conversation_repository: Arc<dyn ConversationRepository>,
    message_repository: Arc<dyn MessageRepository>,
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
    processed_webhook_event_repository: Arc<dyn ProcessedWebhookEventRepository>,
}

impl ProcessTelnyxMessagingWebhookUsecase {
    pub async fn execute(
        &self,
        cmd: ProcessTelnyxWebhookCommand,
    ) -> Result<ProcessTelnyxMessagingWebhookResult, UsecaseError> {
        match self
            .processed_webhook_event_repository
            .find_by_event_id(&cmd.event_id)
            .await
        {
            Ok(_) => return Ok(ProcessTelnyxMessagingWebhookResult::default()),
            Err(RepositoryError::NotFound) => {}
            Err(err) => return Err(err.into()),
        }

        let notification = match cmd.event_type.as_str() {
            "message.sent" => self.handle_message_sent(&cmd).await?,
            "message.finalized" => self.handle_message_finalized(&cmd).await?,
            "message.received" => self.handle_message_received(&cmd).await?,
            _ => None,
        };

        let processed_event = ProcessedWebhookEvent::builder()
            .event_id(cmd.event_id)
            .event_type(cmd.event_type)
            .maybe_provider_message_id(Some(cmd.payload.provider_message_id))
            .occurred_at(cmd.occurred_at)
            .payload_json(cmd.raw_payload)
            .created_at(OffsetDateTime::now_utc())
            .build();

        match self
            .processed_webhook_event_repository
            .create_processed_webhook_event(&processed_event)
            .await
        {
            Ok(()) => {}
            Err(RepositoryError::ConstraintViolation(_)) => {
                return Ok(ProcessTelnyxMessagingWebhookResult::default());
            }
            Err(err) => return Err(err.into()),
        }

        Ok(ProcessTelnyxMessagingWebhookResult { notification })
    }

    async fn handle_message_sent(
        &self,
        cmd: &ProcessTelnyxWebhookCommand,
    ) -> Result<Option<MessagingWebhookNotification>, UsecaseError> {
        let Some(mut message) = self
            .find_message_by_provider_id(&cmd.payload.provider_message_id)
            .await?
        else {
            return Ok(None);
        };

        if is_stale_update(&message, cmd.occurred_at) {
            return Ok(None);
        }

        let Some(mut conversation) = self.find_conversation_for_message(&message).await? else {
            return Ok(None);
        };

        message.status = MessageStatus::Sent;
        message.provider_status = Some("sent".to_owned());
        message.provider_status_updated_at = Some(cmd.occurred_at);
        message.provider_error_code = None;
        message.provider_error_detail = None;
        message.updated_at = max(message.updated_at, cmd.occurred_at);

        let message = self.message_repository.update_message(&message).await?;

        conversation.updated_at = max(conversation.updated_at, message.updated_at);
        self.conversation_repository
            .update_conversation(&conversation)
            .await?;

        Ok(Some(MessagingWebhookNotification {
            user_id: message.user_id,
            kind: MessagingWebhookNotificationKind::MessageUpdated,
            message,
            conversation,
        }))
    }

    async fn handle_message_finalized(
        &self,
        cmd: &ProcessTelnyxWebhookCommand,
    ) -> Result<Option<MessagingWebhookNotification>, UsecaseError> {
        let Some(mut message) = self
            .find_message_by_provider_id(&cmd.payload.provider_message_id)
            .await?
        else {
            return Ok(None);
        };

        if is_stale_update(&message, cmd.occurred_at) {
            return Ok(None);
        }

        let Some(mut conversation) = self.find_conversation_for_message(&message).await? else {
            return Ok(None);
        };

        let provider_status = first_status(&cmd.payload.to).unwrap_or("failed").to_owned();
        let (provider_error_code, provider_error_detail) = first_error_details(&cmd.payload.errors);

        message.status = map_finalized_status(&provider_status);
        message.provider_status = Some(provider_status);
        message.provider_status_updated_at = Some(cmd.occurred_at);
        message.provider_error_code = provider_error_code;
        message.provider_error_detail = provider_error_detail;
        message.updated_at = max(message.updated_at, cmd.occurred_at);

        let message = self.message_repository.update_message(&message).await?;

        conversation.updated_at = max(conversation.updated_at, message.updated_at);
        self.conversation_repository
            .update_conversation(&conversation)
            .await?;

        Ok(Some(MessagingWebhookNotification {
            user_id: message.user_id,
            kind: MessagingWebhookNotificationKind::MessageUpdated,
            message,
            conversation,
        }))
    }

    async fn handle_message_received(
        &self,
        cmd: &ProcessTelnyxWebhookCommand,
    ) -> Result<Option<MessagingWebhookNotification>, UsecaseError> {
        match self
            .message_repository
            .find_by_provider_message_id(&cmd.payload.provider_message_id)
            .await
        {
            Ok(_) => return Ok(None),
            Err(RepositoryError::NotFound) => {}
            Err(err) => return Err(err.into()),
        }

        let recipient_phone_number = first_phone_number(&cmd.payload.to).ok_or_else(|| {
            garde::Error::new("Webhook payload is missing the destination phone number")
        })?;
        let sender_phone_number = cmd
            .payload
            .from_phone_number
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| garde::Error::new("Webhook payload is missing the source phone number"))?
            .to_owned();

        let phone_number = match self
            .phone_number_repository
            .find_by_phone(recipient_phone_number)
            .await
        {
            Ok(phone_number) => phone_number,
            Err(RepositoryError::NotFound) => return Ok(None),
            Err(err) => return Err(err.into()),
        };

        let message_created_at = cmd.payload.received_at.unwrap_or(cmd.occurred_at);
        let mut conversation = match self
            .conversation_repository
            .find_by_phone_number_and_recipient(
                &phone_number.user_id,
                &phone_number.id,
                &sender_phone_number,
            )
            .await
        {
            Ok(conversation) => conversation,
            Err(RepositoryError::NotFound) => {
                let conversation = Conversation::builder()
                    .id(uuid::Uuid::now_v7())
                    .phone_number_id(phone_number.id)
                    .user_id(phone_number.user_id)
                    .maybe_recipient_phone_number(Some(sender_phone_number.clone()))
                    .last_message_at(message_created_at)
                    .created_at(message_created_at)
                    .updated_at(message_created_at)
                    .build();

                self.conversation_repository
                    .create_conversation(&conversation)
                    .await?;
                conversation
            }
            Err(err) => return Err(err.into()),
        };

        let provider_status = first_status(&cmd.payload.to)
            .unwrap_or("received")
            .to_owned();
        let (provider_error_code, provider_error_detail) = first_error_details(&cmd.payload.errors);
        let message = Message::builder()
            .id(uuid::Uuid::now_v7())
            .conversation_id(conversation.id)
            .user_id(phone_number.user_id)
            .message_type(MessageType::Inbound)
            .status(MessageStatus::Delivered)
            .maybe_provider_message_id(Some(cmd.payload.provider_message_id.clone()))
            .maybe_provider_status(Some(provider_status))
            .maybe_provider_status_updated_at(Some(cmd.occurred_at))
            .maybe_provider_error_code(provider_error_code)
            .maybe_provider_error_detail(provider_error_detail)
            .from_number(sender_phone_number)
            .content(cmd.payload.text.clone().unwrap_or_default())
            .created_at(message_created_at)
            .updated_at(max(message_created_at, cmd.occurred_at))
            .build();

        let message = match self.message_repository.create_message(&message).await {
            Ok(message) => message,
            Err(RepositoryError::ConstraintViolation(_)) => return Ok(None),
            Err(err) => return Err(err.into()),
        };

        conversation.last_message_at = message.created_at;
        conversation.updated_at = max(conversation.updated_at, message.updated_at);
        self.conversation_repository
            .update_conversation(&conversation)
            .await?;

        Ok(Some(MessagingWebhookNotification {
            user_id: message.user_id,
            kind: MessagingWebhookNotificationKind::MessageCreated,
            message,
            conversation,
        }))
    }

    async fn find_conversation_for_message(
        &self,
        message: &Message,
    ) -> Result<Option<Conversation>, UsecaseError> {
        match self
            .conversation_repository
            .find_by_id(&message.user_id, &message.conversation_id)
            .await
        {
            Ok(conversation) => Ok(Some(conversation)),
            Err(RepositoryError::NotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    async fn find_message_by_provider_id(
        &self,
        provider_message_id: &str,
    ) -> Result<Option<Message>, UsecaseError> {
        match self
            .message_repository
            .find_by_provider_message_id(provider_message_id)
            .await
        {
            Ok(message) => Ok(Some(message)),
            Err(RepositoryError::NotFound) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

fn first_phone_number(participants: &[TelnyxWebhookMessageParticipant]) -> Option<&str> {
    participants
        .iter()
        .find_map(|participant| participant.phone_number.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn first_status(participants: &[TelnyxWebhookMessageParticipant]) -> Option<&str> {
    participants
        .iter()
        .find_map(|participant| participant.status.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn first_error_details(errors: &[TelnyxWebhookMessageError]) -> (Option<String>, Option<String>) {
    let Some(error) = errors.first() else {
        return (None, None);
    };

    (
        error.code.clone(),
        error.detail.clone().or_else(|| error.title.clone()),
    )
}

fn is_stale_update(message: &Message, occurred_at: OffsetDateTime) -> bool {
    message
        .provider_status_updated_at
        .map(|updated_at| occurred_at < updated_at)
        .unwrap_or(false)
}

fn map_finalized_status(status: &str) -> MessageStatus {
    match status {
        "delivered" => MessageStatus::Delivered,
        "delivery_failed" | "sending_failed" | "failed" | "gw_timeout" | "dlr_timeout" => {
            MessageStatus::Failed
        }
        "queued" => MessageStatus::Queued,
        "sending" | "sent" | "delivery_unconfirmed" => MessageStatus::Sent,
        _ => MessageStatus::Sent,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use async_trait::async_trait;
    use domain::{
        models::{
            conversation::Conversation,
            message::{Message, MessageStatus, MessageType},
            phone_number::PhoneNumber,
            processed_webhook_event::ProcessedWebhookEvent,
        },
        repositories::{
            RepositoryError, conversation_repository::ConversationRepository,
            message_repository::{MessagePage, MessageRepository},
            phone_number_repository::PhoneNumberRepository,
            processed_webhook_event_repository::ProcessedWebhookEventRepository,
        },
    };
    use serde_json::json;
    use time::OffsetDateTime;

    use crate::commands::{
        ProcessTelnyxWebhookCommand, TelnyxWebhookMessageError, TelnyxWebhookMessageParticipant,
        TelnyxWebhookMessagePayload,
    };

    use super::{
        MessagingWebhookNotificationKind, ProcessTelnyxMessagingWebhookUsecase,
        map_finalized_status,
    };

    struct FakeConversationRepository {
        conversations: Mutex<HashMap<uuid::Uuid, Conversation>>,
    }

    #[async_trait]
    impl ConversationRepository for FakeConversationRepository {
        async fn create_conversation(
            &self,
            conversation: &Conversation,
        ) -> Result<(), RepositoryError> {
            self.conversations
                .lock()
                .expect("lock")
                .insert(conversation.id, conversation.clone());
            Ok(())
        }

        async fn update_conversation(
            &self,
            conversation: &Conversation,
        ) -> Result<(), RepositoryError> {
            self.conversations
                .lock()
                .expect("lock")
                .insert(conversation.id, conversation.clone());
            Ok(())
        }

        async fn find_by_id(
            &self,
            _user_id: &uuid::Uuid,
            id: &uuid::Uuid,
        ) -> Result<Conversation, RepositoryError> {
            self.conversations
                .lock()
                .expect("lock")
                .get(id)
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }

        async fn find_by_phone_number_and_recipient(
            &self,
            _user_id: &uuid::Uuid,
            phone_number_id: &uuid::Uuid,
            recipient_phone_number: &str,
        ) -> Result<Conversation, RepositoryError> {
            self.conversations
                .lock()
                .expect("lock")
                .values()
                .find(|conversation| {
                    conversation.phone_number_id == *phone_number_id
                        && conversation.recipient_phone_number.as_deref()
                            == Some(recipient_phone_number)
                })
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }

        async fn list_by_user_id(
            &self,
            user_id: &uuid::Uuid,
        ) -> Result<Vec<Conversation>, RepositoryError> {
            Ok(self
                .conversations
                .lock()
                .expect("lock")
                .values()
                .filter(|conversation| conversation.user_id == *user_id)
                .cloned()
                .collect())
        }

        async fn delete_conversation(
            &self,
            _user_id: &uuid::Uuid,
            id: &uuid::Uuid,
        ) -> Result<(), RepositoryError> {
            self.conversations.lock().expect("lock").remove(id);
            Ok(())
        }
    }

    struct FakeMessageRepository {
        messages: Mutex<HashMap<uuid::Uuid, Message>>,
    }

    #[async_trait]
    impl MessageRepository for FakeMessageRepository {
        async fn create_message(&self, message: &Message) -> Result<Message, RepositoryError> {
            let mut messages = self.messages.lock().expect("lock");
            if let Some(provider_message_id) = message.provider_message_id.as_deref() {
                if messages.values().any(|existing| {
                    existing.provider_message_id.as_deref() == Some(provider_message_id)
                }) {
                    return Err(RepositoryError::ConstraintViolation(
                        "duplicate provider_message_id".to_owned(),
                    ));
                }
            }

            messages.insert(message.id, message.clone());
            Ok(message.clone())
        }

        async fn count_by_user_id(&self, user_id: &uuid::Uuid) -> Result<u64, RepositoryError> {
            Ok(self
                .messages
                .lock()
                .expect("lock")
                .values()
                .filter(|message| message.user_id == *user_id)
                .count() as u64)
        }

        async fn find_by_provider_message_id(
            &self,
            provider_message_id: &str,
        ) -> Result<Message, RepositoryError> {
            self.messages
                .lock()
                .expect("lock")
                .values()
                .find(|message| message.provider_message_id.as_deref() == Some(provider_message_id))
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }

        async fn list_by_conversation_id(
            &self,
            _user_id: &uuid::Uuid,
            conversation_id: &uuid::Uuid,
        ) -> Result<Vec<Message>, RepositoryError> {
            Ok(self
                .messages
                .lock()
                .expect("lock")
                .values()
                .filter(|message| message.conversation_id == *conversation_id)
                .cloned()
                .collect())
        }

        async fn list_page_by_conversation_id(
            &self,
            _user_id: &uuid::Uuid,
            conversation_id: &uuid::Uuid,
            cursor: Option<&uuid::Uuid>,
            limit: usize,
        ) -> Result<MessagePage, RepositoryError> {
            let mut messages = self
                .messages
                .lock()
                .expect("lock")
                .values()
                .filter(|message| message.conversation_id == *conversation_id)
                .cloned()
                .collect::<Vec<_>>();
            messages.sort_by(|a, b| {
                b.created_at
                    .cmp(&a.created_at)
                    .then_with(|| b.id.cmp(&a.id))
            });

            let start = if let Some(cursor_id) = cursor {
                messages
                    .iter()
                    .position(|message| message.id == *cursor_id)
                    .map(|index| index + 1)
                    .ok_or(RepositoryError::NotFound)?
            } else {
                0
            };
            let page = messages.into_iter().skip(start).take(limit + 1).collect::<Vec<_>>();
            let has_more = page.len() > limit;
            let page = if has_more {
                page.into_iter().take(limit).collect::<Vec<_>>()
            } else {
                page
            };
            let next_cursor = if has_more {
                page.last().map(|message| message.id)
            } else {
                None
            };
            let mut ordered = page;
            ordered.sort_by(|a, b| {
                a.created_at
                    .cmp(&b.created_at)
                    .then_with(|| a.id.cmp(&b.id))
            });

            Ok(MessagePage {
                messages: ordered,
                next_cursor,
            })
        }

        async fn update_message(&self, message: &Message) -> Result<Message, RepositoryError> {
            self.messages
                .lock()
                .expect("lock")
                .insert(message.id, message.clone());
            Ok(message.clone())
        }
    }

    struct FakePhoneNumberRepository {
        phone_numbers: Mutex<HashMap<uuid::Uuid, PhoneNumber>>,
    }

    #[async_trait]
    impl PhoneNumberRepository for FakePhoneNumberRepository {
        async fn create_phone_number(
            &self,
            phone_number: &PhoneNumber,
        ) -> Result<(), RepositoryError> {
            self.phone_numbers
                .lock()
                .expect("lock")
                .insert(phone_number.id, phone_number.clone());
            Ok(())
        }

        async fn find_by_id(
            &self,
            _user_id: &uuid::Uuid,
            id: &uuid::Uuid,
        ) -> Result<PhoneNumber, RepositoryError> {
            self.phone_numbers
                .lock()
                .expect("lock")
                .get(id)
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }

        async fn find_by_phone(&self, phone: &str) -> Result<PhoneNumber, RepositoryError> {
            self.phone_numbers
                .lock()
                .expect("lock")
                .values()
                .find(|phone_number| phone_number.phone == phone)
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }

        async fn list_by_user_id(
            &self,
            user_id: &uuid::Uuid,
        ) -> Result<Vec<PhoneNumber>, RepositoryError> {
            Ok(self
                .phone_numbers
                .lock()
                .expect("lock")
                .values()
                .filter(|phone_number| phone_number.user_id == *user_id)
                .cloned()
                .collect())
        }

        async fn delete_phone_number(
            &self,
            _user_id: &uuid::Uuid,
            id: &uuid::Uuid,
        ) -> Result<(), RepositoryError> {
            self.phone_numbers.lock().expect("lock").remove(id);
            Ok(())
        }
    }

    struct FakeProcessedWebhookEventRepository {
        events: Mutex<HashMap<String, ProcessedWebhookEvent>>,
    }

    #[async_trait]
    impl ProcessedWebhookEventRepository for FakeProcessedWebhookEventRepository {
        async fn create_processed_webhook_event(
            &self,
            event: &ProcessedWebhookEvent,
        ) -> Result<(), RepositoryError> {
            let mut events = self.events.lock().expect("lock");
            if events.contains_key(&event.event_id) {
                return Err(RepositoryError::ConstraintViolation(
                    "duplicate webhook event".to_owned(),
                ));
            }

            events.insert(event.event_id.clone(), event.clone());
            Ok(())
        }

        async fn find_by_event_id(
            &self,
            event_id: &str,
        ) -> Result<ProcessedWebhookEvent, RepositoryError> {
            self.events
                .lock()
                .expect("lock")
                .get(event_id)
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }
    }

    fn build_conversation(
        user_id: uuid::Uuid,
        phone_number_id: uuid::Uuid,
        recipient_phone_number: &str,
    ) -> Conversation {
        let now = OffsetDateTime::now_utc();
        Conversation::builder()
            .id(uuid::Uuid::now_v7())
            .phone_number_id(phone_number_id)
            .user_id(user_id)
            .maybe_recipient_phone_number(Some(recipient_phone_number.to_owned()))
            .last_message_at(now)
            .created_at(now)
            .updated_at(now)
            .build()
    }

    fn build_phone_number(user_id: uuid::Uuid, phone: &str) -> PhoneNumber {
        let now = OffsetDateTime::now_utc();
        PhoneNumber::builder()
            .id(uuid::Uuid::now_v7())
            .user_id(user_id)
            .name("Primary".to_owned())
            .phone(phone.to_owned())
            .created_at(now)
            .updated_at(now)
            .build()
    }

    fn build_message(
        user_id: uuid::Uuid,
        conversation_id: uuid::Uuid,
        provider_message_id: &str,
    ) -> Message {
        let now = OffsetDateTime::now_utc();
        Message::builder()
            .id(uuid::Uuid::now_v7())
            .conversation_id(conversation_id)
            .user_id(user_id)
            .message_type(MessageType::Outbound)
            .status(MessageStatus::Queued)
            .maybe_provider_message_id(Some(provider_message_id.to_owned()))
            .maybe_provider_status(Some("queued".to_owned()))
            .maybe_provider_status_updated_at(Some(now))
            .maybe_provider_error_code(None)
            .maybe_provider_error_detail(None)
            .from_number("+13125550100".to_owned())
            .content("Hello".to_owned())
            .created_at(now)
            .updated_at(now)
            .build()
    }

    fn build_payload(
        provider_message_id: &str,
        to_phone: &str,
        status: &str,
    ) -> TelnyxWebhookMessagePayload {
        TelnyxWebhookMessagePayload {
            provider_message_id: provider_message_id.to_owned(),
            from_phone_number: Some("+14155551234".to_owned()),
            to: vec![TelnyxWebhookMessageParticipant {
                phone_number: Some(to_phone.to_owned()),
                status: Some(status.to_owned()),
            }],
            text: Some("Inbound hello".to_owned()),
            received_at: Some(OffsetDateTime::now_utc()),
            sent_at: None,
            completed_at: None,
            errors: Vec::new(),
        }
    }

    fn build_usecase(
        conversation_repository: Arc<FakeConversationRepository>,
        message_repository: Arc<FakeMessageRepository>,
        phone_number_repository: Arc<FakePhoneNumberRepository>,
        processed_webhook_event_repository: Arc<FakeProcessedWebhookEventRepository>,
    ) -> ProcessTelnyxMessagingWebhookUsecase {
        ProcessTelnyxMessagingWebhookUsecase::builder()
            .conversation_repository(conversation_repository)
            .message_repository(message_repository)
            .phone_number_repository(phone_number_repository)
            .processed_webhook_event_repository(processed_webhook_event_repository)
            .build()
    }

    #[tokio::test]
    async fn message_sent_updates_outbound_message_and_records_event() {
        let user_id = uuid::Uuid::now_v7();
        let phone_number = build_phone_number(user_id, "+13125550100");
        let conversation = build_conversation(user_id, phone_number.id, "+14155551234");
        let mut message = build_message(user_id, conversation.id, "provider-message-id");
        message.provider_status_updated_at = None;

        let conversation_repository = Arc::new(FakeConversationRepository {
            conversations: Mutex::new(HashMap::from([(conversation.id, conversation.clone())])),
        });
        let message_repository = Arc::new(FakeMessageRepository {
            messages: Mutex::new(HashMap::from([(message.id, message.clone())])),
        });
        let phone_number_repository = Arc::new(FakePhoneNumberRepository {
            phone_numbers: Mutex::new(HashMap::from([(phone_number.id, phone_number)])),
        });
        let processed_event_repository = Arc::new(FakeProcessedWebhookEventRepository {
            events: Mutex::new(HashMap::new()),
        });
        let usecase = build_usecase(
            conversation_repository,
            message_repository.clone(),
            phone_number_repository,
            processed_event_repository.clone(),
        );

        let result = usecase
            .execute(ProcessTelnyxWebhookCommand {
                event_id: "event-1".to_owned(),
                event_type: "message.sent".to_owned(),
                occurred_at: OffsetDateTime::now_utc(),
                payload: build_payload("provider-message-id", "+14155551234", "sent"),
                raw_payload: json!({ "data": {} }),
            })
            .await
            .expect("webhook should be processed");

        let notification = result.notification.expect("notification should exist");
        assert_eq!(
            notification.kind,
            MessagingWebhookNotificationKind::MessageUpdated
        );
        assert_eq!(notification.message.status, MessageStatus::Sent);
        assert_eq!(
            notification.message.provider_status.as_deref(),
            Some("sent")
        );

        let stored_message = message_repository
            .find_by_provider_message_id("provider-message-id")
            .await
            .expect("message should still exist");
        assert_eq!(stored_message.status, MessageStatus::Sent);

        processed_event_repository
            .find_by_event_id("event-1")
            .await
            .expect("processed event should be stored");
    }

    #[tokio::test]
    async fn message_finalized_failure_updates_error_details() {
        let user_id = uuid::Uuid::now_v7();
        let phone_number = build_phone_number(user_id, "+13125550100");
        let conversation = build_conversation(user_id, phone_number.id, "+14155551234");
        let mut message = build_message(user_id, conversation.id, "provider-message-id");
        message.provider_status_updated_at = None;

        let conversation_repository = Arc::new(FakeConversationRepository {
            conversations: Mutex::new(HashMap::from([(conversation.id, conversation)])),
        });
        let message_repository = Arc::new(FakeMessageRepository {
            messages: Mutex::new(HashMap::from([(message.id, message)])),
        });
        let phone_number_repository = Arc::new(FakePhoneNumberRepository {
            phone_numbers: Mutex::new(HashMap::from([(phone_number.id, phone_number)])),
        });
        let processed_event_repository = Arc::new(FakeProcessedWebhookEventRepository {
            events: Mutex::new(HashMap::new()),
        });
        let usecase = build_usecase(
            conversation_repository,
            message_repository,
            phone_number_repository,
            processed_event_repository,
        );

        let result = usecase
            .execute(ProcessTelnyxWebhookCommand {
                event_id: "event-2".to_owned(),
                event_type: "message.finalized".to_owned(),
                occurred_at: OffsetDateTime::now_utc(),
                payload: TelnyxWebhookMessagePayload {
                    provider_message_id: "provider-message-id".to_owned(),
                    from_phone_number: Some("+13125550100".to_owned()),
                    to: vec![TelnyxWebhookMessageParticipant {
                        phone_number: Some("+14155551234".to_owned()),
                        status: Some("delivery_failed".to_owned()),
                    }],
                    text: Some("Hello".to_owned()),
                    received_at: None,
                    sent_at: None,
                    completed_at: Some(OffsetDateTime::now_utc()),
                    errors: vec![TelnyxWebhookMessageError {
                        code: Some("40300".to_owned()),
                        detail: Some("Destination unreachable".to_owned()),
                        title: None,
                    }],
                },
                raw_payload: json!({ "data": {} }),
            })
            .await
            .expect("webhook should be processed");

        let notification = result.notification.expect("notification should exist");
        assert_eq!(notification.message.status, MessageStatus::Failed);
        assert_eq!(
            notification.message.provider_error_code.as_deref(),
            Some("40300")
        );
        assert_eq!(
            notification.message.provider_error_detail.as_deref(),
            Some("Destination unreachable")
        );
    }

    #[tokio::test]
    async fn stale_status_update_is_ignored() {
        let user_id = uuid::Uuid::now_v7();
        let phone_number = build_phone_number(user_id, "+13125550100");
        let conversation = build_conversation(user_id, phone_number.id, "+14155551234");
        let mut message = build_message(user_id, conversation.id, "provider-message-id");
        let now = OffsetDateTime::now_utc();
        message.provider_status_updated_at = Some(now);

        let conversation_repository = Arc::new(FakeConversationRepository {
            conversations: Mutex::new(HashMap::from([(conversation.id, conversation)])),
        });
        let message_repository = Arc::new(FakeMessageRepository {
            messages: Mutex::new(HashMap::from([(message.id, message.clone())])),
        });
        let phone_number_repository = Arc::new(FakePhoneNumberRepository {
            phone_numbers: Mutex::new(HashMap::from([(phone_number.id, phone_number)])),
        });
        let processed_event_repository = Arc::new(FakeProcessedWebhookEventRepository {
            events: Mutex::new(HashMap::new()),
        });
        let usecase = build_usecase(
            conversation_repository,
            message_repository.clone(),
            phone_number_repository,
            processed_event_repository.clone(),
        );

        let result = usecase
            .execute(ProcessTelnyxWebhookCommand {
                event_id: "event-3".to_owned(),
                event_type: "message.sent".to_owned(),
                occurred_at: now - time::Duration::seconds(5),
                payload: build_payload("provider-message-id", "+14155551234", "sent"),
                raw_payload: json!({ "data": {} }),
            })
            .await
            .expect("webhook should be processed");

        assert!(result.notification.is_none());
        let stored_message = message_repository
            .find_by_provider_message_id("provider-message-id")
            .await
            .expect("message should still exist");
        assert_eq!(stored_message.status, message.status);

        processed_event_repository
            .find_by_event_id("event-3")
            .await
            .expect("stale event should still be recorded");
    }

    #[tokio::test]
    async fn message_received_auto_creates_conversation_and_inbound_message() {
        let user_id = uuid::Uuid::now_v7();
        let phone_number = build_phone_number(user_id, "+17735550002");

        let conversation_repository = Arc::new(FakeConversationRepository {
            conversations: Mutex::new(HashMap::new()),
        });
        let message_repository = Arc::new(FakeMessageRepository {
            messages: Mutex::new(HashMap::new()),
        });
        let phone_number_repository = Arc::new(FakePhoneNumberRepository {
            phone_numbers: Mutex::new(HashMap::from([(phone_number.id, phone_number.clone())])),
        });
        let processed_event_repository = Arc::new(FakeProcessedWebhookEventRepository {
            events: Mutex::new(HashMap::new()),
        });
        let usecase = build_usecase(
            conversation_repository.clone(),
            message_repository.clone(),
            phone_number_repository,
            processed_event_repository,
        );

        let result = usecase
            .execute(ProcessTelnyxWebhookCommand {
                event_id: "event-4".to_owned(),
                event_type: "message.received".to_owned(),
                occurred_at: OffsetDateTime::now_utc(),
                payload: TelnyxWebhookMessagePayload {
                    provider_message_id: "inbound-provider-message-id".to_owned(),
                    from_phone_number: Some("+13125550001".to_owned()),
                    to: vec![TelnyxWebhookMessageParticipant {
                        phone_number: Some(phone_number.phone.clone()),
                        status: Some("webhook_delivered".to_owned()),
                    }],
                    text: Some("Hello from Telnyx!".to_owned()),
                    received_at: Some(OffsetDateTime::now_utc()),
                    sent_at: None,
                    completed_at: None,
                    errors: Vec::new(),
                },
                raw_payload: json!({ "data": {} }),
            })
            .await
            .expect("webhook should be processed");

        let notification = result.notification.expect("notification should exist");
        assert_eq!(
            notification.kind,
            MessagingWebhookNotificationKind::MessageCreated
        );
        assert_eq!(notification.message.message_type, MessageType::Inbound);
        assert_eq!(notification.message.status, MessageStatus::Delivered);
        assert_eq!(
            notification.message.provider_status.as_deref(),
            Some("webhook_delivered")
        );
        assert_eq!(
            notification.conversation.recipient_phone_number.as_deref(),
            Some("+13125550001")
        );

        let conversations = conversation_repository
            .list_by_user_id(&user_id)
            .await
            .expect("conversations should load");
        assert_eq!(conversations.len(), 1);

        let messages = message_repository
            .list_by_conversation_id(&user_id, &notification.conversation.id)
            .await
            .expect("messages should load");
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn finalized_status_mapping_matches_expected_outcomes() {
        assert_eq!(map_finalized_status("delivered"), MessageStatus::Delivered);
        assert_eq!(
            map_finalized_status("delivery_failed"),
            MessageStatus::Failed
        );
        assert_eq!(
            map_finalized_status("delivery_unconfirmed"),
            MessageStatus::Sent
        );
    }
}
