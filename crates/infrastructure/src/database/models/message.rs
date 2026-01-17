use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::database::models::{
    RdbcUuidExt, UuidExt, datetime_to_offset_datetime, offset_datetime_to_datetime,
};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

rbatis::crud!(Message {}, "messages");

impl From<&Message> for domain::models::message::Message {
    fn from(value: &Message) -> Self {
        Self::builder()
            .id(value.id.into_domain())
            .user_id(value.user_id.into_domain())
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
            .user_id(value.user_id.into_db())
            .content(value.content.to_owned())
            .created_at(offset_datetime_to_datetime(value.created_at))
            .updated_at(offset_datetime_to_datetime(value.updated_at))
            .build()
    }
}
