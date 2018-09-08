use atom_syndication::Feed;
use errors::QSParseError;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::hash::Hash;
use utils::NabuResult;

pub trait FeedGenerator {
    type Info: DeserializeOwned + Serialize + Default + Hash;

    const PATH: &'static str;

    fn update(info: &Self::Info) -> NabuResult<Feed>;

    fn clean_query_string(query_string: &str) -> NabuResult<Value> {
        let info = if query_string.is_empty() {
            Self::Info::default()
        } else {
            ::serde_qs::from_str::<Self::Info>(query_string).map_err(|_| QSParseError)?
        };
        Ok(::serde_json::to_value(info)?)
    }

    fn update_by_value(value: Value) -> NabuResult<Feed> {
        let info = ::serde_json::from_value::<Self::Info>(value)?;
        Self::update(&info)
    }
}
