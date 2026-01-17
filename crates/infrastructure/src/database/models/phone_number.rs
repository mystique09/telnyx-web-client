use rbatis::rbdc::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::database::models::{
    RdbcUuidExt, UuidExt, datetime_to_offset_datetime, offset_datetime_to_datetime,
};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

rbatis::crud!(PhoneNumber {}, "phone_numbers");

impl From<&PhoneNumber> for domain::models::phone_number::PhoneNumber {
    fn from(value: &PhoneNumber) -> Self {
        Self::builder()
            .id(value.id.into_domain())
            .name(value.name.to_owned())
            .phone(value.phone.to_owned())
            .created_at(datetime_to_offset_datetime(value.created_at.to_owned()))
            .updated_at(datetime_to_offset_datetime(value.updated_at.to_owned()))
            .build()
    }
}

impl From<&domain::models::phone_number::PhoneNumber> for PhoneNumber {
    fn from(value: &domain::models::phone_number::PhoneNumber) -> Self {
        Self::builder()
            .id(value.id.into_db())
            .name(value.name.to_owned())
            .phone(value.phone.to_owned())
            .created_at(offset_datetime_to_datetime(value.created_at))
            .updated_at(offset_datetime_to_datetime(value.updated_at))
            .build()
    }
}
