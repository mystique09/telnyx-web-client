use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::database::models::{
    RdbcUuidExt, UuidExt, datetime_to_offset_datetime, offset_datetime_to_datetime,
};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct ResetPassword {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub consumed: bool,
    pub consumed_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

rbatis::crud!(ResetPassword {}, "reset_passwords");

impl From<&ResetPassword> for domain::models::reset_password::ResetPassword {
    fn from(value: &ResetPassword) -> Self {
        Self::builder()
            .id(value.id.into_domain())
            .user_id(value.user_id.into_domain())
            .token(value.token.to_owned())
            .consumed(value.consumed)
            .maybe_consumed_at(
                value
                    .consumed_at
                    .to_owned()
                    .map(|dt| datetime_to_offset_datetime(dt)),
            )
            .created_at(datetime_to_offset_datetime(value.created_at.to_owned()))
            .updated_at(datetime_to_offset_datetime(value.updated_at.to_owned()))
            .build()
    }
}

impl From<&domain::models::reset_password::ResetPassword> for ResetPassword {
    fn from(value: &domain::models::reset_password::ResetPassword) -> Self {
        Self::builder()
            .id(value.id.into_db())
            .user_id(value.user_id.into_db())
            .token(value.token.to_owned())
            .consumed(value.consumed)
            .maybe_consumed_at(value.consumed_at.map(|dt| offset_datetime_to_datetime(dt)))
            .created_at(offset_datetime_to_datetime(value.created_at))
            .updated_at(offset_datetime_to_datetime(value.updated_at))
            .build()
    }
}
