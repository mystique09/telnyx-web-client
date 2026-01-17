use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::database::models::{
    RdbcUuidExt, UuidExt, datetime_to_offset_datetime, offset_datetime_to_datetime,
};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hash: String,
    pub salt: String,
    pub email_verified: bool,
    pub email_verified_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

rbatis::crud!(User {}, "users");
rbatis::impl_select_page!(User{ list_users() => ""}, "users");

impl From<&User> for domain::models::user::User {
    fn from(value: &User) -> Self {
        Self::builder()
            .id(value.id.into_domain())
            .email(value.email.to_owned())
            .hash(value.hash.to_owned())
            .salt(value.salt.to_owned())
            .email_verified(value.email_verified)
            .maybe_email_verified_at(
                value
                    .email_verified_at
                    .to_owned()
                    .map(|dt| datetime_to_offset_datetime(dt)),
            )
            .created_at(datetime_to_offset_datetime(value.created_at.to_owned()))
            .updated_at(datetime_to_offset_datetime(value.updated_at.to_owned()))
            .build()
    }
}

impl From<&domain::models::user::User> for User {
    fn from(value: &domain::models::user::User) -> Self {
        Self::builder()
            .id(value.id.into_db())
            .email(value.email.to_owned())
            .hash(value.hash.to_owned())
            .salt(value.salt.to_owned())
            .email_verified(value.email_verified)
            .maybe_email_verified_at(
                value
                    .email_verified_at
                    .map(|dt| offset_datetime_to_datetime(dt)),
            )
            .created_at(offset_datetime_to_datetime(value.created_at))
            .updated_at(offset_datetime_to_datetime(value.updated_at))
            .build()
    }
}
