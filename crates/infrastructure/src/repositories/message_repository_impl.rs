use std::sync::Arc;

use domain::models::message::Message;
use domain::repositories::RepositoryError;
use domain::repositories::message_repository::MessageRepository;

use rbatis::{RBatis, async_trait};
use rbs::value;

use crate::database;
use crate::database::models::UuidExt;
use crate::repositories::RbsErrorExt;

#[derive(Debug, bon::Builder)]
pub struct MessageRepositoryImpl {
    pool: Arc<RBatis>,
}

#[async_trait]
impl MessageRepository for MessageRepositoryImpl {
    async fn create_message(&self, message: &Message) -> Result<Message, RepositoryError> {
        let new_message_db = database::models::message::Message::from(message);
        let created_message = Message::from(&new_message_db);
        let sql = r#"
            INSERT INTO messages (
                id,
                conversation_id,
                user_id,
                message_type,
                status,
                provider_message_id,
                provider_status,
                provider_status_updated_at,
                provider_error_code,
                provider_error_detail,
                from_number,
                content,
                created_at,
                updated_at
            )
            VALUES (
                ?,
                ?,
                ?,
                CAST(? AS message_type),
                CAST(? AS message_status),
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
            )
        "#;

        self.pool
            .as_ref()
            .exec(
                sql,
                vec![
                    value!(new_message_db.id.clone()),
                    value!(new_message_db.conversation_id.clone()),
                    value!(new_message_db.user_id.clone()),
                    value!(new_message_db.message_type.clone()),
                    value!(new_message_db.status.clone()),
                    value!(new_message_db.provider_message_id.clone()),
                    value!(new_message_db.provider_status.clone()),
                    value!(new_message_db.provider_status_updated_at.clone()),
                    value!(new_message_db.provider_error_code.clone()),
                    value!(new_message_db.provider_error_detail.clone()),
                    value!(new_message_db.from_number.clone()),
                    value!(new_message_db.content.clone()),
                    value!(new_message_db.created_at.clone()),
                    value!(new_message_db.updated_at.clone()),
                ],
            )
            .await
            .map_err(|e| e.to_repository_error())?;

        Ok(created_message)
    }

    async fn count_by_user_id(&self, user_id: &uuid::Uuid) -> Result<u64, RepositoryError> {
        let user_id_db = user_id.into_db();
        let records = database::models::message::Message::select_by_map(
            self.pool.as_ref(),
            value! { "user_id": user_id_db },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        Ok(records.len() as u64)
    }

    async fn find_by_provider_message_id(
        &self,
        provider_message_id: &str,
    ) -> Result<Message, RepositoryError> {
        let record = database::models::message::Message::select_by_map(
            self.pool.as_ref(),
            value! { "provider_message_id": provider_message_id },
        )
        .await
        .map_err(|e| e.to_repository_error())?
        .into_iter()
        .next()
        .ok_or(RepositoryError::NotFound)?;

        Ok(Message::from(&record))
    }

    async fn list_by_conversation_id(
        &self,
        user_id: &uuid::Uuid,
        conversation_id: &uuid::Uuid,
    ) -> Result<Vec<Message>, RepositoryError> {
        let user_id_db = user_id.into_db();
        let conversation_id_db = conversation_id.into_db();
        let records = database::models::message::Message::select_by_map(
            self.pool.as_ref(),
            value! { "conversation_id": conversation_id_db, "user_id": user_id_db },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        let mut messages = records.iter().map(Message::from).collect::<Vec<_>>();
        messages.sort_by(|a, b| {
            a.created_at
                .cmp(&b.created_at)
                .then_with(|| a.id.cmp(&b.id))
        });

        Ok(messages)
    }

    async fn update_message(&self, message: &Message) -> Result<Message, RepositoryError> {
        let existing = database::models::message::Message::select_by_map(
            self.pool.as_ref(),
            value! { "id": message.id.into_db(), "user_id": message.user_id.into_db() },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        if existing.is_empty() {
            return Err(RepositoryError::NotFound);
        }

        let updated_message_db = database::models::message::Message::from(message);
        let sql = r#"
            UPDATE messages
            SET
                conversation_id = ?,
                user_id = ?,
                message_type = CAST(? AS message_type),
                status = CAST(? AS message_status),
                provider_message_id = ?,
                provider_status = ?,
                provider_status_updated_at = ?,
                provider_error_code = ?,
                provider_error_detail = ?,
                from_number = ?,
                content = ?,
                created_at = ?,
                updated_at = ?
            WHERE id = ? AND user_id = ?
        "#;

        self.pool
            .as_ref()
            .exec(
                sql,
                vec![
                    value!(updated_message_db.conversation_id.clone()),
                    value!(updated_message_db.user_id.clone()),
                    value!(updated_message_db.message_type.clone()),
                    value!(updated_message_db.status.clone()),
                    value!(updated_message_db.provider_message_id.clone()),
                    value!(updated_message_db.provider_status.clone()),
                    value!(updated_message_db.provider_status_updated_at.clone()),
                    value!(updated_message_db.provider_error_code.clone()),
                    value!(updated_message_db.provider_error_detail.clone()),
                    value!(updated_message_db.from_number.clone()),
                    value!(updated_message_db.content.clone()),
                    value!(updated_message_db.created_at.clone()),
                    value!(updated_message_db.updated_at.clone()),
                    value!(updated_message_db.id.clone()),
                    value!(updated_message_db.user_id.clone()),
                ],
            )
            .await
            .map_err(|e| e.to_repository_error())?;

        Ok(message.clone())
    }
}
