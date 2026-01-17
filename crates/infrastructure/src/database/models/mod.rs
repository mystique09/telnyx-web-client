use std::str::FromStr;

use rbatis::rbdc::{DateTime, Uuid};
use time::OffsetDateTime;

pub mod message;
pub mod phone_number;
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
