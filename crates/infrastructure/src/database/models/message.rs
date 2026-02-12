use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::database::models::{
    RdbcUuidExt, UuidExt, datetime_to_offset_datetime, offset_datetime_to_datetime,
};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub message_type: String,
    pub status: String,
    pub from_number: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

rbatis::crud!(Message {}, "messages");

impl From<&Message> for domain::models::message::Message {
    fn from(value: &Message) -> Self {
        Self::builder()
            .id(value.id.into_domain())
            .conversation_id(value.conversation_id.into_domain())
            .user_id(value.user_id.into_domain())
            .message_type(message_type_from_db(&value.message_type))
            .status(message_status_from_db(&value.status))
            .from_number(value.from_number.to_owned())
            .content(value.content.to_owned())
            .created_at(datetime_to_offset_datetime(value.created_at.to_owned()))
            .updated_at(datetime_to_offset_datetime(value.updated_at.to_owned()))
            .build()
    }
}

impl From<&domain::models::message::Message> for Message {
    fn from(value: &domain::models::message::Message) -> Self {
        Self::builder()
            .id(value.id.into_db())
            .conversation_id(value.conversation_id.into_db())
            .user_id(value.user_id.into_db())
            .message_type(message_type_to_db(value.message_type).to_owned())
            .status(message_status_to_db(value.status).to_owned())
            .from_number(value.from_number.to_owned())
            .content(value.content.to_owned())
            .created_at(offset_datetime_to_datetime(value.created_at))
            .updated_at(offset_datetime_to_datetime(value.updated_at))
            .build()
    }
}

fn message_type_from_db(value: &str) -> domain::models::message::MessageType {
    match value {
        "INBOUND" => domain::models::message::MessageType::Inbound,
        "OUTBOUND" => domain::models::message::MessageType::Outbound,
        _ => domain::models::message::MessageType::Outbound,
    }
}

fn message_type_to_db(value: domain::models::message::MessageType) -> &'static str {
    match value {
        domain::models::message::MessageType::Inbound => "INBOUND",
        domain::models::message::MessageType::Outbound => "OUTBOUND",
    }
}

fn message_status_from_db(value: &str) -> domain::models::message::MessageStatus {
    match value {
        "pending" => domain::models::message::MessageStatus::Pending,
        "delivered" => domain::models::message::MessageStatus::Delivered,
        "failed" => domain::models::message::MessageStatus::Failed,
        "sent" => domain::models::message::MessageStatus::Sent,
        _ => domain::models::message::MessageStatus::Pending,
    }
}

fn message_status_to_db(value: domain::models::message::MessageStatus) -> &'static str {
    match value {
        domain::models::message::MessageStatus::Pending => "pending",
        domain::models::message::MessageStatus::Delivered => "delivered",
        domain::models::message::MessageStatus::Failed => "failed",
        domain::models::message::MessageStatus::Sent => "sent",
    }
}
