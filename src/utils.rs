use atom_syndication::FixedDateTime;
use chrono::prelude::*;

pub type FailureError = ::failure::Error;
pub type NabuResult<T> = ::std::result::Result<T, FailureError>;

pub const ATOM_MIME: &str = "application/atom+xml";
pub const TEXT_PLAIN_UTF_8: &str = "text/plain; charset=utf-8";

pub fn now() -> FixedDateTime {
    Utc::now().with_timezone(&offset())
}

pub fn offset() -> FixedOffset {
    // TODO should be configurable
    FixedOffset::east(8 * 3600)
}
