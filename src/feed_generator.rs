use atom_syndication::Feed;
use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;
use utils::NabuResult;

pub trait FeedGenerator {
    type Info: DeserializeOwned + Serialize + Default + Hash;

    const PATH: &'static str;

    fn update(info: &Self::Info) -> NabuResult<Feed>;
}
