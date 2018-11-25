use crate::config::offset;
use atom_syndication::FixedDateTime;
use chrono::prelude::*;
use serde::{Deserialize, Deserializer};

pub type FailureError = ::failure::Error;
pub type NabuResult<T> = ::std::result::Result<T, FailureError>;

pub const ATOM_MIME: &str = "application/atom+xml";
pub const TEXT_PLAIN_UTF_8: &str = "text/plain; charset=utf-8";

pub fn now() -> FixedDateTime {
    Utc::now().with_timezone(&offset())
}

pub fn millis_to_datetime<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<FixedDateTime, D::Error> {
    let timestamp = <i64 as Deserialize>::deserialize(deserializer)?;
    let result = Utc.timestamp_millis(timestamp).with_timezone(&offset());
    Ok(result)
}

pub fn secord_to_datetime<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<FixedDateTime, D::Error> {
    let timestamp = <i64 as Deserialize>::deserialize(deserializer)?;
    let result = Utc
        .timestamp_millis(timestamp * 1000)
        .with_timezone(&offset());
    Ok(result)
}
