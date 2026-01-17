use rbatis::rbdc::DateTime;
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
