use std::str::FromStr;

use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::{DateTime, Uuid};
use serde::Deserialize;
use time::OffsetDateTime;

pub mod conversation;
pub mod message;
pub mod phone_number;
pub mod processed_webhook_event;
pub mod reset_password;
pub mod user;

pub(crate) fn datetime_to_offset_datetime(dt: DateTime) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp_nanos(dt.unix_timestamp_nano())
        .unwrap_or(OffsetDateTime::UNIX_EPOCH)
}

pub(crate) fn offset_datetime_to_datetime(dt: OffsetDateTime) -> DateTime {
    DateTime::from_timestamp_nano(dt.unix_timestamp_nanos())
}

pub fn uuid_now() -> Uuid {
    let uuid_v7 = uuid::Uuid::now_v7();
    let id = Uuid(uuid_v7.to_string());

    id
}

pub trait RdbcUuidExt {
    fn into_domain(&self) -> uuid::Uuid;
}

impl RdbcUuidExt for Uuid {
    fn into_domain(&self) -> uuid::Uuid {
        let id = uuid::Uuid::from_str(&self.0).unwrap(); // :shrug:

        id
    }
}

pub trait UuidExt {
    fn into_db(&self) -> rbatis::rbdc::Uuid;
}

impl UuidExt for uuid::Uuid {
    fn into_db(&self) -> rbatis::rbdc::Uuid {
        let uuid_str = self.to_string();
        let uuid = rbatis::rbdc::Uuid::from_str(&uuid_str).unwrap();
        uuid
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct MessageCursorRow {
    pub id: Uuid,
    pub created_at: DateTime,
}

pub(crate) struct MessageSql;

impl MessageSql {
    #[rbatis::py_sql(
        "
        SELECT id, created_at
        FROM messages
        WHERE id = #{cursor_id} AND conversation_id = #{conversation_id} AND user_id = #{user_id}
        LIMIT 1
        "
    )]
    pub async fn select_cursor_row(
        rb: &dyn Executor,
        cursor_id: Uuid,
        conversation_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<MessageCursorRow>, rbatis::Error> {
    }

    #[rbatis::py_sql(
        "
        SELECT *
        FROM messages
        WHERE conversation_id = #{conversation_id} AND user_id = #{user_id}
        if cursor_created_at != null:
          AND (created_at, id) < (#{cursor_created_at}, #{cursor_id})
        ORDER BY created_at DESC, id DESC
        LIMIT #{limit}
        "
    )]
    pub async fn select_message_page(
        rb: &dyn Executor,
        conversation_id: Uuid,
        user_id: Uuid,
        cursor_created_at: Option<DateTime>,
        cursor_id: Option<Uuid>,
        limit: i64,
    ) -> Result<Vec<message::Message>, rbatis::Error> {
    }

    #[rbatis::py_sql(
        "
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
            #{record.id},
            #{record.conversation_id},
            #{record.user_id},
            CAST(#{record.message_type} AS message_type),
            CAST(#{record.status} AS message_status),
            #{record.provider_message_id},
            #{record.provider_status},
            #{record.provider_status_updated_at},
            #{record.provider_error_code},
            #{record.provider_error_detail},
            #{record.from_number},
            #{record.content},
            #{record.created_at},
            #{record.updated_at}
        )
        "
    )]
    pub async fn insert_message(
        rb: &dyn Executor,
        record: &message::Message,
    ) -> Result<ExecResult, rbatis::Error> {
    }

    #[rbatis::py_sql(
        "
        UPDATE messages
        SET
            conversation_id = #{record.conversation_id},
            user_id = #{record.user_id},
            message_type = CAST(#{record.message_type} AS message_type),
            status = CAST(#{record.status} AS message_status),
            provider_message_id = #{record.provider_message_id},
            provider_status = #{record.provider_status},
            provider_status_updated_at = #{record.provider_status_updated_at},
            provider_error_code = #{record.provider_error_code},
            provider_error_detail = #{record.provider_error_detail},
            from_number = #{record.from_number},
            content = #{record.content},
            created_at = #{record.created_at},
            updated_at = #{record.updated_at}
        WHERE id = #{record.id} AND user_id = #{record.user_id}
        "
    )]
    pub async fn update_message(
        rb: &dyn Executor,
        record: &message::Message,
    ) -> Result<ExecResult, rbatis::Error> {
    }
}
