use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::database::models::{
    RdbcUuidExt, UuidExt, datetime_to_offset_datetime, offset_datetime_to_datetime,
};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub phone_number_id: Uuid,
    pub user_id: Uuid,
    pub last_message_at: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

rbatis::crud!(Conversation {}, "conversations");

impl From<&Conversation> for domain::models::conversation::Conversation {
    fn from(value: &Conversation) -> Self {
        Self::builder()
            .id(value.id.into_domain())
            .phone_number_id(value.phone_number_id.into_domain())
            .user_id(value.user_id.into_domain())
            .last_message_at(datetime_to_offset_datetime(
                value.last_message_at.to_owned(),
            ))
            .created_at(datetime_to_offset_datetime(value.created_at.to_owned()))
            .updated_at(datetime_to_offset_datetime(value.updated_at.to_owned()))
            .build()
    }
}

impl From<&domain::models::conversation::Conversation> for Conversation {
    fn from(value: &domain::models::conversation::Conversation) -> Self {
        Self::builder()
            .id(value.id.into_db())
            .phone_number_id(value.phone_number_id.into_db())
            .user_id(value.user_id.into_db())
            .last_message_at(offset_datetime_to_datetime(value.last_message_at))
            .created_at(offset_datetime_to_datetime(value.created_at))
            .updated_at(offset_datetime_to_datetime(value.updated_at))
            .build()
    }
}
